use std::{cell::RefCell, rc::Rc, rc::Weak, borrow::{BorrowMut, Borrow}};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Type {
    #[default] None,
    Number,
    Symbol,
}

const NONE_SYMBOL: &str = ".";


#[derive(Clone, Debug, Default)]
pub struct Token {
    typ: Type,
    value: String,

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

    fn add_neighbor(&self, token: &Rc<Token>, pos: NeighborPosition) {
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
        // NeighborPosition::TopMiddle => (*self.top_middle.borrow_mut()) = Rc::downgrade(token),
    }

    fn get_neighbor(&self, pos: NeighborPosition) -> Option<Rc<Token>> {
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
        self.get_neighbor(NeighborPosition::CenterLeft).unwrap_or_default().typ != Type::Number && 
            self.get_neighbor(NeighborPosition::CenterRight).unwrap_or_default().typ == Type::Number
    }
    fn is_end(&mut self) -> bool {
        self.get_neighbor(NeighborPosition::CenterLeft).unwrap_or_default().typ == Type::Number && 
            self.get_neighbor(NeighborPosition::CenterRight).unwrap_or_default().typ != Type::Number
    }

    fn has_symbol_adjacent(&mut self) -> bool {
        self.get_neighbor(NeighborPosition::TopMiddle).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::TopRight).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::CenterRight).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::BottomRight).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::BottomMiddle).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::BottomLeft).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::CenterLeft).unwrap_or_default().typ == Type::Symbol ||
        self.get_neighbor(NeighborPosition::TopLeft).unwrap_or_default().typ == Type::Symbol
    }
}

// tl tm tr
// cl -- cr
// bl bm br
#[derive(Default, Debug)]
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

pub fn run(input_file: &String) -> Result<i32, Box<dyn std::error::Error>> {
    // let a = Rc::new(Token::new("3".to_owned()));
    // let b = Rc::new(Token::new(".".to_owned()));
    // let c = Rc::new(Token::new("#".to_owned()));

    // *a.center_right.borrow_mut() = Rc::downgrade(&b);
    // *b.center_left.borrow_mut() = Rc::downgrade(&a);
    // *b.center_right.borrow_mut() = Rc::downgrade(&c);
    // *c.center_left.borrow_mut() = Rc::downgrade(&b);
    // println!("a == {:?}", a);
    // println!("b == {:?}", b);
    // println!("c == {:?}", c);

    // let left_of_b = b.center_left.borrow().upgrade();
    // println!("left_of_b == {:?}", left_of_b);

    // let left_of_c = c.center_left.borrow().upgrade();
    // let left_left_of_c = c.center_left.borrow().upgrade().unwrap().center_left.borrow().upgrade();
    // println!("left_of_c == {:?}", left_of_c);
    // println!("left_left_of_c == {:?}", left_left_of_c);

    // let mut tokens: Vec<Rc<Token>> = Vec::new();
    // tokens.push(Rc::new(Token::new("3".to_owned())));
    // tokens.push(Rc::new(Token::new(".".to_owned())));
    // tokens.push(Rc::new(Token::new("#".to_owned())));

    // (*tokens[0].center_right.borrow_mut()) = Rc::downgrade(&tokens[1]);
    // (*tokens[1].center_left.borrow_mut()) = Rc::downgrade(&tokens[0]);
    // (*tokens[1].center_right.borrow_mut()) = Rc::downgrade(&tokens[2]);
    // (*tokens[2].center_left.borrow_mut()) = Rc::downgrade(&tokens[1]);

    // println!("tokens[0] == {:?}", tokens[0]);
    // println!("tokens[1] == {:?}", tokens[1]);
    // println!("tokens[2] == {:?}", tokens[2]);

    // let left = &tokens[2].center_left;
    // let leftleft = &tokens[2].center_left.borrow().upgrade().unwrap().center_left;
    // println!("left of tokens[2] == {:?}", left.borrow().upgrade());
    // println!("leftleft of tokens[2] == {:?}", leftleft.borrow().upgrade());


    let mut tokens: Vec<Rc<Token>> = Vec::new();
    tokens.push(Rc::new(Token::new("3".to_owned())));
    tokens.push(Rc::new(Token::new(".".to_owned())));
    tokens.push(Rc::new(Token::new("#".to_owned())));

    tokens[0].add_neighbor(&tokens[1], NeighborPosition::CenterRight);
    tokens[1].add_neighbor(&tokens[0], NeighborPosition::CenterLeft);
    tokens[1].add_neighbor(&tokens[2], NeighborPosition::CenterRight);
    tokens[2].add_neighbor(&tokens[1], NeighborPosition::CenterLeft);

    println!("tokens[0] == {:?}", tokens[0]);
    println!("tokens[1] == {:?}", tokens[1]);
    println!("tokens[2] == {:?}", tokens[2]);

    let left = &tokens[2].center_left;
    let leftleft = &tokens[2].center_left.borrow().upgrade().unwrap().center_left;
    println!("left of tokens[2] == {:?}", left.borrow().upgrade());
    println!("leftleft of tokens[2] == {:?}", leftleft.borrow().upgrade());
    println!("");

    let cleft = tokens[2].get_neighbor(NeighborPosition::CenterLeft).unwrap();
    let cleftleft = tokens[2].get_neighbor(NeighborPosition::CenterLeft).unwrap()
        .get_neighbor(NeighborPosition::CenterLeft);
    println!("left of tokens[2] == {:?}", cleft);
    println!("leftleft of tokens[2] == {:?}", cleftleft);
    println!("");



    Ok(0)
}
