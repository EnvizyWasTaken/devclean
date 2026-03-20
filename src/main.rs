mod scanner;
mod rules;
mod utils;

use clap::Parser;
use scanner::scan;
use utils::format_size;
use std::io::{self, Write};

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(long)]
    clean: bool,
}


fn confirm() -> bool {
    println!("Do you wish to delete these folders (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim(), "Y" | "y")
}

fn main() {
    let args = Args::parse();

    println!("Scanning {}\n", args.path);


    let items: Vec<_> = scan(&args.path)
        .into_iter()
        .filter(|i| i.size > 5_000_000) // ignore tiny junk
        .collect();

    let total: u64 = items.iter().map(|i| i.size).sum();

    for item in &items {
        let label = if item.is_system { "[SYSTEM]" } else { "[USER]" };

        println!(
            "{} {} ({})",
            label,
            item.path.display(),
            format_size(item.size)
        );
    }

    // ARGUMENTS:

    if args.clean {
        if confirm() {
            for item in &items {
                match std::fs::remove_dir_all(&item.path) {
                    Ok(_) => println!("Deleted {}", item.path.display()),
                    Err(e) => eprintln!("Failed to delete {}: {}", item.path.display(), e),
                }
            }
        } else {
            println!("Aborted.");
        }
    }

    println!("\nTotal reclaimable: {}", format_size(total));
}