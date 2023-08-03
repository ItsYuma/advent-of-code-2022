fn main() {
    
}

fn part_1() -> i32 {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.insert("x", 1);
    map.insert("y", 2);
    map.insert("z", 3);

    let input = std::fs::read_to_string("src/day_1_input.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };

    input.lines()
    .map()

}

fn part_2() -> i32 {
    let input = std::fs::read_to_string("src/day_1_input.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };
}