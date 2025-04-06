fn successors(pattern: &str, substrings: &Vec<String>, i: usize) -> Vec<usize> {
    substrings
        .into_iter()
        .filter_map(|s| {
            if pattern[i..].starts_with(s) {
                Some(i + s.len())
            } else {
                None
            }
        })
        .collect()
}

pub fn count_paths(pattern: &str, substrings: &Vec<String>) -> usize {
    pathfinding::directed::count_paths::count_paths(
        0,
        |&i| successors(pattern, substrings, i),
        |&i| i == pattern.len(),
    )
}

pub fn has_path(pattern: &str, substrings: &Vec<String>) -> bool {
    count_paths(pattern, substrings) != 0
}
