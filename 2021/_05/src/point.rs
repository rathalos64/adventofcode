#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x: x, y: y}
    }

    pub fn new_from_string(s: &str) -> Result<Self, String> {
        let split: Vec<&str> = s.split(",").collect();
        if split.len() != 2 {
            return Err(String::from("invalid point given"));
        }

        let x: i32 = match String::from(split[0]).parse::<i32>(){
            Ok(x) => x,
            Err(_) => { return Err(String::from("failed to parse x coordinate")); }
        };
        let y: i32 = match String::from(split[1]).parse::<i32>(){
            Ok(y) => y,
            Err(_) => { return Err(String::from("failed to parse y coordinate")); }
        };
        
        Ok(Self::new(x, y))
    }

    pub fn equal(&self, other: Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    pub fn str(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

pub fn get_line(from: Point, to: Point, max_tries: Option<u32>) -> Result<Vec<Point>, String> {
    let mut line: Vec<Point> = Vec::new();
    line.push(from);

    if from.equal(to) {
        return Ok(line);
    }

    let mut curr: Point = from.clone();
    let mut i: u32 = 0;
    while !curr.equal(to) {
        curr.x += if curr.x < to.x { 1 } else if curr.x > to.x { -1 } else { 0 };
        curr.y += if curr.y < to.y { 1 } else if curr.y > to.y { -1 } else { 0 };
        line.push(curr);

        i += 1;
        if i == max_tries.unwrap_or(10_000) { // default number to prevent infinitiy loop
            return Err(String::from("maximum number of retries reached"));
        }
    }

    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn vec_compare(va: Vec<Point>, vb: Vec<Point>) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| (*a).equal(b))
    }

    #[test]
    fn test_line_short_y() {
        let observed = get_line(Point::new(1, 1), Point::new(1, 3), None).unwrap();
        let expected = vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)];
        assert_eq!(vec_compare(observed, expected), true);
    }

    #[test]
    fn test_line_short_x() {
        let observed = get_line(Point::new(7, 7), Point::new(9, 7), None).unwrap();
        let expected = vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)];
        assert_eq!(vec_compare(observed, expected), true);
    }

    #[test]
    fn test_line_short_x_fail() {
        let observed = get_line(Point::new(7, 7), Point::new(9, 7), None).unwrap();
        let expected = vec![Point::new(9, 7), Point::new(8, 7), Point::new(7, 7)];
        assert_eq!(vec_compare(observed, expected), false);
    }

    #[test]
    fn test_line_short_x_reverse() {
        let observed = get_line(Point::new(9, 7), Point::new(7, 7), None).unwrap();
        let expected = vec![Point::new(9, 7), Point::new(8, 7), Point::new(7, 7)];
        assert_eq!(vec_compare(observed, expected), true);
    }

    #[test]
    fn test_line_short_diagonal() {
        let observed = get_line(Point::new(6, 4), Point::new(4, 5), None).unwrap();
        let expected = vec![
            Point::new(6, 4), 
            Point::new(5, 5), 
            Point::new(4, 5)
        ];
        assert_eq!(vec_compare(observed, expected), true);
    }

    #[test]
    fn test_line_long_diagonal() {
        let observed = get_line(Point::new(0, 0), Point::new(8, 8), None).unwrap();
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3),
            Point::new(4, 4),
            Point::new(5, 5),
            Point::new(6, 6),
            Point::new(7, 7),
            Point::new(8, 8),
        ];
        assert_eq!(vec_compare(observed, expected), true);
    }

    #[test]
    fn test_line_too_long() {
        let res = get_line(Point::new(0, 0), Point::new(12345, 424242), None);
        assert_eq!(res.is_err(), true);
    }
}