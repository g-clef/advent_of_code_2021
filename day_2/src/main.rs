use std::fs;

struct Position{
    traverse: i32,
    depth: i32,
    aim: i32
}

impl Position {
    fn answer(&self) -> i32 {
        return self.traverse * self.depth
    }

    fn parse_action_part_1(&mut self, action: &str, step: i32){
        match action {
           "forward" => self.traverse += step,
           "down" => self.depth += step,
           "up" => self.depth -= step,
           _ => println!("got an unknown command: {}", action)
        }
    }

    fn new() -> Position {
        Position{traverse: 0, depth:0, aim: 0}
    }

    fn parse_action_part_2(&mut self, action: &str, step: i32) {
        match action {
            "forward" => {
                self.traverse += step;
                self.depth += step*self.aim;
            }
            "down" => self.aim += step,
            "up" =>  self.aim -= step,
            _ => println!("got an unknown command: {}", action),
        }

    }
}


fn part_1(){
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_2/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let mut pos = Position::new();
    for line in filehandle.lines(){
        let (action, step) = line.split_once(" ").unwrap();
        let step: i32 = step.parse::<i32>().ok().unwrap();
        pos.parse_action_part_1(action, step);
    }
    println!("part 1 - {}", pos.answer())

}


fn part_2() {
    let filename = "/Users/ageeclough/IdeaProjects/advent_of_code_2021/day_2/input.txt";
    let filehandle = fs::read_to_string(filename).expect("file not found");
    let mut pos = Position::new();
    for line in filehandle.lines() {
        let (action, step) = line.split_once(" ").unwrap();
        let step: i32 = step.parse::<i32>().ok().unwrap();
        pos.parse_action_part_2(action, step)
    }
    println!("part 2 - {}", pos.answer())
}

fn main() {
    part_1();
    part_2();
}
