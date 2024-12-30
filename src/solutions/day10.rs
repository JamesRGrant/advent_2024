use crate::Solve;
use std::collections::HashSet;

pub struct Problem {
    data: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
    visited: HashSet<(isize, isize)>,
}
impl Solve for Problem {
    /// Count of distinct starts to each trail-end
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for r in 0..self.data.len() {
            for c in 0..self.data[0].len() {
                if self.data[r][c] == 0 {
                    self.visited.clear();
                    self.count_paths(r as isize, c as isize);
                    sum += self.visited.len() as i64;
                }
            }
        }
        sum
    }

    /// Unique paths to each trail-end
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for r in 0..self.data.len() {
            for c in 0..self.data[0].len() {
                if self.data[r][c] == 0 {
                    self.visited.clear();
                    sum += self.count_paths(r as isize, c as isize);
                }
            }
        }
        sum
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        // Parse to a 2D array of ints
        let data = input
            .iter()
            .map(|x| x.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect::<Vec<Vec<u32>>>();
        let (rows, cols) = (data.len(), data[0].len());

        Problem {
            data,
            rows,
            cols,
            visited: HashSet::new(),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn out_of_bounds(&self, r: isize, c: isize) -> bool {
        r < 0 || c < 0 || r >= self.rows as isize || c >= self.cols as isize
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn count_paths(&mut self, r: isize, c: isize) -> i64 {
        let mut sum = 0;
        let (mut tr, mut tc): (isize, isize);
        let val = self.data[r as usize][c as usize] + 1;

        (tr, tc) = (r, c + 1); // Right
        sum += self.check_direction(tr, tc, val);
        (tr, tc) = (r, c - 1); // Left
        sum += self.check_direction(tr, tc, val);
        (tr, tc) = (r + 1, c); // Down
        sum += self.check_direction(tr, tc, val);
        (tr, tc) = (r - 1, c); // Up
        sum += self.check_direction(tr, tc, val);

        sum
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn check_direction(&mut self, tr: isize, tc: isize, val: u32) -> i64 {
        if !self.out_of_bounds(tr, tc) && self.data[tr as usize][tc as usize] == val {
            if val == 9 {
                self.visited.insert((tr, tc));
                return 1;
            }
            return self.count_paths(tr, tc);
        }
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
        assert_eq!(Problem::new(&load_file("input/10_test.txt")).p1(), 36);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/10_test.txt")).p2(), 81);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
