use std::fs;

pub fn find_depth() {
    let filename = "files/day_one_input.txt";
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // remove new line at the end of the file
    contents.truncate(contents.len() - 1);
    // converts the string to an
    let mut depths = contents.split("\n");
    let mut count = 0;
    // using `unwrap` like this is unsafe
    //  next returns either `None` or `Some` if the value of the iterator exists
    //  here I know it will exist but that's not always true
    // TODO find a better pattern here
    let mut prev: i32  = depths.next().unwrap().parse().unwrap();
    for text in depths {
        // this reads a string to an int
        let num: i32 = text.parse().unwrap();
        if num > prev {
            count += 1;
        }
        prev = num;
    }
    println!("{}", count);
}
