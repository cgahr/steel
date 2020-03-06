extern crate steel;
#[macro_use]
extern crate steel_derive;

use steel::SteelInterpreter;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use steel::stdlib::PRELUDE;
use steel::unwrap;

use std::any::Any;
use steel::rerrs;
use steel::rvals::{self, CustomType, SteelVal, StructFunctions};

use steel_derive::steel;

use std::process;

fn main() {
    finish(repl());
}

fn finish(result: Result<(), std::io::Error>) -> ! {
    let code = match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!(
                "{}: {}",
                std::env::args().next().unwrap_or_else(|| "steel".into()),
                e
            );
            1
        }
    };

    process::exit(code);
}

#[steel]
#[derive(PartialEq)]
pub struct MyStruct {
    pub field: usize,
    pub stays_the_same: usize,
    pub name: String,
}

pub fn repl() -> std::io::Result<()> {
    let mut interpreter = SteelInterpreter::new();

    if let Err(e) = interpreter.require(PRELUDE) {
        eprintln!("Error loading prelude: {}", e)
    }
    println!("Welcome to Steel 1.0");

    println!("Attempting to insert my own type");

    let testytest = MyStruct {
        field: 69,
        stays_the_same: 0,
        name: "matthew paras".to_string(),
    };
    // let testytest: usize = 420;
    let my_val = testytest.new_steel_val();

    interpreter.insert_binding("test", my_val.clone());

    interpreter.insert_bindings(MyStruct::generate_bindings());

    // interpreter

    println!("{:?}", unwrap!(my_val, MyStruct).unwrap());

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("λ > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.as_str() {
                    ":quit" => return Ok(()),
                    ":reset" => interpreter.reset(),
                    _ => match interpreter.evaluate(&line) {
                        Ok(r) => r.iter().for_each(|x| println!("{}", x)),
                        Err(e) => eprintln!("{}", e),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                println!(
                    "Looking up value and printing: {:?}",
                    unwrap!(interpreter.extract_value("new-test").unwrap(), MyStruct).unwrap(),
                );
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
