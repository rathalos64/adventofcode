use rayon::prelude::*;
use std::iter::FromIterator;

const LANTERN_FISH_NEW_STATE: u32 = 8;
const LANTERN_FISH_RESET_STATE: u32 = 6;

#[derive(Clone, Debug)]
pub struct LanternFish {
    __state: u32,
    __reset: bool
}

impl LanternFish {
    fn new() -> Self { Self::new_with_state(LANTERN_FISH_NEW_STATE) }

    pub fn new_from_string(state: &str) -> Result<Self, String> {
        match String::from(state).parse::<u32>(){
            Ok(parsed) => return Ok(Self::new_with_state(parsed)),
            Err(_)     => return Err(String::from("failed to parse x coordinate"))
        };
    }

    pub fn new_with_state(state: u32) -> Self { Self{ __state: state, __reset: false } }

    fn pass_day(&mut self) {
        if self.__state == 0 { 
            self.__reset = true;
            self.__state = LANTERN_FISH_RESET_STATE; // has completed one cycle
        } 
        else { 
            self.__reset = false;
            self.__state -= 1; 
        }
    }

    fn has_changed(&self) -> bool {
        self.__reset == true
    }
}

#[derive(Clone, Debug)]
pub struct LanternSchool {
    __fish: Vec<LanternFish>,
    __passed_days: i32
}

impl LanternSchool {
    pub fn new_with_states(states: Vec<u32>) -> Self {
        Self::new_from_fish(states.iter().map(|fish| LanternFish::new_with_state(*fish)).collect())
    }

    pub fn new_from_fish(fish: Vec<LanternFish>) -> Self {
        Self{ __fish: fish,  __passed_days: 0 }
    }

    pub fn size(&self) -> usize { self.__fish.len() }

    pub fn pass_day(&mut self) {
        self.__passed_days += 1;
        self.__fish.par_iter_mut().for_each(|fish| (*fish).pass_day()); // the days go by

        // new fishies are born
        let newborn: usize = self.__fish.par_iter().filter(|fish| fish.has_changed()).count();
        self.__fish.extend((0..newborn).map(|_| LanternFish::new()).collect::<Vec<LanternFish>>());
    }
}

impl FromIterator<LanternFish> for LanternSchool {
    fn from_iter<I: IntoIterator<Item=LanternFish>>(iter: I) -> Self {
        let mut to_create: Vec<LanternFish> = Vec::new();

        for i in iter {
            to_create.push(i);
        }

        LanternSchool::new_from_fish(to_create)
    }
}

pub struct LanternResult {
    pub fish: Vec<u64>,
    pub lambdas: Vec<f64>,
    pub days: u64
}

impl LanternResult {
    pub fn new() -> Self {
        Self {fish: Vec::new(), lambdas: Vec::new(), days: 0}
    }

    pub fn add(&mut self, fish: u64, lambda: f64) {
        self.fish.push(fish);
        self.lambdas.push(lambda);
        self.days += 1;
    }
}