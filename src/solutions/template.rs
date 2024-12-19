use crate::Solve;

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
        Problem {
            data: data.to_vec(),
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
        assert_eq!(Problem::new(&load_file("input\\_test.txt")).p1(), 0);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\_test.txt")).p2(), 0);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
