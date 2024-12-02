use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut l1: Vec<u32> = vec![];
    let mut l2: HashMap<u32, u32> = HashMap::new();
    let mut score: u32 = 0;
    let message: String = fs::read_to_string("./src/input.txt")?;
    for line in message.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        l1.push(split[0].parse().unwrap());
        *l2.entry(split[1].parse().unwrap()).or_insert(0) += 1
    }
    for n in 0..l1.len() {
        let freq = l2.get(&l1[n]);
        if freq.is_some() {
            score += l1[n] * freq.unwrap();
        }
    }
    println!("{}", score);
    Ok(())
}
