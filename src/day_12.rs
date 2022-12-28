use std::ffi::c_void;
use std::fs::read_to_string;
use std::thread::current;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn day_12_input() -> String {
        let contents = read_to_string(r"input\day_12\day_12_input").unwrap();
        return contents
    }

    pub fn day_12_input_test() -> String {
        let contents = read_to_string(r"input\day_12\day_12_input_test").unwrap();
        return contents
    }
}

struct HeightMap {
    map: Vec<Vec<i32>>,
    visited: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
    current_indeces: Vec<(usize, usize)>,
}

impl HeightMap {
    pub fn traverse_map(&mut self) -> Option<(usize, usize)> {
        let indeces_iter: Vec<(usize, usize)> = self.current_indeces.drain(..).collect();
        println!("indeces_iter: {:?}", indeces_iter);

        for index in indeces_iter {
            let (x, y) = index;
            let current_height = self.map[x][y];

            if x == self.end.0 && y == self.end.1 {
                println!("Found end!");
                return Some((x, y));
            }

            if x != 0 {
                let left_height = self.map[x - 1][y];

                if (left_height - current_height) <= 1 && !self.visited[x - 1][y] {
                    if (x - 1, y) == self.end {
                        return Some((x - 1, y));
                    }

                    self.current_indeces.push((x - 1, y));
                    self.visited[x - 1][y] = true;
                }
            }

            if x != self.map.len() - 1 {
                let right_height = self.map[x + 1][y];

                if (right_height - current_height) <= 1 && !self.visited[x + 1][y] {

                    if (x + 1, y) == self.end {
                        return Some((x + 1, y));
                    }

                    self.current_indeces.push((x + 1, y));
                    self.visited[x + 1][y] = true;
                }
            }

            if y != 0 {
                let top_height = self.map[x][y - 1];

                if (top_height - current_height) <= 1 && !self.visited[x][y - 1] {
                    if (x, y - 1) == self.end {
                        return Some((x, y - 1));
                    }

                    self.current_indeces.push((x, y - 1));
                    self.visited[x][y - 1] = true;
                }
            }

            if y != self.map[0].len() - 1 {
                let bottom_height = self.map[x][y + 1];

                if (bottom_height - current_height) <= 1 && !self.visited[x][y + 1] {
                    if (x, y + 1) == self.end {
                        return Some((x, y + 1));
                    }

                    self.current_indeces.push((x, y + 1));
                    self.visited[x][y + 1] = true;
                }
            }
        }

        return None;
    }

    pub fn traverse_map_task_2(&mut self) -> Option<(usize, usize)> {
        let indeces_iter: Vec<(usize, usize)> = self.current_indeces.drain(..).collect();
        println!("indeces_iter: {:?}", indeces_iter);

        for index in indeces_iter {
            let (x, y) = index;
            let current_height = self.map[x][y];

            if current_height == 1 {
                println!("Found end!");
                return Some((x, y));
            }

            if x != 0 {
                let left_height = self.map[x - 1][y];

                if (current_height - left_height) <= 1 && !self.visited[x - 1][y] {
                    if self.map[x - 1][y] == 1 {
                        return Some((x - 1, y));
                    }

                    self.current_indeces.push((x - 1, y));
                    self.visited[x - 1][y] = true;
                }
            }

            if x != self.map.len() - 1 {
                let right_height = self.map[x + 1][y];

                if (current_height - right_height) <= 1 && !self.visited[x + 1][y] {

                    if self.map[x + 1][y] == 1 {
                        return Some((x - 1, y));
                    }

                    self.current_indeces.push((x + 1, y));
                    self.visited[x + 1][y] = true;
                }
            }

            if y != 0 {
                let top_height = self.map[x][y - 1];

                if (current_height - top_height) <= 1 && !self.visited[x][y - 1] {
                    if self.map[x][y - 1] == 1 {
                        return Some((x - 1, y));
                    }

                    self.current_indeces.push((x, y - 1));
                    self.visited[x][y - 1] = true;
                }
            }

            if y != self.map[0].len() - 1 {
                let bottom_height = self.map[x][y + 1];

                if (current_height - bottom_height) <= 1 && !self.visited[x][y + 1] {
                    if self.map[x][y + 1] == 1 {
                        return Some((x - 1, y));
                    }
                    self.current_indeces.push((x, y + 1));
                    self.visited[x][y + 1] = true;
                }
            }
        }

        return None;
    }
}

impl Solution {
    pub fn day_12_task_1() -> u32 {
        let contents = InputReader::day_12_input();

        let mut start = Vec::new();
        let mut end = Vec::new();
        for (i, line) in contents.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character == 'S' {
                    start = vec![i, j];
                } else if character == 'E' {
                    end = vec![i, j];
                }
            }
        }

        println!("start: {:?}, end: {:?}", start, end);


        let mut height_map = contents.split("\r\n")
            .map(|line| {
            line.chars()
                .map(|x| {
                    if x == 'S' {
                        return 1;
                    } else if x == 'E' {
                        return 26;
                    } else {
                        let ascii_char = x as u32;
                        return (ascii_char - 96) as i32;
                    }
                }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

        for line in &height_map {
            println!("{:2?}", line);
        }

        let mut visited = vec![vec![false; height_map[0].len()]; height_map.len()];

        let mut height_map = HeightMap {
            map: height_map,
            visited: visited,
            start: (start[0], start[1]),
            end: (end[0], end[1]),
            current_indeces: vec![(start[0], start[1])],
        };

        let mut steps = 0;
        loop {
            steps += 1;
            let done = height_map.traverse_map();

            if done.is_some() {
                break;
            }


        }

        return steps;
    }

    pub fn day_12_task_2() -> i32 {
        let contents = InputReader::day_12_input();
        println!("{:?}", contents);

        let mut start = Vec::new();
        let mut end = Vec::new();
        for (i, line) in contents.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character == 'S' {
                    end = vec![i, j];
                } else if character == 'E' {
                    start = vec![i, j];
                }
            }
        }

        println!("start: {:?}, end: {:?}", start, end);


        let mut height_map = contents.split("\r\n")
            .map(|line| {
                line.chars()
                    .map(|x| {
                        if x == 'S' {
                            return 1;
                        } else if x == 'E' {
                            return 26;
                        } else {
                            let ascii_char = x as u32;
                            return (ascii_char - 96) as i32;
                        }
                    }).collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>();

        for line in &height_map {
            println!("{:2?}", line);
        }

        let mut visited = vec![vec![false; height_map[0].len()]; height_map.len()];

        let mut height_map = HeightMap {
            map: height_map,
            visited: visited,
            start: (start[0], start[1]),
            end: (end[0], end[1]),
            current_indeces: vec![(start[0], start[1])],
        };

        let mut steps = 0;
        loop {
            steps += 1;
            let done = height_map.traverse_map_task_2();

            if done.is_some() {
                break;
            }

            for (i, line) in height_map.visited.iter().enumerate() {
                for (j, visited) in line.iter().enumerate() {
                    if *visited {
                        print!("{:2}#, ", height_map.map[i][j], );
                    } else {
                        print!("{:2}., ", height_map.map[i][j]);
                    }
                }
                println!();
            }
        }

        return steps;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_task_1_test() {
        assert_eq!(Solution::day_12_task_1(), 31);
    }

    #[test]
    fn day_12_task_2_test() {
        assert_eq!(Solution::day_12_task_2(), 29);

    }
}