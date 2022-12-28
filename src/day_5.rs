use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn day_5_task_1() {
        let contents = read_to_string(r"input\day_5\day_5_input").unwrap();
        println!("{}", contents);

        let (stacks, moves) = contents.split_once("\r\n\r\n").unwrap();

        let stacks = stacks.rsplit_once("\r\n").unwrap().0;

        let stacks = stacks.lines().collect::<Vec<&str>>();

        let mut new_stack = Vec::new();

        for line in &stacks {
            let mut new_line = Vec::new();
            for i in 0..9 {
                new_line.push(line.chars().skip(1 + i * 4).next().unwrap_or(' '));
            }

            new_stack.push(new_line);
        }

        let stacks = new_stack.iter().map(|line| {
            line.iter().map(|character| {
                match character {
                    ' ' => { None }
                    _ => { Some(character) }
                }
            }).collect::<Vec<Option<&char>>>()
        }).collect::<Vec<Vec<Option<&char>>>>();

        let mut vertical_stacks: Vec<Vec<&char>> = vec![Vec::new(); 9];

        for line in stacks {
            for (i, character) in line.iter().enumerate() {
                if character != &None {
                    vertical_stacks[i].insert(0, character.unwrap());
                }
            }
        }

        for line in &vertical_stacks {
            println!("{:?}", line);
        }



        let moves = moves.lines().collect::<Vec<&str>>();

        let mut moves_action = Vec::new();

        for line in &moves {
            println!("{}", line);

            let test = line.split(" ").collect::<Vec<&str>>();
            let test = test.iter().map(|x| {
                if x.parse::<usize>().is_ok() {
                    x.parse::<usize>().unwrap()
                } else { 0 }
            }).collect::<Vec<usize>>();

            let mut test: Vec<usize> = test.iter().filter(|x| *x != &0).map(|x| *x).collect();

            test[1] -= 1;
            test[2] -= 1;

            println!("{:?}", test);

            moves_action.push(test);
        }

        println!("{:?}", moves_action);


        for action in moves_action {
            for _ in 0..action[0] {
                println!("{:?}", vertical_stacks[action[1]]);
                if !vertical_stacks[action[1]].is_empty() {
                    let character = vertical_stacks[action[1] as usize].pop().unwrap();
                    vertical_stacks[action[2] as usize].push(character);
                }
            }
        }

        println!("{:?}", vertical_stacks);

        let mut chars = String::new();
        for line in &vertical_stacks {
            chars += &line.clone().pop().unwrap_or(&' ').to_string();
        }

        println!("{}", chars);
    }

    pub fn day_5_task_2() {
        let contents = read_to_string(r"input\day_5\day_5_input").unwrap();
        println!("{}", contents);

        let (stacks, moves) = contents.split_once("\r\n\r\n").unwrap();

        let stacks = stacks.rsplit_once("\r\n").unwrap().0;

        let stacks = stacks.lines().collect::<Vec<&str>>();

        let mut new_stack = Vec::new();

        for line in &stacks {
            let mut new_line = Vec::new();
            for i in 0..9 {
                new_line.push(line.chars().skip(1 + i * 4).next().unwrap_or(' '));
            }

            new_stack.push(new_line);
        }

        let stacks = new_stack.iter().map(|line| {
            line.iter().map(|character| {
                match character {
                    ' ' => { None }
                    _ => { Some(character) }
                }
            }).collect::<Vec<Option<&char>>>()
        }).collect::<Vec<Vec<Option<&char>>>>();

        let mut vertical_stacks: Vec<Vec<&char>> = vec![Vec::new(); 9];

        for line in stacks {
            for (i, character) in line.iter().enumerate() {
                if character != &None {
                    vertical_stacks[i].insert(0, character.unwrap());
                }
            }
        }

        for line in &vertical_stacks {
            println!("{:?}", line);
        }



        let moves = moves.lines().collect::<Vec<&str>>();

        let mut moves_action = Vec::new();

        for line in &moves {
            println!("{}", line);

            let test = line.split(" ").collect::<Vec<&str>>();
            let test = test.iter().map(|x| {
                if x.parse::<usize>().is_ok() {
                    x.parse::<usize>().unwrap()
                } else { 0 }
            }).collect::<Vec<usize>>();

            let mut test: Vec<usize> = test.iter().filter(|x| *x != &0).map(|x| *x).collect();

            test[1] -= 1;
            test[2] -= 1;

            println!("{:?}", test);

            moves_action.push(test);
        }

        println!("{:?}", moves_action);


        for action in moves_action {
            let mut temp = Vec::new();
            for _ in 0..action[0] {
                println!("{:?}", vertical_stacks);

                if !vertical_stacks[action[1]].is_empty() {
                    let character = vertical_stacks[action[1] as usize].pop().unwrap();
                    temp.push(character);
                }
            }

            for x in temp.iter().rev() {
                vertical_stacks[action[2] as usize].push(x);
            }
        }


        println!("{:?}", vertical_stacks);

        let mut chars = String::new();
        for line in &vertical_stacks {
            chars += &line.clone().pop().unwrap_or(&' ').to_string();
        }

        println!("{}", chars);
    }
}