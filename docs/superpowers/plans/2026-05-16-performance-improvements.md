# geo-rs Performance Improvements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Maximize parse throughput via pre-computed data structures, Aho-Corasick multi-pattern matching, and reduced per-parse allocations.

**Architecture:** Three sequential phases — Phase A pre-computes data that is currently rebuilt on every parse call; Phase B replaces O(n_patterns × input_len) loops with single-pass Aho-Corasick automata; Phase C shrinks heap allocation count per call with SmallVec and Cow<str>. All 30 existing tests must pass after every task.

**Tech Stack:** Rust 2021, `aho-corasick = "1"`, `smallvec = "1"` (both new deps), existing `std::collections::{HashMap, HashSet}`, `std::borrow::Cow`.

---

## File Map

| File | Changes |
|------|---------|
| `lib/Cargo.toml` | Add `aho-corasick = "1"` and `smallvec = "1"` |
| `lib/src/nodes/city.rs` | Add `city_names_set: HashSet<String>` to `CitiesMap`; update `Default` + `read_cities`; update `benchmark_fill_city` |
| `lib/src/nodes/state.rs` | Add `code_to_name_lower`, `name_lower_set`, `name_ac`, `name_patterns` to `StatesMap`; update `read_states`; rewrite first loop in `fill_state`; update `benchmark_fill_state` |
| `lib/src/nodes/country.rs` | Add `name_ac`, `name_patterns` to `CountriesMap`; update `read_countries`; rewrite name loop in `fill_country`; update `benchmark_fill_country` |
| `lib/src/nodes/zipcode.rs` | Fix `benchmark_fill_zipcode` only |
| `lib/src/lib.rs` | Fix `benchmark_parse_location` only |

---

## Task 1: Fix Benchmark Overhead (Phase A1)

All five benchmark functions call `mocks::get_mocks()` inside the timed loop, rebuilding ~100 HashMap entries on every outer iteration. Move calls outside `Instant::now()`.

**Files:**
- Modify: `lib/src/lib.rs:216-230`
- Modify: `lib/src/nodes/state.rs:559-582`
- Modify: `lib/src/nodes/city.rs:354-378`
- Modify: `lib/src/nodes/zipcode.rs:244-267`
- Modify: `lib/src/nodes/country.rs:336-360`

- [ ] **Step 1: Update `benchmark_parse_location` in `lib/src/lib.rs`**

Replace the body of `benchmark_parse_location` (lines 217–230):

```rust
fn benchmark_parse_location() {
    let n = 250;
    let parser = Parser::new();
    let mocks = mocks::get_mocks();
    let before = std::time::Instant::now();
    for _ in 0..n {
        for input in mocks.keys() {
            parser.parse_location(input);
        }
    }
    println!(
        "Elapsed time: {:.2?}, {:.2?} each",
        before.elapsed(),
        before.elapsed() / (n * mocks.len() as u32)
    );
}
```

- [ ] **Step 2: Update `benchmark_fill_state` in `lib/src/nodes/state.rs`**

Replace the body of `benchmark_fill_state` (lines 560–582):

```rust
fn benchmark_fill_state() {
    let n = 250;
    let parser = Parser::new();
    let mocks = mocks::get_mocks();
    let before = std::time::Instant::now();
    for _ in 0..n {
        for input in mocks.keys() {
            let mut location = Location {
                city: None,
                state: None,
                country: None,
                zipcode: None,
                address: None,
            };
            parser.fill_state(&mut location, &input);
        }
    }
    println!(
        "Elapsed time: {:.2?}, {:.2?} each",
        before.elapsed(),
        before.elapsed() / (n * mocks.len() as u32)
    );
}
```

- [ ] **Step 3: Update `benchmark_fill_city` in `lib/src/nodes/city.rs`**

Replace the body of `benchmark_fill_city` (lines 355–378):

```rust
fn benchmark_fill_city() {
    let n = 250;
    let parser = Parser::new();
    let mocks = mocks::get_mocks();
    let before = std::time::Instant::now();
    for _ in 0..n {
        for (input, output) in &mocks {
            let mut location = Location {
                city: None,
                state: output.1.clone(),
                country: output.2.clone(),
                zipcode: None,
                address: None,
            };
            parser.fill_city(&mut location, input);
        }
    }
    println!(
        "Elapsed time: {:.2?}, {:.2?} each",
        before.elapsed(),
        before.elapsed() / (n * mocks.len() as u32)
    );
}
```

