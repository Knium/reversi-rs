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

macro_rules! find_reversable_positions {
    ($self: ident, $x_range: expr, $y_range: expr ) => {{
        let mut betweened = false;
        let mut candidates = vec![];
        for (x, y) in $x_range.zip($y_range) {
            let putted_color = $self.get((x, y));
            if let Some(color) = putted_color {
                if $self.turn == color {
                    betweened = true;
                    break;
                } else {
                    candidates.push((x, y));
                }
            } else {
                break;
            }
        }
        if betweened {
            candidates
        } else {
            vec![]
        }
    };};
}

use self::Color::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::repeat;

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

#[derive(Clone, Debug)]
struct Game {
    board: [[Option<Color>; 8]; 8],
    unput_positions: HashSet<Position>,
    puttable_positions: HashSet<Position>,
    both_skip: bool,
    latest: (usize, usize),
    turn: Color,
    black_points: i32,
    white_points: i32,
}

impl Game {
    fn new() -> Self {
        let mut b = Game {
            board: [[None; 8]; 8],
            unput_positions: HashSet::new(),
            puttable_positions: HashSet::new(),
            both_skip: false,
            latest: (4, 4),
            turn: Black,
            black_points: 0,
            white_points: 0,
        };
        for i in 0..8 {
            for j in 0..8 {
                b.unput_positions.insert((i, j));
            }
        }
        b.set_with_color((3, 3), Black); // Black
        b.set_with_color((3, 4), White); // White
        b.set_with_color((4, 4), Black); // Black
        b.set_with_color((4, 3), White); // White
        b
    }

    fn start(&mut self) {
        loop {
            println!("{:?}'s turn!", self.turn);
            self.puttable_positions = self.find_puttable_positions();
            if self.puttable_positions.len() == 0 {
                println!("Hmm, you can't put anywhere... skip your turn!");
                self.turn = self.turn.another();
                if self.both_skip {
                    break;
                } else {
                    self.both_skip = true;
                    continue;
                }
            } else {
                self.both_skip = false;
                println!("puttable positions: {:?}", self.puttable_positions);
            }
            println!("{}", self);
            input! {
                x: (usize, usize)
            };
            self.put(x);
            if self.black_points + self.white_points == 64 {
                break;
            }
        }
        println!("{}", self);
        println!(
            "WINNER: {} !!",
            match self.black_points.cmp(&self.white_points) {
                Ordering::Greater => "Black",
                Ordering::Less => "White",
                _ => "Draw",
            }
        );
    }

    fn get(&self, (x, y): Position) -> Option<Color> {
        self.board[y][x]
    }

    fn set(&mut self, (x, y): Position, color: Option<Color>) {
        self.board[y][x] = color;
    }

    fn set_with_color(&mut self, position: Position, color: Color) {
        self.set(position, Some(color));
        self.incr_points(color);
        self.unput_positions.remove(&position);
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

    fn find_puttable_positions(&mut self) -> HashSet<Position> {
        let ex = self.latest;
        let mut set = HashSet::new();
        for position in self.unput_positions.clone().into_iter() {
            self.set(position, Some(self.turn));
            self.latest = position;
            if self.reversable_positions().len() != 0 {
                set.insert(position);
            }
            self.set(position, None);
            self.latest = ex;
        }
        set
    }

    fn put(&mut self, position: Position) {
        if self.puttable_positions.contains(&position) {
            self.set_with_color(position, self.turn);
            self.latest = position;
            for position in self.reversable_positions().iter() {
                self.set_with_color(*position, self.turn);
                self.decr_points(self.turn.another());
            }
            self.turn = self.turn.another();
        } else if let Some(color) = self.get(position) {
            println!("{:?} is already put!! {:?}", position, color);
        } else {
            println!("perhaps, you input invalid position: {:?}", position);
        }
    }

    fn reversable_positions(&self) -> Vec<Position> {
        let mut h = self.horizontal();
        let mut v = self.vertical();
        let mut d = self.diagonal();
        h.append(&mut v);
        h.append(&mut d);
        h
    }

    fn horizontal(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut left = find_reversable_positions!(self, (0..x).rev(), repeat(y));
        let mut right = find_reversable_positions!(self, (x + 1)..8, repeat(y));
        left.append(&mut right);
        left
    }

    fn vertical(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut up = find_reversable_positions!(self, repeat(x), (0..y).rev());
        let mut bottom = find_reversable_positions!(self, repeat(x), (y + 1)..8);
        up.append(&mut bottom);
        up
    }

    fn diagonal(&self) -> Vec<Position> {
        let (x, y) = self.latest;
        let mut left_up = find_reversable_positions!(self, (0..x).rev(), (0..y).rev());
        let mut left_bottom = find_reversable_positions!(self, (0..x).rev(), (y + 1)..8);
        let mut right_bottom = find_reversable_positions!(self, ((x + 1)..8), (y + 1)..8);
        let mut right_up = find_reversable_positions!(self, ((x + 1)..8), (0..y).rev());
        left_up.append(&mut left_bottom);
        left_up.append(&mut right_up);
        left_up.append(&mut right_bottom);
        left_up
    }
}

impl Display for Game {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "B: {}, W: {}", self.black_points, self.white_points);
        for column in self.board.iter() {
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
    let mut game = Game::new();
    game.start();
}
