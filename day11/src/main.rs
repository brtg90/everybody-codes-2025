use utils::read_lines;


fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let mut columns = parse_columns("inputs/day11pt1.txt");

    let mut checksum = 0;
    let mut changed = true;

    let mut round = 0;

    // Phase 1
    while changed {
        (columns, changed) = phase_1(columns);
        if changed {
            round += 1;
        }

        if round == 10 {
            checksum = get_checksum(&columns);
            break;
        }
    }

    // Phase 2
    changed = true;

    while changed {
        (columns, changed) = phase_2(columns);
        if changed {
            round += 1;
        }

        if round == 10 {
            checksum = get_checksum(&columns);
            break;
        }
    }
    println!("Part 1: {:?}", checksum);
}

fn part_2() {
    let mut columns = parse_columns("inputs/day11pt2.txt");

    let mut changed = true;
    let mut round = 0;

    // Phase 1
    while changed {
        (columns, changed) = phase_1(columns);
        if changed {
            round += 1;
        }
    }

    round += phase_2_rapid(columns);

    println!("Part 2: {:?}", round);
}

fn part_3() {
    let mut columns = parse_columns("inputs/day11pt3.txt");

    let mut changed = true;
    let mut round = 0;

    // Phase 1
    while changed {
        (columns, changed) = phase_1(columns);
        if changed {
            round += 1;
        }
    }
    
    round += phase_2_rapid(columns);

    println!("Part 3: {:?}", round);
}

fn parse_columns(filename: &str) -> Vec<usize> {
    read_lines(filename)
        .into_iter()
        .map(|line| line.parse::<usize>().expect("Could not parse usize from input"))
        .collect::<Vec<usize>>()
}

fn phase_1(mut columns: Vec<usize>) -> (Vec<usize>, bool) {
    let mut changed = false;

    for i in 0..columns.len() - 1 {
        if columns[i + 1] < columns[i] {
            changed = true;
            columns[i + 1] += 1;
            columns[i] -= 1;
        }
    }
    (columns, changed)
}

fn phase_2(mut columns: Vec<usize>) -> (Vec<usize>, bool) {
    let mut changed = false;

    for i in 0..columns.len() - 1 {
        if columns[i + 1] > columns[i] {
            changed = true;
            columns[i + 1] -= 1;
            columns[i] += 1;
        }
    }
    (columns, changed)
}

fn phase_2_rapid(columns: Vec<usize>) -> usize {
    // Checkout how many ducks are missing below the expected number of ducks per column
    let ducks_per_column: usize = columns.iter().sum::<usize>() / columns.len();

    columns.iter()
        .filter(|&&ducks| ducks < ducks_per_column)
        .map(|&ducks| (ducks_per_column - ducks))
        .sum()
}

fn get_checksum(column: &Vec<usize>) -> usize {
    column.iter()
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum()
}