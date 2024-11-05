mod func_calc;
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input expression");
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let expression = buffer.trim();
        if expression == "q" {
            break;
        } else {
            print!("=> ");
            match func_calc::calc(expression) {
                Ok(data) => {
                    println!("{}", data);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    Ok(())
}
