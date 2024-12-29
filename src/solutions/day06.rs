use crate::Solve;
use std::collections::HashSet;

pub struct Problem {
    data: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Find unique visited locations
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let (mut x, mut y, mut dx, mut dy) = self.find_start();
        visited.insert((x, y));

        loop {
            (x, y) = (x + dx, y + dy);
            if self.out_of_bounds(x, y) {
                break;
            }

            let c = self.data[y as usize][x as usize];
            if c == '#' {
                // hit an obstacle, go back and turn right
                (x, y) = (x - dx, y - dy);
                (dx, dy) = Problem::turn_right(dx, dy);
            } else if c == '.' || c == '^' {
                visited.insert((x, y));
            }
        }

        visited.len() as i64
    }

    /// Count how many single blocks you can add to create endless loops
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::similar_names)]
    fn p2(&mut self) -> i64 {
        let (mut x, mut y, mut dx, mut dy) = self.find_start();
        let (mut sx, mut sy, mut sdx, mut sdy) = (x, y, dx, dy);
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        visited.insert((x, y));

        let mut cnt: i64 = 0;

        loop {
            (x, y) = (x + dx, y + dy);
            if self.out_of_bounds(x, y) {
                break;
            }

            let c = self.data[y as usize][x as usize];
            if c == '#' {
                // hit an obstacle, go back and turn right
                (x, y) = (x - dx, y - dy);
                (dx, dy) = Problem::turn_right(dx, dy);
            } else if c == '.' && !visited.contains(&(x, y)) {
                self.data[y as usize][x as usize] = '#';
                if self.is_loop(sx, sy, sdx, sdy) {
                    cnt += 1;
                }
                self.data[y as usize][x as usize] = '.';
                visited.insert((x, y));

                // Update the last valid position before a block
                // (This prevents us from going from the start each time)
                (sx, sy, sdx, sdy) = (x, y, dx, dy);
            }
        }
        cnt
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem {
            data: data.iter().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>(),
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::similar_names)]
    pub fn is_loop(&self, sx: isize, sy: isize, sdx: isize, sdy: isize) -> bool {
        // let (mut x, mut y, mut dx, mut dy) = self.find_start();
        let (mut x, mut y, mut dx, mut dy) = (sx, sy, sdx, sdy);
        let mut turns: HashSet<(isize, isize, isize, isize)> = HashSet::new();

        loop {
            (x, y) = (x + dx, y + dy);
            if self.out_of_bounds(x, y) {
                break;
            }

            let c = self.data[y as usize][x as usize];
            if c == '#' {
                // hit an obstacle, go back and turn right
                (x, y) = (x - dx, y - dy);
                (dx, dy) = Problem::turn_right(dx, dy);
                if !turns.insert((x, y, dx, dy)) {
                    return true;
                }
            }
        }
        false
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn find_start(&self) -> (isize, isize, isize, isize) {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                if self.data[i][j] == '^' {
                    return (j as isize, i as isize, 0, -1);
                }
            }
        }
        (0, 0, 0, 0)
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || y >= self.data.len() as isize || x >= self.data[0].len() as isize
    }

    pub fn turn_right(dx: isize, dy: isize) -> (isize, isize) {
        match (dx, dy) {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => (0, 0),
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
        assert_eq!(Problem::new(&load_file("input/06_test.txt")).p1(), 41);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/06_test.txt")).p2(), 6);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
