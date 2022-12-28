use std::cmp::max;
use std::fs::read_to_string;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn read_input_day_8() -> String {
        let contents = read_to_string(r"input\day_8\day_8_input").unwrap();
        return contents;
    }
}

impl Solution {
    pub fn day_8_task_1() -> u32 {
        let contents = InputReader::read_input_day_8();

        let trees_vec = contents.lines().map(|line| {
            line.chars().map(|char| {
                char.to_digit(10).unwrap() as i32
            }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

        for line in &trees_vec {
            println!("{:?}", line);
        }

        let mut visible = vec![vec![0; trees_vec[0].len()];trees_vec.len()];

        for i in 0..trees_vec.len() {
            let mut left_highest = -1;
            let mut right_highest = -1;
            let mut up_highest = -1;
            let mut down_highest = -1;

            let len = trees_vec[i].len();

            for j in 0..trees_vec[i].len() {
                println!("Left:  {}", trees_vec[i][j]);
                println!("Right: {}", trees_vec[i][len - j - 1]);
                println!("Up:    {}", trees_vec[j][i]);
                println!("Down:  {}", trees_vec[len - j - 1][i]);
                println!("");



                if trees_vec[i][j] > left_highest {
                    visible[i][j] = 1;
                    left_highest = trees_vec[i][j];
                }

                if trees_vec[i][len - j - 1] > right_highest {
                    visible[i][len - j - 1] = 1;
                    right_highest = trees_vec[i][len - j - 1];
                }

                if trees_vec[j][i] > up_highest {
                    visible[j][i] = 1;
                    up_highest = trees_vec[j][i];
                }

                if trees_vec[len - j - 1][i] > down_highest {
                    visible[len - j - 1][i] = 1;
                    down_highest = trees_vec[len - j - 1][i];
                }


            }
        }


        for line in &visible {
            println!("{:?}", line);
        }

        let mut count = visible.iter().map(|line| {
            line.iter().sum::<u32>()
        }).sum::<u32>();



        return count;
    }

    pub fn day_8_task_2() -> u32 {
        let contents = InputReader::read_input_day_8();

        let trees_vec = contents.lines().map(|line| {
            line.chars().map(|char| {
                char.to_digit(10).unwrap()
            }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();

        let mut max_score = 0;

        for (i, line) in trees_vec.iter().enumerate() {
            for (j, current_tree_height) in line.iter().enumerate() {
                let mut left_score = 0;
                let mut right_score = 0;
                let mut up_score = 0;
                let mut down_score = 0;


                let len = trees_vec[i].len();

                if j == 0 || j == len - 1 {
                    continue
                } else if i == 0 || i == len - 1 {
                    continue
                }




                println!("{}, {}", i, j);

                for k in j+1..len {
                    println!("{}", k);

                    if trees_vec[i][k] < *current_tree_height {
                        right_score += 1;
                    } else {
                        right_score += 1;

                        break
                    }
                }

                for k in (0..j).rev() {
                    println!("{}", k);

                    if trees_vec[i][k] < *current_tree_height {
                        left_score += 1;
                    } else {
                        left_score += 1;

                        break
                    }
                }

                for k in i+1..len {
                    println!("{}", k);

                    if trees_vec[k][j] < *current_tree_height {
                        down_score += 1;
                    } else {
                        down_score += 1;

                        break
                    }
                }

                for k in (0..i).rev() {
                    println!("{}", k);

                    if trees_vec[k][j] < *current_tree_height {
                        up_score += 1;
                    } else {
                        up_score += 1;

                        break
                    }
                }

                println!("right_score: {}", right_score);






                println!("max_score: {}", left_score * right_score * up_score * down_score);
                max_score = max(max_score, left_score * right_score * up_score * down_score);
            }

        }


        return max_score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day_8_task_1() {
        assert_eq!(Solution::day_8_task_1(), 21);
    }

    #[test]
    fn test_day_8_task_2() {
        assert_eq!(Solution::day_8_task_2(), 8);
    }
}