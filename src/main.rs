use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

#[derive(Copy,Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl TryFrom<char> for Direction{
    type Error = IllegalChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            '>' => Ok(Self::East),
            _ => Err(IllegalChar(c)),
        }
    }
}

struct Moves {
    moves: Vec<Direction>,
}

impl FromStr for Moves {
    type Err = IllegalChar; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
        .chars()
        .map(Direction::try_from)
        .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok(Self { moves })
    }
}

pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

impl VisitedHouses {
    #[must_use]
    pub fn new() -> Self {
        let initial_pos = Pos::new(0, 0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_pos);

        Self {
            visited_houses,
            current_position: initial_pos,
        }
    }

    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    #[must_use]
    pub const fn current_pos(&self) -> Pos {
        self.current_position
    }

    pub fn perform_move(&mut self, direction: Direction){
        let new_position: Pos = match direction {
            Direction::North => Pos::new(self.current_position.x, self.current_position.y + 1),
            Direction::South => Pos::new(self.current_position.x, self.current_position.y - 1),
            Direction::East => Pos::new(self.current_position.x + 1, self.current_position.y),
            Direction::West => Pos::new(self.current_position.x - 1, self.current_position.y),
        };
        self.current_position = new_position;
        self.visited_houses.insert(new_position);
    }

    pub fn perform_moves(&mut self, moves: Moves) {
        for direction in moves.moves {
            self.perform_move(direction);
        }
    }
}

impl Default for VisitedHouses {
    fn default() -> Self {
        Self::new()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        assert_eq!(visited_houses.current_pos(), Pos{x: 0, y: 0});
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_east() {
        let mut visited_houses = VisitedHouses::new();
        visited_houses.perform_move(Direction::East);
        assert_eq!(visited_houses.num_visited_houses(), 2);
        assert_eq!(visited_houses.current_pos(), Pos::new(1, 0));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 4);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2565);
    // }
}
