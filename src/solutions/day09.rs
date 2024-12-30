use crate::Solve;
// use std::collections::VecDeque;

pub struct Problem {
    data: Vec<i64>,
    spaces: Vec<i64>,
}
impl Solve for Problem {
    /// Short Description
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut checksum = 0;
        let mut key_index = 0;
        let mut i = 0;
        let mut back_index = self.data.len() - 1;
        let mut dec = self.data[back_index];

        loop {
            // data first
            for _ in 0..self.data[key_index] {
                // println!("      {checksum}");
                checksum += i * key_index as i64;
                i += 1;
            }

            // spaces
            if key_index < self.spaces.len() {
                for _ in 0..self.spaces[key_index] {
                    loop {
                        // Try to use a number from the back
                        if dec > 0 {
                            dec -= 1;
                            checksum += i * back_index as i64;
                            i += 1;
                            break;
                        }

                        // Move to the next key
                        back_index -= 1;
                        if back_index == key_index {
                            return checksum;
                        }
                        dec = self.data[back_index];
                    }
                }
            }

            key_index += 1;

            if key_index == back_index {
                // Use up the items and break
                while dec > 0 {
                    dec -= 1;
                    checksum += i * back_index as i64;
                    i += 1;
                }
                break;
            }
        }

        checksum
    }

    /// Short Description
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let mut checksum = 0;
        let mut i = 0;
        // let mut back_index = self.data.len() - 1;

        let mut space_index = 0;
        let mut available_files = Vec::new();
        for i in 0..self.data.len() {
            available_files.push(i);
        }
        let mut key_index = 0;

        while !available_files.is_empty() {
            if available_files.contains(&key_index) {
                // data first: take the next file
                let file_idx = available_files.remove(0);
                for _ in 0..self.data[file_idx] {
                    checksum += i * file_idx as i64;
                    // println!(" {i} * {file_idx} = {checksum}");
                    i += 1;
                }
            } else {
                i += self.data[key_index];
            }
            key_index += 1;

            // spaces: try to fit any future file
            let mut found = false;
            let mut dec = self.spaces[space_index];
            // println!("  Filling {dec} spaces");
            while dec > 0 {
                for j in (0..self.data.len()).rev() {
                    // println!("  Looking at {j}");
                    if self.data[j] <= dec && available_files.contains(&j) {
                        found = true;
                        let file_idx = available_files.remove(available_files.iter().position(|&x| x == j).unwrap());
                        // println!("  Found {j} with {file_idx}");
                        for _ in 0..self.data[file_idx] {
                            dec -= 1;
                            checksum += i * j as i64;
                            // println!(" {i} * {j} = {checksum}");
                            i += 1;
                        }
                        break;
                    }
                }
                // println!("  {dec} spaces left, found = {found}, {:?}", available_files);
                if found {
                    found = false;
                } else {
                    // println!("  Exiting at {dec} spaces");
                    break;
                }
            }
            // Forward the remaining index
            i += dec;

            space_index += 1;

            // println!("   END LOOP");
        }

        checksum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let (mut files, mut spaces): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());

        for (i, c) in data[0].chars().enumerate() {
            if i % 2 == 0 {
                files.push(i64::from(c.to_digit(10).unwrap()));
            } else {
                spaces.push(i64::from(c.to_digit(10).unwrap()));
            }
        }
        Problem { data: files, spaces }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/09_test.txt")).p1(), 1928);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/09_test.txt")).p2(), 2858);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
