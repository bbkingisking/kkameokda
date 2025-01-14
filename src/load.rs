// load.rs
use std::fs;
use std::path::{Path, PathBuf};
use color_eyre::Result;
use crate::model::{Card, Deck};
use clap::Parser;
use crate::args::Cli;
pub fn load_decks() -> Result<Vec<Deck>> {
    let flashcards_dir = get_flashcards_dir();
    
    if !flashcards_dir.exists() {
        return Err(color_eyre::eyre::eyre!("Flashcards directory does not exist at {:?}", &flashcards_dir));
    }

    let mut decks = Vec::new();
    
    for entry in fs::read_dir(&flashcards_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            match load_deck_from_directory(&path) {
                Ok(deck) => decks.push(deck),
                Err(e) => eprintln!("Error loading deck from {:?}: {}", path, e),
            }
        }
    }

    if decks.is_empty() {
        return Err(color_eyre::eyre::eyre!("No decks found in {:?}", flashcards_dir));
    }

    Ok(decks)
}

fn load_deck_from_directory(path: &Path) -> Result<Deck> {
    if !path.is_dir() {
        return Err(color_eyre::eyre::eyre!("Path is not a directory"));
    }

    // Get the path relative to the flashcards directory
    let flashcards_dir = get_flashcards_dir();
    let relative_path = path.strip_prefix(&flashcards_dir)
        .unwrap_or(path.as_ref());
    
    // Use the full path as the deck name
    let deck_name = relative_path.to_string_lossy().replace('\\', "/");
    
    let mut cards = Vec::new();
    let mut subdecks = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
            let contents = fs::read_to_string(&entry_path)?;
            let mut card: Card = serde_yaml::from_str(&contents)?;
            card.file_path = Some(entry_path.clone());
            // Initialize review data for cards that don't have it
            card.initialize_review_data();
            cards.push(card);
        } else if entry_path.is_dir() {
            match load_deck_from_directory(&entry_path) {
                Ok(subdeck) => subdecks.push(subdeck),
                Err(e) => eprintln!("Error loading subdeck from {:?}: {}", entry_path, e),
            }
        }
    }
    
    Ok(Deck {
        name: deck_name,
        cards,
        subdecks, 
    })
}

pub fn get_flashcards_dir() -> PathBuf {
    let cli = Cli::parse();
    if cli.directory.is_some() {
        PathBuf::from(cli.directory.unwrap())
    } else {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join("flashcards")
    }
}