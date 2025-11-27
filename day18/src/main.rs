use std::collections::{HashMap, HashSet, VecDeque};

use utils::read_input;

#[derive(Debug, Clone)]
struct Plant {
    id: usize,
    thickness: isize,
    energy: isize,
    parents: Vec<usize>,
}

impl Plant {
    fn from_description(description: &str, plants: &HashMap<usize, Plant>, instruction_map: Option<&HashMap<usize, bool>>) -> Plant {
        let mut lines = description.lines();
        let first_line = lines.next().unwrap();
        let numbers = first_line[6..first_line.len() - 1]
            .split("with thickness")
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let id = numbers[0] as usize;
        let thickness = numbers[1];

        let mut energy = 0;
        let mut parents = Vec::new();

        for line in lines {
            if line.contains("free") {
                if let Some(instruction) = instruction_map
                    && instruction.get(&id) == Some(&false) {
                        continue;
                }
                let branch_thickness = line[29..].parse::<isize>().unwrap();
                energy += branch_thickness;
            }
            else {
                let numbers = line[18..].split("with thickness ")
                    .map(|x| x.trim().parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                let parent = numbers[0] as usize;
                parents.push(parent);
                let parent_thickness = numbers[1];

                energy += plants.get(&parent).unwrap().energy * parent_thickness;
            }
        }

        if energy < thickness {
            energy = 0;
        }

        Plant { id, thickness, energy, parents }

    }

    fn new_max_possible(description: &str, max_free: usize) -> Option<Plant> {
        let mut lines = description.lines();
        let first_line = lines.next().unwrap();
        let numbers = first_line[6..first_line.len() - 1]
            .split("with thickness")
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let id = numbers[0] as usize;
        if id < max_free {
            return None
        }
        let thickness = numbers[1];

        let mut energy = 0;
        let mut parents = Vec::new();

        for line in lines {
            let numbers = line[18..].split("with thickness ")
                .map(|x| x.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let parent = numbers[0] as usize;
            parents.push(parent);

            if parent > max_free {
                return None
            }
            let parent_thickness = numbers[1];

            if parent_thickness > 0 {
                energy += parent_thickness;
            }

        }

        if energy < thickness {
            return Some(Plant { id, thickness, energy: 0 , parents });
        }

        Some(Plant { id, thickness, energy , parents })
    }

    fn useful_node(description: &str, max_free: usize, not_useful: &HashMap<usize, Plant>) -> Option<Plant> {
        let mut lines = description.lines();
        let first_line = lines.next().unwrap();
        let numbers = first_line[6..first_line.len() - 1]
            .split("with thickness")
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let id = numbers[0] as usize;
        if id < max_free {
            return None
        }
        let mut parents = Vec::new();

        for line in lines {
            let numbers = line[18..].split("with thickness ")
                .map(|x| x.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let parent = numbers[0] as usize;
            parents.push(parent);

            if parent > max_free {
                return None
            }

        }

        if parents.iter().all(|p| not_useful.contains_key(p)) {
            return None;
        }

        Some(Plant { id, thickness: 0, energy: 0 , parents })
    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day18pt1.txt");
    let mut plants: HashMap<usize, Plant> = HashMap::new();
    input.iter()
        .for_each(|descr| {
            let plant = Plant::from_description(descr, &plants, None);
            plants.insert(plant.id, plant);
        });
    let total = plants.iter()
        .max_by_key(|(k, _)| *k)
        .map(|(_, v)| v.energy)
        .unwrap();

    println!("Part 1: {:?}", total);
}

fn part_2() {
    let input = parse("inputs/day18pt2.txt");
    let mut instructions = parse_instructions(&input);
    let mut total = 0;

    while let Some(instruction) = instructions.pop() {
        total += try_instructions(&instruction, &input);
    }

    println!("Part 2: {:?}", total);
}

fn part_3() {
    // The plants with free branches are not linked to grandchildren directly. Furthermore,
    // the input shows that the grandchildren do not link to their parents with negative thicknesses.
    // Moreover, some first children seem to be unable to be activated because their positive
    // contributions are too low. This means that some nodes can be trimmed
    let input = parse("inputs/day18pt3.txt");
    let mut instructions = parse_instructions(&input);
    let mut max = 0;

    let num_free_plants = get_instruction(&instructions[0]).len();
    println!("num free: {}", num_free_plants);

    let mut plants: HashMap<usize, Plant> = HashMap::new();

    input.iter()
        .filter(|line| line.contains("Plant"))
        .for_each(|descr| {
            let plant = Plant::from_description(descr, &plants, None);
            plants.insert(plant.id, plant);
        });

    let mut zero_plants : HashMap<usize, Plant> = HashMap::new();
    let mut useful_plants : HashMap<usize, Plant> = HashMap::new();

    input.iter()
        .filter(|line| line.contains("Plant"))
        .skip(num_free_plants)
        .for_each(|descr| {
            if let Some(plant) = Plant::new_max_possible(descr, num_free_plants) {
                if plant.energy == 0 {
                    zero_plants.insert(plant.id, plant);
                }
                else {
                    useful_plants.insert(plant.id, plant);
                }
            }
        });

    let new_max = std::cmp::max(
        zero_plants.keys().max(),
        useful_plants.keys().max()
    ).unwrap();

    input.iter()
        .filter(|line| line.contains("Plant"))
        .skip(*new_max)
        .for_each(|descr| {
            if let Some(plant) = Plant::useful_node(descr, num_free_plants, &zero_plants) {
                useful_plants.insert(plant.id, plant);
            }
        });

    let mut useful_roots: HashSet<usize> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.extend(useful_plants.keys());

    while let Some(plant_id) = queue.pop_front() {
        queue.extend(&plants.get(&plant_id).unwrap().parents);
        if plant_id <= num_free_plants {
            useful_roots.insert(plant_id);
        }
    }

    println!("useful roots: {:?}", useful_roots.len());

    while let Some(instruction) = instructions.pop() {
        let result = try_instructions(&instruction, &input);
        if result > max {
            max = result;
        }

    }
    println!("Part 3: {:?}", max);
}

fn try_instructions(instruction: &str, input: &[String]) -> isize {
    let instruction = get_instruction(&instruction);
    let mut plants: HashMap<usize, Plant> = HashMap::new();
    input.iter()
        .filter(|line| line.contains("Plant"))
        .for_each(|descr| {
            let plant = Plant::from_description(descr, &plants, Some(&instruction));
            plants.insert(plant.id, plant);
        });
    plants.iter()
        .max_by_key(|(k, _)| *k)
        .map(|(_, v)| v.energy)
        .unwrap()
}

fn get_instruction(instr: &str) -> HashMap<usize, bool> {
    let mut instructions: HashMap<usize, bool> = HashMap::new();
    instr.split_whitespace()
        .enumerate()
        .for_each(|(i, str)| {
            let active = str.parse::<usize>().unwrap() == 1;
            instructions.insert(i + 1, active);
        });
    instructions
}

fn parse_instructions(input: &[String]) -> Vec<String> {
    input.iter()
        .filter(|line| !line.contains("Plant"))
        .flat_map(|line| line.split("\r\n"))
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

fn parse(filename: &str) -> Vec<String> {
    read_input(filename).split("\r\n\r\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}