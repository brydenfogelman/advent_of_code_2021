use std::io::{BufRead, BufReader};
use std::fs;

pub fn find_power_consumption(filename: &str) {
    let reader = BufReader::new(fs::File::open(filename).expect("Something went wrong reading the file"));

    let mut numbers: [u16; 12] = [0; 12];
    let mut total_lines: u16 = 0;

    for line in reader.lines() {
        let line = line.unwrap(); // woah this is weird, https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
        for (i, char) in line.chars().enumerate() {
            let val = char.to_digit(10).unwrap() as u16;
            numbers[i] += val;
        }
        total_lines += 1;
    }

    let mut gamma_rate_binary = String::new();
    for i in 0..numbers.len() {
        gamma_rate_binary.push_str(if numbers[i] > total_lines / 2 { "1" } else { "0" });
    }

    let gamma_rate = u16::from_str_radix(&*gamma_rate_binary, 2).unwrap();
    let epsilon_rate = !gamma_rate & u16::from_str_radix("0000111111111111", 2).unwrap();
    println!("{}", gamma_rate as u32 * epsilon_rate as u32);
    println!("{}",gamma_rate);
    println!("{}",epsilon_rate);
    println!("{:12b}",gamma_rate);
    println!("{:12b}",epsilon_rate);
}

pub fn new_find_life_support_rating(filename: &str) {
    let reader = BufReader::new(fs::File::open(filename).expect("Something went wrong reading the file"));

    let mut arr: Vec<String> = vec![];
    for line in reader.lines() {
        arr.push(line.unwrap());
    }

    let mut search = String::new();
    for i in 0..12 {
        let possible = arr.iter().filter(|s| s[..i] == search).collect::<Vec<&String>>();
        if possible.len() == 1 {
            search = possible.first().unwrap().to_string();
            break;
        }
        let possible = possible.iter().map(|&s| s.chars().nth(i).unwrap()).collect::<Vec<char>>();
        let possible = possible.iter().map(|&s| s.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let ones: usize = possible.iter().filter(|&num| *num == 1).collect::<Vec<&u32>>().len();
        let zeros: usize = possible.iter().filter(|&num| *num == 0).collect::<Vec<&u32>>().len();
        if (ones >= zeros) {
            search.push_str("1");
        } else {
            search.push_str("0");
        }
    }
    let o2_rate = u32::from_str_radix(search.as_str(), 2).unwrap();

    let mut search = String::new();
    for i in 0..12 {
        let possible = arr.iter().filter(|s| s[..i] == search).collect::<Vec<&String>>();
        if possible.len() == 1 {
            search = possible.first().unwrap().to_string();
            break;
        }
        let possible = possible.iter().map(|&s| s.chars().nth(i).unwrap()).collect::<Vec<char>>();
        let possible = possible.iter().map(|&s| s.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let ones: usize = possible.iter().filter(|&num| *num == 1).collect::<Vec<&u32>>().len();
        let zeros: usize = possible.iter().filter(|&num| *num == 0).collect::<Vec<&u32>>().len();
        if (ones < zeros) {
            search.push_str("1");
        } else {
            search.push_str("0");
        }
    }
    let co2_rate = u32::from_str_radix(search.as_str(), 2).unwrap();

    println!("{}", o2_rate * co2_rate);
}
