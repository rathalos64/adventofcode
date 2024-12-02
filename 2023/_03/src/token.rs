use std::{cell::RefCell, rc::Rc, rc::Weak};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
// https://jacksonshi.vercel.app/blog/rust/rc-weak-rust

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Type {
    #[default] None,
    Number,
    Symbol,
}

// tl tm tr
// cl -- cr
// bl bm br
#[derive(Default, Clone, Copy, Debug, PartialEq, EnumIter)]
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

const NONE_SYMBOL: &str = ".";
const GEAR_SYMBOL: &str = "*";

#[derive(Clone, Debug, Default)]
pub struct Grid {
    pub tokens: Vec<Rc<Token>>,
    cols: i32,
}

impl Grid {
    pub fn append(&mut self, v: &Vec<String>) -> Result<(), String> {
        if self.cols > 0 && (self.cols != v.len() as i32) {
            return Err("given array deviates to cols".to_owned())
        } 
        self.cols = v.len() as i32;

        for char in v.iter() {
            self.tokens.push(Rc::new(Token::new(char.to_owned())));
        }
        Ok(())
    }

    pub fn populate(&mut self) -> Result<(), String> {
        let ntokens: i32 = self.tokens.len() as i32;
        if ntokens % (self.cols as i32) != 0 {
            return Err("not evenly spaced grod".to_owned())
        }

        for i in 0..ntokens {
            let y: i32 = i as i32 / self.cols as i32;
            let x: i32 = i as i32 % self.cols as i32;
            let token = &self.tokens[i as usize];

            // start adding the neighbors
            for pos in NeighborPosition::iter() { // nice crate
                if let Some(index) = match pos {
                    NeighborPosition::TopMiddle => {
                        let index = (y - 1) * self.cols + x;
                        if index >= 0 { Some(index) } else { None }
                    },
                    NeighborPosition::TopRight => {
                        let index = (y - 1) * self.cols + (x + 1);
                        if index >= 0 && (x + 1) < self.cols { Some(index) } else { None }
                    },        
                    NeighborPosition::CenterRight => {
                        let index = y * self.cols + (x + 1);
                        if (x + 1) < self.cols { Some(index) } else { None }
                    },    
                    NeighborPosition::BottomRight => {
                        let index = (y + 1) * self.cols + (x + 1);
                        if index < ntokens && (x + 1) < self.cols { Some(index) } else { None }
                    },        
                    NeighborPosition::BottomMiddle => {
                        let index = (y + 1) * self.cols + x;
                        if index < ntokens { Some(index) } else { None }
                    },        
                    NeighborPosition::BottomLeft => {
                        let index = (y + 1) * self.cols + (x - 1);
                        if index < ntokens && (x - 1) >= 0 { Some(index) } else { None }
                    },        
                    NeighborPosition::CenterLeft => {
                        let index = y * self.cols + (x - 1);
                        if (x - 1) >= 0 { Some(index) } else { None }
                    },        
                    NeighborPosition::TopLeft => {
                        let index = (y - 1) * self.cols + (x - 1);
                        if index >= 0 && (x - 1) >= 0 { Some(index) } else { None }
                    },
                } {
                    token.add_neighbor(&self.tokens[index as usize], pos);
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Token {
    typ: Type,
    pub value: String,

    top_middle: RefCell<Weak<Token>>,
    top_right: RefCell<Weak<Token>>,
    center_right: RefCell<Weak<Token>>,
    bottom_right: RefCell<Weak<Token>>,
    bottom_middle: RefCell<Weak<Token>>,
    bottom_left: RefCell<Weak<Token>>,
    center_left: RefCell<Weak<Token>>,
    top_left: RefCell<Weak<Token>>,
}

impl Token {
    pub fn new(value: String) -> Token {
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

    pub fn add_neighbor(&self, token: &Rc<Token>, pos: NeighborPosition) {
        let downgraded = Rc::downgrade(token);
        let link = match pos {
            NeighborPosition::TopMiddle => &self.top_middle,
            NeighborPosition::TopRight => &self.top_right,
            NeighborPosition::CenterRight => &self.center_right,
            NeighborPosition::BottomRight => &self.bottom_right,
            NeighborPosition::BottomMiddle => &self.bottom_middle,
            NeighborPosition::BottomLeft => &self.bottom_left,
            NeighborPosition::CenterLeft => &self.center_left,
            NeighborPosition::TopLeft => &self.top_left,
        };
        (*link.borrow_mut()) = downgraded;
    }

    pub fn get_neighbor(&self, pos: NeighborPosition) -> Option<Rc<Token>> {
        let link = match pos {
            NeighborPosition::TopMiddle => &self.top_middle,
            NeighborPosition::TopRight => &self.top_right,
            NeighborPosition::CenterRight => &self.center_right,
            NeighborPosition::BottomRight => &self.bottom_right,
            NeighborPosition::BottomMiddle => &self.bottom_middle,
            NeighborPosition::BottomLeft => &self.bottom_left,
            NeighborPosition::CenterLeft => &self.center_left,
            NeighborPosition::TopLeft => &self.top_left,
        };
        link.borrow().upgrade()
    }    

    fn is_begin(&self) -> bool {
        self.typ == Type::Number && self.get_neighbor(NeighborPosition::CenterLeft).unwrap_or_default().typ != Type::Number 
    }
    fn is_end(&self) -> bool {
        self.typ == Type::Number && self.get_neighbor(NeighborPosition::CenterRight).unwrap_or_default().typ != Type::Number
    }

    fn has_adjacent(&self, typ: Type) -> bool {
        self.get_neighbor(NeighborPosition::TopMiddle).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::TopRight).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::CenterRight).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::BottomRight).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::BottomMiddle).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::BottomLeft).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::CenterLeft).unwrap_or_default().typ == typ ||
        self.get_neighbor(NeighborPosition::TopLeft).unwrap_or_default().typ == typ
    }

