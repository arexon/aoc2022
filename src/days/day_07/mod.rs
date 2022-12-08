use std::{collections::HashMap, str::Lines};

pub fn run(input: &str) -> (String, String) {
    (part_one(input.lines()), part_two(input.lines()))
}

const AVAILABLE_SPACE: usize = 70_000_000;
const UNUSED_SPACE: usize = 30_000_000;
const THRESHOLD: usize = 100_000;

fn part_one(input: Lines) -> String {
    let map = paths(input);
    map.values()
        .filter(|&size| size <= &THRESHOLD)
        .sum::<usize>()
        .to_string()
}

fn part_two(input: Lines) -> String {
    let map = paths(input);
    map.values()
        .filter(|&size| size >= &(UNUSED_SPACE - (AVAILABLE_SPACE - map["."])))
        .min()
        .unwrap()
        .to_string()
}

fn paths(input: Lines) -> HashMap<String, usize> {
    let (_, map) = input
        .filter(|&output| output != "$ ls" && &output[0..3] != "dir")
        .fold(
            (String::with_capacity(70), HashMap::with_capacity(170)),
            |(mut cwd, mut map), line| {
                match line {
                    "$ cd /" => cwd.push('.'),
                    "$ cd .." => cwd.truncate(cwd.rfind('/').unwrap()),
                    _ if line.starts_with("$ cd") => {
                        cwd.push('/');
                        cwd.push_str(&line[5..]);
                    }
                    _ => {
                        let size1: usize = line.split_whitespace().next().unwrap().parse().unwrap();
                        let size2 = map.entry(cwd.clone()).or_insert(0);
                        *size2 += size1;
                        cwd.match_indices('/').for_each(|(i, _)| {
                            let size3 = map.entry(cwd[0..i].to_string()).or_insert(0);
                            *size3 += size1;
                        })
                    }
                };
                (cwd, map)
            },
        );
    map
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
    fn test_part_one() {
        assert_eq!(part_one(INPUT.lines()), "95437");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.lines()), "24933642");
    }
}
