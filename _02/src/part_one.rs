use std::fs;

struct Point {
    x: i32,
    y: i32
}

pub struct Submarine {
    __position: Point
}

enum Movement {
    Forward(u32),
    Back(u32),
    Up(u32),
    Down(u32)
}

impl Submarine {
    fn new() -> Self {
        Self{ __position: Point{ x: 0, y: 0 } }
    }

    fn go(&mut self, mv: Movement) {
        match mv {
            Movement::Forward(x) => { self.__position.x = self.__position.x + x as i32 }
            Movement::Back(x) => { self.__position.x = self.__position.x - x as i32 }
            Movement::Up(y) => { self.__position.y = self.__position.y - y as i32 }
            Movement::Down(y) => { self.__position.y = self.__position.y + y as i32 }
        }
    }

    fn go_multiple(&mut self, moves: Vec<Movement>) {
        for mv in moves {
            self.go(mv)
        }
    }

    pub fn horizontal(&self) -> i32 {
        self.__position.x
    }

    pub fn depth(&self) -> i32 {
        self.__position.y
    }
}

pub fn run(input_path: &String) -> Submarine {
    let moves: Vec<Movement> = fs::read_to_string(input_path) // oh boy
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            assert_eq!(split.len(), 2);
            
            let action = split[0]; // consumed after call
            let value = String::from(split[1]).parse::<u32>().unwrap();
            match action {
                "forward" => Ok(Movement::Forward(value)),
                "back" => Ok(Movement::Back(value)),
                "up" => Ok(Movement::Up(value)),
                "down" => Ok(Movement::Down(value)),
                _ => Err("invalid action")
            }
        })
        .filter(|movement| movement.is_ok())
        .map(|movement| movement.unwrap())
        .collect();

    let mut submarine = Submarine::new();
    submarine.go_multiple(moves);
    submarine
}
