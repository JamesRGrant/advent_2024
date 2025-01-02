use crate::Solve;
use std::collections::HashMap;
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [55312, 1_900_433_601, 186_996, 221_683_913_164_898];

pub struct Problem {
    // data: VecDeque<u64>,
    data: HashMap<u64, u64>,
    singles: [u64; 10],
    x2024: [u64; 10],
    s2: [u64; 10],
    s4: [u64; 10],
    s8: [u64; 10],
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        self.blink(25);
        self.score()
    }

    fn p2(&mut self) -> i64 {
        self.blink(50);
        self.score()
    }
}
impl Problem {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn new(data: &[String]) -> Self {
        // Take out the single digits
        let mut singles = [0; 10];
        let mut h = HashMap::new();
        for i in data[0].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>() {
            if i < 10 {
                singles[i as usize] += 1;
            } else {
                *h.entry(i).or_insert(0) += 1;
            }
        }

        Problem {
            data: h,
            singles,
            x2024: [0; 10],
            s2: [0; 10],
            s4: [0; 10],
            s8: [0; 10],
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn blink(&mut self, count: usize) {
        for _ in 0..count {
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

            // x2024 to s2 [1-4] and s8 [5-9], s8 to s4 [5-9], s4 to s2 [5-9]
            s2_new[1..=4].copy_from_slice(&self.x2024[1..=4]);
            s8_new[5..=9].copy_from_slice(&self.x2024[5..=9]);
            s4_new[5..=9].copy_from_slice(&self.s8[5..=9]);
            s2_new[5..=9].copy_from_slice(&self.s4[5..=9]);

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
            let mut new_data = HashMap::new();
            for (k, v) in &self.data {
                let str = format!("{k}");
                if str.len() % 2 == 0 {
                    let (l, r) = str.split_at(str.len() / 2);
                    let ln = l.parse::<u64>().unwrap();
                    let rn = r.parse::<u64>().unwrap();

                    if ln < 10 {
                        singles_new[ln as usize] += v;
                    } else {
                        *new_data.entry(ln).or_insert(0) += v;
                    }
                    if rn < 10 {
                        singles_new[rn as usize] += v;
                    } else {
                        *new_data.entry(rn).or_insert(0) += v;
                    }
                } else {
                    *new_data.entry(k * 2024).or_insert(0) += v;
                }
            }

            // Update the arrays
            self.data = new_data;
            self.singles = singles_new;
            self.x2024 = x2024_new;
            self.s2 = s2_new;
            self.s4 = s4_new;
            self.s8 = s8_new;
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn score(&self) -> i64 {
        let mut score = 0;

        score += self.data.values().sum::<u64>();
        for i in 0..10 {
            score += self.singles[i] + self.x2024[i] + self.s8[i];
            if i > 4 {
                score += self.s2[i] * 4 + self.s4[i] * 2;
                if i == 8 {
                    score -= self.s2[i]; // 08 is pulled to singles early, so exlude
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
        assert_eq!(Problem::new(&load_file("input/11_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        assert_eq!(Problem::new(&load_file("input/11_test.txt")).p2(), ANSWERS[1]);
    }

    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/11.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        let mut p = Problem::new(&load_file("input/11.txt"));
        p.p1();
        assert_eq!(p.p2(), ANSWERS[3]);
    }
}
