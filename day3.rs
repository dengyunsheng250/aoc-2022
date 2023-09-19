use std::{collections::HashSet, fs, hash::Hash};
// fn split(s: &str) -> (&str, &str) {
//     let len = s.len() / 2;
//     (&s[..len], &s[len..])
// }
// fn compare(t: (&str, &str)) -> i32 {
//     let (a, b) = t;
//     for (i, c) in a.chars().enumerate() {
//         if b.contains(&a[i..i + 1]) {
//             if c.is_ascii_uppercase() {
//                 return c as i32 + 27 - b'A';
//             } else {
//                 return c as i32 + 1 - b'a';
//             }
//         }
//     }
//     0
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = 0;
    let str = fs::read_to_string("input.txt")?;
    let v: Vec<&str> = str.lines().collect();
    for lines in v.chunks(3) {
        let mut sets: Vec<HashSet<char>> = lines
            .iter()
            .map(|l| HashSet::from_iter(l.chars()))
            .collect();
        let mut dup = sets.pop().unwrap();
        for set in sets {
            dup = set.intersection(&dup).copied().collect();
        }
        let ch = *dup.iter().next().unwrap();
        res += match ch {
            'a'..='z' => ch as u8 - b'a' + 1,
            'A'..='Z' => ch as u8 - b'A' + 27,
            _ => 0,
        } as i32;
    }
    println!("{}", res);
    Ok(())
}
