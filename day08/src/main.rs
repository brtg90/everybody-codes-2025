use utils::read_input;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    // 32 nails, so a distance difference of 16 crosses the center
    let center_hits: usize = parse_input("inputs/day08pt1.txt")
        .windows(2)
        .map(|w| if (w[0] - w[1]).abs() == 16 {1} else {0})
        .sum();
    println!("Part 1: {:?}", center_hits);
}

fn part_2() {
    let instructions = parse_input("inputs/day08pt2.txt");
    let lines: Vec<_> = instructions.windows(2).collect();
    let mut sum = 0;
    for i in 1..lines.len() {
        for j in 0..i {
            sum += check_for_knot(lines[i], lines[j], false)
        }
    }

    println!("Part 2: {:?}", sum);
}

fn part_3() {
    let num_nails: isize = 256;
    let instructions = parse_input("inputs/day08pt3.txt");
    let lines: Vec<_> = instructions.windows(2).collect();
    let possible_cuts = (1..num_nails + 1)
        .flat_map(|start| (start + 1..num_nails + 1).map(move |end| [start, end])).collect::<Vec<_>>();
    let mut max = 0;
    for cut in possible_cuts {
        let sum: usize = lines.iter()
            .map(|line| check_for_knot(line, &cut, true))
            .sum();
        if sum > max {
            max = sum;
        }
    }

    println!("Part 3: {:?}", max);
}

fn parse_input(file_name: &str) -> Vec<isize> {
    read_input(file_name)
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

fn check_for_knot(line_0: &[isize], line_1: &[isize], include_identical: bool) -> usize {
    // First ensure the lines are always running from the lowest to the highest number
    let line_0 = (
        std::cmp::min(line_0[0], line_0[1]),
        std::cmp::max(line_0[0], line_0[1])
    );
    let line_1 = (
        std::cmp::min(line_1[0], line_1[1]),
        std::cmp::max(line_1[0], line_1[1])
    );

    if include_identical && line_0.0 == line_1.0 && line_0.1 == line_1.1 {
        return 1;
    }

    if line_0.0 < line_1.0 && line_0.1 > line_1.0 && line_0.1 < line_1.1 {
        return 1;
    }

    if line_1.0 < line_0.0 && line_1.1 > line_0.0 && line_1.1 < line_0.1 {
        return 1;
    }

    0
}
