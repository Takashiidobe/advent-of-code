use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Turn {
    Left = 0,
    Right = 1,
}

impl TurnDirection for Direction {
    fn turn(&mut self, dir_to_turn: Turn) {
        if self.Direction == Direction::North && dir_to_turn == Turn::Left {
            self.Direction = Direction::West;
        } else if self.Direction == Direction::West && dir_to_turn == Turn::Right {
            self.Direction = Direction::North;
        } else if dir_to_turn == Turn::Left {
            self.Direction -= 1;
        } else if dir_to_turn == Turn::Right {
            self.Direction += 1;
        }
    }
}

fn main() {
    let mut loc = (0, 0);
    let mut direction = Direction::North;
    direction.turn(Turn::Left);
    println!("{:#?}", &direction);
}
