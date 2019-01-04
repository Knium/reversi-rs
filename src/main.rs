#![warn(unused_must_use)]

// Rustで競技プログラミングの入力をスッキリ記述するマクロ
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

use self::Color::*;
use std::fmt::{Display, Formatter, Result};

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn to_s(&self) -> &'static str {
        match self {
            Color::White => "w",
            Color::Black => "b",
        }
    }

    fn another(&self) -> Self {
        match self {
            Color::White => Black,
            _ => White,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Board {
    map: [[Option<Color>; 8]; 8],
    latest: (usize, usize),
    turn: Color,
}

impl Board {
    fn new() -> Self {
        let mut b = Board {
            map: [[None; 8]; 8],
            latest: (4, 4),
            turn: Black,
        };
        b.set((3, 3), Black); // Black
        b.set((3, 4), White); // White
        b.set((4, 4), Black); // Black
        b.set((4, 3), White); // White
        b
    }

    fn set(&mut self, (x, y): (usize, usize), color: Color) {
        let s = self.map.get_mut(y).unwrap();
        s[x] = Some(color);
    }

    fn put(&mut self, (x, y): (usize, usize)) {
        let s = self.map.get_mut(y).unwrap();
        s[x] = Some(self.turn);
        self.latest = (x, y);
        self.reverse();
        self.turn = self.turn.another();
    }

    fn reverse(&mut self) {
        self.horizontal();
        self.vertical();
        self.diagonal();
    }

    fn horizontal(&mut self) {
        let (_, y) = self.latest;
        let column = self.map.get_mut(y).unwrap();
        let mut points = vec![];
        let mut betweened_flag = false;
        for (x, point) in column.iter().enumerate() {
            if let Some(color) = point {
                match (&self.turn == color, betweened_flag) {
                    (true, false) => betweened_flag = true,
                    (false, true) => points.push((x,y)),
                    (true, true) => betweened_flag = false,
                    (false, false) => (),
                }
            }
        }
        for point in points.iter() {
            self.set(*point, self.turn);
        }
    }

    fn vertical(&self) {}

    fn diagonal(&self) {}
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for column in self.map.iter() {
            writeln!(f, "---------------------------------");
            for elem in column.iter() {
                write!(
                    f,
                    "| {} ",
                    if let Some(color) = elem {
                        color.to_s()
                    } else {
                        " "
                    }
                );
            }
            writeln!(f, "|");
        }
        writeln!(f, "---------------------------------")
    }
}

fn main() {
    let mut board = Board::new();
    loop {
        println!("{}", board);
        input! {
            x: (usize, usize)
        };
        board.put(x);
    }
}
