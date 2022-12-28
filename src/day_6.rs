use std::fs::read_to_string;
use crate::Solution;
use crate::InputReader;

impl InputReader {
    pub fn read_input_day_6() -> String {
        let contents = read_to_string(r"input\day_6\day_6_input").unwrap();
        return contents;
    }
}

impl Solution {
    pub fn day_6_task_1() -> String {
        let contents = InputReader::read_input_day_6();
        println!("{:?}", contents);

        for i in 0..contents.len() {
            let one = contents.chars().nth(i).unwrap();
            let two = contents.chars().nth(i + 1).unwrap();
            let three = contents.chars().nth(i + 2).unwrap();
            let four = contents.chars().nth(i + 3).unwrap();

            let mut chars = vec![one, two, three, four];

            let test: Vec<usize> = chars.clone().iter().map(|char| {
                chars.iter().filter(|&x| x == char).count()
            }).collect();

            println!("{:?}", test);

            if test.iter().all(|&x| x == 1) {
                return (i+4).to_string()
            }
        }

        return "No solution found".to_string()
    }

    pub fn day_6_task_2() -> String {
        let contents = read_to_string(r"input\day_6\day_6_input").unwrap();
        println!("{:?}", contents);

        for i in 0..contents.len() {
            let mut chars = Vec::new();

            for j in i..i+14 {
                chars.push(contents.chars().nth(j).unwrap());
            }

            let test: Vec<usize> = chars.clone().iter().map(|char| {
                chars.iter().filter(|&x| x == char).count()
            }).collect();

            if test.iter().all(|&x| x == 1) {
                return (i+14).to_string();
            }
        }

        return "No solution found".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_6_task_1() {
        assert_eq!(Solution::day_6_task_1(), 7.to_string());
    }

    #[test]
    fn day_6_task_2() {
        assert_eq!(Solution::day_6_task_2(), 19.to_string());
    }
}