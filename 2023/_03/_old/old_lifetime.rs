use std::fs::read_to_string;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub enum Type {
    #[default] None,
    Number,
    Symbol,
}

const NONE_SYMBOL: &str = ".";


#[derive(Clone, Default)]
pub struct Token<'a> {
    typ: Type,
    value: String,

    neighbors: Neighborhood<'a>,
}

impl<'a> Token<'a> {
    fn new(value: String) -> Token<'a> {
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
    
    fn add_neighbor(&'a mut self, token: &'a mut Token, pos: NeighborPosition) {
        self.neighbors.add(token, pos)
    }

    fn is_begin(&'a self) -> bool {
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(&Token::default()).typ != Type::Number && 
            self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(&Token::default()).typ == Type::Number
    }

    fn is_end(&'a self) -> bool {
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(&Token::default()).typ == Type::Number && 
            self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(&Token::default()).typ != Type::Number
    }

    fn has_symbol_adjacent(&'a self) -> bool {
        self.neighbors.get(NeighborPosition::TopMiddle).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::TopRight).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomRight).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomMiddle).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::BottomLeft).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::CenterLeft).unwrap_or(&Token::default()).typ == Type::Symbol ||
        self.neighbors.get(NeighborPosition::TopLeft).unwrap_or(&Token::default()).typ == Type::Symbol
    }

    fn is_part_number(&'a self) -> bool {
        if self.is_end() {
            return self.has_symbol_adjacent();
        }
        self.has_symbol_adjacent() && self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(&Token::default()).is_part_number()
    }

    fn get_part_number(&'a self) -> Result<i32, String> {
        if !self.is_begin() {
            return Err("is not begin of number".to_string());
        }
        let pn = self._get_part_number();
        pn.parse::<i32>().map_err(|e| e.to_string())
    }

    fn _get_part_number(&'a self) -> String {
        if self.is_end() {
            return self.value.to_owned()
        }
        [self.value.to_owned(), self.neighbors.get(NeighborPosition::CenterRight).unwrap_or(&Token::default())._get_part_number()].join("")
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

// TODO: work with Options in Neighborhood
#[derive(Clone)]
pub struct Neighborhood<'a> {
    top_middle: Option<&'a Token<'a>>,
    top_right: Option<&'a Token<'a>>,
    center_right: Option<&'a Token<'a>>,
    bottom_right: Option<&'a Token<'a>>,
    bottom_middle: Option<&'a Token<'a>>,
    bottom_left: Option<&'a Token<'a>>,
    center_left: Option<&'a Token<'a>>,
    top_left: Option<&'a Token<'a>>,
}


impl<'a> Default for Neighborhood<'a> {
    fn default() -> Self {
        Self { 
            top_middle: None,
            top_right: None,
            center_right: None,
            bottom_right: None,
            bottom_middle: None,
            bottom_left: None,
            center_left: None,
            top_left: None,
        }
    }
}


impl<'a> Neighborhood<'a> {
    fn add(&'a mut self, token: &'a Token, pos: NeighborPosition) {
        match pos {
            NeighborPosition::TopMiddle => self.top_middle = Some(token),
            NeighborPosition::TopRight => self.top_right = Some(token),
            NeighborPosition::CenterRight => self.center_right = Some(token),
            NeighborPosition::BottomRight => self.bottom_right = Some(token),
            NeighborPosition::BottomMiddle => self.bottom_middle = Some(token),
            NeighborPosition::BottomLeft => self.bottom_left = Some(token),
            NeighborPosition::CenterLeft => self.center_left = Some(token),
            NeighborPosition::TopLeft => self.top_left = Some(token),
        }
    }

    fn get(&'a self, pos: NeighborPosition) -> Option<&Token> {
        match pos {
            NeighborPosition::TopMiddle => self.top_middle,
            NeighborPosition::TopRight => self.top_right,
            NeighborPosition::CenterRight => self.center_right,
            NeighborPosition::BottomRight => self.bottom_right,
            NeighborPosition::BottomMiddle => self.bottom_middle,
            NeighborPosition::BottomLeft => self.bottom_left,
            NeighborPosition::CenterLeft => self.center_left,
            NeighborPosition::TopLeft => self.top_left,
        }
    }
}

fn add<'a> (tokens: &'a mut Vec<Token<'a>>, i: usize, neighbor: &'a mut Token, pos: NeighborPosition) {
    tokens[i].add_neighbor(neighbor, pos)
} 

fn add_token<'a> (token: &'a mut Token<'a>, neighbor: &'a mut Token, pos: NeighborPosition) {
    token.add_neighbor(neighbor, pos)
} 

pub fn run(input_file: &String) -> Result<i32, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines: Vec<String> = body.lines().map(String::from).collect();
    assert_ne!(0, lines.len());

    let mut tokens: Vec<Token> = Vec::new();
    let mut nchars: usize = 0;
    let mut i: usize = 0;
    while i < lines.len() {
        let chars: Vec<String> = lines[i].chars().map(String::from).collect();
        if i > 1 {
            assert_eq!(nchars, chars.len())
        } else {
            nchars = chars.len();
        }
        tokens.push(Token::new(chars[i].to_owned()));
        i = i + 1;
    }

    // fill neighborhood
    let mut j = 0;
    let mut a = Token::new(String::from("hello"));
    while j < i {
        // let (left, _) = tokens.split_at_mut(j);
        // left[j].neighbors.add(&a, NeighborPosition::BottomLeft);
        tokens[j].add_neighbor(&mut a, NeighborPosition::BottomLeft);
        // let (left, right) = tokens.split_at_mut(j);
        // add_token(&mut left[0], &mut right[0], NeighborPosition::BottomLeft);
        // tokens[j].add_neighbor(&mut tokens[j], NeighborPosition::BottomLeft);
        // let (from, to) = {
        //     
        //     (&mut left[i], &mut right[0])
        // };
        // add_token(from, to, NeighborPosition::BottomLeft);
        // add(&mut tokens, j, &mut tokens[j+1], NeighborPosition::BottomLeft);
        j = j + 1;
    }
    // add(&mut tokens, j, &mut tokens[j+1], NeighborPosition::BottomLeft); // works

    // for t in tokens.iter_mut() {
    //     t.add_neighbor(&mut tokens[j], NeighborPosition::BottomLeft);
    // }

    Ok(0)

    // let mut valid_games_sum: u32 = 0;
    // for line in lines {
    //     let id_sets: Vec<&str> = line.split(":").collect();
    //     assert_eq!(id_sets.len(), 2);

    //     let game_id: u32 = match id_sets[0].strip_prefix("Game ").unwrap().parse::<u32>() {
    //         Ok(i) => i,
    //         Err(e) => return Err(Box::new(e))
    //     };

    //     let mut valid = true;
    //     for game_set in id_sets[1].split(";").collect::<Vec<&str>>() {
    //         for cube in game_set.trim().split(",").collect::<Vec<&str>>() {
    //             let count_color = cube.trim().split_whitespace().collect::<Vec<&str>>();
    //             assert_eq!(count_color.len(), 2);
}
