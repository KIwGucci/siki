mod func_calc;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use rustyline::{config::Configurer, DefaultEditor};
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_readline = DefaultEditor::new()?;
    my_readline.set_max_history_size(20)?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    println!("Input expression");
    loop {
        let readline = my_readline.readline(">> ");
        if let Ok(token) = readline {
            if token == "q" || token == "quit" {
                break;
            } else {
                match func_calc::calc(&token) {
                    Ok(data) => {
                        println!("=> {}", data);
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
