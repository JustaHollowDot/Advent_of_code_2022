use std::fs::read_to_string;
use crate::Solution;

impl Solution {
    pub fn day_3_task_1() {
        let contents = read_to_string(r"input\day_3\day_3_input").unwrap();

        let lines: Vec<&str> = contents.lines().collect();

        let mut doubles: Vec<Vec<&str>> = Vec::new();

        for line in lines {
            let temp = line.split_at((line.len() / 2) as usize);

            doubles.push(vec![temp.0, temp.1])
        }

        let mut char_values = Vec::new();

        for line in &doubles {
            println!("{:?}", line);

            for character in line[0].chars() {
                if line[1].contains(character) {
                    println!("{}", character);
                    if character.is_uppercase() {
                        println!("{:?}", character as u32 - 38);
                        char_values.push(character as u32 - 38);
                    } else {
                        println!("{:?}", character as i32 - 96);
                        char_values.push(character as u32 - 96);
                    }

                    break
                }
            }
        }

        println!("{}", char_values.iter().sum::<u32>());
    }

    pub fn day_3_task_2() {
        let contents = read_to_string(r"input\day_3\day_3_input").unwrap();

        let lines: Vec<&str> = contents.lines().collect();

        let mut doubles: Vec<Vec<&str>> = Vec::new();

        for line in &lines {
            let temp = line.split_at((line.len() / 2) as usize);

            doubles.push(vec![temp.0, temp.1])
        }

        let mut char_values = Vec::new();

        for (i, line) in lines.iter().enumerate().step_by(3) {
            for character in line.chars() {
                if lines[i+1].contains(character) && lines[i+2].contains(character) {
                    println!("{}", character);
                    if character.is_uppercase() {
                        println!("{:?}", character as u32 - 38);
                        char_values.push(character as u32 - 38);
                    } else {
                        println!("{:?}", character as i32 - 96);
                        char_values.push(character as u32 - 96);
                    }
                    break
                }
            }


            println!("{}", line);
        }


        println!("{}", char_values.iter().sum::<u32>());
    }
}