fn main() {
    part_1();
    
}

fn part_1() {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;

    let input = std::fs::read_to_string("input_day_3.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };

    for line in input.lines() {
        //recupere la taille de la line et créer 2 variable, 1 avec la première moitié et l'autre la deuxieme
        let (first, second) = line.split_at(line.len() / 2);

        for str in second.chars() {
            if first.contains(str){
                sum += priority.find(str).unwrap() + 1;
                break;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn part_2(){
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;

    let input = std::fs::read_to_string("input_day_3.txt");
    let input = match input {
        Ok(input) => input,
        Err(_) => panic!("Could not read input file"),
    };
}
