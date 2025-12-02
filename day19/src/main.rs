use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day19pt1.txt");
    let flaps = bfs(&input);
    println!("Part 1: {:?}", flaps);
}

fn part_2() {
    let input = parse("inputs/day19pt2.txt");
    let flaps = bfs(&input);
    println!("Part 2: {:?}", flaps);
}

fn part_3() {
    println!("Part 3: {:?}", 0);
}

fn bfs(triplets: &HashMap<isize, Vec<Vec<isize>>>) -> isize {
    let mut x_points = triplets.keys().cloned().collect::<Vec<_>>();
    x_points.sort();
    x_points.insert(0, 0);

    let mut current: HashMap<isize, isize> = HashMap::new();
    current.insert(0, 0);

    for (i, x) in x_points.iter().enumerate() {
        if i == x_points.len() - 1 {
            break;
        }
        let mut new: HashMap<isize, isize> = HashMap::new();

        for (&curr_y, &curr_flaps) in &current {
            for opening in &triplets[&x_points[i + 1]] {
                if let Some(options) = take_obstacle(opening, *x, curr_y) {
                    for option in options {
                        new.entry(option[1])
                            .and_modify(|v| *v = (*v).min(curr_flaps + option[2]))
                            .or_insert(curr_flaps + option[2]);
                    }
                }
            }
        }
        current = new;

    }
    *current.values().min().unwrap()
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

fn parse(filename: &str) -> HashMap<isize, Vec<Vec<isize>>> {
    let mut obstacles = HashMap::new();
    read_lines(filename)
        .iter()
        .for_each(|line| {
            let vec = line.split(",")
                .map(|x| x.parse::<isize>()
                    .unwrap())
                .collect::<Vec<isize>>();
            obstacles.entry(vec[0])
                .and_modify(|v: &mut Vec<Vec<isize>>| v.push(vec.clone()))
                .or_insert(vec![vec]);
        });
    obstacles
}