use std::vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("src/08/input")?;
    // let input = "30373\n25512\n65332\n33549\n35390";
    let forest: Vec<Vec<u8>> = input
        .lines()
        .map(|x| x.as_bytes().iter().map(|x| x - 48).collect::<Vec<u8>>())
        .collect();

    let cols = forest.first().unwrap().len();
    let rows = forest.len();

    let mut count = 0;

    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            let curr_height = forest[y][x];
            // search west
            let mut lower_trees_count = 0;
            for i in (0..x).rev() {
                if forest[y][i] < curr_height {
                    lower_trees_count += 1;
                }
            }
            if lower_trees_count == x {
                count += 1;
                continue;
            }
            // search east
            let mut lower_trees_count = 0;
            for i in x + 1..cols {
                if forest[y][i] < curr_height {
                    lower_trees_count += 1;
                }
            }
            if lower_trees_count == cols - x - 1 {
                count += 1;
                continue;
            }
            // search north
            let mut lower_trees_count = 0;
            for i in (0..y).rev() {
                if forest[i][x] < curr_height {
                    lower_trees_count += 1;
                }
            }
            if lower_trees_count == y {
                count += 1;
                continue;
            }
            // search south
            let mut lower_trees_count = 0;
            for i in y..rows {
                if forest[i][x] < curr_height {
                    lower_trees_count += 1;
                }
            }
            if lower_trees_count == rows - y - 1 {
                count += 1;
            }
        }
    }

    println!("solution 1: {}", count + (rows + cols) * 2 - 4);

    //
    let mut max_score = 0;
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            let mut scores = vec![];
            let curr_height = forest[y][x];
            // search west
            let mut score = 0;
            for i in (0..x).rev() {
                score += 1;
                if forest[y][i] >= curr_height {
                    break;
                }
            }
            scores.push(score);
            // search east
            let mut score = 0;
            for i in x + 1..cols {
                score += 1;
                if forest[y][i] >= curr_height {
                    break;
                }
            }
            scores.push(score);
            // search north
            let mut score = 0;
            for i in (0..y).rev() {
                score += 1;
                if forest[i][x] >= curr_height {
                    break;
                }
            }
            scores.push(score);
            // search south
            let mut score = 0;
            for i in y + 1..rows {
                score += 1;
                if forest[i][x] >= curr_height {
                    break;
                }
            }
            scores.push(score);
            let score = scores.iter().product();
            // print!("{}({:?}={}) ", curr_height, scores, score);
            max_score = max_score.max(score);
        }
        // println!("");
    }
    println!("solution 2: {}", max_score);
    Ok(())
}
