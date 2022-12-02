use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // a
    let f = fs::read_to_string("src/01/input")?;
    let mut accum = 0;
    let mut max_calories = 0;
    for line in f.lines() {
        if line.is_empty() {
            if accum > max_calories {
                max_calories = accum;
            }
            accum = 0;
        } else {
            accum += line.parse::<u32>()?;
        }
    }
    println!("max calories: {max_calories}");

    // b

    let mut accum = 0;
    let mut max_calories = vec![0, 0, 0];
    for line in f.lines() {
        if line.is_empty() {
            let min_item = max_calories
                .iter()
                .enumerate()
                .reduce(|accum, item| if accum.1 <= item.1 { accum } else { item })
                .unwrap_or((0, &0))
                .0;

            if accum > max_calories[min_item] {
                max_calories[min_item] = accum;
            }
            accum = 0;
        } else {
            accum += line.parse::<u32>()?;
        }
    }
    println!(
        "max calories top 3: {:?}",
        max_calories[0] + max_calories[1] + max_calories[2]
    );

    Ok(())
}
