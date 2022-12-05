use tinyvec::{array_vec, ArrayVec};

pub fn run(input: &str) -> (String, String) {
    let input = input.split_once("\n\n").unwrap();
    (part_one(input), part_two(input))
}

type Stacks = ArrayVec<[Containers; 9]>;
type Containers = ArrayVec<[char; 64]>;

fn part_one<'a>((stacks, instructions): (&'a str, &'a str)) -> String {
    rearrange((stacks, instructions), false)
}

fn part_two<'a>((stacks, instructions): (&'a str, &'a str)) -> String {
    rearrange((stacks, instructions), true)
}

fn rearrange<'a>((stacks, instructions): (&'a str, &'a str), do_reverse: bool) -> String {
    let mut stacks = stacks
        .lines()
        .rev()
        .skip(1)
        .flat_map(|container| {
            container
                .char_indices()
                .skip(1)
                .step_by(4)
                .filter_map(|(index, id)| {
                    if id != ' ' {
                        Some((index / 4, id))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, char)>>()
        })
        .enumerate()
        .fold(
            array_vec![array_vec![' '; 64]; 9],
            |mut vec, (row_index, (col_index, container))| {
                vec[col_index][row_index] = container;
                vec
            },
        )
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .filter(|&c| c.is_alphabetic())
                .collect::<Containers>()
        })
        .collect::<Stacks>();

    instructions.lines().for_each(|instru| {
        let [amount, from, to] = instru
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>()
            .into_iter()
            .enumerate()
            .fold([0, 0, 0], |mut arr, (i, instru)| {
                arr[i] = instru;
                arr
            });

        let mut containrs = (0..amount)
            .map(|_| stacks[from - 1].pop().unwrap())
            .collect::<Containers>();
        if do_reverse {
            containrs.reverse();
        }
        stacks[to - 1].append(&mut containrs);
    });

    stacks
        .iter()
        .filter_map(|s| s.last())
        .fold(String::new(), |mut str, char| {
            str.push(*char);
            str
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.split_once("\n\n").unwrap()), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.split_once("\n\n").unwrap()), "MCD");
    }
}
