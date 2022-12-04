fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string("src/04/input")?;

    let pairs = f.lines().map(|line| {
        line.split(&['-', ','])
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect::<Vec<u32>>()
    });

    let contain_pairs = pairs
        .clone()
        .filter(|x| (x[0] <= x[2] && x[1] >= x[3]) || (x[2] <= x[0] && x[3] >= x[1]));

    let overlap_pairs = pairs.filter(|x| (x[1] >= x[2] && x[0] <= x[3]));

    println!("solution 1: {}", contain_pairs.count());
    println!("solution 2: {}", overlap_pairs.count());

    Ok(())
}
