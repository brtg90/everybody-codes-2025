use std::collections::{HashSet, VecDeque};

use utils::read_input;

const DIRECTIONS : &[Point] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
type Point = (isize, isize);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn contains_point(&self, point: &Point) -> bool {
        let low = std::cmp::min(self.start, self.end);
        let high = std::cmp::max(self.start, self.end);
        point.0 >= low.0 && point.0 <= high.0 && point.1 >= low.1 && point.1 <= high.1
    }

    fn get_max_row(&self) -> isize {
        std::cmp::max(self.start.0, self.end.0)
    }

    fn get_min_row(&self) -> isize {
        std::cmp::min(self.start.0, self.end.0)
    }

    fn get_max_col(&self) -> isize {
        std::cmp::max(self.start.1, self.end.1)
    }

    fn get_min_col(&self) -> isize {
        std::cmp::min(self.start.1, self.end.1)
    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day15pt1.txt");
    let distance = calculate_end_point_distance(&input);
    println!("Part 1: {:?}", distance);
}

fn part_2() {
    let input = parse("inputs/day15pt2.txt");
    let distance = calculate_end_point_distance(&input);
    println!("Part 2: {:?}", distance);
}

fn part_3() {
    let input = parse("inputs/day15pt3.txt");
    let distance = calculate_end_point_distance(&input);
    println!("Part 3: {:?}", distance);
}

fn calculate_end_point_distance(instructions: &[String]) -> usize {
    let (wall, end) = parse_wall(HashSet::new(), instructions, 0, (0,0));
    let corners = find_grid_corners(&wall);

    let mut queue : VecDeque<(Point, usize)> = VecDeque::from([((0, 0), 0)]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut depth = 0;

    while let Some((current, length)) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        let mut new_squares = DIRECTIONS.iter()
            .map(|(drow, dcol)| (current.0 + drow, current.1 + dcol))
            .filter(|point| !visited.contains(point)
                &&
                (*point == end || (!wall_contains_point(&wall, point) && is_valid_point(point, &corners))))
            .collect::<Vec<Point>>();

        if new_squares.contains(&end) {
            depth = length + 1;
            break;
        }
        // Sort by ascending Manhattan distance
        new_squares.sort_by_key(|point| - ((point.0 - end.0).abs() + (point.1 - end.1).abs()));
        new_squares.into_iter()
            .for_each(|point| queue.push_back((point, length + 1)));

    }

    depth
}

fn wall_contains_point(wall: &HashSet<Line>, point: &Point) -> bool {
    wall.iter()
        .any(|line| line.contains_point(point))
}

fn is_valid_point(point: &Point, corners: &[Point]) -> bool {
    point.0 >= corners[0].0 && point.0 <= corners[1].0 && point.1 >= corners[0].1 && point.1 <= corners[1].1
}

fn find_grid_corners(wall: &HashSet<Line>) -> [Point; 2] {
    let max_row = wall.iter().max_by_key(|a| a.get_max_row()).unwrap().get_max_row() + 2;
    let min_row = wall.iter().min_by_key(|a| a.get_min_row()).unwrap().get_min_row() - 2;
    let max_col = wall.iter().max_by_key(|a| a.get_max_col()).unwrap().get_max_col() + 2;
    let min_col = wall.iter().min_by_key(|a| a.get_min_col()).unwrap().get_min_col() - 2;

    [(min_row, min_col), (max_row, max_col)]
}

fn parse_wall(mut lines: HashSet<Line>, instructions: &[String], old_direction: usize, last: Point) -> (HashSet<Line>, Point) {
    if instructions.is_empty() {
        return (lines, last);
    }

    let instruction = &instructions[0];

    let letter = instruction.chars().next().unwrap();
    let distance = instruction[1..].parse::<isize>().expect("Could not parse string to isize");

    let mut direction = (old_direction + 1) % 4;

    if letter == 'L' {
        direction = (old_direction + 3) % 4;
    }

    let end = (last.0 + distance * DIRECTIONS[direction].0, last.1 + distance * DIRECTIONS[direction].1);
    let line = Line::new(last, end);

    lines.insert(line);

    parse_wall(lines, &instructions[1..], direction, end)
}

fn parse(filename: &str) -> Vec<String> {
    read_input(filename)
        .split(",")
        .map(|x| x.to_string())
        .collect()
}