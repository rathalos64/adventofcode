use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
enum CaveType {
    CaveTypeStart,
    CaveTypeEnd,
    CaveTypeSmall,
    CaveTypeBig,
}

#[derive(Debug)]
struct Cave {
    name: String,
    cave_type: CaveType,
    connections: Vec<String>
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: &String = &args.pop().unwrap_or_default();
    println!("> cwd {:?}, input {:?}", env::current_dir().unwrap_or_default(), input);
    assert!(Path::new(&input).exists());

    let lines: Vec<String> = fs::read_to_string(input).unwrap_or("".into()).split('\n')
        .filter(|line| *line != "")
        .map(|line| line.into())
        .collect();
    assert_ne!(lines.len(), 0);

    let mut caves: HashMap<String, Cave> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split('-').map(|s| s.into()).collect();
        assert_eq!(split.len(), 2);

        for s in split.clone() {
            let name: String = String::from(s.clone());
            let cave_type: CaveType = {
                if s == "start" {
                    CaveType::CaveTypeStart
                } else if s == "end" {
                    CaveType::CaveTypeEnd
                } else if s.chars().map(|c| c as usize)
                    .all(|c| 65 <= c && c <= 90) {
                    CaveType::CaveTypeBig
                } else if s.chars().map(|c| c as usize)
                    .all(|c| 97 <= c && c <= 122) {
                    CaveType::CaveTypeSmall
                } else {
                    panic!("{}", format!("invalid cave type given: {}", s))
                }
            };

            caves.entry(String::from(s)).or_insert(Cave{
                name: name,
                cave_type: cave_type,
                connections: Vec::new()
            });
        }

        // account for connections
        caves.get_mut(&String::from(split[0])).unwrap().connections.push(String::from(split[1]));
        caves.get_mut(&String::from(split[1])).unwrap().connections.push(String::from(split[0]));
    }

    println!("{:?}", caves);
}