- [ ] **Step 4: Update `benchmark_fill_zipcode` in `lib/src/nodes/zipcode.rs`**

Replace the body of `benchmark_fill_zipcode` (lines 245–267):

```rust
fn benchmark_fill_zipcode() {
    let n = 250;
    let parser = Parser::new();
    let mocks = mocks::get_mocks();
    let before = std::time::Instant::now();
    for _ in 0..n {
        for zipcode in mocks.keys() {
            let mut location = Location {
                city: None,
                state: None,
                country: None,
                zipcode: None,
                address: None,
            };
            parser.fill_zipcode(&mut location, &zipcode);
        }
    }
    println!(
        "Elapsed time: {:.2?}, {:.2?} each",
        before.elapsed(),
        before.elapsed() / (n * mocks.len() as u32)
    );
}
```

- [ ] **Step 5: Update `benchmark_fill_country` in `lib/src/nodes/country.rs`**

Replace the body of `benchmark_fill_country` (lines 337–360):

```rust
fn benchmark_fill_country() {
    let n = 250;
    let parser = Parser::new();
    let mocks = mocks::get_mocks();
    let before = std::time::Instant::now();
    for _ in 0..n {
        for country in mocks.keys() {
            let mut location = Location {
                city: None,
                state: None,
                country: None,
                zipcode: None,
                address: None,
            };
            parser.fill_country(&mut location, &country);
        }
    }
    println!(
        "Elapsed time: {:.2?}, {:.2?} each",
        before.elapsed(),
        before.elapsed() / (n * mocks.len() as u32)
    );
}
```

- [ ] **Step 6: Verify all tests pass**

```
cargo test
```

Expected: all 30 tests pass, 4 ignored.

- [ ] **Step 7: Record baseline benchmark numbers**

```
cargo test benchmark_parse_location -- --nocapture --ignored
cargo test benchmark_fill_state -- --nocapture --ignored
cargo test benchmark_fill_city -- --nocapture --ignored
```

Record the "each" timing from each. These are the baseline numbers Phase A will improve.

- [ ] **Step 8: Commit**

```bash
git add lib/src/lib.rs lib/src/nodes/state.rs lib/src/nodes/city.rs lib/src/nodes/zipcode.rs lib/src/nodes/country.rs
git commit -m "perf: fix benchmark measurement overhead — move get_mocks() before Instant::now()"
```

---

## Task 2: Add `city_names_set` to `CitiesMap` (Phase A2)

`fill_state` currently rebuilds `Vec<_>` of all 36,908 city names on every parse call (`.values().flatten().collect()`) just to answer "is this state name also a city?" and does O(n) `contains` per state name. Replace with an O(1) `HashSet` built once at startup.

**Files:**
- Modify: `lib/src/nodes/city.rs`
- Modify: `lib/src/nodes/state.rs`

- [ ] **Step 1: Write the failing test in `city.rs`**

Add to the `tests` module in `lib/src/nodes/city.rs`:

```rust
#[test]
fn test_city_names_set_populated() {
    let cities = read_cities();
    let us = cities.get("US").unwrap();
    assert!(us.city_names_set.contains("new york"), "city_names_set should contain lowercase city names");
    assert!(us.city_names_set.contains("los angeles"));
    let ca = cities.get("CA").unwrap();
    assert!(ca.city_names_set.contains("toronto"));
}
```

- [ ] **Step 2: Run the test — expect compile error**

```
cargo test test_city_names_set_populated
```

Expected: compile error — `no field city_names_set on type CitiesMap`.

- [ ] **Step 3: Add `HashSet` import to `city.rs`**

At the top of `lib/src/nodes/city.rs`, change:

```rust
use std::collections::HashMap;
```

to:

```rust
use std::collections::{HashMap, HashSet};
```

- [ ] **Step 4: Add `city_names_set` field to `CitiesMap` struct**

In `lib/src/nodes/city.rs`, update the `CitiesMap` struct (around line 208):

