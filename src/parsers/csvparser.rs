use std::fs::File;
use std::io::{self, BufRead};
use crate::Error;

pub async fn read_numbers_from_csv(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename);
    match file {
        Ok(data) => {
            let reader:io::BufReader<File> = io::BufReader::new(data);
            let mut numbers = Vec::new();
        
            for line in reader.lines() {
                let line = line?;
                for num_str in line.trim().split(',') {
                    if let Ok(num) = num_str.parse::<i32>() {
                        numbers.push(num);
                    }
                }
            }
        
            Ok(numbers)
        },
        Err(_) => {
            println!("[ERROR] Unable to find the file.");
            Err("Unable to find or read file".into())
        },
    }

}