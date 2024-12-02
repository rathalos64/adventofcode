use std::borrow::BorrowMut;
use std::fs::read_to_string;
use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Token>>>;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub enum Type {
    #[default] None,
    Number,
    Symbol,
}

const NONE_SYMBOL: &str = ".";


#[derive(Clone, Default)]
pub struct Token {
    typ: Type,
    value: String,

    neighbors: Neighborhood,
}

impl Token {
    fn new(value: String) -> Token {
        let typ: Type = if value == NONE_SYMBOL {
            Type::None 
        } else if value.parse::<u8>().is_ok() {
            Type::Number
        } else {
            Type::Symbol
        };

        let mut token = Token::default();
        token.typ = typ;
        token.value = value;
        token
    }
    
    fn add_neighbor(&mut self, token: &Rc<RefCell<Token>>, pos: NeighborPosition) {
        self.neighbors.add(token, pos)
    }

    fn is_begin(&mut self) -> bool {
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(Token::default()).typ != Type::Number && 
            self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default()).typ == Type::Number
    }

    fn is_end(&mut self) -> bool {
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(Token::default()).typ == Type::Number && 
            self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default()).typ != Type::Number
    }

    fn has_symbol_adjacent(&mut self) -> bool {
        self.neighbors.get(NeighborPosition::TopMiddle).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::TopRight).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomRight).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomMiddle).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomLeft).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::TopLeft).unwrap_or(Token::default()).typ == Type::Symbol
    }

    fn is_part_number(&mut self) -> bool {
        if self.is_end() {
            return self.has_symbol_adjacent();
        }
        self.has_symbol_adjacent() && self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default()).is_part_number()
    }

    fn get_part_number(&mut self) -> Result<i32, String> {
        if !self.is_begin() {
            return Err("is not begin of number".to_string());
        }
        let pn = self._get_part_number();
        pn.parse::<i32>().map_err(|e| e.to_string())
    }

    fn _get_part_number(&mut self) -> String {
        if self.is_end() {
            return self.value.to_owned()
        }
        [self.value.to_owned(), self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default())._get_part_number()].join("")
    }
}

// tl tm tr
// cl -- cr
// bl bm br
#[derive(Default)]
pub enum NeighborPosition {
    #[default] TopMiddle,
    TopRight,
    CenterRight,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    CenterLeft,
    TopLeft,
}

#[derive(Clone, Default)]
pub struct Neighborhood {
    top_middle: Link,
    top_right: Link,
    center_right: Link,
    bottom_right: Link,
    bottom_middle: Link,
    bottom_left: Link,
    center_left: Link,
    top_left: Link,
}


// https://rtoch.com/posts/rust-doubly-linked-list/
impl Neighborhood {
    fn add(&mut self, token: &Rc<RefCell<Token>>, pos: NeighborPosition) {
        match pos {
            NeighborPosition::TopMiddle => self.top_middle = Some(Rc::clone(token)),
            NeighborPosition::TopRight => self.top_right = Some(Rc::clone(token)),
            NeighborPosition::CenterRight => self.center_right = Some(Rc::clone(token)),
            NeighborPosition::BottomRight => self.bottom_right = Some(Rc::clone(token)),
            NeighborPosition::BottomMiddle => self.bottom_middle = Some(Rc::clone(token)),
            NeighborPosition::BottomLeft => self.bottom_left = Some(Rc::clone(token)),
            NeighborPosition::CenterLeft => self.center_left = Some(Rc::clone(token)),
            NeighborPosition::TopLeft => self.top_left = Some(Rc::clone(token)),
        }
    }

    fn get(&mut self, pos: NeighborPosition) -> Option<Token> {
        match pos {
            NeighborPosition::TopMiddle => if let Some(a) = self.top_middle.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::TopRight => if let Some(a) = self.top_right.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::CenterRight => if let Some(a) = self.center_right.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::BottomRight => if let Some(a) = self.bottom_right.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::BottomMiddle => if let Some(a) = self.bottom_middle.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::BottomLeft => if let Some(a) = self.bottom_left.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::CenterLeft => if let Some(a) = self.center_left.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
            NeighborPosition::TopLeft => if let Some(a) = self.top_left.borrow_mut().take() { Some(Rc::try_unwrap(a).ok().unwrap().into_inner()); },
        }
        None
    }
}

pub fn run(input_file: &String) -> Result<i32, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut tokens: Vec<Rc<RefCell<Token>>> = Vec::new();
    let mut nchars: usize = 0;
    let mut i: usize = 0;
    while i < lines.len() {
        let chars: Vec<String> = lines[i].chars().map(String::from).collect();
        if i > 1 {
            assert_eq!(nchars, chars.len())
        } else {
            nchars = chars.len();
        }
        for char in chars.iter() {
            tokens.push(Rc::new(RefCell::new(Token::new(char.to_owned()))));
        }
        i = i + 1;
    }

    // fill neighborhood
    let mut j = 0;
    let cols = nchars as i32;
    let ntokens = (tokens.len()) as i32;
    // https://www.reddit.com/r/rust/comments/zi5e03/cannot_borrow_as_mutable_more_than_once_at_a_time/
    while j < ntokens {
        let y: i32 = j as i32 / cols as i32;
        let x: i32 = j as i32 % cols as i32;
        // println!("{} {}", y, x);
        // println!("{}", tokens[j as usize].take().value);

        let token = tokens[j].take();
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y - 1) * cols + x;
            if index > 0 { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::TopMiddle); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y - 1) * cols + (x + 1);
            if index > 0 && (x + 1) < cols { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::TopRight); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = y * cols + (x + 1);
            if (x + 1) < cols { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::CenterRight); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y + 1) * cols + (x + 1);
            if index < ntokens && (x + 1) < cols { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::BottomRight); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y + 1) * cols + x;
            if index < ntokens { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::BottomMiddle); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y + 1) * cols + (x - 1);
            if index < ntokens && (x - 1) > 0 { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::BottomLeft); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = y * cols + (x - 1);
            if (x - 1) > 0 { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::CenterLeft); }
        }
        if let Some(token) = tokens.get_mut(j as usize) {
            let index = (y - 1) * cols + (x - 1);
            if index > 0 && (x - 1) > 0 { token.add_neighbor(&Rc::new(RefCell::new(tokens[index as usize])), NeighborPosition::TopLeft); }
        }
        j = j + 1;
    }

    let h = 0;
    while h < ntokens {
        println!("{}", tokens[h as usize].value);
        if tokens[h as usize].is_part_number() {
            let a = tokens[h as usize].get_part_number()?;
            println!("{}", a);
        }
        break
    }

    Ok(0)
}