```rust
#[derive(Debug)]
pub struct CitiesMap {
    pub cities_by_state: HashMap<String, Vec<String>>,
    // Maps city_name (lowercase) → all state codes that contain it.
    pub city_to_states: HashMap<String, Vec<String>>,
    // All city names (lowercase) for O(1) membership check.
    pub city_names_set: HashSet<String>,
}
```

- [ ] **Step 5: Update the `Default` impl for `CitiesMap`**

```rust
impl Default for CitiesMap {
    fn default() -> Self {
        CitiesMap {
            cities_by_state: HashMap::new(),
            city_to_states: HashMap::new(),
            city_names_set: HashSet::new(),
        }
    }
}
```

- [ ] **Step 6: Build `city_names_set` in `read_cities`**

In `lib/src/nodes/city.rs`, update `read_cities` to populate the new field. The full updated function:

```rust
pub fn read_cities() -> HashMap<String, CitiesMap> {
    static US_CITIES: &str = include_str!("../data/US/cities.txt");
    static CA_CITIES: &str = include_str!("../data/CA/cities.txt");

    let mut data: HashMap<String, CitiesMap> = HashMap::new();
    for (country, content) in [("US", US_CITIES), ("CA", CA_CITIES)] {
        let mut cities_by_state: HashMap<String, Vec<String>> = HashMap::new();
        let mut city_to_states: HashMap<String, Vec<String>> = HashMap::new();
        let mut city_names_set: HashSet<String> = HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 2 || parts[1].len() <= 3 { continue; }
            let state_code = parts[0].to_string();
            let city_lower = parts[1].to_lowercase();
            cities_by_state
                .entry(state_code.clone())
                .or_default()
                .push(city_lower.clone());
            city_to_states
                .entry(city_lower.clone())
                .or_default()
                .push(state_code);
            city_names_set.insert(city_lower);
        }
        data.insert(country.to_string(), CitiesMap { cities_by_state, city_to_states, city_names_set });
    }
    data
}
```

- [ ] **Step 7: Run the test — expect pass**

```
cargo test test_city_names_set_populated
```

Expected: PASS.

- [ ] **Step 8: Use `city_names_set` in `fill_state` + fix A4 redundant alloc**

In `lib/src/nodes/state.rs`, inside `fill_state`, replace the first `for c in &countries` block (the "Search by a full match" block, ~lines 68–95) with the following. Two changes happen simultaneously: (a) remove the `city_names` Vec rebuild, (b) fix `as_lowercase` allocation.

First, at the top of `fill_state`, change:

```rust
let as_lowercase = input.to_lowercase().to_string();
```

to:

```rust
let as_lowercase = input.to_lowercase();
```

Then replace the entire "Search by a full match of input and state name" block:

```rust
// Search by a full match of input and state name
for c in &countries {
    let default = CitiesMap::default();
    let country_cities = self.cities.get(&c.code).unwrap_or(&default);
    if let Some(states) = self.states.get(&c.code) {
        for (code, name) in &states.code_to_name {
            if country_cities.city_names_set.contains(&name.to_lowercase()) {
                continue;
            }
            if as_lowercase.contains(&name.to_lowercase()) {
                location.state = Some(State {
                    code: code.clone(),
                    name: name.clone(),
                });
                if location.country.is_none() {
                    location.country = Some(c.clone());
                }
                return;
            }
        }
    }
}
```

(The old block computed `city_names` as a Vec and called `city_names.contains()` — delete all of that.)

- [ ] **Step 9: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 10: Commit**

```bash
git add lib/src/nodes/city.rs lib/src/nodes/state.rs
git commit -m "perf(A2+A4): add city_names_set HashSet to CitiesMap; O(1) membership check in fill_state"
```

---

## Task 3: Add Pre-lowercased Maps to `StatesMap` (Phase A3)

`fill_state`'s second (candidate) loop calls `s.to_lowercase()` on each word of each state name when checking input parts. Add `code_to_name_lower` (uppercase code → lowercase name) and `name_lower_set` (all lowercase state names) to `StatesMap` so these allocations happen once at startup. Also fix the `part.to_string()` unnecessary allocation in the code-match branch.

