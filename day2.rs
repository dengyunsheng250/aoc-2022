use core::panic;
use std::fs;
enum Move {
    Rock,
    Paper,
    Scissors,
}
enum Out {
    Lose,
    Draw,
    Win,
}
impl Out {
    fn new(x: &str) -> Self {
        match x {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("unimplemented!"),
        }
    }
    fn score(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}
fn succ(t: (&str, Out)) -> i32 {
    match t {
        ("A", Out::Lose) => 3,
        ("A", Out::Draw) => 1,
        ("A", Out::Win) => 2,
        ("B", Out::Lose) => 1,
        ("B", Out::Draw) => 2,
        ("B", Out::Win) => 3,
        ("C", Out::Lose) => 2,
        ("C", Out::Draw) => 3,
        ("C", Out::Win) => 1,
        _ => panic!("unimplemented!"),
    }
}
fn turn_to_tuple(s: &str) -> (&str, Out) {
    let x = &s[0..1];
    let y = Out::new(&s[2..]);
    (x, y)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("input.txt")?;
    let scores1: i32 = s.lines().map(turn_to_tuple).map(succ).sum();
    let scores2: i32 = s.lines().map(turn_to_tuple).map(|(_, o)| o.score()).sum();
    println!("{}", scores1 + scores2);
    Ok(())
}
