use std::ops::Deref;

#[derive(Debug, Clone)]
enum Item {
    Directory { name: String, items: Vec<Item> },
    File { size: u64, name: String },
}

impl Item {
    // recursively gets the directory size
    fn get_size(&self) -> u64 {
        match self {
            Item::File { size, .. } => *size,
            Item::Directory { items, .. } => {
                let mut total_size = 0;
                for item in items {
                    total_size += item.get_size();
                }
                total_size
            }
        }
    }

    fn add_item(&mut self, item: Item) {
        match self {
            Item::Directory { items, .. } => {
                items.push(item);
            }
            _ => panic!("Cannot add item to file"),
        }
    }
}

// part one
// get the sum of the total size of every directory that has a size < 100000
fn sum_directories_part_one(item: &mut Item) -> u64 {
    let mut total_size_current_directory = 0;
    let size = item.get_size();

    if let Item::Directory { items, .. } = item {
        if size < 100000 {
            total_size_current_directory += size
        }

        for item in items {
            total_size_current_directory += sum_directories_part_one(item);
        }
    }

    total_size_current_directory
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Unable to read file")
}

// take in vec<&str> and create that path in the given directory
fn get_dir_from_path<'a>(path: &'a Vec<&str>, dir: &'a mut Item) -> &'a mut Item {
    let mut current_directory = dir;

    for directory_name in path {
        // if root directory, ignore
        if *directory_name == "/" {
            continue;
        }

        // find directory based on name
        if let Item::Directory { items, .. } = current_directory {
            current_directory = items
                .iter_mut()
                .find(|d| {
                    if let Item::Directory { name, .. } = d {
                        name == directory_name
                    } else {
                        false
                    }
                })
                .expect("unable to find directory from path");
        }
    }

    current_directory
}

// return root directory
fn process_input(input: &str) -> Item {
    let mut root = Item::Directory {
        name: "/".to_string(),
        items: vec![],
    };

    let mut cwd = vec!["/"];

    for line in input.lines() {
        // find directory based on name
        let current_directory = get_dir_from_path(&cwd, &mut root);

        let split_line: Vec<&str> = line.split(' ').collect();
        let iter = &mut split_line.iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        let is_command = first == &"$";
        if is_command {
            let command = second.deref();
            match command {
                "cd" => {
                    // either .. or directory name
                    let cd_move = iter.next().unwrap();

                    match cd_move {
                        &".." => {
                            cwd.pop();

                            continue;
                        }
                        directory_name => {
                            // if root directory, ignore
                            if *directory_name == "/" {
                                continue;
                            }

                            // go into new directory
                            cwd.push(directory_name);
                        }
                    }
                }
                "ls" => {
                    // just ignore, the next iteration will start
                    // adding items in
                }
                _ => panic!("Invalid command"),
            }
            continue;
        }

        if first == &"dir" {
            // add directory to current directory
            current_directory.add_item(Item::Directory {
                name: second.to_string(),
                items: vec![],
            });

            continue;
        }

        // if first split is a number, add a file to current directory
        if let Ok(size) = first.parse::<u64>() {
            current_directory.add_item(Item::File {
                size,
                name: second.to_string(),
            });

            continue;
        }
    }

    root
}

fn main() {
    println!("hello world");

    let input = read_input();
    let mut root = process_input(&input);
    let total_size = sum_directories_part_one(&mut root);

    println!("total size: {}", total_size);
}

#[cfg(test)]
mod test {
    #[test]
    fn part_one_example() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;
        let mut root = super::process_input(input);
        let total_size = super::sum_directories_part_one(&mut root);
        assert_eq!(total_size, 95437);
    }
}