**Files:**
- Modify: `lib/src/nodes/state.rs`

- [ ] **Step 1: Write the failing test**

Add to the `tests` module in `lib/src/nodes/state.rs`:

```rust
#[test]
fn test_states_map_lower_fields() {
    let states = read_states();
    let us = states.get("US").unwrap();
    assert_eq!(us.code_to_name_lower.get("CA"), Some(&"california".to_string()));
    assert_eq!(us.code_to_name_lower.get("NY"), Some(&"new york".to_string()));
    assert!(us.name_lower_set.contains("california"));
    assert!(us.name_lower_set.contains("new york"));
    let ca = states.get("CA").unwrap();
    assert_eq!(ca.code_to_name_lower.get("ON"), Some(&"ontario".to_string()));
    assert!(ca.name_lower_set.contains("ontario"));
}
```

- [ ] **Step 2: Run the test — expect compile error**

```
cargo test test_states_map_lower_fields
```

Expected: compile error — fields `code_to_name_lower` and `name_lower_set` do not exist.

- [ ] **Step 3: Add `HashSet` import to `state.rs`**

At the top of `lib/src/nodes/state.rs`, change:

```rust
use std::collections::HashMap;
```

to:

```rust
use std::collections::{HashMap, HashSet};
```

- [ ] **Step 4: Update `StatesMap` struct**

```rust
#[derive(Debug)]
pub struct StatesMap {
    pub code_to_name: HashMap<String, String>,
    pub name_to_code: HashMap<String, String>,
    // key: uppercase state code (e.g. "CA"), value: lowercase state name (e.g. "california")
    pub code_to_name_lower: HashMap<String, String>,
    // all lowercase state names, for O(1) country disambiguation
    pub name_lower_set: HashSet<String>,
}
```

- [ ] **Step 5: Build new fields in `read_states`**

Replace the full `read_states` function:

```rust
pub fn read_states() -> HashMap<String, StatesMap> {
    static US_STATES: &str = include_str!("../data/US/states.txt");
    static CA_STATES: &str = include_str!("../data/CA/states.txt");

    let mut data: HashMap<String, StatesMap> = HashMap::new();
    for (country, content) in [("US", US_STATES), ("CA", CA_STATES)] {
        let mut name_to_code: HashMap<String, String> = HashMap::new();
        let mut code_to_name: HashMap<String, String> = HashMap::new();
        let mut code_to_name_lower: HashMap<String, String> = HashMap::new();
        let mut name_lower_set: HashSet<String> = HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 2 { continue; }
            let code = parts[0].to_string();
            let name = parts[1].to_string();
            let name_lower = name.to_lowercase();
            code_to_name_lower.insert(code.clone(), name_lower.clone());
            name_lower_set.insert(name_lower);
            name_to_code.insert(name, code.clone());
            code_to_name.insert(code, parts[1].to_string());
        }
        data.insert(country.to_string(), StatesMap {
            name_to_code, code_to_name, code_to_name_lower, name_lower_set,
        });
    }
    data
}
```

- [ ] **Step 6: Use `code_to_name_lower` in `fill_state` candidate name-parts loop**

In `fill_state`, locate the second loop (around line 99–120, "Search by input containing state code or state name"). Replace the name-parts matching inside the loop:

Old code:
```rust
if name.split_whitespace().all(|s| {
    return parts_lowercase.contains(&s.to_lowercase().as_str());
}) {
    let state = State {
        code: code.clone(),
        name: name.clone(),
    };
    candidates.push((state, c.clone()));
}
```

New code (uses pre-lowercased name to avoid per-word `.to_lowercase()` allocation):
```rust
if let Some(name_lower) = states.code_to_name_lower.get(code.as_str()) {
    if name_lower.split_whitespace().all(|s| parts_lowercase.contains(&s)) {
        let state = State {
            code: code.clone(),
            name: name.clone(),
        };
        candidates.push((state, c.clone()));
    }
}
```

Also fix the code-match branch in the same loop. Old:
```rust
if code == &part.to_string() {
```

New (avoids `part.to_string()` allocation):
```rust
if code.as_str() == *part {
```

- [ ] **Step 7: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 8: Run Phase A benchmark and record numbers**

