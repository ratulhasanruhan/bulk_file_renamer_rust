use std::fs;
use std::path::Path;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use colored::*;

fn main() {
    println!("{}", "🔧 Bulk File Renamer".bold().bright_green());

    let theme = ColorfulTheme::default();

    // Interactive Inputs
    let folder: String = Input::with_theme(&theme)
        .with_prompt("📁 Enter folder path")
        .interact_text()
        .unwrap();

    let old_pattern: String = Input::with_theme(&theme)
        .with_prompt("🔍 Pattern to replace")
        .interact_text()
        .unwrap();

    let new_pattern: String = Input::with_theme(&theme)
        .with_prompt("✏️  Replace with")
        .interact_text()
        .unwrap();

    let path = Path::new(&folder);
    if !path.exists() || !path.is_dir() {
        eprintln!("{}", "❌ Invalid folder path.".bold().red());
        return;
    }

    let mut rename_list = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in fs::read_dir(path).unwrap().flatten() {
    let file_path = entry.path();

    if let Some(file_name) = file_path.file_name().and_then(|f| f.to_str()) {
        if file_name.contains(&old_pattern) {
            let file_path_cloned = file_path.clone();  // clone before moving

            let new_file_name = file_name.replace(&old_pattern, &new_pattern);
            let new_path = path.join(&new_file_name);

            rename_list.push((file_path_cloned, new_path, file_name.to_string(), new_file_name));
        }
    }
}
        }
        Err(e) => {
            eprintln!("{} {}", "⚠️ Failed to read folder:".red(), e);
            return;
        }
    }

    if rename_list.is_empty() {
        println!("{}", "📂 No files matched the pattern.".yellow());
        return;
    }

    println!("\n🔎 Preview of changes:\n");

    for (_, _, old, new) in &rename_list {
        println!("{} → {}", old.green(), new.yellow());
    }

    let confirm = Confirm::with_theme(&theme)
        .with_prompt("⚠️ Do you want to rename these files?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        for (old_path, new_path, old, new) in rename_list {
            match fs::rename(&old_path, &new_path) {
                Ok(_) => println!("✅ {} → {}", old.green(), new.yellow()),
                Err(e) => eprintln!("{} {}: {}", "⚠️ Failed to rename".red(), old, e),
            }
        }

        println!("\n{}", "🎉 Rename complete!".bright_green().bold());
    } else {
        println!("{}", "❌ Operation cancelled.".dimmed());
    }
}
