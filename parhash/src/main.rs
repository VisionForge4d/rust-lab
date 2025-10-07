use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "parhash", version, about = "Parallel folder hasher (BLAKE3)")]
struct Opts {
    #[arg(default_value = ".")]
    dir: PathBuf,
    #[arg(long, default_value_t = 0)]
    min_kb: u64,
}

fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let files: Vec<_> = WalkDir::new(&opts.dir)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| {
            if opts.min_kb == 0 {
                return true;
            }
            p.metadata()
                .ok()
                .map(|m| m.len() >= (opts.min_kb * 1024u64))
                .unwrap_or(false)
        })
        .collect();

    files.par_iter().for_each(|path| match hash_file(path) {
        Ok(d) => println!("{d}  {}", path.display()),
        Err(e) => eprintln!("ERR  {}  ({e})", path.display()),
    });
    Ok(())
}
