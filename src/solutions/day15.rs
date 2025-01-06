use crate::Solve;
#[allow(dead_code)]
const ANSWERS: [i64; 4] = [10092, 9021, 1_538_871, 1_543_338];

pub struct Problem {
    map: Vec<Vec<char>>,
    inst: Vec<char>,
    start: (usize, usize),
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut pos = self.start;
        let mut map = self.map.clone();
        for c in &self.inst {
            match c {
                '>' => pos = Problem::push(&mut map, pos, 0, 1),
                '^' => pos = Problem::push(&mut map, pos, -1, 0),
                'v' => pos = Problem::push(&mut map, pos, 1, 0),
                '<' => pos = Problem::push(&mut map, pos, 0, -1),
                _ => (),
            }
        }
        Problem::score(&map, 'O')
    }

    fn p2(&mut self) -> i64 {
        let (mut fatmap, start) = self.fatten_map();
        let mut pos = start;
        for c in &self.inst {
            match c {
                '>' => pos = Problem::push(&mut fatmap, pos, 0, 1),
                '<' => pos = Problem::push(&mut fatmap, pos, 0, -1),
                '^' => pos = Problem::big_push(&mut fatmap, pos, -1),
                'v' => pos = Problem::big_push(&mut fatmap, pos, 1),
                _ => (),
            }
        }
        Problem::score(&fatmap, '[')
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut inst: Vec<char> = Vec::new();
        let mut first = true;
        for line in data {
            let mut chars = line.chars().collect::<Vec<char>>();
            if chars.is_empty() {
                first = false;
            } else if first {
                map.push(chars);
            } else {
                inst.append(&mut chars);
            }
        }

        let mut start = (0, 0);
        for (r, row) in map.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                if *v == '@' {
                    start = (r, c);
                }
            }
        }

        Problem { map, inst, start }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss, clippy::match_on_vec_items)]
    fn push(map: &mut [Vec<char>], pos: (usize, usize), dr: isize, dc: isize) -> (usize, usize) {
        let mut pos = pos;
        // find the first open spot
        let (mut wr, mut wc) = (pos.0 as isize, pos.1 as isize);
        'outer: loop {
            (wr, wc) = (wr + dr, wc + dc);
            match map[wr as usize][wc as usize] {
                '#' => break,
                '.' => loop {
                    // Move everything by one
                    map[wr as usize][wc as usize] = map[(wr - dr) as usize][(wc - dc) as usize];
                    (wr, wc) = (wr - dr, wc - dc);
                    if wr == pos.0 as isize && wc == pos.1 as isize {
                        map[wr as usize][wc as usize] = '.';
                        pos = ((wr + dr) as usize, (wc + dc) as usize);
                        break 'outer;
                    }
                },
                _ => (),
            }
        }
        pos
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn big_push(map: &mut [Vec<char>], pos: (usize, usize), dr: isize) -> (usize, usize) {
        let new_pos = ((pos.0 as isize + dr) as usize, pos.1);
        if Problem::check(map, new_pos, dr, 0) {
            Problem::fat_push(map, pos, dr, 0);
            return new_pos;
        }
        pos
    }

    #[allow(clippy::match_on_vec_items, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn check(map: &mut [Vec<char>], pos: (usize, usize), dr: isize, dc: isize) -> bool {
        let mut ok;
        let next_row = (pos.0 as isize + dr) as usize;
        ok = match map[pos.0][pos.1] {
            '.' => true,
            '[' => Problem::check(map, (next_row, pos.1), dr, 1),
            ']' => Problem::check(map, (next_row, pos.1), dr, -1),
            _ => false,
        };
        if ok && dc == 1 {
            ok = ok
                && match map[pos.0][pos.1 + 1] {
                    '.' | ']' => true,
                    '[' => Problem::check(map, (next_row, pos.1 + 1), dr, 1),
                    _ => false,
                };
        } else if ok && dc == -1 {
            ok = ok
                && match map[pos.0][pos.1 - 1] {
                    '.' | '[' => true,
                    ']' => Problem::check(map, (next_row, pos.1 - 1), dr, -1),
                    _ => false,
                };
        }

        ok
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn fat_push(map: &mut [Vec<char>], pos: (usize, usize), dr: isize, dc: isize) {
        let new_row = (pos.0 as isize + dr) as usize;
        let check1 = (new_row, pos.1);

        // Move the first one
        let c = map[check1.0][check1.1];
        match c {
            '.' => (),
            '[' => Problem::fat_push(map, check1, dr, 1),
            ']' => Problem::fat_push(map, check1, dr, -1),
            _ => panic!("Unexpected character: {c}"),
        }
        map[check1.0][check1.1] = map[pos.0][pos.1];
        map[pos.0][pos.1] = '.';

        // Move the second one if requested
        if dc != 0 {
            let other_col = (pos.1 as isize + dc) as usize;
            let check2 = (new_row, other_col);
            let c = map[check2.0][check2.1];
            if c != '.' {
                if c == '[' && dc == 1 {
                    Problem::fat_push(map, check2, dr, 1);
                } else if c == ']' && dc == -1 {
                    Problem::fat_push(map, check2, dr, -1);
                } else {
                    panic!("Unexpected character: {c}");
                }
            }

            // Move them
            map[check2.0][check2.1] = map[pos.0][(pos.1 as isize + dc) as usize];
            map[pos.0][(pos.1 as isize + dc) as usize] = '.';
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn score(map: &[Vec<char>], target: char) -> i64 {
        let mut score = 0;
        for (r, row) in map.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                if *v == target {
                    score += r * 100 + c;
                }
            }
        }
        score as i64
    }

    fn fatten_map(&self) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut fatmap: Vec<Vec<char>> = Vec::new();
        let mut start: (usize, usize) = (0, 0);
        for (r, row) in self.map.iter().enumerate() {
            let mut new_row = Vec::new();
            for (c, v) in row.iter().enumerate() {
                match *v {
                    '.' => new_row.append(&mut vec!['.', '.']),
                    '#' => new_row.append(&mut vec!['#', '#']),
                    'O' => new_row.append(&mut vec!['[', ']']),
                    '@' => {
                        new_row.append(&mut vec!['@', '.']);
                        start = (r, c * 2);
                    }
                    _ => panic!("Unexpected character: {v}"),
                }
            }
            fatmap.push(new_row);
        }
        (fatmap, start)
    }

    #[allow(dead_code)]
    fn print(map: &[Vec<char>]) {
        for row in map {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        assert_eq!(Problem::new(&load_file("input/15_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        assert_eq!(Problem::new(&load_file("input/15_test.txt")).p2(), ANSWERS[1]);
    }
    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/15.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        assert_eq!(Problem::new(&load_file("input/15.txt")).p2(), ANSWERS[3]);
    }
}
