use std::fs;

pub fn run(input: &String) -> i32 {
    let mut increases: i32 = 0;
    let mut prev: i32 = -1;
    let mut i = 0;

    for v_str in fs::read_to_string(input).unwrap_or_default().split('\n') {
        if v_str == "" {
            continue
        }
        
        let val: i32;
        let parsed = String::from(v_str).parse::<i32>();
        match parsed {
            Ok(p) => { val = p }
            Err(err) => { panic!("{}", err); }
        }

        i = i + 1;

        if val > prev && prev != -1{
            increases = increases + 1;
        }

        prev = val;
    }

    increases
}
