use std::collections::{HashMap, VecDeque};

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day19pt1.txt");
    println!("{:?}", input);
    // let flaps = count_flaps_for_triplets(&input, 0, 0);
    let flaps = bfs(&input, 0, 0);
    println!("Part 1: {:?}", flaps);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn part_3() {
    println!("Part 3: {:?}", 0);
}

fn bfs(triplets: &Vec<Vec<isize>>, x: isize, y: isize) -> isize {
    let mut queue = VecDeque::new();
    queue.push_back((x, y, 0, 0));

    // Track best flaps for each (x, y, triplet_num) state
    let mut best: HashMap<(isize, isize, usize), isize> = HashMap::new();
    let mut min_flaps = isize::MAX;

    while let Some((x, y, flaps, triplet_num)) = queue.pop_front() {
        let state = (x, y, triplet_num);
        if let Some(&prev_best) = best.get(&state) {
            if flaps > prev_best {
                continue;
            }
        }
        best.insert(state, flaps);

        if triplet_num == triplets.len() {
            min_flaps = min_flaps.min(flaps);
            continue;
        }

        if let Some(options) = take_obstacle(&triplets[triplet_num], x, y) {
            for option in options {
                queue.push_back((option[0], option[1], flaps + option[2], triplet_num + 1));
            }
        }
    }
    min_flaps
}



fn take_obstacle(obstacle: &[isize], x: isize, y: isize) -> Option<Vec<Vec<isize>>> {
    let dx = obstacle[0] - x;
    // Each step in x-direction is either a flap or not (i.e. difference of two in height if one
    // is swapped for the other)
    let y_max = dx + y;
    let y_min = -dx + y;

    let obstacle_opening: Vec<isize> = (obstacle[1]..obstacle[1] + obstacle[2])
        .collect();

    let possible: Vec<(usize, isize)> = (y_min..=y_max).step_by(2)
        .enumerate()
        .filter(|(_, y_new)| obstacle_opening.contains(y_new))
        .collect();

    if possible.is_empty() {
        return None;
    }

    let possible_end_points = possible.iter()
        .map(|(i, y_new)| vec![obstacle[0], *y_new, *i as isize])
        .collect();

    Some(possible_end_points)
}

fn parse(filename: &str) -> Vec<Vec<isize>> {
    read_lines(filename)
        .iter()
        .map(|line| line.split(",")
            .map(|x| x.parse::<isize>()
                .unwrap())
            .collect::<Vec<isize>>())
        .collect()
}