use std::cmp::Ordering;
use std::collections::HashMap;
use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

struct Sword {
    id: usize,
    spine: Vec<usize>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
}

impl Sword {
    fn new(id: usize) -> Sword {
        Sword {
            id,
            spine: vec![],
            left: vec![],
            right: vec![],
        }
    }

    fn from_line(line: &str) -> Sword {
        let id = &line.split(':').collect::<Vec<&str>>()[0]
            .parse::<usize>()
            .expect("Could not parse id");

        let mut sword = Sword::new(*id);
        sword
    }

    fn construct_spine(mut self, line: &str) -> Sword {
        let numbers = line.split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().expect("Could not parse number"))
            .collect::<Vec<usize>>();

        self.push_to_spine(numbers[0]);

        for i in &numbers[1..] {
            match i.cmp(&self.spine[self.spine.len() - 1]) {
                Ordering::Less => {
                    if !self.left.contains(&None) {
                        self.push_to_spine(*i);
                    }
                    else {
                        // Find the first suitable place starting from the top
                        let mut start_idx = 0;
                        let mut stop = false;
                        while self.left[start_idx..].contains(&None) && !stop {
                            let pos = self.left.iter().skip(start_idx).position(|&x| x == None).unwrap() + start_idx;
                            start_idx = pos + 1;
                            if self.spine[pos] > *i {
                                self.left[pos] = Some(*i);
                                stop = true;
                            }
                        }
                        if !stop {
                            self.push_to_spine(*i);
                        }
                    }
                },
                Ordering::Equal => {
                    self.push_to_spine(*i);
                },
                Ordering::Greater => {
                    if !self.right.contains(&None) {
                        self.push_to_spine(*i);
                    }
                    else {
                        // Find the first suitable place starting from the top
                        let mut start_idx = 0;
                        let mut stop = false;
                        while self.right[start_idx..].contains(&None) && !stop {
                            let pos = self.right.iter().skip(start_idx).position(|&x| x == None).unwrap() + start_idx;
                            start_idx = pos + 1;
                            if self.spine[pos] < *i {
                                self.right[pos] = Some(*i);
                                stop = true;
                            }
                        }
                        if !stop {
                            self.push_to_spine(*i);
                        }
                    }
                }
            }
        }
        self
    }

    fn push_to_spine(&mut self, value: usize) {
        self.spine.push(value);
        self.left.push(None);
        self.right.push(None);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct SwordScore {
    id: usize,
    levels: Vec<usize>,
    value: usize,
}

impl SwordScore {
    fn from_sword(sword: Sword, value: usize) -> SwordScore {
        let mut levels = Vec::with_capacity(sword.spine.len());
        for i in 0..sword.spine.len() {
            let mut digits_string = String::new();
            if let Some(l) = sword.left[i] {
                digits_string.push_str(&l.to_string());
            }
            digits_string.push_str(&sword.spine[i].to_string());
            if let Some(r) = sword.right[i] {
                digits_string.push_str(&r.to_string());
            }
            levels.push(digits_string.parse::<usize>().expect("Could not parse level"));
        }
        SwordScore {
            id: sword.id,
            levels,
            value
        }
    }
}


fn part_1() {
    let input = &read_lines("inputs/day05pt1.txt")[0];
    let answer = get_sword_and_value_from_line(input).1;
    println!("Part 1: {:?}", answer);
}

fn get_sword_and_value_from_line(line: &str) -> (Sword, usize) {
    let sword = Sword::from_line(&line);
    let sword = sword.construct_spine(&line);
    let string_number: usize = sword.spine.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .expect("Could not parse number from string");
    (sword, string_number)
}

fn part_2() {
    let input = &read_lines("inputs/day05pt2.txt");
    let mut swords_values: Vec<usize> = vec![];
    input.iter()
        .for_each(|line| swords_values.push(get_sword_and_value_from_line(line).1));
    swords_values.sort();
    let difference = swords_values[swords_values.len() - 1] - swords_values[0];
    println!("Part 2: {:?}", difference);
}

fn part_3() {
    let input = &read_lines("inputs/day05pt3.txt");
    let mut swords_vec: Vec<SwordScore> = vec![];
    input.iter()
        .for_each(|line|  {
            let result = get_sword_and_value_from_line(line);
            let sword = result.0;
            let value = result.1;
            let sword_score = SwordScore::from_sword(sword, value);
            swords_vec.push(sword_score);
        });

    swords_vec.sort_by(|a, b| compare_sword_scores(a, b));
    // The sorted vec needs to be reversed to conform to the greater to lower ranking
    swords_vec.reverse();
    let score = swords_vec.iter()
        .enumerate()
        .map(|(pos, sword_score)| sword_score.id * (pos + 1) )
        .sum::<usize>();
    println!("Part 3: {:?}", score);
}

fn compare_sword_scores(a: &SwordScore, b: &SwordScore) -> Ordering {
    if a.value < b.value {
        return Ordering::Less;
    }

    if a.value == b.value {
        let length_a = a.levels.len();
        let length_b = b.levels.len();
        let length = std::cmp::min(length_a, length_b);

        for i in 0..length {
            if a.levels[i] == b.levels[i] {
                continue;
            }
            if a.levels[i] < b.levels[i] {
                return Ordering::Less;
            }

            if a.levels[i] > b.levels[i] {
                return Ordering::Greater;
            }
        }

        if a.id < b.id {
            Ordering::Less
        }

        else {
            Ordering::Greater
        }
    }

    else {
        Ordering::Greater
    }
}