use crate::Solve;
use std::{collections::VecDeque, io::Write};

pub struct Problem {
    data: VecDeque<u64>,
    singles: [u64; 10],
    x2024: [u64; 10],
    s2: [u64; 10],
    s4: [u64; 10],
    s8: [u64; 10],
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        for _ in 0..25 {
            self.blink();
        }
        self.score()
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        for j in 25..75 {
            print!("\r...{j}, {}", self.data[0]);
            std::io::stdout().flush().unwrap();
            self.blink();
        }
        print!("\r");
        self.score()
    }
}
impl Problem {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn new(data: &[String]) -> Self {
        // Using a VecDeque to make it easier to remove items
        let mut data: VecDeque<u64> = data[0].split_whitespace().map(|x| x.parse().unwrap()).collect();

        // Take out the single digits
        let mut singles = [0; 10];
        let mut i = 0;
        while i < data.len() {
            if data[i] < 10 {
                singles[data[i] as usize] += 1;
                data.remove(i);
            } else {
                i += 1;
            }
        }

        Problem {
            data,
            singles,
            x2024: [0; 10],
            s2: [0; 10],
            s4: [0; 10],
            s8: [0; 10],
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn blink(&mut self) {
        // Single digits are multiplied by 2024
        //  1-4: split into 2 digits, split into 1 digits -> (x2024, s2, singles)
        //  5-8: x2024, x2024, split into 4, split into 2 , split into 1  -> (x2024, s8, s4, s2, singles)
        // So, we can just track each phase...
        //  1	1024     1 -> 1024 -> 10 24 -> 1 0 2 4
        //  2	2048
        //  3	3072
        //  4	4096
        //  5	20482880 5 -> 10120 -> 20482880 -> 2048 2880 -> 20 48 28 80 -> 2 0 4 8 2 8 8 0
        //  6	24579456
        //  7	28676032
        //  8	32772608 --> tricky!  becomes 32 77 26 8
        //  9	36869184
        // Therfore, we just track single digits by phase.
        // Otherwise, track other numbers until they become single digits
        //  * Even digit-count numbers break down to single
        //  * Odd digit-count numbers eventually get to even...

        let mut singles_new = [0; 10];
        let mut s2_new = [0; 10];
        let mut s4_new = [0; 10];
        let mut s8_new = [0; 10];

        // singles to x2024 (except 0 goes to 1)
        let mut x2024_new = self.singles;
        x2024_new[0] = 0;
        singles_new[1] = self.singles[0];

        // x2024 to s2 [1-4] and s8 [5-9]
        s2_new[1] = self.x2024[1];
        s2_new[2] = self.x2024[2];
        s2_new[3] = self.x2024[3];
        s2_new[4] = self.x2024[4];

        s8_new[5] = self.x2024[5];
        s8_new[6] = self.x2024[6];
        s8_new[7] = self.x2024[7];
        s8_new[8] = self.x2024[8];
        s8_new[9] = self.x2024[9];

        // s8 to s4
        s4_new[5] = self.s8[5];
        s4_new[6] = self.s8[6];
        s4_new[7] = self.s8[7];
        s4_new[8] = self.s8[8];
        s4_new[9] = self.s8[9];

        // s4 to s2  [5-9]
        s2_new[5] = self.s4[5];
        s2_new[6] = self.s4[6];
        s2_new[7] = self.s4[7];
        s2_new[8] = self.s4[8];
        s2_new[9] = self.s4[9];

        // s2 to singles (see map above)
        //   Note exception for [8] - it pulls in a single digit early from s4
        //   Note += on [1] because we already put something in there
        singles_new[0] = self.s2[1] + self.s2[2] + self.s2[3] + self.s2[4] + self.s2[5] * 2 + self.s2[7];
        singles_new[1] += self.s2[9];
        singles_new[2] = self.s2[1] * 2 + self.s2[3] + self.s2[5] * 2 + self.s2[6] + self.s2[7] * 2 + self.s2[8] * 2;
        singles_new[3] = self.s2[7] + self.s2[8] + self.s2[9];
        singles_new[4] = self.s2[1] + self.s2[2] * 2 + self.s2[5] + self.s2[6] * 2 + self.s2[9];
        singles_new[5] = self.s2[6] * 2;
        singles_new[6] = self.s2[3] + self.s2[4] + self.s2[6] + self.s2[7] * 2 + self.s2[8] + self.s2[9] * 2;
        singles_new[7] = self.s2[3] + self.s2[6] + self.s2[7] + self.s2[8] * 2;
        singles_new[8] = self.s2[2] + self.s2[4] + self.s2[5] * 3 + self.s2[7] + self.s4[8] + self.s2[9] * 2;
        singles_new[9] = self.s2[4] + self.s2[6] + self.s2[9];

        // Now the normal rules from the problem
        let mut i = 0;
        while i < self.data.len() {
            let str = format!("{}", self.data[i]);
            if self.data[i] == 0 {
                // Puzzle rule: 0 becomes 1
                singles_new[1] += 1;
                self.data.remove(i);
            } else if self.data[i] < 10 {
                // Single digit - move to singles array
                singles_new[self.data[i] as usize] += 1;
                self.data.remove(i);
            } else if str.len() == 2 {
                // 2 digit - move to singles array
                let (l, r) = str.split_at(str.len() / 2);
                singles_new[l.parse::<usize>().unwrap()] += 1;
                singles_new[r.parse::<usize>().unwrap()] += 1;
                self.data.remove(i);
            } else if str.len() % 2 == 0 {
                let (l, r) = str.split_at(str.len() / 2);

                // left should always be a big number, so overwrite
                self.data[i] = l.parse().unwrap();

                // right could go down to a single digit, otherwise insert
                let rn = r.parse().unwrap();
                if rn < 10 {
                    singles_new[rn as usize] += 1;
                } else {
                    self.data.insert(i, rn);
                    // Advance because we inserted
                    i += 1;
                }

                // Did not remove, so advance
                i += 1;
            } else {
                self.data[i] *= 2024;
                // Did not remove, so advance
                i += 1;
            }
        }

        // Update the arrays
        self.singles = singles_new;
        self.x2024 = x2024_new;
        self.s2 = s2_new;
        self.s4 = s4_new;
        self.s8 = s8_new;
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn score(&self) -> i64 {
        let mut score = 0;
        score += self.data.len() as u64;
        for i in 0..10 {
            score += self.singles[i];
            score += self.x2024[i];
            score += self.s8[i];
            if i > 4 {
                score += self.s2[i] * 4;
                score += self.s4[i] * 2;
                if i == 8 {
                    // 08 is pulled to singles early, so we have to exclude it here
                    score -= self.s2[i];
                }
            } else {
                score += self.s2[i] * 2;
            }
        }
        score as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/11_test.txt")).p1(), 55312);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input/11_test.txt")).p2(), 1_900_433_601);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
