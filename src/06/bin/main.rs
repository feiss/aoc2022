use std::{collections::HashSet, str::Bytes};

fn different(bytes: Bytes, n: usize) -> Option<usize> {
    for i in 0..bytes.len() - n {
        // cloning bytes on each iteration.. can this be more innefficient? --YOLO!
        let set: HashSet<u8> = HashSet::from_iter(bytes.clone().skip(i).take(n));
        if set.len() == n {
            return Some(i + n);
        }
    }
    return None;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string("src/06/input")?;
    println!("solution 1: {:?}", different(f.bytes(), 4));
    println!("solution 2: {:?}", different(f.bytes(), 14));
    Ok(())
}
