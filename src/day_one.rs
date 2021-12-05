use std::fs;

fn read_file_to_string(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file").trim().to_string();
}

fn strings_to_ints(strings: Vec<&str>) -> Vec<i32> {
    return strings.iter().map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}

fn find_num_increasing(nums: Vec<i32>) -> i32 {
    return nums.iter().zip(nums.iter().skip(1)).map(|(a,b)| (b > a) as i32).sum();
}

pub fn find_depth_summed(filename: &str, sum_range: i32) {
    let contents = read_file_to_string(filename);
    let contents_as_list = contents.split("\n").collect::<Vec<&str>>();
    let depths = strings_to_ints(contents_as_list);
    let mut combined = depths.clone();
    for i in 1..sum_range {
        combined = combined.iter().zip(depths.iter().skip(i as usize)).map(|(a,b)| a + b).collect();
    }
    let sum: i32 = find_num_increasing(combined);
    println!("{}", sum);
}
