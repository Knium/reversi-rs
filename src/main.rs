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
use std::cmp::Ordering;
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
    black_points: i32,
    white_points: i32,
}

impl Board {
    fn new() -> Self {
        let mut b = Board {
            map: [[None; 8]; 8],
            latest: (4, 4),
            turn: Black,
            black_points: 0,
            white_points: 0,
        };
        b.set_with_color((3, 3), Black); // Black
        b.set_with_color((3, 4), White); // White
        b.set_with_color((4, 4), Black); // Black
        b.set_with_color((4, 3), White); // White
        b
    }

    fn get(&self, (x, y): Position) -> Option<Color> {
        self.map[y][x]
    }

    fn set(&mut self, (x, y): Position, color: Option<Color>) {
        self.map[y][x] = color;
    }

    fn set_with_color(&mut self, position: Position, color: Color) {
        self.set(position, Some(color));
        self.incr_points(color);
    }

    fn plus_points_with_color(&mut self, n: i32, color: Color) {
        if color == Black {
            self.black_points += n;
        } else {
            self.white_points += n;
        }
    }

    fn incr_points(&mut self, color: Color) {
        self.plus_points_with_color(1, color);
    }

    fn decr_points(&mut self, color: Color) {
        self.plus_points_with_color(-1, color);
    }

    fn put(&mut self, (x, y): (usize, usize)) {
        let position = (x, y);
        if x <= 8 && y <= 8 {
            if let None = self.get(position) {
                let ex = self.latest;
                self.set_with_color(position, self.turn);
                self.latest = position;
                let positions = self.reversable_points();
                if positions.len() == 0 {
                    self.set(self.latest, None);
                    self.decr_points(self.turn);
                    println!("{:?} has no reversable points, try again!", self.latest);
                    self.latest = ex;
                    return;
                } else {
                    for position in positions {
                        self.set_with_color(position, self.turn);
                        self.decr_points(self.turn.another());
                    }
                }
                self.turn = self.turn.another();
            } else {
                println!(
                    "{:?} is already put!! {}",
                    (x, y),
                    self.get((x, y)).unwrap().to_s()
                );
            }
        } else {
            println!("You tried to put invalid position {:?}", (x, y));
        }
    }

    fn reversable_points(&self) -> Vec<Position> {
        let mut h = self.horizontal();
        let mut v = self.vertical();
        let mut d = self.diagonal();
        h.append(&mut v);
        h.append(&mut d);
        h
    }

    fn horizontal(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut left = {
            let mut betweened = false;
            let mut points = vec![];
            for x in (0..x).rev() {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };
        let mut right = {
            let mut betweened = false;
            let mut points = vec![];
            for x in (x + 1)..8 {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };
        left.append(&mut right);
        left
    }

    fn vertical(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut up = {
            let mut betweened = false;
            let mut points = vec![];
            for y in (0..y).rev() {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };
        let mut bottom = {
            let mut betweened = false;
            let mut points = vec![];
            for y in (y + 1)..8 {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };
        up.append(&mut bottom);
        up
    }
    fn diagonal(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut left_up = {
            let mut points = vec![];
            let mut betweened = false;
            for (x, y) in (0..x).rev().zip((0..y).rev()) {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };

        let mut right_bottom = {
            let mut points = vec![];
            let mut betweened = false;
            for (x, y) in ((x + 1)..8).zip((y + 1)..8) {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };

        let mut right_up = {
            let mut points = vec![];
            let mut betweened = false;
            for (x, y) in ((x + 1)..8).zip((0..y).rev()) {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };

        let mut left_bottom = {
            let mut points = vec![];
            let mut betweened = false;
            for (x, y) in (0..x).rev().zip((y + 1)..8) {
                let putted_color = self.get((x, y));
                if let Some(color) = putted_color {
                    if self.turn == color {
                        betweened = true;
                        break;
                    } else {
                        points.push((x, y));
                    }
                } else {
                    break;
                }
            }
            if betweened {
                points
            } else {
                vec![]
            }
        };

        left_up.append(&mut left_bottom);
        left_up.append(&mut right_up);
        left_up.append(&mut right_bottom);
        left_up
    }
}

impl Display for Board {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "B: {}, W: {}", self.black_points, self.white_points);
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
        println!("{:?}'s turn!", board.turn);
        println!("{}", board);
        input! {
            x: (usize, usize)
        };
        board.put(x);
        if board.black_points + board.white_points == 64 {
            break;
        }
    }
    println!("{}", board);
    println!(
        "WINNER: {} !!",
        match board.black_points.cmp(&board.white_points) {
            Ordering::Greater => "Black",
            Ordering::Less => "White",
            _ => "Draw",
        }
    );
}
