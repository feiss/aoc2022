type EntryId = usize;

#[derive(Debug)]
struct Entry {
    is_dir: bool,
    parent: Option<EntryId>,
    children: Vec<EntryId>,
    //data
    name: String,
    size: usize,
}

fn find_dir(hd: &Vec<Entry>, entry: &str, with_parent: Option<EntryId>) -> Option<EntryId> {
    hd.iter()
        .rposition(|x| x.name == entry && x.is_dir && x.parent == with_parent)
}

// fn calc_sizes(hd: &mut Vec<Entry>, entry: EntryId) -> usize {
//     let mut size = 0;
//     let children = hd[entry].children.clone();
//     for item in children {
//         if let Some(e) = hd.get(item) {
//             if e.children.is_empty() {
//                 size += e.size;
//             } else {
//                 size += calc_sizes(hd, item);
//             }
//         }
//     }
//     hd[entry].size = size;
//     size
// }

// fn print_folder(hd: &Vec<Entry>, entry: EntryId, depth: usize) {
//     let mut size = 0;
//     let children = hd[entry].children.clone();
//     for item in children {
//         if let Some(e) = hd.get(item) {
//             if e.children.is_empty() {
//                 println!("{}{} {}", " ".repeat(depth), e.name, e.size);
//             } else {
//                 println!("{}[{}] {}", " ".repeat(depth), e.name, e.size);
//                 print_folder(hd, item, depth + 4);
//             }
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string("src/07/input")?;

    // let f = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    let mut hd: Vec<Entry> = vec![];
    let mut curr_dir: Option<EntryId> = None;

    for (_i, line) in f.lines().skip(1).enumerate() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("$") => {
                let cmd = parts.next();
                match cmd {
                    Some("cd") => match parts.next() {
                        Some("..") => {
                            if curr_dir.is_some() {
                                let curr = hd.get(curr_dir.unwrap());
                                if let Some(child) = curr {
                                    curr_dir = child.parent;
                                    if let Some(dir) = curr_dir {
                                        hd[dir].size += child.size;
                                    }
                                }
                            }
                        }
                        Some(dir) => {
                            curr_dir = find_dir(&hd, dir, curr_dir);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            Some("dir") => {
                if curr_dir.is_some() {
                    let entryid = hd.len();
                    if let Some(dir) = hd.get_mut(curr_dir.unwrap()) {
                        dir.children.push(entryid);
                    }
                }

                hd.push(Entry {
                    is_dir: true,
                    parent: curr_dir,
                    children: vec![],
                    name: parts.next().unwrap_or("noname").to_string(),
                    size: 0,
                });
            }
            Some(size) => {
                let s = size.parse::<usize>().unwrap();

                if curr_dir.is_some() {
                    let entryid = hd.len();
                    if let Some(dir) = hd.get_mut(curr_dir.unwrap()) {
                        dir.children.push(entryid);
                        dir.size += s;
                    }
                }
                hd.push(Entry {
                    is_dir: false,
                    parent: curr_dir,
                    children: vec![],
                    name: parts.next().unwrap_or("noname").to_string(),
                    size: s,
                });
            }

            None => {}
        }
    }

    let sol1: usize = hd
        .iter()
        .filter(|x| !x.children.is_empty() && x.size <= 100_000)
        .map(|x| x.size)
        .sum();

    println!("solution 1: {}", sol1);

    ///////////

    let space_used: usize = hd
        .iter()
        .filter(|x| x.parent.is_none())
        .map(|x| x.size)
        .sum();

    let space_needed = 30000000 - (70000000 - space_used as isize);
    println!("used: {}, needed: {}", space_used, space_needed);
    println!(
        "solution 2: {}",
        hd.iter()
            .filter(|x| x.is_dir && x.size >= space_needed as usize)
            .map(|x| x.size)
            .min()
            .unwrap()
    );
    Ok(())
}
