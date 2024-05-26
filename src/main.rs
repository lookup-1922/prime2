use indicatif::{ProgressBar, ProgressStyle};
use std::io;

fn main() {
    println!("任意の自然数を入力してください。");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().unwrap();

    //u64の最大値は18446744073709551615(20桁)

    let divisor = find_divisor(number);

    println!("{:?}", divisor);

    if mersenne_primes_chcker(number) {
        println!("メルセンヌ素数です。")
    }

    if perfect_number_chcker(number, divisor) {
        println!("完全数です。")
    }
}

fn find_divisor(number: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 1;

    let amount: u64 = (number as f64).sqrt() as u64;
    let bar = ProgressBar::new(amount);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Failed to set template")
            .progress_chars("#>-")
    );
    bar.set_message("checking divisor");

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

fn mersenne_primes_chcker(number: u64) -> bool {
    if (number + 1 & number) == 0 {
        let mut i = 1;
        while i * i <= number {
            if number % i == 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }
    return false;
}

fn perfect_number_chcker(number: u64, divisor: Vec<u64>) -> bool {
    let sum_divisors = divisor.iter().sum();
    if number * 2 == sum_divisors {
        return true;
    } else {
        return false;
    }
}
