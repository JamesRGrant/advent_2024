use crate::Solve;

pub struct Problem {
    data: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Find count of "XMAS" in any direction
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                if self.data[i][j] == 'X' {
                    // search each direction
                    if i > 2 && j > 2 && self.data[i - 1][j - 1] == 'M' && self.data[i - 2][j - 2] == 'A' && self.data[i - 3][j - 3] == 'S' {
                        sum += 1;
                    }
                    if j > 2 && self.data[i][j - 1] == 'M' && self.data[i][j - 2] == 'A' && self.data[i][j - 3] == 'S' {
                        sum += 1;
                    }
                    if i < self.data.len() - 3 && j > 2 && self.data[i + 1][j - 1] == 'M' && self.data[i + 2][j - 2] == 'A' && self.data[i + 3][j - 3] == 'S' {
                        sum += 1;
                    }
                    if i > 2 && self.data[i - 1][j] == 'M' && self.data[i - 2][j] == 'A' && self.data[i - 3][j] == 'S' {
                        sum += 1;
                    }
                    if i < self.data.len() - 3 && self.data[i + 1][j] == 'M' && self.data[i + 2][j] == 'A' && self.data[i + 3][j] == 'S' {
                        sum += 1;
                    }
                    if i > 2 && j < self.data[0].len() - 3 && self.data[i - 1][j + 1] == 'M' && self.data[i - 2][j + 2] == 'A' && self.data[i - 3][j + 3] == 'S'
                    {
                        sum += 1;
                    }
                    if j < self.data[0].len() - 3 && self.data[i][j + 1] == 'M' && self.data[i][j + 2] == 'A' && self.data[i][j + 3] == 'S' {
                        sum += 1;
                    }
                    if i < self.data.len() - 3
                        && j < self.data[0].len() - 3
                        && self.data[i + 1][j + 1] == 'M'
                        && self.data[i + 2][j + 2] == 'A'
                        && self.data[i + 3][j + 3] == 'S'
                    {
                        sum += 1;
                    }
                }
            }
        }
        sum
    }

    /// Find "MAS" in a cross pattern
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for i in 1..self.data.len() - 1 {
            for j in 1..self.data[0].len() - 1 {
                // Options:
                // MM  MS  SM  SS
                // SS  MS  MS  MM
                if self.data[i][j] == 'A'
                    && ((self.data[i - 1][j - 1] == 'M' && self.data[i + 1][j + 1] == 'S')
                        || (self.data[i - 1][j - 1] == 'S' && self.data[i + 1][j + 1] == 'M'))
                    && ((self.data[i + 1][j - 1] == 'M' && self.data[i - 1][j + 1] == 'S')
                        || (self.data[i + 1][j - 1] == 'S' && self.data[i - 1][j + 1] == 'M'))
                {
                    sum += 1;
                }
            }
        }
        sum
    }
}

impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut v = Vec::new();
        for line in data {
            v.push(line.chars().collect());
        }
        Problem { data: v }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/04_test.txt")).p1(), 18);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/04_test.txt")).p2(), 9);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
