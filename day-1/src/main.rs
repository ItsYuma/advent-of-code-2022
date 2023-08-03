fn main() {
    let sum = sum();
    println!("Sum: {}", sum);
    println!("Best of three {}", sum_of_three());
}

//create a function that dont take input and  return a int
fn sum() -> i32 {
    let input = std::fs::read_to_string("src/day_1_input.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };

    /*let elf_most_calories = input.trim().split("\n\n").map(|elf| {
        elf.lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
    });*/
    let elf_most_calories = input.trim().split("\n\n").map(|elf| {
        elf.lines()
            .map(|line| line.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .unwrap_or(Vec::new()) // handle the error
            .iter()
            .sum::<i32>()
    });

    //handle the option enum
    match elf_most_calories.max() {
        Some(elf_most_calories) => return elf_most_calories,
        None => panic!("Could not find max"),
    };
}

//generate a fun that return a vec of i32
fn sum_of_three() -> i32 {
    let input = std::fs::read_to_string("src/day_1_input.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };

    let mut elf_calories: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect();

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));
    elf_calories.iter().take(3).sum::<i32>()
}