```
cargo test benchmark_parse_location -- --nocapture --ignored
cargo test benchmark_fill_state -- --nocapture --ignored
```

Record the "each" timing. Compare against Task 1 baseline.

- [ ] **Step 9: Commit**

```bash
git add lib/src/nodes/state.rs
git commit -m "perf(A3): add code_to_name_lower and name_lower_set to StatesMap; avoid per-parse name allocations"
```

---

## Task 4: Aho-Corasick for State Name Matching (Phase B1)

The first loop in `fill_state` iterates all state names (51 US + 13 CA) and calls `as_lowercase.contains(name_lower)` for each — O(n_states × input_len) per parse. Replace with a single Aho-Corasick pass: O(input_len) finds all matches simultaneously.

**Files:**
- Modify: `lib/Cargo.toml`
- Modify: `lib/src/nodes/state.rs`

- [ ] **Step 1: Add `aho-corasick` dependency**

In `lib/Cargo.toml`, add to `[dependencies]`:

```toml
aho-corasick = "1"
```

- [ ] **Step 2: Write the failing test**

Add to the `tests` module in `lib/src/nodes/state.rs`:

```rust
#[test]
fn test_states_map_has_ac() {
    let states = read_states();
    let us = states.get("US").unwrap();
    // AC should find "california" in a test string
    let mut found = false;
    for mat in us.name_ac.find_iter("location in california somewhere") {
        let (code, _) = &us.name_patterns[mat.pattern().as_usize()];
        if code == "CA" { found = true; }
    }
    assert!(found, "AC should find California in the input string");
}
```

- [ ] **Step 3: Run the test — expect compile error**

```
cargo test test_states_map_has_ac
```

Expected: compile error — `name_ac` and `name_patterns` fields do not exist.

- [ ] **Step 4: Add `aho-corasick` import to `state.rs`**

At the top of `lib/src/nodes/state.rs`:

```rust
use aho_corasick::AhoCorasick;
```

- [ ] **Step 5: Add `name_ac` and `name_patterns` to `StatesMap`**

```rust
#[derive(Debug)]
pub struct StatesMap {
    pub code_to_name: HashMap<String, String>,
    pub name_to_code: HashMap<String, String>,
    pub code_to_name_lower: HashMap<String, String>,
    pub name_lower_set: HashSet<String>,
    // Aho-Corasick automaton for single-pass state name detection.
    // name_patterns[i] = (uppercase_code, lowercase_name); AC pattern index i matches name_patterns[i].
    pub name_ac: AhoCorasick,
    pub name_patterns: Vec<(String, String)>,
}
```

- [ ] **Step 6: Build `name_ac` and `name_patterns` in `read_states`**

Replace the full `read_states` function:

```rust
pub fn read_states() -> HashMap<String, StatesMap> {
    static US_STATES: &str = include_str!("../data/US/states.txt");
    static CA_STATES: &str = include_str!("../data/CA/states.txt");

    let mut data: HashMap<String, StatesMap> = HashMap::new();
    for (country, content) in [("US", US_STATES), ("CA", CA_STATES)] {
        let mut name_to_code: HashMap<String, String> = HashMap::new();
        let mut code_to_name: HashMap<String, String> = HashMap::new();
        let mut code_to_name_lower: HashMap<String, String> = HashMap::new();
        let mut name_lower_set: HashSet<String> = HashSet::new();
        let mut name_patterns: Vec<(String, String)> = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 2 { continue; }
            let code = parts[0].to_string();
            let name = parts[1].to_string();
            let name_lower = name.to_lowercase();
            code_to_name_lower.insert(code.clone(), name_lower.clone());
            name_lower_set.insert(name_lower.clone());
            name_patterns.push((code.clone(), name_lower));
            name_to_code.insert(name.clone(), code.clone());
            code_to_name.insert(code, name);
        }
        let ac_patterns: Vec<&str> = name_patterns.iter().map(|(_, n)| n.as_str()).collect();
        let name_ac = AhoCorasick::new(&ac_patterns).expect("failed to build state name AC automaton");
        data.insert(country.to_string(), StatesMap {
            name_to_code, code_to_name, code_to_name_lower, name_lower_set, name_ac, name_patterns,
        });
    }
    data
}
```

