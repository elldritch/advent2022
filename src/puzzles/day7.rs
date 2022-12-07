use std::{cell::RefCell, collections::HashMap, process::exit, rc::Rc};

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
    let commands = super::shared::must_parse(parse_commands, input.as_str());

    // Create root node.
    assert_eq!(commands[0], ChangeDir(Root));
    let root = Rc::new(RefCell::new(Directory {
        parent: None,
        files: HashMap::new(),
        dirs: HashMap::new(),
    }));

    // Build a tree, going up and down inodes and filling out file entries as
    // list output directs us.
    let mut current_ptr = &root;
    for command in &commands[1..] {
        println!("{command:?}");

        match command {
            List(entries) => {
                let mut current = current_ptr.borrow_mut();
                for entry in entries {
                    match entry {
                        ListEntry::File { name, size } => {
                            current.files.insert(name, *size);
                        }
                        ListEntry::Directory { name } => {
                            let child = Directory {
                                parent: Some(current_ptr.clone()),
                                files: HashMap::new(),
                                dirs: HashMap::new(),
                            };
                            current.dirs.insert(name, Rc::new(RefCell::new(child)));
                        }
                    }
                }
            }
            ChangeDir(In { dir }) => {
                let current = current_ptr.borrow();
                let target = current.dirs.get(dir).unwrap_or_else(|| {
                    println!("Impossible: navigated into non-existent directory");
                    exit(1)
                }).clone();
                current_ptr = &target;
            }
            ChangeDir(Out) => {
                // let parent = parents.pop().unwrap_or_else(|| {
                //     println!("Impossible: navigated out of filesystem");
                //     exit(1)
                // });
                // current = parent;
            }
            ChangeDir(Root) => {
                current_ptr = &root;
            }
        }
    }
    todo!()
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
    // Alternative implementations include a zipper or Vec/index-based nodes.
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

pub fn part2(input: String) -> u32 {
    todo!()
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
        assert_eq!(part2(EXAMPLE_INPUT.into()), 0)
    }
}
