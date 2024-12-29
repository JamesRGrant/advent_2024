use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    data: HashMap<i64, Vec<i64>>,
}
impl Solve for Problem {
    /// Add valid answers, Operators could be +, *
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (k, v) in &self.data {
            if Problem::possible(*k, v, 1, v[0]) {
                sum += k;
            }
        }
        sum
    }

    /// Add valid answers, Operators could be +, *, or concat strings (1, 23) => 123
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for (k, v) in &self.data {
            if Problem::impossible(*k, v, 1, v[0]) {
                sum += k;
            }
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
        for line in data {
            let parts: Vec<&str> = line.split(':').collect();
            let key = parts[0].parse::<i64>().unwrap();
            let values: Vec<i64> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
            map.entry(key).or_default().extend(values);
        }
        Problem { data: map }
    }

    pub fn possible(key: i64, val: &[i64], index: usize, sum: i64) -> bool {
        let mut tmp = sum + val[index];
        if index == val.len() - 1 {
            if tmp == key {
                return true;
            }
        } else if Problem::possible(key, val, index + 1, tmp) {
            return true;
        }

        tmp = sum * val[index];
        if index == val.len() - 1 {
            return tmp == key;
        }
        Problem::possible(key, val, index + 1, tmp)
    }

    pub fn impossible(key: i64, val: &[i64], index: usize, sum: i64) -> bool {
        let mut tmp = sum + val[index];
        if index == val.len() - 1 {
            if tmp == key {
                return true;
            }
        } else if Problem::impossible(key, val, index + 1, tmp) {
            return true;
        }

        tmp = sum * val[index];
        if index == val.len() - 1 {
            if tmp == key {
                return true;
            }
        } else if Problem::impossible(key, val, index + 1, tmp) {
            return true;
        }

        // crazy concat
        tmp = format!("{}{}", sum, val[index]).parse().unwrap();
        // println!("{sum} + {} = {tmp}", val[index]);
        if index == val.len() - 1 {
            return tmp == key;
        }
        Problem::impossible(key, val, index + 1, tmp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/07_test.txt")).p1(), 3749);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/07_test.txt")).p2(), 11387);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
