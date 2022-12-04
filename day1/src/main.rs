use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut calories = String::new();
    file.read_to_string(&mut calories)?;

    let mut total_calories = calories
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .into_iter()
                .sum()
        })
        .collect::<Vec<i32>>();

    total_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_calories: i32 = total_calories.iter().take(3).sum();

    println!("{:?}", top_calories);

    Ok(())
}
