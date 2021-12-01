mod types {

    #[derive(Debug, Clone)]
    pub struct SlidingWindow{
        __idx: usize, // keeps track of current index (prevent overpushing into vec)
        __container: Vec<i32>,
        __capacity: usize
    }

    impl SlidingWindow {
        pub fn new(capacity: usize) -> Result<Self, ()> {
            Self::new_with_value(capacity, 0)
        }

        pub fn new_with_value(capacity: usize, val: i32) -> Result<Self, ()> {
            if capacity == 0 {
                Err(())
            } else {
                let mut vec: Vec<i32> = Vec::with_capacity(capacity);
                vec.push(val);

                Ok(Self{ 
                    __idx: 0, 
                    __container:  vec.clone(), 
                    __capacity: capacity 
                })
            }
        }

        pub fn sum(&self) -> i32 {
            (0..self.__container.len())
                .map(|i| self.__container[i])
                .fold(0, |a, b| a + b)
        }
        
        pub fn len(&self) -> usize {
            self.__container.len()
        }

        pub fn push(&mut self, val: i32) {
            if self.__idx > (self.__capacity-1) {
                return
            }
            self.__container.push(val);
            self.__idx = self.__idx + 1;
        }
    }
}

use std::fs;
use types::SlidingWindow;

pub fn run(input: &String, sliding_size: usize) -> i32 {
    let processed: Vec<i32> = fs::read_to_string(input) // oh boy
        .unwrap_or_default()
        .split('\n')
        .filter(|line| *line != "")
        .map(|line| String::from(line).parse::<i32>())
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect();

    let mut increases: i32 = 0;
    let mut tuples: Vec<SlidingWindow> = Vec::new();
    let mut last: SlidingWindow = SlidingWindow::new(sliding_size).unwrap();
    let mut cnt: usize = 1;

    for value in processed {
        for i in 0..tuples.len() { // add to existing
            tuples[i].push(value);
        }
        tuples.push(SlidingWindow::new_with_value(sliding_size, value).unwrap());

        if cnt >= sliding_size { // start comparing the increases
            let current = tuples[cnt - sliding_size].clone();
            assert_eq!(current.len(), sliding_size);

            if last.sum() > 0 && current.sum() > last.sum() {
                increases = increases + 1;
            }

            last = current;
            assert_eq!(last.len(), sliding_size);
        }

        cnt = cnt + 1;
    }

    increases
}
