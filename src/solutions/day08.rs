use crate::Solve;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Problem {
    data: HashMap<char, Vec<(isize, isize)>>,
    rows: usize,
    cols: usize,
}
impl Solve for Problem {
    /// Short Description
    #[allow(clippy::cast_possible_wrap, clippy::similar_names)]
    fn p1(&mut self) -> i64 {
        let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();

        for nodes in self.data.values() {
            for i in 0..nodes.len() - 1 {
                for j in (i + 1)..nodes.len() {
                    let (r1, c1) = nodes[i];
                    let (r2, c2) = nodes[j];

                    let (dr, dc) = (r2 - r1, c2 - c1);
                    let (a1r, a1c) = (r1 - dr, c1 - dc);
                    let (a2r, a2c) = (r2 + dr, c2 + dc);

                    if a1r >= 0 && a1r < self.rows as isize && a1c >= 0 && a1c < self.cols as isize {
                        anti_nodes.insert((a1r, a1c));
                    }
                    if a2r >= 0 && a2r < self.rows as isize && a2c >= 0 && a2c < self.cols as isize {
                        anti_nodes.insert((a2r, a2c));
                    }
                }
            }
        }

        anti_nodes.len() as i64
    }

    /// Short Description
    #[allow(clippy::cast_possible_wrap, clippy::similar_names)]
    fn p2(&mut self) -> i64 {
        let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();

        for nodes in self.data.values() {
            for i in 0..nodes.len() - 1 {
                for j in (i + 1)..nodes.len() {
                    let (r1, c1) = nodes[i];
                    let (r2, c2) = nodes[j];

                    anti_nodes.insert((r1, c1));
                    anti_nodes.insert((r2, c2));
                    let (dr, dc) = (r2 - r1, c2 - c1);

                    let (mut a1r, mut a1c) = (r1, c1);
                    loop {
                        (a1r, a1c) = (a1r - dr, a1c - dc);
                        if a1r >= 0 && a1r < self.rows as isize && a1c >= 0 && a1c < self.cols as isize {
                            anti_nodes.insert((a1r, a1c));
                        } else {
                            break;
                        }
                    }

                    let (mut a2r, mut a2c) = (r2, c2);
                    loop {
                        (a2r, a2c) = (a2r + dr, a2c + dc);
                        if a2r >= 0 && a2r < self.rows as isize && a2c >= 0 && a2c < self.cols as isize {
                            anti_nodes.insert((a2r, a2c));
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        anti_nodes.len() as i64
    }
}
impl Problem {
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(data: &[String]) -> Self {
        // Create a map of signal char and list of positions
        let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        for (row, line) in data.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    map.entry(c).or_default().push((row as isize, col as isize));
                }
            }
        }
        Problem {
            data: map,
            rows: data.len(),
            cols: data[0].len(),
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
        assert_eq!(Problem::new(&load_file("input/08_test.txt")).p1(), 14);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/08_test.txt")).p2(), 34);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
