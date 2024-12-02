use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct WastelandError(pub String);
impl Error for WastelandError {}
impl fmt::Display for WastelandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the wasteland is lost: {}", self.0)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Key {pub key: String}
impl TryFrom<&str> for Key {
    type Error = WastelandError;
    fn try_from(value: &str) -> Result<Self, WastelandError> {
        if value.len() != 3 { return Err(WastelandError(format!("given '{value}' is not length of three")))}
        Ok(Self { key: value.to_string() })
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.key)
    }
}

pub enum Instruction { Left, Right }
impl TryFrom<char> for Instruction {
    type Error = WastelandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => { return Err(WastelandError(format!("invalid char found {value}"))) }
        })
    }
}

#[derive(Clone)]
pub struct Tuple {
    left: Key, 
    right: Key
}
impl Tuple {
    pub const fn new(left: Key, right: Key) -> Self {
        Self{left, right}
    }

    pub fn get_direction(self, inst: &Instruction) -> Key {
        match inst {
            Instruction::Left => self.left,
            Instruction::Right => self.right
        }
    }
}

pub struct Wasteland {
    pub instructions: Vec<Instruction>,
    pub network: HashMap<Key, Tuple>
}
impl TryFrom<String> for Wasteland{
    type Error = WastelandError;

    fn try_from(body: String) -> Result<Self, Self::Error> {
        let mut instructions = Vec::<Instruction>::new();
        let mut network = HashMap::<Key, Tuple>::new();
        
        let lines = body.lines();
        let iterator = lines.clone().enumerate();
        for (i, line) in iterator {
            if i == 0 {
                instructions = line.trim().chars().map(Instruction::try_from)
                    .collect::<Result<Vec<Instruction>, WastelandError>>()?;
                continue
            }
            if i == 1 {
                continue
            }

            let (key, tuple_string) = line.split_at(5);
            let (left, right) = tuple_string.trim().split_at(4);
            network.insert(
                Key::try_from(key[..key.len()-1].trim())?, 
                Tuple::new(Key::try_from(left[1..].trim())?, Key::try_from(right[1..right.len()-1].trim())?));
        }
        assert_eq!(lines.count()-2, network.len()); // ignore first two lines
        
        Ok(Self::new(instructions, network))
    }
}
impl Wasteland {
    pub const fn new(instructions: Vec<Instruction>, network: HashMap<Key, Tuple>) -> Self{
        Self{instructions, network}
    }
}