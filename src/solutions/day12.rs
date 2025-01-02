use crate::Solve;
use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [1930, 1206, 1_489_582, 914_966];

pub struct Problem {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    perm: i64,
    area: i64,
    visited: HashSet<(usize, usize)>,
    p2: i64,
    edges_u: HashMap<usize, Vec<usize>>,
    edges_d: HashMap<usize, Vec<usize>>,
    edges_l: HashMap<usize, Vec<usize>>,
    edges_r: HashMap<usize, Vec<usize>>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.data[r][c] != '.' {
                    self.perm = 0;
                    self.area = 0;
                    self.visited.clear();
                    self.edges_u.clear();
                    self.edges_d.clear();
                    self.edges_l.clear();
                    self.edges_r.clear();
                    self.visited.clear();
                    self.get_plot(r, c, self.data[r][c]);
                    let sides = self.count_sides();
                    sum += self.perm * self.area;
                    self.p2 += sides * self.area;
                }
            }
        }
        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        self.p2
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let data = data.iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();
        let (rows, cols) = (data.len(), data[0].len());
        Problem {
            data,
            rows,
            cols,
            perm: 0,
            area: 0,
            visited: HashSet::new(),
            p2: 0,
            edges_u: HashMap::new(),
            edges_d: HashMap::new(),
            edges_l: HashMap::new(),
            edges_r: HashMap::new(),
        }
    }

    pub fn get_plot(&mut self, r: usize, c: usize, sym: char) {
        assert!(self.data[r][c] == sym, "Jumped to an invalid plot");

        self.data[r][c] = '.';
        self.visited.insert((r, c));
        self.area += 1;

        // Check up
        if r == 0 {
            self.perm += 1;
            self.edges_u.entry(r).or_default().push(c);
        } else if self.data[r - 1][c] == sym {
            self.get_plot(r - 1, c, sym);
        } else if !self.visited.contains(&(r - 1, c)) {
            self.perm += 1;
            self.edges_u.entry(r).or_default().push(c);
        }

        // Check down
        if r == self.rows - 1 {
            self.perm += 1;
            self.edges_d.entry(r).or_default().push(c);
        } else if self.data[r + 1][c] == sym {
            self.get_plot(r + 1, c, sym);
        } else if !self.visited.contains(&(r + 1, c)) {
            self.perm += 1;
            self.edges_d.entry(r).or_default().push(c);
        }

        // Check left
        if c == 0 {
            self.perm += 1;
            self.edges_l.entry(c).or_default().push(r);
        } else if self.data[r][c - 1] == sym {
            self.get_plot(r, c - 1, sym);
        } else if !self.visited.contains(&(r, c - 1)) {
            self.perm += 1;
            self.edges_l.entry(c).or_default().push(r);
        }

        // Check right
        if c == self.cols - 1 {
            self.perm += 1;
            self.edges_r.entry(c).or_default().push(r);
        } else if self.data[r][c + 1] == sym {
            self.get_plot(r, c + 1, sym);
        } else if !self.visited.contains(&(r, c + 1)) {
            self.perm += 1;
            self.edges_r.entry(c).or_default().push(r);
        }
    }

    pub fn count_sides(&mut self) -> i64 {
        Problem::determine_sides(&mut self.edges_u)
            + Problem::determine_sides(&mut self.edges_d.clone())
            + Problem::determine_sides(&mut self.edges_l.clone())
            + Problem::determine_sides(&mut self.edges_r.clone())
    }

    pub fn determine_sides(maps: &mut HashMap<usize, Vec<usize>>) -> i64 {
        let mut sides = 0;
        for points in maps.values_mut() {
            points.sort_unstable();

            let mut last = usize::MAX - 1;
            for x in points {
                if last + 1 != *x {
                    sides += 1;
                }
                last = *x;
            }
        }
        sides
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        assert_eq!(Problem::new(&load_file("input/12_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        let mut p = Problem::new(&load_file("input/12_test.txt"));
        p.p1();
        assert_eq!(p.p2(), ANSWERS[1]);
    }

    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/12.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        let mut p = Problem::new(&load_file("input/12.txt"));
        p.p1();
        assert_eq!(p.p2(), ANSWERS[3]);
    }
}
