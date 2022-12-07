use std::{cell::RefCell, cmp::min, collections::HashMap, process::exit, rc::Rc, u32::MAX};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{alpha1, char, newline, u32},
        is_alphanumeric,
    },
    combinator::map,
    multi::many1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub fn part1(input: String) -> u32 {
    let root = parse_filesystem(input.as_str());

    // Traverse the tree from the bottom up, computing total sizes.
    let (_, count) = count_subdirs_by_size(&root.borrow());

    count
}

type TotalSize = u32;

type CountedSize = u32;

fn count_subdirs_by_size(dir: &Directory) -> (TotalSize, CountedSize) {
    let direct_size: u32 = dir.files.iter().map(|(_, size)| size).copied().sum();
    let (subdir_total_size, subdir_counted_size) = dir
        .dirs
        .iter()
        .map(|(_, subdir)| count_subdirs_by_size(&subdir.borrow()))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    let total_size = direct_size + subdir_total_size;
    (
        total_size,
        if total_size <= 100_000 { total_size } else { 0 } + subdir_counted_size,
    )
}

pub fn part2(input: String) -> u32 {
    let root = parse_filesystem(input.as_str());

    // Compute space needed.
    let space_available = 70_000_000;
    let space_needed = 30_000_000;
    let (space_used, _) = count_subdirs_by_size(&root.borrow());
    let min_space_to_delete = space_used - (space_available - space_needed);

    // Find the smallest directory larger than the threshold.
    let (_, smallest_to_delete_size) =
        smallest_subdir_over_threshold(&root.borrow(), min_space_to_delete);

    smallest_to_delete_size
}

type SmallestSoFarSize = u32;

fn smallest_subdir_over_threshold(
    dir: &Directory,
    threshold: u32,
) -> (TotalSize, SmallestSoFarSize) {
    let direct_size: u32 = dir.files.iter().map(|(_, size)| size).copied().sum();
    let (subdir_total_size, subdir_smallest_so_far_size) = dir
        .dirs
        .iter()
        .map(|(_, subdir)| smallest_subdir_over_threshold(&subdir.borrow(), threshold))
        .fold((0, MAX), |(a, b), (c, d)| (a + c, min(b, d)));
    let total_size = direct_size + subdir_total_size;
    (
        total_size,
        if total_size > threshold {
            min(total_size, subdir_smallest_so_far_size)
        } else {
            subdir_smallest_so_far_size
        },
    )
}

#[derive(Debug)]
struct Directory<'a> {
    // We need the Rc because multiple children can have pointers to their
    // parents, and the RefCell because the parent needs interior mutation
    // during construction while some of the children (who have already been
    // constructed) hold a pointer to the parent.
    //
    // If we interpret the puzzle's semantics strictly, there is no way to
    // guarantee that we can construct all children before their parents,
    // because the terminal commands don't have a guaranteed traversal order.
    //
    // Alternative implementations include:
    //
    // - Using a Zipper.
    // - Using a Vec-backed tree implementation (each node has its own index,
    //   and refers to other nodes by index).
    // - Using a Map-backed tree implementation (each node is keyed by its
    //   directory path, and refers to other nodes by path).
    //
    // We only use the parent pointer during the _construction_ of the tree -
    // once constructed, should we throw it away somehow?
    parent: Option<Rc<RefCell<Directory<'a>>>>,
    files: HashMap<&'a str, u32>,
    dirs: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
}

#[derive(Debug, PartialEq)]
enum ChangeDirTarget<'a> {
    In { dir: &'a str },
    Out,
    Root,
}
use ChangeDirTarget::*;

#[derive(Debug, PartialEq)]
enum ListEntry<'a> {
    File { name: &'a str, size: u32 },
    Directory { name: &'a str },
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    ChangeDir(ChangeDirTarget<'a>),
    List(Vec<ListEntry<'a>>),
}

use Command::*;

fn parse_filesystem(input: &str) -> Rc<RefCell<Directory>> {
    let commands = super::shared::must_parse(parse_commands, input);
    build_filesystem(commands)
}

fn build_filesystem(commands: Vec<Command>) -> Rc<RefCell<Directory>> {
    // Create root node.
    assert_eq!(commands[0], ChangeDir(Root));
    let root = Rc::new(RefCell::new(Directory {
        parent: None,
        files: HashMap::new(),
        dirs: HashMap::new(),
    }));

    // Build a tree, going up and down inodes and filling out file entries as
    // list output directs us.
    let mut current_ptr = root.clone();
    for command in &commands[1..] {
        match command {
            List(entries) => {
                let mut current = current_ptr.borrow_mut();
                for entry in entries {
                    match entry {
                        ListEntry::File { name, size } => {
                            current.files.insert(name, *size);
                        }
                        ListEntry::Directory { name } => {
                            let child = Rc::new(RefCell::new(Directory {
                                parent: Some(current_ptr.clone()),
                                files: HashMap::new(),
                                dirs: HashMap::new(),
                            }));
                            current.dirs.insert(name, child);
                        }
                    }
                }
            }
            ChangeDir(In { dir }) => {
                current_ptr = {
                    let current = current_ptr.borrow();
                    current
                        .dirs
                        .get(dir)
                        .unwrap_or_else(|| {
                            println!("Impossible: navigated into non-existent directory");
                            exit(1)
                        })
                        .clone()
                };
            }
            ChangeDir(Out) => {
                current_ptr = {
                    let current = current_ptr.borrow();
                    current
                        .parent
                        .as_ref()
                        .unwrap_or_else(|| {
                            println!("Impossible: navigated outside of filesystem");
                            exit(1)
                        })
                        .clone()
                }
            }
            ChangeDir(Root) => {
                current_ptr = root.clone();
            }
        }
    }

    root
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many1(preceded(
        tag("$ "),
        alt((
            map(delimited(tag("cd "), alpha1, newline), |dir| {
                ChangeDir(In { dir })
            }),
            map(tag("cd ..\n"), |_| ChangeDir(Out)),
            map(tag("cd /\n"), |_| ChangeDir(Root)),
            preceded(
                tag("ls\n"),
                map(
                    many1(terminated(
                        alt((
                            map(separated_pair(u32, char(' '), filename), |(size, name)| {
                                ListEntry::File { name, size }
                            }),
                            map(preceded(tag("dir "), filename), |name| {
                                ListEntry::Directory { name }
                            }),
                        )),
                        newline,
                    )),
                    |entries| List(entries),
                ),
            ),
        )),
    ))(input)
}

fn filename(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| is_alphanumeric(c as u8) || c == '.')(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "$ cd /
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
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.into()), 95437)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.into()), 24933642)
    }
}
