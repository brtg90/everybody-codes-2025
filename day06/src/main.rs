use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();

}

fn part_1() {
    let input = read_lines("inputs/day06pt1.txt");
    let char_map = collect_char_map(&input[0]);
    let counts = count_possible_mentors(&char_map, 'a');
    println!("Part 1: {:?}", counts);
}

fn part_2() {
    let input = read_lines("inputs/day06pt2.txt");
    let char_map = collect_char_map(&input[0]);
    let mut sum = 0;
    let apprentices: Vec<char> = char_map.keys()
        .filter_map(|&k| k.is_lowercase().then(|| k))
        .collect();
    for apprentice in apprentices {
        sum += count_possible_mentors(&char_map, apprentice);
    }
    println!("Part 2: {:?}", sum);
}

fn part_3() {
    // Repeating the structure 1000 times leads to a contribution of 1000 regular combinations,
    // 999 left edge cases and 999 right edge cases
    let max_distance = 1000;
    let num_repeats = 1000;
    let input = &read_lines("inputs/day06pt3.txt")[0];

    // Construct both left and right inputs, where the to be appended part should only consist of
    // mentors and the old part only of apprentices (to avoid counting double)
    let length_input = input.len();
    let left_mentors = input[length_input - max_distance..]
        .chars()
        .map(|c| if c.is_lowercase() { '#' } else { c })
        .collect::<String>();
    let left_apprentices = input[..max_distance]
        .chars()
        .map(|c| if c.is_uppercase() { '#' } else { c })
        .collect::<String>();
    let input_left = [left_mentors, left_apprentices].join("");

    let right_mentors = input[..max_distance]
        .chars()
        .map(|c| if c.is_lowercase() { '#' } else { c })
        .collect::<String>();
    let right_apprentices = input[length_input - max_distance..]
        .chars()
        .map(|c| if c.is_uppercase() { '#' } else { c })
        .collect::<String>();
    let input_right = [right_apprentices, right_mentors].join("");

    let char_map_left = collect_char_map(&input_left);
    let char_map_right = collect_char_map(&input_right);
    let char_map_full = collect_char_map(&input);
    let mut sum = 0;
    let apprentices: Vec<char> = char_map_full.keys()
        .filter_map(|&k| k.is_lowercase().then(|| k))
        .collect();
    for apprentice in apprentices {
        let regular= count_possible_mentors_distance_wise(&char_map_full, apprentice, max_distance as isize);
        sum += regular * num_repeats;
        let left = count_possible_mentors_distance_wise(&char_map_left, apprentice, max_distance as isize);
        sum += left * (num_repeats - 1);
        let right = count_possible_mentors_distance_wise(&char_map_right, apprentice, max_distance as isize);
        sum += right * (num_repeats - 1);
    }
    println!("Part 3: {:?}", sum);
}

fn collect_char_map(line: &str) -> HashMap<char, Vec<usize>> {
    let mut char_map: HashMap<char, Vec<usize>> = HashMap::new();
    line.chars()
        .enumerate()
        .for_each(|(index, c)| char_map.entry(c).or_insert(Vec::new()).push(index));

    char_map
}

fn count_possible_mentors(char_map: &HashMap<char, Vec<usize>>, char_type: char) -> usize {
    let mentor_char = char_type.to_uppercase().next().unwrap();
    let apprentices = char_map.get(&char_type)
        .expect("Could not find apprentice type in map");
    let mentors = char_map.get(&mentor_char)
        .expect("Could not find mentor type in map");

    // Since the vectors are both naturally sorted, binary search can be used
    let counts = apprentices.iter()
        .map(|&pos_ap| {
            mentors.binary_search(&pos_ap).unwrap_or_else(|pos_men| pos_men)
        })
        .sum();

    counts
}

fn count_possible_mentors_distance_wise(char_map: &HashMap<char, Vec<usize>>, char_type: char, max_distance: isize) -> usize {
    let mentor_char = char_type.to_uppercase().next().unwrap();
    let apprentices = char_map.get(&char_type);
    let mentors = char_map.get(&mentor_char);

    if apprentices.is_none() || mentors.is_none() {
        return 0;
    }

    let distances = apprentices.unwrap().iter()
        .flat_map(|&pos_ap| {
            mentors.unwrap().iter()
                .map(|&pos_men| (pos_ap as isize - pos_men as isize).abs())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<isize>>();

    distances.iter()
        .filter(|&&dist| dist <= max_distance)
        .count()
}