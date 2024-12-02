use std::fs;

struct Point {
    x: i32,
    y: i32
}

pub struct Submarine {
    __position: Point,
    __aim: i32
}

#[derive(Clone)]
enum Movement {
    Forward(u32),
    Up(u32),
    Down(u32)
}

const MOVEMENT_FORWARD: &str = "forward";
const MOVEMENT_UP: &str = "up";
const MOVEMENT_DOWN: &str = "down";

impl Submarine {
    fn new() -> Self {
        Self{ __position: Point{ x: 0, y: 0 }, __aim: 0 }
    }

    fn go(&mut self, mv: Movement) {
        match mv {
            Movement::Forward(x) => { 
                self.__position.x = self.__position.x + x as i32;
                self.__position.y = self.__position.y + (self.__aim * x as i32);
            }
            Movement::Up(aim) => { self.__aim = self.__aim - aim as i32 }
            Movement::Down(aim) => { self.__aim = self.__aim + aim as i32 }
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
    let processed: Vec<Result<Movement, &str>> = fs::read_to_string(input_path) // oh boy
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            assert_eq!(split.len(), 2);
            
            let action = split[0];
            let value = {
                match String::from(split[1]).parse::<u32>() {
                    Ok(v) => v,
                    Err(err) => panic!("invalid value given: {}", err)
                }
            };
            match action {
                MOVEMENT_FORWARD => Ok(Movement::Forward(value)),
                MOVEMENT_UP => Ok(Movement::Up(value)),
                MOVEMENT_DOWN => Ok(Movement::Down(value)),
                _ => panic!("invalid action given: {}", action)
            }
        })
        .collect();

    // error handling (eh, a little bit sketchy)
    let errors: Vec<&str> = processed.clone().into_iter()
        .filter(|mv| mv.is_err())
        .map(|mv| mv.err())
        .flatten()
        .collect();
    if errors.len() > 0 {
        panic!("errors were found: {:?}", errors);
    }

    let moves: Vec<Movement> = processed.clone().into_iter()
        .filter(|mv| mv.is_ok())
        .map(|mv| mv.unwrap())
        .collect();

    let mut submarine = Submarine::new();
    submarine.go_multiple(moves);
    submarine
}
