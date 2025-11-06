use std::collections::{HashMap, HashSet};

use utils::read_input;

fn main() {
    part_1();
    part_2();
    part_3();
}


fn part_1() {
    let set = get_set("inputs/day03pt1.txt");
    let set_size = set.iter()
        .fold(0, |acc, x| acc + x);
    println!("Part 1: {:?}", set_size);
}

fn part_2() {
    let set = get_set("inputs/day03pt2.txt");
    let mut vec_set = set.iter().collect::<Vec<_>>();
    vec_set.sort();
    let set_size = vec_set[0..20].iter().fold(0, |acc, x| acc + *x);
    println!("Part 2: {:?}", set_size);
}

fn part_3() {
    let input = read_input("inputs/day03pt3.txt");
    let numbers = input.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<isize>().expect("Could not parse number"))
        .collect::<Vec<isize>>();
    // To find out how many sets are needed, find the highest number of duplicates
    let mut frequency_map: HashMap<isize, usize> = HashMap::new();
    numbers.iter()
        .for_each(|x| *frequency_map.entry(*x).or_insert(0) += 1);
    let max = frequency_map.values()
        .max()
        .expect("No maximum found");
    println!("Part 3: {:?}", max);
}

fn get_set(filename: &str) -> HashSet<isize> {
    let input = read_input(filename);
    let mut set: HashSet<isize> = HashSet::new();
    input.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|x| {
            let number = x.parse::<isize>().expect("Could not parse number");
            set.insert(number);
        });
    set
}