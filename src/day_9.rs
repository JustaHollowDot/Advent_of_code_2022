use std::fs::read_to_string;
use crate::Solution;
use crate::InputReader;

impl InputReader {
    pub fn read_input_day_9() -> String {
        let contents = read_to_string(r"input\day_9\day_9_input").unwrap();
        return contents;
    }

    pub fn read_input_day_9_test() -> String {
        let contents = read_to_string(r"input\day_9\day_9_input_test").unwrap();
        return contents;
    }

    pub fn read_input_day_9_test_2() -> String {
        let contents = read_to_string(r"input\day_9\day_9_input_test_2").unwrap();
        return contents;
    }
}

impl Solution {
    pub fn day_9_task_1() -> i32 {
        let contents = InputReader::read_input_day_9();
        println!("{}", contents);

        let mut h: (i32, i32) = (0, 0);
        let mut t: (i32, i32) = (0, 0);

        let mut positions = Vec::new();

        for action in contents.lines() {
            let action = action.split_whitespace().collect::<Vec<&str>>();
            println!("{:?}", action);
            let direction = action[0];
            let distance = action[1].parse::<i32>().unwrap();

            for _ in 0..distance {
                if !positions.contains(&t) {
                    positions.push(t);
                }

                match direction {
                    "U" => h.1 += 1,
                    "D" => h.1 -= 1,
                    "R" => h.0 += 1,
                    "L" => h.0 -= 1,
                    _ => panic!("Invalid direction"),
                }

                if i32::abs(h.0 - t.0) > 1 && i32::abs(h.1 - t.1) == 0 {
                    t.0 = h.0 - (h.0 - t.0).signum();
                } else if i32::abs(h.1 - t.1) > 1 && i32::abs(h.0 - t.0) == 0 {
                    t.1 = h.1 - (h.1 - t.1).signum();
                } else if i32::abs(h.0 - t.0) > 1 && i32::abs(h.1 - t.1) == 1 {
                    t.0 = h.0 - (h.0 - t.0).signum();
                    t.1 = h.1;
                } else if i32::abs(h.1 - t.1) > 1 && i32::abs(h.0 - t.0) == 1 {
                    t.0 = h.0;
                    t.1 = h.1 - (h.1 - t.1).signum();
                }


                println!("Head: {:?}", h);
                println!("Tail: {:?}", t);
            }
        }

        if !positions.contains(&t) {
            positions.push(t);
        }

        println!("{:?}", positions);


        return positions.len() as i32;
    }

    pub fn day_9_task_2() -> i32 {
        let contents = InputReader::read_input_day_9();

        let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];

        let mut positions = Vec::new();

        for action in contents.lines() {
            let action = action.split_whitespace().collect::<Vec<&str>>();
            println!("{:?}", action);
            let direction = action[0];
            let distance = action[1].parse::<i32>().unwrap();

            for _ in 0..distance {
                if !positions.contains(&snake[snake.len() - 1]) {
                    positions.push(snake[snake.len() - 1]);
                }

                match direction {
                    "U" => snake[0].1 += 1,
                    "D" => snake[0].1 -= 1,
                    "R" => snake[0].0 += 1,
                    "L" => snake[0].0 -= 1,
                    _ => panic!("Invalid direction"),
                }

                for mut i in 1..snake.len() {
                    if (snake[i - 1].0 - snake[i].0).abs() > 1 && (snake[i - 1].1 - snake[i].1).abs() == 0 {
                        snake[i].0 = snake[i - 1].0 - (snake[i - 1].0 - snake[i].0).signum();
                    } else if (snake[i - 1].1 - snake[i].1).abs() > 1 && (snake[i - 1].0 - snake[i].0).abs() == 0 {
                        snake[i].1 = snake[i - 1].1 - (snake[i - 1].1 - snake[i].1).signum();
                    } else if (snake[i - 1].0 - snake[i].0).abs() > 1 && (snake[i - 1].1 - snake[i].1).abs() > 0 {
                        snake[i].0 = snake[i - 1].0 - (snake[i - 1].0 - snake[i].0).signum();
                        snake[i].1 += (snake[i - 1].1 - snake[i].1).signum();
                    } else if (snake[i - 1].1 - snake[i].1).abs() > 1 && (snake[i - 1].0 - snake[i].0).abs() > 0 {
                        snake[i].0 += (snake[i - 1].0 - snake[i].0).signum();
                        snake[i].1 = snake[i - 1].1 - (snake[i - 1].1 - snake[i].1).signum();
                    }


                }

                println!("{:?}", snake);
            }



        }

        if !positions.contains(&snake[snake.len() - 1]) {
            positions.push(snake[snake.len() - 1]);
        }

        println!("{:?}", positions);

        return positions.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_9_task_1() {
        assert_eq!(Solution::day_9_task_1(), 13);
    }

    #[test]
    fn day_9_task_2() {
        assert_eq!(Solution::day_9_task_2(), 1);
    }

    #[test]
    fn day_9_task_2_test_2() {
        assert_eq!(Solution::day_9_task_2(), 36);
    }
}