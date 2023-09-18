use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = fs::read_to_string("input.txt")?;
    let v = lines.lines().collect::<Vec<_>>();
    let mut groups = v
        .split(|line| line.is_empty())
        .map(|s| {
            s.iter()
                .map(|r| r.parse::<i32>())
                .map(Result::unwrap)
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    groups.sort();
    println!("{:?}", groups.iter().rev().take(3).sum::<i32>());
    Ok(())
}
