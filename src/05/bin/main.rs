fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ship: Vec<String> = vec![
        "QWPSZRHD".to_string(),
        "VBRWQHF".to_string(),
        "CVSH".to_string(),
        "HFG".to_string(),
        "PGJBZ".to_string(),
        "QTJHWFL".to_string(),
        "ZTWDLVJN".to_string(),
        "DTZCJGHF".to_string(),
        "WPVMBH".to_string(),
    ];

    let f = std::fs::read_to_string("src/05/input")?;
    let mut reading_stack = true;
    for l in f.lines() {
        if l.is_empty() {
            reading_stack = false;
            continue;
        }
        if !reading_stack {
            let op: Vec<usize> = l
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            for _ in 0..op[0] {
                if let Some(creit) = ship[op[1] - 1].pop() {
                    ship[op[2] - 1].push(creit);
                }
            }
        }
    }
    let mut result = String::new();
    for stack in ship {
        result.push(stack.chars().last().unwrap_or('?'));
    }
    println!("solution 1: {:?}", result);

    ////////////////

    let mut ship: Vec<String> = vec![
        "QWPSZRHD".to_string(),
        "VBRWQHF".to_string(),
        "CVSH".to_string(),
        "HFG".to_string(),
        "PGJBZ".to_string(),
        "QTJHWFL".to_string(),
        "ZTWDLVJN".to_string(),
        "DTZCJGHF".to_string(),
        "WPVMBH".to_string(),
    ];

    let mut reading_stack = true;
    for l in f.lines() {
        if l.is_empty() {
            reading_stack = false;
            continue;
        }
        if !reading_stack {
            let op: Vec<usize> = l
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            let stack = ship[op[1] - 1].clone();
            let (crates1, crates2) = stack.split_at(ship[op[1] - 1].len() - op[0]);
            ship[op[1] - 1] = crates1.clone().to_string();
            ship[op[2] - 1].push_str(crates2.clone());
        }
    }
    let mut result = String::new();
    for stack in ship {
        result.push(stack.chars().last().unwrap_or('?'));
    }
    println!("solution 2: {:?}", result);
    Ok(())
}
