use std::collections::{HashMap, HashSet};

use utils::read_lines;

type Grid = Vec<Vec<usize>>;
type Point = (isize, isize); // (row, col)
const MOVES: &[Point] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day12pt1.txt");
    let barrels = dfs_connected_barrels(input, vec![(0, 0)]);
    println!("Part 1: {:?}", barrels.len());
}

fn part_2() {
    let input = parse("inputs/day12pt2.txt");
    let max_row = (input.len() - 1) as isize;
    let max_col = (input[0].len() - 1) as isize;
    let barrels = dfs_connected_barrels(input, vec![(0, 0), (max_row, max_col)]);
    println!("Part 2: {:?}", barrels.len());
}

fn part_3() {
    let input = parse("inputs/day12pt3.txt");

    let mut exclude : HashSet<Point> = HashSet::new();
    let mut starts: Vec<Point> = Vec::new();

    for _ in 0..3 {
        let result = greedy_check(&input, &exclude);
        starts.push(result.0);
        exclude.extend(result.1);
    }
    let barrels = dfs_connected_barrels(input, starts);
    println!("Part 3: {:?}", barrels.len());
}

fn greedy_check(grid: &Grid, exclude: &HashSet<Point>) -> (Point, HashSet<Point>) {
    let max_row = (grid.len() - 1) as isize;
    let max_col = (grid[0].len() - 1) as isize;
    let mut barrels_per_point : HashMap<Point, usize> = HashMap::new();
    let mut visited : HashSet<Point> = HashSet::new();

    let points = (0..max_row + 1)
        .flat_map(|row| {
            (0..max_col + 1)
                .map(|col| (row, col))
                .collect::<Vec<(isize, isize)>>()
        })
        .collect::<Vec<_>>();

    // Start from points with highest value, slightly faster than naively starting at (0, 0)
    let mut points = points.into_iter()
        .map(|(row, col)| ((row, col), grid[row as usize][col as usize]))
        .collect::<Vec<((isize, isize), usize)>>();
    points.sort_by_key(|(_, value)| -(*value as isize));

    for point in points {
        let point = point.0;
        if visited.contains(&point) || exclude.contains(&point) {
            continue;
        }
        let barrels = dfs_connected_barrels(grid.clone(), vec![point]);
        let new: HashSet<_> = barrels.difference(exclude).cloned().collect();
        visited.extend(&barrels);
        barrels_per_point.insert(point, new.len());
    }

    let max_key_value = barrels_per_point.into_iter()
        .max_by_key(|(_, value)| *value)
        .unwrap();

    let max_point = max_key_value.0;
    let visited = dfs_connected_barrels(grid.clone(), vec![max_point]);
    (max_point, visited)
}

fn dfs_connected_barrels(grid: Grid, start: Vec<Point>) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut stack: Vec<Point> = Vec::new();
    stack.extend(start);

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let options = MOVES.iter()
            .map(|m| (current.0 + m.0, current.1 + m.1))
            .filter(|new| is_valid_move(&grid, &current, new))
            .collect::<Vec<Point>>();

        stack.extend(options);

    }
    visited
}

fn is_valid_move(grid: &Grid, point_0: &Point, point_1: &Point) -> bool {
    let width = grid[0].len();
    let height = grid.len();

    point_1.0 >= 0 && point_1.0 < height as isize && point_1.1 >= 0 && point_1.1 < width as isize
     && grid[point_0.0 as usize][point_0.1 as usize] >= grid[point_1.0 as usize][point_1.1 as usize]
}

fn parse(filename: &str) -> Grid {
    read_lines(filename)
        .iter()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
        })
    .collect()
}
