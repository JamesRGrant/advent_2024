use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    left: Vec<i64>,
    right: Vec<i64>,
}
impl Solve for Problem {
    /// sum of differences
    fn p1(&mut self) -> i64 {
        self.left.sort_unstable();
        self.right.sort_unstable();
        let mut sum = 0;

        for i in 0..self.left.len() {
            sum += (self.left[i] - self.right[i]).abs();
        }
        sum
    }

    /// For each left, sum of instances in right * left value
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        let mut counts = HashMap::new();

        // Count the right side
        for x in &self.right {
            *counts.entry(x).or_insert(0) += 1;
        }

        for x in &self.left {
            sum += x * counts.get(x).unwrap_or(&0);
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();
        for line in data {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse().unwrap());
            right.push(parts.next().unwrap().parse().unwrap());
        }

        Problem { left, right }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/01_test.txt")).p1(), 11);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/01_test.txt")).p2(), 31);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
