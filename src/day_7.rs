use std::fs::read_to_string;
use crate::{InputReader, Solution};

impl InputReader {
    pub fn read_input_day_7() -> String {
        let contents = read_to_string(r"input\day_7\day_7_input").unwrap();
        return contents;
    }
}

#[derive(Debug)]
struct Directory {
    path: Vec<String>,
    name: String,
    value: i128,
    sub_directories: Vec<Directory>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    value: i128,
}

impl Directory {
    pub fn new(path: Vec<String>, name: String) -> Directory {
        Directory {
            path,
            name,
            value: 0,
            sub_directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn print_directories(&self) {
        println!("{} :\n{:#?}", self.name, self.files);
        for sub_directory in &self.sub_directories {
            sub_directory.print_directories();
        }
    }

    pub fn get_value(&self, value_vec: &mut Vec<i128>) -> (Vec<i128>, i128) {
        let mut value = self.value;

        for sub_directory in &self.sub_directories {
            value += sub_directory.get_value(value_vec).1;
        }

        for file in &self.files {
            value += file.value;
        }

        if value <= 100_000 {
            value_vec.push(value);
        }

        return (value_vec.to_vec(), value);
    }

    pub fn get_all_values(&self, value_vec: &mut Vec<i128>) -> (Vec<i128>, i128) {
        let mut value = self.value;

        for sub_directory in &self.sub_directories {
            value += sub_directory.get_all_values(value_vec).1;
        }

        for file in &self.files {
            value += file.value;
        }

        value_vec.push(value);

        return (value_vec.to_vec(), value);
    }


    pub fn add_sub_directory(&mut self, sub_directory: Directory) {
        self.sub_directories.push(sub_directory);
    }

    pub fn add_directory_to_sub_directory(&mut self, path: Vec<String>, name: String, index: usize) {

        //println!("Path: {:?}", path);
        println!("Name: {:?}", name);
        //println!("Index: {:?}", index);


        if path.len() == 2 + index {
            println!("Adding directory to sub directory\n");
            self.sub_directories.push(Directory::new(path, name));
        } else {
            //println!("Going deeper");
            let directory_name = path[index+1].clone();

            for sub_directory in &mut self.sub_directories {
                if sub_directory.name == directory_name {
                    //println!("Found sub directory");
                    sub_directory.add_directory_to_sub_directory(path.clone(), name.clone(), index + 1);
                }
            }
        }
    }

    pub fn add_file_to_sub_directory(&mut self, file: (String, i128), path: Vec<String>, index: usize) {

        //println!("Path: {:?}", path);
        println!("File: {:?}", file);
        //println!("Index: {:?}", index);


        if path.len() == 1 + index {
            println!("Adding file to sub directory\n");
            self.files.push(File {
                name: file.0,
                value: file.1,
            });

        } else {
            //println!("Going deeper");
            let mut new_path = path.clone();
            let sub_directory_name = new_path[index+1].clone();

            for sub_directory in &mut self.sub_directories {
                if sub_directory.name == sub_directory_name {
                    //println!("Found sub directory");
                    sub_directory.add_file_to_sub_directory(file.clone(), new_path.clone(), index + 1);
                }
            }
        }
    }
}

impl Solution {
    pub fn day_7_task_1() -> i128 {
        let contents = InputReader::read_input_day_7();
        println!("{}", contents);

        let mut iter = contents.lines();

        let action = iter.next().unwrap();
        println!("{}", action);

        let mut main_directory = Directory::new(vec![], action.split_whitespace().nth(2).unwrap().to_string());
        println!("main_directory: {}", main_directory.name);

        let mut path = Vec::new();

        path.push(main_directory.name.clone());

        for line in contents.lines().skip(1) {

            match line {
                "$ cd .." => {
                    path.pop();
                },
                "$ ls" => {
                    continue;
                },
                _ => {
                    if line.contains("$ cd") {
                        path.push(line.split_whitespace().nth(2).unwrap().to_string());
                    } else if line.contains("dir ") {
                        let name = line.split_whitespace().nth(1).unwrap().to_string();
                        let mut new_path = path.clone();
                        new_path.push(name.clone());
                        main_directory.add_directory_to_sub_directory(new_path, name, 0);
                    } else {
                        let name = line.split_whitespace().nth(1).unwrap().to_string();
                        let value = line.split_whitespace().nth(0).unwrap().parse::<i128>().unwrap();

                        main_directory.add_file_to_sub_directory((name, value), path.clone(), 0);
                    }
                }
            }
        }

        let test = main_directory.get_value(&mut Vec::new());

        println!("test: {:?}", test);

        return test.0.iter().sum();
    }

    pub fn day_7_task_2() -> i128 {
        let contents = InputReader::read_input_day_7();
        println!("{}", contents);

        let mut iter = contents.lines();

        let action = iter.next().unwrap();
        println!("{}", action);

        let mut main_directory = Directory::new(vec![], action.split_whitespace().nth(2).unwrap().to_string());
        println!("main_directory: {}", main_directory.name);

        let mut path = Vec::new();

        path.push(main_directory.name.clone());

        for line in contents.lines().skip(1) {

            match line {
                "$ cd .." => {
                    path.pop();
                },
                "$ ls" => {
                    continue;
                },
                _ => {
                    if line.contains("$ cd") {
                        path.push(line.split_whitespace().nth(2).unwrap().to_string());
                    } else if line.contains("dir ") {
                        let name = line.split_whitespace().nth(1).unwrap().to_string();
                        let mut new_path = path.clone();
                        new_path.push(name.clone());
                        main_directory.add_directory_to_sub_directory(new_path, name, 0);
                    } else {
                        let name = line.split_whitespace().nth(1).unwrap().to_string();
                        let value = line.split_whitespace().nth(0).unwrap().parse::<i128>().unwrap();

                        main_directory.add_file_to_sub_directory((name, value), path.clone(), 0);
                    }
                }
            }
        }

        let test = main_directory.get_all_values(&mut Vec::new());



        println!("test: {:?}", test);
        println!("number: {}", 30_000_000 - (70000000 - test.1));

        let return_value = test.0.iter().find(|&x| *x  > (30_000_000 - (70_000_000 - test.1))).unwrap();

        println!("return_value: {}", return_value);

        return return_value.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, read_to_string};

    struct InputReader{
    }

    impl InputReader {
        pub fn read_input_day_7() -> String {
            let contents = read_to_string(r"input\day_7\day_7_input_test").unwrap();
            return contents;
        }
    }

    #[test]
    fn test_day_7_task_1() {
        let solution = Solution::day_7_task_1();
        assert_eq!(solution, 95437);
    }

    #[test]
    fn test_day_7_task_2() {
        let solution = Solution::day_7_task_2();
        assert_eq!(solution, 24933642);
    }
}