use std::error::Error;

mod parsers;
mod binarysearch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filename = "numbers.csv";
    let numbers = parsers::csvparser::read_numbers_from_csv(filename).await;

    // Perform binary search
    let look_for = 10_000_000;
    // Specify how many threads you want to use
    let num_threads = 8;

    match numbers {
        Ok(data) => {
            
            let result = binarysearch::parallel_binary_search(data, look_for, num_threads);
        
            match result {
                Some(index) => println!("Found {} at index {}", look_for, index),
                None => println!("{} not found", look_for),
            }
        },
        Err(err) => {
            println!("[ERROR] : {err}");
        },
    }

    Ok(())
}
