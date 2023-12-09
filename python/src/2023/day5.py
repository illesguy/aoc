import sys
from inputs import get_input_for_day


class Range:

    def __init__(self, start, length):
        self.start = start
        self.length = length
        self.end = start + length

    def is_in_range(self, val):
        return val >= self.start and val < self.end

    def position_in_range(self, val):
        if self.is_in_range(val):
            return val - self.start
        return None

    def value_for_position(self, pos):
        return self.start + pos

    def __repr__(self):
        return f'[{self.start}, {self.end})'

    @classmethod
    def from_start_end(cls, start, end):
        return cls(start, end - start)


class SubMapping:

    def __init__(self, start, length, dest):
        self.start = start
        self.end = start + length
        self.dest = dest
        self.source = Range(start, length)
        self.target = Range(dest, length)
    
    def apply_to_value(self, val):
        return val + (self.dest - self.start)

    def __repr__(self):
        return f'{self.source} -> {self.target}'


class Mapping:

    def __init__(self, sub_mappings):
        self.sub_mappings = sub_mappings

    def apply_to_value(self, val):
        for sm in self.sub_mappings:
            if sm.source.is_in_range(val):
                return sm.apply_to_value(val)
        return val

    def apply_to_range(self, srange):
        outputs = []

        submappings_to_apply = [sm for sm in self.sub_mappings if srange.start < sm.end and srange.end > sm.start]
        if not submappings_to_apply:
            return [srange]

        overlapped = []
        for subm in submappings_to_apply:
            overlapped.append(Range.from_start_end(subm.apply_to_value(max(subm.start, srange.start)), subm.apply_to_value(min(subm.end, srange.end))))
        outputs.extend(overlapped)

        if srange.start < min([sm.start for sm in self.sub_mappings]):
            outputs.append(Range.from_start_end(srange.start, min([sm.start for sm in self.sub_mappings])))

        if srange.end > max([sm.end for sm in self.sub_mappings]):
            outputs.append(Range.from_start_end(max([sm.end for sm in self.sub_mappings]), srange.end))
        
        return outputs

    @classmethod
    def from_input_lines(cls, lines):
        sub_mappings = []
        for line in lines.split("\n")[1:]:
            dest, start, length = [int(v) for v in line.split()]
            sub_mappings.append(SubMapping(start, length, dest))
        return cls(sub_mappings)

    def __repr__(self):
        return "\n".join([str(sm) for sm in self.sub_mappings])



def parse_seeds(seeds):
    return [int(s) for s in seeds.replace("seeds: ", "").split()]


def apply_mappings_to_value(seed, mappings):
    s = seed
    for m in mappings:
        s = m.apply_to_value(s)
    return s


def apply_mappings_to_range(seed_range, mappings):
    ranges = [seed_range]
    for m in mappings:
        ranges = [r for sr in ranges for r in m.apply_to_range(sr)]
    return ranges


def part1(seeds, mappings):
    return min([apply_mappings_to_value(s, mappings) for s in seeds])


def part2(seeds, mappings):
    seed_ranges = [Range(seeds[i], seeds[i+1]) for i in range(0, len(seeds), 2)]
    return min([fr.start for sr in seed_ranges for fr in apply_mappings_to_range(sr, mappings)])


inp = get_input_for_day(2023, 5, str, separator="\n\n")
seeds = parse_seeds(inp[0])
mappings = [Mapping.from_input_lines(l) for l in inp[1:]]


print("part 1:", part1(seeds, mappings))
print("part 2:", part2(seeds, mappings))
