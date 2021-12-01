
use std::fs::File;
use std::io::prelude::*;


struct Window {
    first_val: i32,
    second_val: i32,
    third_val: i32
}

impl Window {
    fn total(&self)  -> i32 {
        self.first_val + self.second_val + self.third_val
    }
}


fn part_1() {
    let mut total: i32 = 0;
    let mut last_val: i32 = -1;
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let mut filehandle = File::open(filename).expect("file not found");
    let mut contents = String::new();
    // read each line of text
    filehandle.read_to_string(&mut contents).expect("error reading file");
    let lines = contents.split("\n");
    // convert the line value to integer
    for line in lines {
        let int_val: i32 = line.parse().unwrap();
        // compare value to previous line (if it exists)
        if last_val == -1 {
            ()
        } else if int_val > last_val {
            //   if the value is bigger than the previous one, increase the total count
            total += 1;
        }
        last_val = int_val;
    }
    // print the total count
    println!("part 1 - {}", total);
}

fn part_2() {
    let mut total: i32 = 0;
    let mut second_val: i32 = -1;
    let mut third_val: i32 = -1;
    let mut last_window: Option<Window> = None;
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let mut filehandle = File::open(filename).expect("file not found");
    let mut contents = String::new();
    // read each line of text
    filehandle.read_to_string(&mut contents).expect("error reading file");
    let lines = contents.split("\n");
    // convert the line value to integer
    for line in lines {
        let int_val: i32 = line.parse().unwrap();
        // compare value to previous line (if it exists)
        if second_val == -1 || third_val == -1 {
            third_val = second_val;
            second_val = int_val;
            continue
        }
        let present_window = Window{first_val:int_val, second_val, third_val};
        if last_window.is_some() {
            let last = last_window.unwrap();
            if last.total() < present_window.total() {
                total += 1;
            }
        }
        third_val = second_val;
        second_val = int_val;
        last_window = Option::Some(present_window);

    }
    // print the total count
    println!("part 2 - {}", total);

}


fn main() {
    part_1();
    part_2();
}
