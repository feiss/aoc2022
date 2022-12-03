fn priority(a: u8) -> u32 {
    let (l, offset) = if a.is_ascii_uppercase() {
        ('A', 27)
    } else {
        ('a', 1)
    };
    a as u32 - l as u32 + offset
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string("src/03/input")?;
    let mut sum = 0;
    for line in f.lines() {
        let count = line.len() / 2;
        let (comp1, comp2) = line.as_bytes().split_at(count);

        'outer: for i in 0..count {
            for j in 0..count {
                if comp1[i] == comp2[j] {
                    sum += priority(comp1[i]);
                    break 'outer;
                }
            }
        }
    }
    println!("solution 1: {sum}");

    let mut sum = 0;
    let mut lines = f.lines();
    loop {
        let aa = match lines.next() {
            Some(a) => a,
            None => {
                break;
            }
        };
        let bb = lines.next().unwrap();
        let cc = lines.next().unwrap();
        // yes, I know:
        'outer2: for a in aa.bytes() {
            for b in bb.bytes() {
                for c in cc.bytes() {
                    if a == b && b == c {
                        sum += priority(a);
                        break 'outer2;
                    }
                }
            }
        }
    }
    println!("solution 2: {sum}");
    Ok(())
}
