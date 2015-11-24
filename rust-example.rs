fn main() {
    let program = "+ + * - /";
    
    let res = program.chars().fold(0, | x, x1 | 
        match x1 {
            '+' => x + 1,
            '-' => x - 1,
            '*' => x * 2,
            '/' => x / 2,
            _ => x
        }
    );

    println!("The program \"{}\" calculates the value {}",
              program, res);
}
