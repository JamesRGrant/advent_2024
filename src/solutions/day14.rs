use std::collections::HashMap;

use crate::Solve;
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [12, 0, 230_435_667, 7709];

pub struct Problem {
    data: Vec<(i64, i64, i64, i64)>,
    data2: Vec<(i64, i64, i64, i64)>,
    width: i64,
    height: i64,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        // Move the robots
        for _ in 0..100 {
            for (x, y, vx, vy) in &mut self.data {
                *x += *vx;
                *y += *vy;
            }
        }
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        // Reset the robots to range
        // Count the quadrents
        for (x, y, _vx, _vy) in &mut self.data {
            let dx = *x / self.width;
            *x -= self.width * dx;
            if *x < 0 {
                *x += self.width;
            }
            let dy = *y / self.height;
            *y -= self.height * dy;
            if *y < 0 {
                *y += self.height;
            }

            if *x < self.width / 2 && *y < self.height / 2 {
                q1 += 1;
            } else if *x > self.width / 2 && *y < self.height / 2 {
                q2 += 1;
            } else if *x < self.width / 2 && *y > self.height / 2 {
                q3 += 1;
            } else if *x > self.width / 2 && *y > self.height / 2 {
                q4 += 1;
            }
        }

        q1 * q2 * q3 * q4
    }

    /// Short Description
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn p2(&mut self) -> i64 {
        // Move the robots
        for i in 0..10000 {
            let mut map: HashMap<i64, i64> = HashMap::new();
            for (x, y, vx, vy) in &mut self.data2 {
                *x += *vx;
                *y += *vy;
                while *x >= self.width {
                    *x -= self.width;
                }
                while *x < 0 {
                    *x += self.width;
                }
                while *y >= self.height {
                    *y -= self.height;
                }
                while *y < 0 {
                    *y += self.height;
                }

                *map.entry(*y).or_default() += 1;
            }

            // see if they are the same...
            if map.values().max().unwrap() > &29 {
                // Verify
                let mut board = vec![vec![' '; self.width as usize]; self.height as usize];
                for (x, y, _, _) in &self.data2 {
                    board[*y as usize][*x as usize] = '#';
                }
                for row in &board {
                    if row.iter().collect::<String>().contains("#########") {
                        return i + 1;
                    }
                }
            }
        }
        0
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut d: Vec<(i64, i64, i64, i64)> = Vec::new();
        for line in data {
            let s = line.split(['=', ',', ' ']).collect::<Vec<&str>>();
            d.push((
                s[1].parse::<i64>().unwrap(),
                s[2].parse::<i64>().unwrap(),
                s[4].parse::<i64>().unwrap(),
                s[5].parse::<i64>().unwrap(),
            ));
        }

        Problem {
            data: d.clone(),
            data2: d,
            width: 101,
            height: 103,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let mut p = Problem::new(&load_file("input/14_test.txt"));
        p.width = 11;
        p.height = 7;
        assert_eq!(p.p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        let mut p = Problem::new(&load_file("input/14_test.txt"));
        p.width = 11;
        p.height = 7;
        assert_eq!(p.p2(), ANSWERS[1]);
    }
    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/14.txt")).p1(), ANSWERS[2]);
    }

    // #[test]
    // fn f2() {
    //     assert_eq!(Problem::new(&load_file("input/14.txt")).p2(), ANSWERS[3]);
    // }
}
