use std::fs;

fn main() -> std::io::Result<()> {
    let mut l1: Vec<u32> = vec![];
    let mut l2: Vec<u32> = vec![];
    let mut dist: Vec<u32> = vec![];
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    let message: String = fs::read_to_string("./src/input.txt")?;
    for line in message.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        l1.push(split[0].parse().unwrap());
        l2.push(split[1].parse().unwrap());
    }
    l1.sort();
    l2.sort();
    for n in 0..l1.len() {
        dist.push(if l1[n] > l2[n] {l1[n] - l2[n]} else {l2[n] - l1[n]});
    }
    println!("{}", dist.into_iter().sum::<u32>());
    Ok(())
}
