use std::collections::{HashMap, HashSet};

use utils::read_lines;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ChessBoard {
    sheep: Vec<(isize, isize)>,
    dragon: (isize, isize),
    safe: Vec<(isize, isize)>,
    width: isize,
    height: isize,
}

impl ChessBoard {
    fn from_text(filename: &str) -> ChessBoard {
        let lines = read_lines(filename);
        let height = lines.len() as isize;
        let width = lines[0].len() as isize;

        let mut sheep: Vec<(isize, isize)> = Vec::new();
        let mut dragon: (isize, isize) = (0, 0);
        let mut safe: Vec<(isize, isize)> = Vec::new();

        lines.iter()
            .enumerate()
            .for_each(|(row, column)| {
                column.chars().enumerate().for_each(|(col, c)| {
                    if c == 'S' {
                        sheep.push((row as isize, col as isize));
                    }
                    else if c == 'D' {
                        dragon = (row as isize, col as isize);
                    }
                    else if c == '#' {
                        safe.push((row as isize, col as isize));
                    }
                });
            });

        let mut board = ChessBoard {
            sheep,
            dragon,
            safe,
            width,
            height,
        };
        board.sort_sheep();
        board
    }

    fn sort_sheep(&mut self) {
        self.sheep.sort_unstable();
    }

    fn move_all_sheep(&mut self) {
        self.sheep = self.sheep.iter_mut()
            .map(|(row, col)| (*row + 1, *col))
            .filter(|pos| is_valid_move(pos, self.width, self.height))
            .collect();
    }

    fn move_one_sheep(&mut self, old_pos: (isize, isize)) {
        self.sheep = self.sheep.iter_mut()
            .map(|pos| if pos == &old_pos {(pos.0 + 1, pos.1)} else {*pos})
            .collect();
        self.sort_sheep();
    }

    fn get_possible_sheep_moves(&self) -> Vec<((isize, isize),(isize, isize))> {
        let potential_moves: Vec<((isize, isize),(isize, isize))> = self.sheep.iter()
            .map(|(row, col)| ((*row, *col), (*row + 1, *col)))
            .filter(|(_, pos)| pos != &self.dragon || self.safe.contains(pos))
            .collect();
        potential_moves
    }

    fn sheep_escapes(&self, sheep_pos: &(isize, isize)) -> bool {
        sheep_pos.0 >= self.height
    }

    fn remove_sheep(&mut self, visited: &[(isize, isize)]) -> usize {
        let before = self.sheep.len();
        self.sheep.retain(|&pos| !visited.contains(&pos));
        self.sort_sheep();
        before - self.sheep.len()
    }

    fn remove_sheep_current_dragon(&mut self) {
        self.sheep.retain(|&pos| self.dragon != pos || self.safe.contains(&pos));
        self.sort_sheep();
    }

    fn move_sheep_get_boards(&self) -> Vec<ChessBoard> {
        // Return a copy of the boards after each possible sheep move on the current board
        // Filter out the boards where the sheep escapes, since this is a failure of the dragon
        let sheep_moves = self.get_possible_sheep_moves();

        if sheep_moves.is_empty() {
            return vec![self.clone()];
        }

        sheep_moves.into_iter()
            .filter(|(_, new)| !self.sheep_escapes(new))
            .map(|(old, _)| {
                let mut board = self.clone();
                board.move_one_sheep(old);
                board
            })
            .collect::<Vec<ChessBoard>>()
    }

