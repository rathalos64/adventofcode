use std::fs;

#[derive(Clone, Copy, Debug)]
struct Pair {
    number: i32,
    checked: bool,
}

#[derive(Clone)]
struct Board {
    __container: Vec<Pair>,
    __rows: Vec<Vec<Pair>>, // I really would have liked to save them as references
    __cols: Vec<Vec<Pair>>, // I really would have liked to save them as references
    __won: bool,

    __capacity: usize,
    __idx: usize,
}

impl Board {
    fn new(capacity: usize) -> Result<Self, String> {
        if capacity == 0 {
            return Err(String::from("capacity must not need be lower than 1"));
        }

        let mut rows: Vec<Vec<Pair>> = Vec::with_capacity(capacity);
        let mut cols: Vec<Vec<Pair>> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            rows.push(Vec::with_capacity(capacity));
            cols.push(Vec::with_capacity(capacity));
        }

        Ok(Self{
            __container: Vec::with_capacity(capacity * capacity),
            __rows: rows,
            __cols: cols,
            __won: false,

            __capacity: capacity,
            __idx: 0,
        })
    }

    fn cap(&self) -> usize {
        self.__capacity
    }

    fn wellformed(&self) -> bool {
        let rows: i32 = self.__rows.iter().map(|row| row.len() as i32).sum::<i32>();
        let cols: i32 = self.__cols.iter().map(|row| row.len() as i32).sum::<i32>();
        
        (rows + cols) == ((self.__capacity * self.__capacity) as i32) + self.__idx as i32
    }

    fn enter(&mut self, number: i32) -> Result<(), &str> {
        if self.__idx == self.__capacity * self.__capacity {
            return Err("bingo is already full")
        }

        let pair: Pair = Pair{number: number, checked: false};
        self.__container.push(pair);

        let row_idx: usize = f64::floor(self.__idx as f64 / self.__capacity as f64) as usize;
        let col_idx: usize = self.__idx % self.__capacity;
        
        self.__rows[row_idx].push(pair);
        self.__cols[col_idx].push(pair);

        self.__idx += 1;

        Ok(())
    }

    fn draw(&mut self, number: i32) {
        for i in 0..(self.__capacity * self.__capacity) {
            let row_idx: usize = f64::floor(i as f64 / self.__capacity as f64) as usize;
            let col_idx: usize = i % self.__capacity;

            if self.__container[i].number == number {
                self.__container[i].checked = true
            }
            if self.__rows[row_idx][col_idx].number == number {
                self.__rows[row_idx][col_idx].checked = true
            }
            if self.__cols[col_idx][row_idx].number == number {
                self.__cols[col_idx][row_idx].checked = true
            }
        }

        self.__check_win()
    }

    fn __check_win(&mut self) {
        self.__won = self.__rows.iter()
            .map(|row| row.iter().map(|e| (*e).checked).fold(true, |acc, e| acc && e))
            .fold(false, |acc, row| acc || row)  
        || self.__cols.iter()
            .map(|col| col.iter().map(|e| (*e).checked).fold(true, |acc, e| acc && e))
            .fold(false, |acc, col| acc || col) 
    }

    fn won(&self) -> bool {
        self.__won
    }

    fn get_unmarked(&self) -> Vec<i32> {
        self.__container.iter()
            .filter(|n| n.checked == false)
            .map(|n| n.number)
            .collect()
    }
}

pub struct Game {
    __boards: Vec<Board>,
    __already_won: Vec<usize>,
    __last_draw: i32
}

impl Game {
    pub fn init(input_path: String) -> Result<Self, String> {
        let mut lines: Vec<String> = fs::read_to_string(input_path)
            .unwrap_or_default()
            .split('\n')
            .map(|line| String::from(line))
            .collect();
        if lines.len() == 0 {
            return Err(String::from("file does not contain lines"));
        }
        lines.push(String::from("")); // for convenience

        let mut boards: Vec<Board> = Vec::new();
        let mut curr: Board = Board::new(1)?;
        let mut reading_board = false;
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                continue
            }

            if line == "" {
                if reading_board {
                    if !curr.wellformed() {
                        return Err(String::from("current board is not wellformed",))
                    }
                    boards.push(curr.clone());
                    reading_board = false; // we are finished with our current board
                }

                continue
            }

            let row: Vec<i32> = line.split_whitespace()
                .filter(|n| *n != "")
                .map(|n| String::from(n).parse::<i32>().unwrap())
                .collect();
            if !reading_board {
                curr = Board::new(row.len())?;
                reading_board = true;
            }

            if reading_board {
                if row.len() != curr.cap() {
                    return Err(String::from("current board does not have the same dimension as the capacity"));
                }

                for n in row {
                    curr.enter(n)?
                }
            }
        }

        Ok(Game {
            __boards: boards,
            __already_won: Vec::new(),
            __last_draw: -1
        })
    }

    pub fn draw(&mut self, draw: i32) {
        for i in 0..self.__boards.len() {
            self.__boards[i].draw(draw);
        }
        self.__last_draw = draw;
    }

    pub fn check_winner(&mut self) -> Option<Vec<(usize, i32)>> {
        let mut winners: Vec<(usize, i32)> = Vec::new();
        for i in 0..self.__boards.len() {
            if self.__already_won.contains(&i) {
                continue
            }

            if self.__boards[i].won() {
                let unmarked: i32 = self.__boards[i].get_unmarked().iter().sum();
                winners.push((i, unmarked * self.__last_draw));
                self.__already_won.push(i);
            }
        }

        if winners.len() > 0 {
            Some(winners)
        } else {
            None
        }
    }
}