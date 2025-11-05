use std::collections::HashSet;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn parse_numbers(line: &[String]) -> Vec<(isize, isize)> {
    line.iter()
        .filter_map(|x| {
            let content = x.strip_prefix("A=[")?.strip_suffix("]")?;

            let mut numbers = content.split(',');
            let first = numbers.next()?.parse().ok()?;
            let second = numbers.next()?.parse().ok()?;

            Some((first, second))
        })
        .collect()
}

fn part_1() {
    let lines_pt1 = read_lines("inputs/day02pt1.txt");
    let complex_1 = parse_numbers(&lines_pt1)[0];

    let mut result = (0, 0);

    for _i in 0..3 {
        result = multiply(result, result);
        result = divide(result, (10, 10));
        result = sum(result, complex_1);
    }
    println!("Part 1: {:?}", result);
}

fn part_2() {
    println!("Part 2: {:?}", perform_grid_calculation_with_step_size(10, "inputs/day02pt2.txt"));
}

fn part_3() {
    println!("Part 3: {:?}", crate::perform_grid_calculation_with_step_size(1, "inputs/day02pt3.txt"));
}

fn perform_grid_calculation_with_step_size(step_size: usize, filename: &str) -> usize {
    let top_left = parse_numbers(&read_lines(filename))[0];
    let bottom_right = sum(top_left, (1000, 1000));

    let mut engraved_points: HashSet<(isize, isize)> = HashSet::new();

    for x in (top_left.0..bottom_right.0 + 1).step_by(step_size) {
        for y in (top_left.1..bottom_right.1 + 1).step_by(step_size) {
            let mut result = (0, 0);
            for _run in 0..100 {
                result = multiply(result, result);
                result = divide(result, (100000,100000));
                result = sum(result, (x, y));

                if !within_bounds_part_2(&result) {
                    break;
                }
            }
            if within_bounds_part_2(&result) {
                engraved_points.insert((x, y));
            }
        }
    }
    engraved_points.len()
}

fn within_bounds_part_2(number: &(isize, isize)) -> bool {
    if number.0 > 1000000 || number.1 > 1000000 || number.0 < -1000000 || number.1 < -1000000 {
        false
    }
    else {
        true
    }
}

fn multiply(num_1: (isize, isize), num_2: (isize, isize)) -> (isize, isize) {
    // [X1,Y1] * [X2,Y2] = [X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2]
    let x1 = num_1.0;
    let y1 = num_1.1;
    let x2 = num_2.0;
    let y2 = num_2.1;

    let x = x1 * x2 - y1 * y2;
    let y = x1 * y2 + y1 * x2;
    (x, y)
}

fn sum(num_1: (isize, isize), num_2: (isize, isize)) -> (isize, isize) {
    (num_1.0 + num_2.0, num_1.1 + num_2.1)
}

fn divide(num_1: (isize, isize), num_2: (isize, isize)) -> (isize, isize) {
    //[X1,Y1] / [X2,Y2] = [X1 / X2, Y1 / Y2] using integer division
    let x1 = num_1.0;
    let y1 = num_1.1;
    let x2 = num_2.0;
    let y2 = num_2.1;

    (x1 / x2, y1 / y2)
}