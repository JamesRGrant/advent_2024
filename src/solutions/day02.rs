use crate::Solve;

pub struct Problem {
    data: Vec<Vec<i64>>,
}
impl Solve for Problem {
    /// same direction (neutral bad) and no more than 3 steps
    fn p1(&mut self) -> i64 {
        let mut safe = 0;
        for row in &self.data {
            if is_safe(row) {
                safe += 1;
            }
        }
        safe
    }

    /// able to skip one bad entry (could be any, not just the first)
    fn p2(&mut self) -> i64 {
        let mut safe = 0;
        'outer: for row in &self.data {
            for i in 0..row.len() {
                let mut sub_row = row.clone();
                sub_row.remove(i);
                if is_safe(&sub_row) {
                    safe += 1;
                    continue 'outer;
                }
            }
        }
        safe
    }
}

impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut input: Vec<Vec<i64>> = Vec::new();
        for line in data {
            input.push(line.split_whitespace().map(|s| s.parse().unwrap()).collect());
        }
        Problem { data: input }
    }
}

fn is_safe(row: &[i64]) -> bool {
    let positive = row[1] > row[0];
    for i in 1..row.len() {
        if positive != (row[i] > row[i - 1]) || row[i] == row[i - 1] || (row[i] - row[i - 1]).abs() > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/02_test.txt")).p1(), 2);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/02_test.txt")).p2(), 4);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