- [ ] **Step 7: Replace the first loop in `fill_state` with AC**

In `fill_state`, replace the entire "Search by a full match of input and state name" block (which currently iterates `states.code_to_name`) with:

```rust
// Search by a full match of input and state name — single AC pass, O(input_len)
for c in &countries {
    let default = CitiesMap::default();
    let country_cities = self.cities.get(&c.code).unwrap_or(&default);
    if let Some(states) = self.states.get(&c.code) {
        for mat in states.name_ac.find_iter(&as_lowercase) {
            let (code, name_lower) = &states.name_patterns[mat.pattern().as_usize()];
            if country_cities.city_names_set.contains(name_lower.as_str()) {
                continue;
            }
            location.state = Some(State {
                code: code.clone(),
                name: states.code_to_name.get(code).unwrap().clone(),
            });
            if location.country.is_none() {
                location.country = Some(c.clone());
            }
            return;
        }
    }
}
```

- [ ] **Step 8: Run the failing test — expect pass**

```
cargo test test_states_map_has_ac
```

Expected: PASS.

- [ ] **Step 9: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 10: Commit**

```bash
git add lib/Cargo.toml lib/src/nodes/state.rs
git commit -m "perf(B1): Aho-Corasick for state name detection; O(input_len) replacing O(n_states*input_len)"
```

---

## Task 5: Aho-Corasick for Country Name Matching (Phase B2)

`fill_country` iterates all 249 country names calling `utils::split().contains(country_name_lower)` — O(249 × n_tokens) per parse. Replace with a single AC pass plus word-boundary verification.

**Files:**
- Modify: `lib/src/nodes/country.rs`

- [ ] **Step 1: Write the failing test**

Add to the `tests` module in `lib/src/nodes/country.rs`:

```rust
#[test]
fn test_countries_map_has_ac() {
    use geo_rs::nodes::read_countries;
    let countries = read_countries();
    let mut found_spain = false;
    for mat in countries.name_ac.find_iter("barcelona spain es") {
        let (_, name_lower) = &countries.name_patterns[mat.pattern().as_usize()];
        if name_lower == "spain" { found_spain = true; }
    }
    assert!(found_spain, "AC should find Spain in the input");
}
```

- [ ] **Step 2: Run the test — expect compile error**

```
cargo test test_countries_map_has_ac
```

Expected: compile error — `name_ac` and `name_patterns` do not exist on `CountriesMap`.

- [ ] **Step 3: Add `aho-corasick` import to `country.rs`**

At the top of `lib/src/nodes/country.rs`:

```rust
use aho_corasick::AhoCorasick;
```

- [ ] **Step 4: Update `CountriesMap` struct**

```rust
#[derive(Debug)]
pub struct CountriesMap {
    pub code_to_name: HashMap<String, String>,
    pub name_to_code: HashMap<String, String>,
    // Aho-Corasick for single-pass country name detection.
    // name_patterns[i] = (country_code, lowercase_name).
    pub name_ac: AhoCorasick,
    pub name_patterns: Vec<(String, String)>,
}
```

- [ ] **Step 5: Build AC in `read_countries`**

Replace the full `read_countries` function:

```rust
pub fn read_countries() -> CountriesMap {
    let content = include_str!("../data/countries.txt");
    let mut name_to_code: HashMap<String, String> = HashMap::new();
    let mut code_to_name: HashMap<String, String> = HashMap::new();
    let mut name_patterns: Vec<(String, String)> = Vec::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 2 { continue; }
        let name = parts[0].to_string();
        let code = parts[1].to_string();
        name_patterns.push((code.clone(), name.to_lowercase()));
        code_to_name.insert(code.clone(), name.clone());
        name_to_code.insert(name, code);
    }
    let ac_patterns: Vec<&str> = name_patterns.iter().map(|(_, n)| n.as_str()).collect();
    let name_ac = AhoCorasick::new(&ac_patterns).expect("failed to build country name AC automaton");
    CountriesMap { name_to_code, code_to_name, name_ac, name_patterns }
}
```

- [ ] **Step 6: Replace the country name loop in `fill_country` with AC**

