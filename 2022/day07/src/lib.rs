use nom::{
    bytes::complete::{tag, take_till1, take_until},
    character::{complete::line_ending, is_newline, streaming::not_line_ending},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
struct FileTreeNode {
    name: String,
    content: FileTreeContent,
    size: u32,
}

#[derive(Debug)]
enum FileTreeContent {
    Dir(Vec<FileTreeNode>),
    File,
}

fn directory_parser(input: &str) -> IResult<&str, FileTreeNode> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir_name) = not_line_ending(input)?;
    //dbg!(dir_name);
    let (input, _) = line_ending(input)?;
    //dbg!(input);
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((
        input,
        FileTreeNode {
            name: dir_name.to_string(),
            content: FileTreeContent::File,
            size: 0,
        },
    ))
}

pub fn puzzle1(input: &str) -> String {
    let (_, node) = directory_parser(input).unwrap();
    dbg!(node);
    "None".to_string()
}

pub fn puzzle2(input: &str) -> String {
    "None".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn puzzle1_works() {
        assert_eq!(puzzle1(INPUT), "95437");
    }

    #[test]
    fn puzzle2_works() {
        assert_eq!(puzzle2(INPUT), "Result");
    }
}
