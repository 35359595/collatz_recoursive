fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, input start number [2..max]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    println!("Parsing {}", &input.trim());
    let target: u64 = input.trim().parse()?;

    println!("Processing {}...", &target);
    println!("took {} steps to get to 1...", collatz(target) - 1); // ignorring first step as 0...

    Ok(())
}

fn collatz(inp: u64) -> u64 {
    let mut counter: u64 = 1;
    if inp == 1 { return counter; }
    match inp % 2 {
        0 => {
            counter += collatz(inp / 2);
        },
        _ => {
            counter += collatz(inp * 3 + 1)
        }
	}
    counter
}

#[cfg(test)]
mod collatz_tests {
    use super::*;
    #[test]
    fn couple_numbers() {
        let numbers: [u64; 5] = [2, 24, 23, 678910, 10987654321];
        for n in numbers.iter() {
            assert!(collatz(*n) > 0);
		}
	}
}
