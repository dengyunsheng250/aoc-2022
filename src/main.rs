use std::fs;

#[derive(Debug)]
struct Pair(i32, i32);
impl Pair {
    fn new(s: &str) -> Self {
        let v = s
            .split('-')
            .map(|s| s.parse::<i32>())
            .map(Result::unwrap_or_default)
            .collect::<Vec<_>>();
        Self(v[0], v[1])
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = (1, 2);
    let v = fs::read_to_string("input.txt")?
        .lines()
        .map(|s| s.split(',').map(|s| Pair::new(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let res: i32 = v
        .iter()
        .map(move |s| -> i32 {
            let a = &s[0];
            let b = &s[1];
            if a.0 >= b.0 && a.1 <= b.1 {
                return 1;
            }
            if a.0 <= b.0 && a.1 >= b.1 {
                return 1;
            }
            return 0;
        })
        .sum();
    println!("{}", res);
    // println!("{:?}", v);
    Ok(())
}
