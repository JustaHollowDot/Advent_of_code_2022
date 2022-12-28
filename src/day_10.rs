use std::fs::read_to_string;
use crate::Solution;
use crate::InputReader;

impl InputReader {
    pub fn read_input_day_10() -> String {
        let contents = read_to_string(r"input\day_10\day_10_input").unwrap();
        return contents;
    }

    pub fn read_input_day_10_test() -> String {
        let contents = read_to_string(r"input\day_10\day_10_input_test").unwrap();
        return contents;
    }
}


impl Solution {
    pub fn day_10_task_1() -> i32 {
        let contents = InputReader::read_input_day_10();

        let mut register_values: Vec<i32> = Vec::new();

        for action in contents.lines() {
            let action = action.split_whitespace().collect::<Vec<&str>>();
            println!("{}, {:?}", register_values.len(), action);

            println!("1 -> {}", register_values.last().unwrap_or(&1));

            match action[0] {
                "noop" => {
                    register_values.push(register_values.last().unwrap_or(&1).clone());
                },
                "addx" => {
                    register_values.push(register_values.last().unwrap_or(&1).clone());

                    let value = action[1].parse::<i32>().unwrap();
                    register_values.push(register_values.last().unwrap().clone() + value);
                },
                _ => panic!("Invalid action"),
            }
            println!("{}, {:?}", register_values.len(), action);

            println!("2 -> {}", register_values.last().unwrap_or(&1));
        }

        println!("{:?}", register_values);

        let mut total_value = 0;

        for i in 0..6 {
            println!("{}: {} -> {}", i*40 + 20, register_values[i*40 + 18], register_values[i*40 + 18] * ((i * 40) + 20) as i32);
            total_value += register_values[i*40 + 18] * (i * 40 + 20) as i32;
        }

        return total_value;
    }

    pub fn day_10_task_2() -> Vec<Vec<i32>> {
        let contents = InputReader::read_input_day_10();

        let mut register_values: Vec<i32> = Vec::new();
        register_values.push(1);

        for action in contents.lines() {
            let action = action.split_whitespace().collect::<Vec<&str>>();
            println!("{}, {:?}", register_values.len(), action);

            println!("1 -> {}", register_values.last().unwrap_or(&1));

            match action[0] {
                "noop" => {
                    register_values.push(register_values.last().unwrap_or(&1).clone());
                },
                "addx" => {
                    register_values.push(register_values.last().unwrap_or(&1).clone());

                    let value = action[1].parse::<i32>().unwrap();
                    register_values.push(register_values.last().unwrap().clone() + value);
                },
                _ => panic!("Invalid action"),
            }
            println!("{}, {:?}", register_values.len(), action);

            println!("2 -> {}", register_values.last().unwrap_or(&1));
        }

        println!("{:?}", register_values);

        let mut drawn_values:Vec<Vec<i32>> = Vec::new();
        for i in 0..6 {
            drawn_values.push(Vec::new());
            for j in 0..40 {
                let value = register_values[i*40 + j];
                if i == 0 {
                    println!("{} -> {:?}", j, (value-1)..(value+1));
                    println!("{}", ((value-1)..(value+1)).contains(&(j as i32)));
                }
                if ((value-1)..(value+2)).contains(&(j as i32)) {
                    drawn_values[i].push(1);
                } else {
                    drawn_values[i].push(0);
                }
            }
        }

        for line in drawn_values.iter() {
            for value in line.iter() {
                if *value == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }

        return drawn_values;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_task_1() {
        assert_eq!(Solution::day_10_task_1(), 13140);
    }
}

