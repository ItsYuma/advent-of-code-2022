use std::fs::read_to_string;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;

    let input = read_to_string("/Users/stevystefko/Downloads/input-3.txt");
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

fn part_2() {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    let file = read_to_string("/Users/stevystefko/Downloads/input-3.txt").unwrap();
    let mut i = 0;
    let mut group = Vec::new();

    for line in file.lines(){
        i += 1;
        group.push(line);
        if i == 3 {
            for str in group[0].chars(){
                if group[1].contains(str) && group[2].contains(str){
                    let i = priority.find(str).unwrap() + 1;
                    sum += i;
                    break;
                }
            }
            i = 0;
            group.clear();
        }
    }
    println!("Sum: {}", sum);
}

