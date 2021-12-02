use itertools::Itertools;
use std::fs;


fn part_1() {
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let total = filehandle
                            //read each line
                            .lines()
                            //parse each line into a usize, skipping the ones that don't parse
                            .filter_map(|s| s.parse::<usize>().ok())
                            // collect the results into a vector
                            .collect_vec()
                            // make an iterator over that vector
                            .iter()
                            // make a 2-entry sliding window over that iterator
                            .tuple_windows()
                            // compare the entries in the window, keeping only the ones where the second value is bigger than the first
                            .filter(|(a,b)| b > a)
                            // count the # that passed the filter.
                            .count();
    println!("part 1 - {}", total);
}

fn part_2() {
    // open text file for reading
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_1/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let total = filehandle
                        //read each line
                        .lines()
                        //parse each line into a usize, skipping the ones that don't parse
                        .filter_map(|s| s.parse::<usize>().ok())
                        // collect the results into a vector
                       .collect_vec()
                        // make an iterator over that vector
                        .iter()
                        // make a 3-entry sliding window over that iterator
                        .tuple_windows::<(_,_,_)>()
                        // make a 2-entry sliding window over the set of 3-entries above
                        .tuple_windows::<(_,_)>()
                        // compare the sums of the 3-entry tuples, only keep the ones where the second's sum is bigger than the first
                        .filter(|(a, b) | b.0 + b.1 + b.2 > a.0 + a.1 + a.2 )
                        // count the # that passed the filter.
                        .count();
    // print the total count
    println!("part 2 - {}", total);

}


fn main() {
    part_1();
    part_2();
}
