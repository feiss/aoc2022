fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut score1 = 0;
    let mut score2 = 0;
    let f = std::fs::read_to_string("src/02/input")?;
    for line in f.lines() {
        // part 1
        let their_hand = line.chars().nth(0).unwrap_or('?') as u32 - 'A' as u32;
        let my_hand = line.chars().nth(2).unwrap_or('?') as u32 - 'X' as u32;

        let result = if (their_hand + 2) % 3 == my_hand {
            0
        } else if their_hand == my_hand {
            3
        } else {
            6
        };
        score1 += (my_hand + 1) + result;

        // part 2
        let result = my_hand;
        let my_hand = if result == 1 {
            their_hand
        } else if result == 0 {
            (their_hand + 2) % 3
        } else {
            (their_hand + 1) % 3
        };

        score2 += (my_hand + 1) + result * 3;
    }
    println!("solution 1 - score: {score1}");
    println!("solution 2 - score: {score2}");

    Ok(())
}
