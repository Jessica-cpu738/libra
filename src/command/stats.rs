use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use std::collections::BTreeMap;

pub async fn run(json: bool, path: Option<PathBuf>) -> Result<()> {
    let target_dir = path.unwrap_or_else(|| PathBuf::from("."));
    let target_dir = target_dir.canonicalize()?;
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    
    walk_dir(&target_dir, &mut counts)?;
    
    let total: u64 = counts.values().sum();
    
    if json {
        println!("{{");
        println!("  \"stats\": {{");
        for (i, (ext, count)) in counts.iter().enumerate() {
            let comma = if i < counts.len() - 1 { "," } else { "" };
            println!("    \"{}\": {}{}", ext, count, comma);
        }
        println!("  }},");
        println!("  \"total\": {}," total);
        println!("  \"path\": \"{}\"", target_dir.display());
        println!("}}");
    } else {
        println!("\nFile Statistics");
        println!("Directory: {}\n", target_dir.display());
        println!("{:<20} {}", "Extension", "Count");
        println!("{}", "-".repeat(30));
        for (ext, count) in &counts {
            println!("{:<20} {}", ext, count);
        }
        println!("{}", "-".repeat(30));
        println!("Total files: {}", total);
    }
    Ok(())
}

fn walk_dir(dir: &Path, counts: &mut BTreeMap<String, u64>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        
        if path.is_dir() && (name == ".libra" || name == "target") {
            continue;
        }
        
        if path.is_dir() {
            walk_dir(&path, counts)?;
        } else if path.is_file() {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "no_extension".to_string());
            *counts.entry(ext).or_insert(0) += 1;
        }
    }
    Ok(())
}