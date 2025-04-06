pub fn find_path(
    grid: &pathfinding::grid::Grid,
    source: (usize, usize),
    target: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    Some(
        pathfinding::directed::dijkstra::dijkstra(
            &source,
            |&position| {
                grid.neighbours(position)
                    .into_iter()
                    .map(|position| (position, 1))
            },
            |&position| position == target,
        )?
        .0,
    )
}
