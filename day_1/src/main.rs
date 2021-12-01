use itertools::Itertools;
use std::fs;


fn part_1() {
    //let mut total: i32 = 0;
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let total = filehandle
                            .lines()
                            .filter_map(|s| s.parse::<usize>().ok())
                            .collect_vec()
                            .iter()
                            .tuple_windows()
                            .filter(|(a,b)| b > a)
                            .count();
    println!("part 1 - {}", total);
}

fn part_2() {
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let total = filehandle
                        .lines()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect_vec()
                        .iter()
                        .tuple_windows::<(_,_,_)>()
                        .tuple_windows::<(_,_)>()
                        .filter(|(a, b) | b.0 + b.1 + b.2 > a.0 + a.1 + a.2 )
                        .count();
    // print the total count
    println!("part 2 - {}", total);

}


fn main() {
    part_1();
    part_2();
}
