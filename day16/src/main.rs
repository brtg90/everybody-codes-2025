use utils::read_input;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day16pt1.txt");
    let blocks = calculate_blocks_in_wall(&input, 90);
    println!("Part 1: {:?}", blocks);
}

fn part_2() {
    let input = parse("inputs/day16pt2.txt");
    let product: usize = get_spell_from_wall(&input, 1, vec![]).iter().product();
    println!("Part 2: {:?}", product);
}

fn part_3() {
    let input = parse("inputs/day16pt3.txt");
    let spells = get_spell_from_wall(&input, 1, vec![]);
    // At least one block of each spell number is used, hence start from the maximum number
    let length = get_length_wall(&spells, 202520252025000, spells[spells.len() - 1], usize::MAX);
    println!("Part 3: {:?}", length);
}

fn calculate_blocks_in_wall(instructions: &[usize], length: usize) -> usize {
    instructions.iter()
        .map(|&num| length / num)
        .sum()
}

fn get_spell_from_wall(wall: &[usize], number: usize, mut spells: Vec<usize>) -> Vec<usize> {
    let mut new = wall.to_vec();
    let valid_spell_number = wall.iter()
        .enumerate()
        .filter(|(index, _)| (index + 1) % number == 0)
        .all(|(_, &num)| num > 0);

    if valid_spell_number {
        new = wall.iter()
            .enumerate()
            .map(|(index, &num)| if (index + 1) % number == 0 { num - 1 } else { num } )
            .collect();
        spells.push(number);
    }

    if !new.iter().all(|&num| num == 0) {
        spells = get_spell_from_wall(&new, number + 1, spells);
    }

    spells
}

fn get_length_wall(spells: &[usize], blocks: usize, mut lower: usize, mut upper: usize) -> usize {
    let mut old = 0;
    let mut length = 1;

    while length != old {
        old = length;
        length = lower + (upper - lower) / 2;

        let needed = calculate_blocks_in_wall(spells, length);

        if needed > blocks {
            upper = length;
        }

        else if needed == blocks {
            return length;
        }

        else {
            lower = length;
        }
    }
    
    length
}

fn parse(filename: &str) -> Vec<usize> {
    read_input(filename)
        .split(",")
        .map(|x| x.parse::<usize>().expect("Couldn't parse number"))
        .collect()

}