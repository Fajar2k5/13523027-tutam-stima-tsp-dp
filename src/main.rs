mod app;
mod tsp_solver;
mod ui;

use std::{env, fs, io::{self, Write}};

fn main() {
    // command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = &args[1];
        if let Err(e) = run_batch(path) {
            eprintln!("Error: {}", e);
        }
    } else {
        print!("Enter number of vertices (n): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line from stdin");

        let n = match line.trim().parse::<usize>() {
            Ok(val) if val > 0 => val,
            _ => {
                eprintln!("Invalid input: please enter a positive integer.");
                return;
            }
        };

        if let Err(e) = ui::run_app_with_n(n) {
            eprintln!("Error running TUI: {:?}", e);
        }
    }
}

fn run_batch(path: &str) -> Result<(), String> {
    // (Same as before)
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to open or read '{}': {}", path, e))?;

    let mut iter = contents.split_whitespace();
    let n_str = iter
        .next()
        .ok_or_else(|| format!("File '{}' is empty", path))?;
    let n: usize = n_str
        .parse()
        .map_err(|e| format!("Failed to parse n = '{}': {}", n_str, e))?;

    let mut dist: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let token = iter
                .next()
                .ok_or_else(|| format!("Missing dist[{}][{}] in '{}'", i, j, path))?;
            dist[i][j] = token
                .parse()
                .map_err(|e| format!("Failed to parse dist[{}][{}] = '{}': {}", i, j, token, e))?;
        }
    }

    let (best_cost, tour) = tsp_solver::solve_tsp(dist);
    println!("Cost: {}", best_cost);
    print!("Tour:");
    for v in &tour {
        print!(" {}", v);
    }
    println!();
    Ok(())
}