    fn print_board(&self) {
        (0..self.height)
            .for_each(|row| {
                let column = (0..self.width)
                    .map(|col| {
                        let pos = (row, col);
                        if pos == self.dragon {
                            'D'
                        } else if self.sheep.contains(&pos) {
                            'S'
                        } else if self.safe.contains(&pos) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<Vec<char>>();
                println!("{}", column.iter().collect::<String>());
            });
        println!("\n\n");
    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let board = ChessBoard::from_text("inputs/day10pt1.txt");
    let visited = move_dragon_max_times(board.dragon, 0, 4, (board.width, board.height));
    let killed_sheep = get_killed_sheep(&visited, &board).len();
    println!("Part 1: {:?}", killed_sheep);
}

fn part_2() {
    let mut board = ChessBoard::from_text("inputs/day10pt2.txt");
    let mut total_sheep = 0;

    let mut dragon_pos: Vec<(isize, isize)> = vec![board.dragon];

    for _ in 0..20 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        dragon_pos.iter()
            .map(|&pos| move_dragon_max_times(pos, 0, 1, (board.width, board.height)))
            .for_each(|set| visited.extend(&set));
        // First check which sheep are at the new dragon positions
        let killed_sheep = get_killed_sheep(&visited, &board);
        total_sheep += board.remove_sheep(&killed_sheep);

        // Check which sheep walk into dragon positions
       board.move_all_sheep();
        let killed_sheep = get_killed_sheep(&visited, &board);
        total_sheep += board.remove_sheep(&killed_sheep);

        dragon_pos = visited.into_iter().collect();
    }
    println!("Part 2: {:?}", total_sheep);
}

fn part_3() {
    let board = ChessBoard::from_text("inputs/day10pt3.txt");
    let mut memo = HashMap::new();
    let unique_sequences = dfs_memo(board, &mut memo, false);

    println!("Part 3: {}", unique_sequences);
}


fn dfs_memo(board: ChessBoard, memo: &mut HashMap<(ChessBoard, bool), usize>, dragon_turn: bool) -> usize {
    let original = board.clone();

    if let Some(&result) = memo.get(&(board.clone(), dragon_turn)) {
        return result;
    }

    if board.sheep.is_empty() {
        memo.insert((board, dragon_turn), 1);
        return 1;
    }

    let mut total = 0;

    if dragon_turn {
        let dragon_boards = move_dragon_get_boards(board);

        for mut dragon_board in dragon_boards {
            dragon_board.remove_sheep_current_dragon();
            let result = dfs_memo(dragon_board.clone(), memo, false);
            total += result;
        }
    }
    else {
        let sheep_move_boards = board.move_sheep_get_boards();

        if sheep_move_boards.is_empty() {
            memo.insert((original, dragon_turn), 0);
            return 0;
        }


        for sheep_board in sheep_move_boards {
            let result = dfs_memo(sheep_board.clone(), memo, true);
            total += result;

        }
    }
    memo.insert((original, dragon_turn), total);
    total
}

fn move_dragon_get_boards(board: ChessBoard) -> Vec<ChessBoard> {
    // Return a copy of the boards after each possible dragon move on the current board
    let row = board.dragon.0;
    let col = board.dragon.1;

    let mut new_points = Vec::from([(row + 2, col + 1), (row + 2, col - 1), (row + 1, col + 2), (row + 1, col - 2),
        (row - 1, col + 2), (row - 1, col - 2), (row - 2, col + 1), (row - 2, col - 1)]);
    new_points = new_points.into_iter()
        .filter(|&point| is_valid_move(&point, board.width, board.height))
        .collect::<Vec<(isize, isize)>>();

    new_points.into_iter()
        .map(|new| {
            let mut board_clone = board.clone();
            board_clone.dragon = new;
            board_clone
        })
        .collect::<Vec<ChessBoard>>()
}

fn get_killed_sheep(visited: &HashSet<(isize, isize)>, board: &ChessBoard) -> Vec<(isize, isize)> {
    board.sheep.iter()
        .filter(|&pos| visited.contains(pos) && !board.safe.contains(pos))
        .cloned()
        .collect()
}

fn move_dragon_max_times(dragon: (isize, isize), n: isize, max: isize, dimensions: (isize, isize)) -> HashSet<(isize, isize)> {
    let width = dimensions.0;
    let height = dimensions.1;

    if n == max {
        return HashSet::new();
    }

    let row = dragon.0;
    let col = dragon.1;

    let mut new_points = HashSet::from([(row + 2, col + 1), (row + 2, col - 1), (row + 1, col + 2), (row + 1, col - 2),
                          (row - 1, col + 2), (row - 1, col - 2), (row - 2, col + 1), (row - 2, col - 1)]);
    new_points = new_points.into_iter()
        .filter(|&point| is_valid_move(&point, width, height))
        .collect::<HashSet<(isize, isize)>>();

    new_points.extend(new_points.iter()
        .flat_map(|point| move_dragon_max_times(*point, n + 1, max, dimensions))
        .collect::<HashSet<(isize, isize)>>());
    new_points

}

fn is_valid_move(end: &(isize, isize), width: isize, height: isize) -> bool {
    end.0 >= 0 && end.0 < height && end.1 >= 0 && end.1 < width
}
