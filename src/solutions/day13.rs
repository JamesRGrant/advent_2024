use crate::Solve;
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [480, 875_318_608_908, 39996, 73_267_584_326_867];

pub struct Problem {
    data: Vec<(i64, i64, i64, i64, i64, i64)>,
}
impl Solve for Problem {
    /// This problem has a system of equations that has one solution (the problem eludes their are multiple)
    /// The only question is if the solution is integers or not
    fn p1(&mut self) -> i64 {
        let mut coins = 0;
        for (ax, ay, bx, by, x, y) in &self.data {
            coins += score_if_ints(*ax, *bx, *x, *ay, *by, *y);
        }
        coins
    }

    /// Same, but modify the x and y
    fn p2(&mut self) -> i64 {
        let mut coins = 0;
        for (ax, ay, bx, by, x, y) in &self.data {
            let x = x + 10_000_000_000_000;
            let y = y + 10_000_000_000_000;
            coins += score_if_ints(*ax, *bx, x, *ay, *by, y);
        }
        coins
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut input: Vec<(i64, i64, i64, i64, i64, i64)> = Vec::new();
        let mut i = 0;
        while i < data.len() {
            let a = data[i].split([',', '+']).collect::<Vec<&str>>();
            let b = data[i + 1].split([',', '+']).collect::<Vec<&str>>();
            let c = data[i + 2].split([',', '=']).collect::<Vec<&str>>();
            input.push((
                a[1].parse::<i64>().unwrap(),
                a[3].parse::<i64>().unwrap(),
                b[1].parse::<i64>().unwrap(),
                b[3].parse::<i64>().unwrap(),
                c[1].parse::<i64>().unwrap(),
                c[3].parse::<i64>().unwrap(),
            ));
            i += 4;
        }
        Problem { data: input }
    }
}

#[allow(clippy::many_single_char_names, clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn score_if_ints(a: i64, b: i64, x: i64, c: i64, d: i64, y: i64) -> i64 {
    let (i, j) = solve_system(a as f64, b as f64, x as f64, c as f64, d as f64, y as f64);
    let i: i64 = i.round() as i64;
    let j: i64 = j.round() as i64;
    if a * i + b * j == x && c * i + d * j == y {
        i * 3 + j
    } else {
        0
    }
}

// Solve a system of 2 equations with 2 independent variables
// ai + bj = x
// ci + dj = y
#[allow(clippy::many_single_char_names)]
fn solve_system(a: f64, b: f64, x: f64, c: f64, d: f64, y: f64) -> (f64, f64) {
    let jtop = y / d - c * x / (a * d);
    let jbot = 1.0 - c * b / (a * d);
    let j = jtop / jbot;
    let i = (x - b * j) / a;
    (i, j)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        assert_eq!(Problem::new(&load_file("input/13_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        assert_eq!(Problem::new(&load_file("input/13_test.txt")).p2(), ANSWERS[1]);
    }

    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/13.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        assert_eq!(Problem::new(&load_file("input/13.txt")).p2(), ANSWERS[3]);
    }
}