In `lib/src/nodes/country.rs`, find the comment "Search fill country name in the input string" and replace that entire block (the `for (country_name, country_code) in self.countries.name_to_code.iter()` loop at ~lines 158–188) with:

```rust
// Search country name in the input — single AC pass with word-boundary verification
for mat in self.countries.name_ac.find_iter(&as_lowercase) {
    let (country_code, name_lower) = &self.countries.name_patterns[mat.pattern().as_usize()];
    // Verify the match falls on a word boundary (non-alphanumeric surroundings)
    let start = mat.start();
    let end = mat.end();
    let bytes = as_lowercase.as_bytes();
    let before_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
    let after_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
    if !before_ok || !after_ok {
        continue;
    }
    // Skip if the country name is also a US or CA state name
    if let Some(us_states) = self.states.get("US") {
        if us_states.name_lower_set.contains(name_lower.as_str()) {
            continue;
        }
    }
    if let Some(ca_states) = self.states.get("CA") {
        if ca_states.name_lower_set.contains(name_lower.as_str()) {
            continue;
        }
    }
    let country_name = self.countries.code_to_name.get(country_code).unwrap().clone();
    location.country = Some(Country {
        name: country_name,
        code: country_code.clone(),
    });
    return;
}
```

Note: `name_lower_set` was added to `StatesMap` in Task 3. This replaces the O(n_state_names) `find()` scan with O(1) HashSet lookup per match.

- [ ] **Step 7: Run the failing test — expect pass**

```
cargo test test_countries_map_has_ac
```

Expected: PASS.

- [ ] **Step 8: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 9: Run Phase B benchmark and record numbers**

```
cargo test benchmark_parse_location -- --nocapture --ignored
cargo test benchmark_fill_state -- --nocapture --ignored
cargo test benchmark_fill_country -- --nocapture --ignored
```

Record and compare against Phase A numbers.

- [ ] **Step 10: Commit**

```bash
git add lib/src/nodes/country.rs
git commit -m "perf(B2): Aho-Corasick for country name detection; O(input_len) replacing O(249*n_tokens)"
```

---

## Task 6: SmallVec for Candidate Lists (Phase C1)

The `candidates` and `ranged_candidates` Vec allocations in `fill_city` and `fill_state` almost always hold 0–3 elements but always heap-allocate. `SmallVec<[T; 4]>` stack-allocates for the common case.

**Files:**
- Modify: `lib/Cargo.toml`
- Modify: `lib/src/nodes/city.rs`
- Modify: `lib/src/nodes/state.rs`

- [ ] **Step 1: Add `smallvec` dependency**

In `lib/Cargo.toml`, add:

```toml
smallvec = "1"
```

- [ ] **Step 2: Add `smallvec` import to `city.rs`**

At the top of `lib/src/nodes/city.rs`:

```rust
use smallvec::SmallVec;
```

- [ ] **Step 3: Update candidate declarations in `fill_city`**

In `fill_city`, find (around line 110):

```rust
let mut candidates: Vec<(String, String)> = vec![];
```

Replace with:

```rust
let mut candidates: SmallVec<[(String, String); 4]> = SmallVec::new();
```

Find (around line 138):

```rust
let mut ranged_candidates: Vec<(String, String)> = vec![];
```

Replace with:

```rust
let mut ranged_candidates: SmallVec<[(String, String); 4]> = SmallVec::new();
```

- [ ] **Step 4: Run all tests after city.rs change**

```
cargo test
```

Expected: all 30 tests pass. `SmallVec` implements `Deref<Target=[T]>` so `.len()`, `.iter()`, `.first()`, `.is_empty()`, `.push()`, `.insert()` all work unchanged.

- [ ] **Step 5: Add `smallvec` import to `state.rs`**

At the top of `lib/src/nodes/state.rs`:

```rust
use smallvec::SmallVec;
```

- [ ] **Step 6: Update candidate declarations in `fill_state`**

In `fill_state`, find (around line 97):

```rust
let mut candidates: Vec<(State, Country)> = vec![];
```

Replace with:

```rust
let mut candidates: SmallVec<[(State, Country); 4]> = SmallVec::new();
```

Find (around line 122):

```rust
let mut candidates_deduped: Vec<(State, Country)> = vec![];
```

