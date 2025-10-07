use anyhow::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut items: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .map(|e| {
            let p = e.into_path();
            let s = p.metadata().ok().map(|m| m.len()).unwrap_or(0);
            (p, s)
        })
        .collect();

    items.sort_by_key(|(_, s)| std::cmp::Reverse(*s));
    for (p, s) in items.iter().take(50) {
        println!("{:>10}  {}", human(*s), p.display());
    }
    Ok(())
}

fn human(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut i = 0;
    let mut x = n as f64;
    while x >= 1024.0 && i < units.len() - 1 {
        x /= 1024.0;
        i += 1;
    }
    format!("{:.1}{}", x, units[i])
}
