# geo-rs Performance Improvements Design

**Date:** 2026-05-16
**Goal:** Maximize throughput for high-volume batch parsing (millions of records).

---

## Context

The current parsing pipeline processes each location string in roughly 0.8ms. For batch workloads at scale this is the primary bottleneck. Profiling and code analysis identified three independent optimization layers that each deliver measurable gains with no correctness risk.

Baseline measurements (from existing benchmarks) must be recorded before any change. Each phase re-runs the same benchmarks so gains are attributable.

---

## Phase A — Pre-compute Hot Data Structures

**Goal:** Eliminate per-parse work that can be done once at startup.

### A1 — Fix benchmark measurement overhead

The existing benchmarks call `mocks::get_mocks()` inside the timed loop, which rebuilds a `HashMap<_, _>` of 100+ entries (with `String` allocations) on every outer iteration. This inflates all timing numbers and makes cross-run comparisons unreliable.

**Fix:** Move `mocks::get_mocks()` outside the `before = Instant::now()` line in every benchmark (`lib.rs`, `state.rs`, `city.rs`, `zipcode.rs`, `country.rs`).

### A2 — HashSet for city-name membership in `fill_state`

`fill_state` currently rebuilds a `Vec<String>` of all 36,908 city names on **every parse call** (via `.values().flatten().collect()`) just to answer "is this state name also a city name?" Then it does an O(n) `Vec::contains` per state name candidate.

**Fix:** Add `city_names_set: HashSet<String>` to `CitiesMap`, built once in `read_cities`. Change `fill_state`'s membership check from `cities_vec.contains(&name)` to `city_names_set.contains(&name)` — O(1) vs O(36,908).

Files: `lib/src/nodes/city.rs` (add field + build in `read_cities`), `lib/src/nodes/state.rs` (use it).

### A3 — Pre-lowercased code map in `StatesMap`

`fill_state` calls `.to_lowercase()` on every state code in the map (64 total per country) to compare against lowercased input parts. Each `.to_lowercase()` allocates a new `String`.

**Fix:** Add `code_to_name_lower: HashMap<String, String>` to `StatesMap` (keys are lowercase state codes, built once in `read_states`). Use this map instead of calling `.to_lowercase()` at match time.

Files: `lib/src/nodes/state.rs`.

### A4 — Eliminate redundant `.to_string()` after `.to_lowercase()`

`.to_lowercase()` already returns `String`, so `.to_lowercase().to_string()` allocates twice. Several call sites in `fill_country` and `fill_state` have this pattern.

**Fix:** Remove the trailing `.to_string()` calls.

Files: `lib/src/nodes/country.rs`, `lib/src/nodes/state.rs`.

---

## Phase B — Aho-Corasick for Name Matching

**Goal:** Replace O(n_patterns × input_len) substring loops with a single O(input_len) automaton pass.

### B1 — State name matching

`fill_state` iterates over all state names (51 US + 13 CA) and checks `input.contains(name)` for each. This is O(n_states × input_len). With Aho-Corasick, one pass over the input finds all matching state names simultaneously.

**Implementation:**
- Add `aho-corasick = "1"` to `lib/Cargo.toml`.
- Add `name_ac: AhoCorasick` field to `StatesMap`, built at `read_states` time from pre-lowercased state names.
- In `fill_state`: replace the `for (name, code) in name_to_code` loop with a single `name_ac.find_iter(&lowercased_input)` call. Each match maps pattern index back to `(name, code)` via a parallel `Vec<(String, String)>` stored alongside the automaton.

Files: `lib/src/nodes/state.rs`, `lib/Cargo.toml`.

### B2 — Country name matching

`fill_country` has a similar O(n_countries × input_len) loop over 249 country names.

**Implementation:**
- Add `name_ac: AhoCorasick` to `CountriesMap`, built at `read_countries` time.
- Same pattern-index-to-name mapping via parallel `Vec<(String, String)>`.
- Replace the `for (country_name, country_code) in name_to_code` loop with `name_ac.find_iter`.

Files: `lib/src/nodes/country.rs`.

---

## Phase C — Reduce Per-Parse Allocations

**Goal:** Shrink heap allocation count per parse call.

### C1 — SmallVec for candidate lists

The city and state candidate lists (`Vec<(String, String)>`) typically contain 0–3 elements but always heap-allocate. Using `SmallVec<[(String, String); 4]>` stack-allocates for the common case.

**Implementation:**
- Add `smallvec = "1"` to `lib/Cargo.toml`.
- Change candidate `Vec` declarations in `fill_city` and `fill_state` to `SmallVec<[(String, String); 4]>`.

Files: `lib/src/nodes/city.rs`, `lib/src/nodes/state.rs`.

### C2 — Cow<str> for lowercased input

Several functions lowercase the entire input string unconditionally. If the input is already lowercase, this allocates unnecessarily.

**Implementation:**
- Use `Cow<str>` — borrow the original if `.chars().all(|c| c.is_lowercase() || !c.is_alphabetic())`, otherwise allocate the lowercased copy.
- Apply to the `as_lowercase` binding in `fill_state` and `fill_country`.

Files: `lib/src/nodes/state.rs`, `lib/src/nodes/country.rs`.

---

## Data Structures Summary

| Struct | New Field | Built In | Purpose |
|--------|-----------|----------|---------|
| `CitiesMap` | `city_names_set: HashSet<String>` | `read_cities` | O(1) city-name membership check in `fill_state` |
| `StatesMap` | `code_to_name_lower: HashMap<String,String>` | `read_states` | avoid `.to_lowercase()` allocations per match |
| `StatesMap` | `name_ac: AhoCorasick` + `name_patterns: Vec<(String,String)>` | `read_states` | single-pass state name detection |
| `CountriesMap` | `name_ac: AhoCorasick` + `name_patterns: Vec<(String,String)>` | `read_countries` | single-pass country name detection |

---

## New Dependencies

| Crate | Version | Reason |
|-------|---------|--------|
| `aho-corasick` | `"1"` | Multi-pattern string matching (Phase B) |
| `smallvec` | `"1"` | Stack-allocated small vecs (Phase C) |

---

## Testing & Correctness

- All 30 existing tests must pass after every phase.
- No new correctness changes — only data structure and algorithmic substitutions.
- Each phase ends with `cargo test` (all green) before proceeding.
- Benchmarks run before Phase A (baseline), after Phase A, after Phase B, after Phase C. Numbers recorded in commit messages.

---

## Implementation Order

1. **Phase A** — no new dependencies, pure refactoring, lowest risk.
2. **Phase B** — adds Aho-Corasick; builds on Phase A's cleaner state/country structs.
3. **Phase C** — adds SmallVec + Cow; independent of B but benefits from A's structural changes.

Phases are independent enough that each can be committed and benchmarked separately.

---

## Non-Goals

- Python/FFI bindings performance (out of scope).
- Parallelism / rayon (the library is stateless and callers can parallelize externally).
- Changing parsing semantics or adding new location formats.
- Optimizing `unidecode` (external crate, not in scope).