    fn get_adjacent(&self, typ: Type) -> Vec<Rc<Token>> {
        let mut adjacent = Vec::new();
        let mut poss: Vec<NeighborPosition> = Vec::new(); 
        for pos in NeighborPosition::iter() {
            if let Some(neighbor) = self.get_neighbor(pos) {
                if neighbor.typ == typ {
                    poss.push(pos);
                }
            }
        }

        // keep in top and bottom only one digit
        // crappy logic, don't think about it too much
        if poss.contains(&NeighborPosition::BottomRight) && 
            poss.contains(&NeighborPosition::BottomMiddle) && 
            poss.contains(&NeighborPosition::BottomLeft) {
            poss.retain(|&x| x != NeighborPosition::BottomMiddle);
            poss.retain(|&x| x != NeighborPosition::BottomLeft);

        } else if poss.contains(&NeighborPosition::BottomRight) && 
            poss.contains(&NeighborPosition::BottomMiddle) {
            poss.retain(|&x| x != NeighborPosition::BottomMiddle);

        } else if poss.contains(&NeighborPosition::BottomMiddle) && 
            poss.contains(&NeighborPosition::BottomLeft) {
            poss.retain(|&x| x != NeighborPosition::BottomMiddle);
        }

        if poss.contains(&NeighborPosition::TopRight) && 
            poss.contains(&NeighborPosition::TopMiddle) && 
            poss.contains(&NeighborPosition::TopLeft) {
            poss.retain(|&x| x != NeighborPosition::TopMiddle);
            poss.retain(|&x| x != NeighborPosition::TopLeft);

        } else if poss.contains(&NeighborPosition::TopRight) && 
            poss.contains(&NeighborPosition::TopMiddle) {
            poss.retain(|&x| x != NeighborPosition::TopMiddle);

        } else if poss.contains(&NeighborPosition::TopMiddle) && 
            poss.contains(&NeighborPosition::TopLeft) {
            poss.retain(|&x| x != NeighborPosition::TopMiddle);
        }

        for pos in poss.iter() {
            if let Some(neighbor) = self.get_neighbor(*pos) {
                adjacent.push(neighbor);
            }
        }
        adjacent
    }

    pub fn is_part_number(&self) -> bool {
        self.is_begin() && self._is_part_number()
    }
    fn _is_part_number(&self) -> bool {
        if self.is_end() {
            return self.has_adjacent(Type::Symbol);
        }
        self.has_adjacent(Type::Symbol) || match self.get_neighbor(NeighborPosition::CenterRight) {
            Some(neighbor) => neighbor._is_part_number(),
            None => false
        }
    }

    pub fn get_part_number(&self) -> Result<i32, String> {
        if !self.is_part_number() {
            return Err("is not a part number".to_owned());
        }
        self._get_part_number().parse::<i32>().map_err(|e| e.to_string())
    }

    fn get_part_number_beginning(&self) -> Result<i32, String> {
        if self.typ != Type::Number {
            return Err("start position not a number, can't traverse".to_owned());
        }
        if self.is_begin() {
            return self.get_part_number();
        }
        match self.get_neighbor(NeighborPosition::CenterLeft) {
            Some(neighbor) => neighbor.get_part_number_beginning(),
            None => return Err("failed to find begin".to_owned())
        }
    }

    fn _get_part_number(&self) -> String {
        if self.is_end() {
            return self.value.to_owned()
        }
        [self.value.to_owned(), self.get_neighbor(NeighborPosition::CenterRight)
            .unwrap_or_default()._get_part_number()].join("")
    }

    pub fn is_gear(&self) -> bool {
        self.typ == Type::Symbol && self.value == GEAR_SYMBOL &&
            self.has_adjacent(Type::Number) && self.get_adjacent(Type::Number).len() == 2 // cheeky, every adjacent number must be a part number
    }

    pub fn get_gear_ratio(&self) -> Result<i32, String> {
        if !self.is_gear() {
            return Err("is not a part number".to_owned());
        }
        let adjacent = self.get_adjacent(Type::Number);
        let lhs = adjacent[0].get_part_number_beginning()?;
        let rhs = adjacent[1].get_part_number_beginning()?;
        Ok(lhs * rhs)
    }

}