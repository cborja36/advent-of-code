static INPUT: &str = include_str!("./input.txt");

fn parse_elf_calories(value: &str) -> u32 {
    let mut total = 0;
    for line in value.lines() {
        total += line.parse::<u32>().unwrap();
    }
    total
}

fn main() {
    let mut calories_per_elve = INPUT
        .trim()
        .split("\n\n")
        .map(|x| parse_elf_calories(&x))
        .collect::<Vec<u32>>();
    calories_per_elve.sort();

    println!("Max: {:?}", &calories_per_elve.iter().max().unwrap());

    let slice = &calories_per_elve[calories_per_elve.len() - 3..];
    println!("Sum top 3: {}", slice.iter().sum::<u32>());
}
