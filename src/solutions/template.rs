use crate::Solve;
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [0, 0, 0, 0];

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        self.data.len() as i64
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        0
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem { data: data.to_vec() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        assert_eq!(Problem::new(&load_file("input/_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        assert_eq!(Problem::new(&load_file("input/_test.txt")).p2(), ANSWERS[1]);
    }
    // #[test]
    // fn f1() {
    //     assert_eq!(Problem::new(&load_file("input/.txt")).p1(), ANSWERS[2]);
    // }

    // #[test]
    // fn f2() {
    //     assert_eq!(Problem::new(&load_file("input/.txt")).p2(), ANSWERS[3]);
    // }
}