Replace with:

```rust
let mut candidates_deduped: SmallVec<[(State, Country); 4]> = SmallVec::new();
```

Also update `filtered_candidates` (around line 146) — it must be `SmallVec` too because the `Some(_)` arm of the match returns `candidates_deduped.clone()` (a SmallVec) and both match arms must have the same type:

```rust
let mut filtered_candidates: SmallVec<[(State, Country); 4]> = match &location.country {
    Some(_) => candidates_deduped.clone(),
    None => candidates_deduped
        .into_iter()
        .filter(|(x, _)| !country_codes.contains(&x.code))
        .collect(),
};
```

`SmallVec` implements `FromIterator` so `.collect()` works, and it implements `Clone`, `sort_by`, `len`, `first`, `is_empty` — all downstream operations compile unchanged.

- [ ] **Step 7: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 8: Commit**

```bash
git add lib/Cargo.toml lib/src/nodes/city.rs lib/src/nodes/state.rs
git commit -m "perf(C1): SmallVec for candidate lists in fill_city and fill_state; stack-alloc for ≤4 elements"
```

---

## Task 7: `Cow<str>` for Lowercased Input (Phase C2)

`fill_state` and `fill_country` unconditionally call `input.to_lowercase()` even when the input is already lowercase. Using `Cow<str>` borrows the input when it's already lowercase, avoiding the allocation.

**Files:**
- Modify: `lib/src/nodes/state.rs`
- Modify: `lib/src/nodes/country.rs`

- [ ] **Step 1: Add `Cow` import to `state.rs`**

At the top of `lib/src/nodes/state.rs`:

```rust
use std::borrow::Cow;
```

- [ ] **Step 2: Update `as_lowercase` in `fill_state`**

In `fill_state`, replace:

```rust
let as_lowercase = input.to_lowercase();
```

with:

```rust
let as_lowercase: Cow<str> = if input.bytes().any(|b| b.is_ascii_uppercase()) {
    Cow::Owned(input.to_lowercase())
} else {
    Cow::Borrowed(input)
};
```

`Cow<str>` derefs to `&str`, so all existing `.contains()`, `.find()`, `utils::split(&as_lowercase)` calls work unchanged (they accept `&str`).

- [ ] **Step 3: Run all tests after state.rs change**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 4: Add `Cow` import to `country.rs`**

At the top of `lib/src/nodes/country.rs`:

```rust
use std::borrow::Cow;
```

- [ ] **Step 5: Update `as_lowercase` in `fill_country`**

In `fill_country`, replace:

```rust
let as_lowercase = input.to_lowercase().to_string();
```

with:

```rust
let as_lowercase: Cow<str> = if input.bytes().any(|b| b.is_ascii_uppercase()) {
    Cow::Owned(input.to_lowercase())
} else {
    Cow::Borrowed(input)
};
```

Also, `as_lowercase.to_string()` appears later in the function (line 159 in the original, now replaced by AC). If any remaining code still calls `.to_string()` on `as_lowercase`, remove those redundant calls — `Cow<str>` derefs to `&str` which is sufficient for all `utils::split`, `.contains`, and `.find_iter` calls.

- [ ] **Step 6: Run all tests**

```
cargo test
```

Expected: all 30 tests pass.

- [ ] **Step 7: Run Phase C benchmark and record final numbers**

```
cargo test benchmark_parse_location -- --nocapture --ignored
cargo test benchmark_fill_state -- --nocapture --ignored
cargo test benchmark_fill_city -- --nocapture --ignored
cargo test benchmark_fill_country -- --nocapture --ignored
```

Record and compare against Phase B numbers. Summarize total improvement in the commit message.

- [ ] **Step 8: Commit**

```bash
git add lib/src/nodes/state.rs lib/src/nodes/country.rs
git commit -m "perf(C2): Cow<str> for lowercased input; borrows when already lowercase to avoid allocation"
```

---

## Completion Checklist

- [ ] All 30 existing tests pass on `master` (no regressions)
- [ ] Benchmarks recorded at baseline, after Phase A, after Phase B, after Phase C
- [ ] No changes to parsing semantics — only data structures and algorithms changed
- [ ] `cargo build` clean with zero warnings
