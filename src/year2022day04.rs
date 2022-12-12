use std::collections::HashSet;

fn elf_assignments(input: String) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut pairs = Vec::new();
    for cap in re.captures_iter(&input) {
        let cap = cap
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut assignments = (HashSet::new(), HashSet::new());
        tracing::debug!("captured input: {:?}", cap);
        assignments.0.extend(cap[0]..cap[1] + 1);
        assignments.1.extend(cap[2]..cap[3] + 1);
        pairs.push(assignments);
    }
    pairs
}

pub fn part1(input: &str) {
    let mut ans = 0;
    for assignments in elf_assignments(input.to_string()) {
        if assignments.0.is_superset(&assignments.1) || assignments.0.is_subset(&assignments.1) {
            tracing::debug!("total overlap: {assignments:?}");
            ans += 1;
        }
    }
    tracing::info!("Part 1 Answer: {ans}");
}
pub fn part2(input: &str) {
    let mut ans = 0;
    for assignments in elf_assignments(input.to_string()) {
        if !assignments
            .0
            .intersection(&assignments.1)
            .collect::<HashSet<_>>()
            .is_empty()
        {
            {
                tracing::debug!("overlapping: {assignments:?}");
                ans += 1;
            }
        }
    }
    tracing::info!("Part 2 Answer: {ans}");
}
