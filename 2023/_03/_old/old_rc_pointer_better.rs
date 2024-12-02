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

    fn get_neighbor(&mut self, pos: NeighborPosition) -> Option<Token> {
        self.neighbors.get(pos)
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
        println!("{} => {}", self.value, self.neighbors.get(NeighborPosition::CenterRight).unwrap().value);
        false
        // if self.is_end() {
        //     return self.has_symbol_adjacent();
        // }
        // self.has_symbol_adjacent() && self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(Token::default()).is_part_number()
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
        // let a = Rc::clone(token);
        // println!("{}", a.take().value);
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
            NeighborPosition::TopMiddle => if let Some(a) = &self.top_middle { 
                let v = a.take();
                println!("no {}", v.value);
                // let cloned = Rc::clone(a);
                // let token = a.take(); self.top_middle = Some(cloned);
                // return Some(token);
            },
            NeighborPosition::TopRight => if let Some(a) = &self.top_right { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.top_right = Some(cloned); 
                return Some(token);
            },
            NeighborPosition::CenterRight => if let Some(a) = &self.center_right { 
                // let cloned = Rc::clone(a);
                // let token = cloned.take(); self.center_right = Some(cloned); 
                // println!("center right {}", token.value);
                // return Some(token);
                // let v = a.take();
                // // println!("{:p}", a);
                // // let v = Rc::try_unwrap(a.to_owned()).ok().unwrap().into_inner();
                // // println!("no {}", v.value);
                // let a = Some(v.clone());
                // self.center_right = Some(Rc::new(RefCell::new(v)));
                // return a;
                let token = a.take(); 
                let copy = token.clone(); // self.center_right = Some();
                self.center_right = Some(Rc::new(RefCell::new(token))); 
                return Some(copy);
            },
            NeighborPosition::BottomRight => if let Some(a) = &self.bottom_right { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.bottom_right = Some(cloned); 
                return Some(token);
            },
            NeighborPosition::BottomMiddle => if let Some(a) = &self.bottom_middle { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.bottom_middle = Some(cloned); 
                return Some(token);
            },
            NeighborPosition::BottomLeft => if let Some(a) = &self.bottom_left { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.bottom_left = Some(cloned); 
                return Some(token);
            },
            NeighborPosition::CenterLeft => if let Some(a) = &self.center_left { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.center_left = Some(cloned); 
                return Some(token);
            },
            NeighborPosition::TopLeft => if let Some(a) = &self.top_left { 
                let cloned = Rc::clone(a);
                let token = a.take(); self.top_left = Some(cloned); 
                return Some(token);
            },
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

        // let token = tokens[j as usize].take();
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y - 1) * cols + x;
        //     if index > 0 { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::TopMiddle); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y - 1) * cols + (x + 1);
        //     if index > 0 && (x + 1) < cols { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::TopRight); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        if let Some(token) = tokens.get(j as usize) {
            let mut t = token.take();
            let index = y * cols + (x + 1);
            // println!("{}, {}, {}, {}", cols, index, x, y);
            if (x + 1) < cols { 
                println!("for {}={}, v:{}|[{}, {}] add right link to index {}", j, index, t.value, x, y, index);
                let n = Rc::clone(&(tokens.get(index as usize).unwrap()));
                println!("{:p}", n);
                println!("{}", Rc::strong_count(&n));
                t.add_neighbor(&n, NeighborPosition::CenterRight);
            }
            tokens[j as usize] = Rc::new(RefCell::new(t));

            //works | pointer is functioning correctly
            let a = tokens.get_mut(j as usize).unwrap();
            let mut b = a.take();
            let v = b.get_neighbor(NeighborPosition::CenterRight).unwrap();
            println!("after reassigning rc {}", v.value);
            tokens[j as usize] = Rc::new(RefCell::new(b));
            // println!("{}", Rc::strong_count(&a));
            println!("");
        }
        if j == 2 {
            break
        }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y + 1) * cols + (x + 1);
        //     if index < ntokens && (x + 1) < cols { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::BottomRight); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y + 1) * cols + x;
        //     if index < ntokens { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::BottomMiddle); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y + 1) * cols + (x - 1);
        //     if index < ntokens && (x - 1) > 0 { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::BottomLeft); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = y * cols + (x - 1);
        //     if (x - 1) > 0 { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::CenterLeft); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        // if let Some(token) = tokens.get_mut(j as usize) {
        //     let mut t = token.take();
        //     let index = (y - 1) * cols + (x - 1);
        //     if index > 0 && (x - 1) > 0 { t.add_neighbor(Rc::clone(&tokens[index as usize]), NeighborPosition::TopLeft); }
        //     tokens[j as usize] = Rc::new(RefCell::new(t));
        // }
        j = j + 1;
    }

    let mut h = 0;
    while h < ntokens {
        // let s = tokens.get_mut(h as usize).unwrap();
        // let mut t = s.take();
        // let mut v = t.get_neighbor(NeighborPosition::CenterRight).unwrap();
        // let v2 = v.get_neighbor(NeighborPosition::CenterRight).unwrap();
        // println!("{} -> {} -> {}", t.value, v.value, v2.value);
        // // if t.is_part_number() {
        // //     println!("is part number: true");
        // // }
        // tokens[h as usize] = Rc::new(RefCell::new(t));


        // tokens[h as usize];
        // let t = Rc::clone(tokens.get_mut(h as usize).unwrap());
        // let val = Rc::try_unwrap(t).ok().unwrap().into_inner().value;
        // println!("{}", val);
        // if tokens[h as usize].is_part_number() {
        //     let a = tokens[h as usize].get_part_number()?;
        //     println!("{}", a);
        // }
        // break
        if h == 2 {
            break
        }
        h = h + 1;
    }

    Ok(0)
}
