use indicatif::{ProgressBar, ProgressStyle};
use std::io;

fn main() {
    println!("任意の自然数を入力してください。");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().unwrap();

    let divisor = find_divisor(number);

    println!("{:?}", divisor);
}

fn find_divisor(number: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 1;

    let amount:u64 = (number as f64).sqrt() as u64;
    let bar = ProgressBar::new(amount);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Failed to set template")
            .progress_chars("#>-")
    );

    while i * i <= number {
        if number % i == 0 {
            result.push(i);
            if i * i != number {
                result.push(number / i)
            };
        }
        i += 1;
        bar.set_position(i as u64);
    }

    bar.finish_with_message("Done!");

    result.sort();
    return result;
}
