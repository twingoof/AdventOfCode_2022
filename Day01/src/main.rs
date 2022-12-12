use std::fs;
use std::io;
use std::io::BufRead;

/**
 * Open the input.txt file and read the content,
 * when an empty line is found the toal of calories are added to the vector
 * and value is set to zero to count the next elve stock
 */
fn get_elves_stocks() -> Vec<i32> {
    let file = fs::File::open("./assets/input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut total_calories: i32 = 0;
    let mut stocks: Vec<i32> = Vec::new();

    for line in reader.lines().enumerate() {
        let line = line.1.unwrap();

        if line == "" {
            stocks.push(total_calories);
            total_calories = 0;
            continue;
        }

        total_calories += line.parse::<i32>().unwrap();
    }

    return stocks;
}

/**
 * Find the biggest stock of calories and remove it from the vector and return
 * the value
 */
fn find_biggest_stock(stocks: &mut Vec<i32>) -> i32 {
    let mut max = -1;
    let mut max_index = 0;

    for (index, stock) in stocks.iter().enumerate() {
        if *stock > max {
            max = *stock;
            max_index = index;
        }
    }

    println!("Elve #{max_index} has biggest stock with {max} calories");
    stocks.remove(max_index);
    return max;
}

fn main() {
    let mut elves_stock = get_elves_stocks();
    let mut top_three_stocks = 0;
    
    for _i in 0 .. 3 {
        top_three_stocks += find_biggest_stock(&mut elves_stock);
    }

    println!("Top three stocks has {top_three_stocks} calories");
}
