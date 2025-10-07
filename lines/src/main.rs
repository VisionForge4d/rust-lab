use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut total = 0usize;
    for e in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let p = e.path();
        if p.is_file() && p.extension().is_some_and(|e| e == "rs") {
            let n = fs::read_to_string(p)?.lines().count();
            total += n;
            println!("{:<6} {}", n, p.display());
        }
    }
    println!("TOTAL lines: {total}");
    Ok(())
}
