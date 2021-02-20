use std::error::Error;

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let (config, documents) = yarner_lib::parse_input(std::io::stdin())?;

    let out_json = yarner_lib::to_json(&config, &documents)?;
    println!("{}", out_json);
    Ok(())
}
