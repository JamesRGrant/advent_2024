use crate::Solve;
use regex::Regex;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    /// look for mul(x,y) and sum the results
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            // Use a regex with capturing groups: (\d{1,3})
            let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
            let re = Regex::new(pattern).unwrap();

            for cap in re.captures_iter(line) {
                let num1: i64 = cap[1].parse().unwrap();
                let num2: i64 = cap[2].parse().unwrap();
                sum += num1 * num2;
            }
        }
        sum
    }

    /// Same, but ``don't()`` turns off and ``do()`` turns back on
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        let mut enabled = true;
        for line in &self.data {
            // Use a regex with capturing groups: (\d{1,3})
            let pattern = r"mul\((?:\d{1,3}),(?:\d{1,3})\)|do\(\)|don't\(\)";
            let re = Regex::new(pattern).unwrap();

            for cap in re.find_iter(line) {
                match cap.as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if enabled {
                            sum += extract_numbers(cap.as_str());
                        }
                    }
                }
            }
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem { data: data.to_vec() }
    }
}
fn extract_numbers(s: &str) -> i64 {
    let pattern = r"(\d{1,3}),(\d{1,3})";
    let re = Regex::new(pattern).unwrap();
    if let Some(cap) = re.captures(s) {
        let num1: i64 = cap[1].parse().unwrap();
        let num2: i64 = cap[2].parse().unwrap();
        num1 * num2
    } else {
        0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/03_test.txt")).p1(), 161);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/03_test2.txt")).p2(), 48);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
