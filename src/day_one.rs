use std::fs;

pub fn find_depth() {
    let filename = "files/day_one_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file").trim().to_string();
    let depths = contents.split("\n").map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let sum: i32 = depths.iter().zip(depths.iter().skip(1)).map(|(a,b)| (b > a) as i32).sum();
    println!("{}", sum);
}
