use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    forward: HashMap<i64, Vec<i64>>,
    reverse: HashMap<i64, Vec<i64>>,
    things: Vec<Vec<i64>>,
    incorrect: Vec<Vec<i64>>,
}
impl Solve for Problem {
    /// Makes sure items are in the right order; add the middle num of valid
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;

        for thing in &self.things {
            let mut valid = true;
            'outer: for i in 0..thing.len() {
                //check forward
                if i < thing.len() - 1 {
                    for j in i + 1..thing.len() {
                        let test = self.reverse.get(&thing[i]);
                        if test.is_some() && test.unwrap().contains(&thing[j]) {
                            valid = false;
                            break 'outer;
                        }
                    }
                }

                if valid && i > 0 {
                    for j in (0..i).rev() {
                        let test = self.forward.get(&thing[i]);
                        if test.is_some() && test.unwrap().contains(&thing[j]) {
                            valid = false;
                            break 'outer;
                        }
                    }
                }
            }
            if valid {
                sum += thing[(thing.len() - 1) / 2];
            } else {
                self.incorrect.push(thing.clone());
            }
        }

        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;
        if self.incorrect.is_empty() {
            self.p1();
        }

        // Sort the lists based on forward and reverse
        for thing in &self.incorrect {
            let mut sorted = thing.clone();
            sorted.sort_by(|a, b| {
                let after = self.forward.get(a);
                if after.is_some() && after.unwrap().contains(b) {
                    return std::cmp::Ordering::Less;
                } else if self.reverse.get(a).unwrap().contains(b) {
                    return std::cmp::Ordering::Greater;
                }
                std::cmp::Ordering::Equal
            });

            sum += sorted[(sorted.len() - 1) / 2];
        }

        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut forward: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut reverse: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut things: Vec<Vec<i64>> = Vec::new();

        let mut phase1 = true;
        for line in data {
            if line.is_empty() {
                phase1 = false;
            } else if phase1 {
                let values: Vec<i64> = line.split('|').filter_map(|s| s.parse().ok()).collect();
                forward.entry(values[0]).or_default().push(values[1]);
                reverse.entry(values[1]).or_default().push(values[0]);
            } else {
                things.push(line.split(',').filter_map(|s| s.parse().ok()).collect());
            }
        }

        Problem {
            forward,
            reverse,
            things,
            incorrect: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/05_test.txt")).p1(), 143);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/05_test.txt")).p2(), 123);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
