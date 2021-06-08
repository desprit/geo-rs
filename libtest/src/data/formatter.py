"""
Read `cities` file and format it so that for each state cities
are ordered from longest one to shortest, e.g:
    - AB:city_name_xxx
    - AB:city_name_yy
    - AB:city_name_z
"""

with open("./US/cities.txt") as f:
    content = f.readlines()
    content = [c.strip() for c in content]

states = {}
for line in content:
    code, name = line.split(";")
    states.setdefault(code, [])
    states[code].append(name)

with open("./tmp.txt", "w") as w:
    for k, v in states.items():
        v = sorted(v, key=lambda x: len(x), reverse=True)
        for city in v:
            w.write(f"{k};{city}\n")
