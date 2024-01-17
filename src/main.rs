use clap::Parser;
use std::{fs::OpenOptions, io::Write};

/// Generates a function to check if a number is even or odd
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How many numbers to generate
    amount: u32,

    /// Where to write the function
    #[arg(short, long, default_value = "main.rs")]
    out: String,

    /// Where to start the function
    #[arg(short, long, default_value = "0")]
    start: u32,
}

fn main() {
    let args = Args::parse();
    let out_file = args.out;
    let start = args.start;
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&out_file)
        .expect("Unable to open file");

    f.write_all(b"fn is_even_or_odd(number: u32) -> bool {\n")
        .expect("Unable to write data");
    f.write_all(b"\tmatch number {\n")
        .expect("Unable to write data");

    for i in start..start+args.amount+1 {
        f.write_all(
            format!(
                "\t\t{} => {},\n",
                i,
                if i % 2 == 0 { "true" } else { "false" }
            )
            .as_bytes(),
        )
        .expect("Unable to write data");
    }

    f.write_all(b"\t}\n").expect("Unable to write data");
    f.write_all(b"}\n").expect("Unable to write data");

    println!(
        "Done! You can go checkout {} to see the generated function!",
        out_file
    );
}
