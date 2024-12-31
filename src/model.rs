// model.rs
use crate::utilities::current_unix_time;
use std::fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use color_eyre::Result;

#[derive(Deserialize, Serialize)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
    pub subdecks: Vec<Deck>,
}

#[derive(Deserialize, Serialize)]
pub struct Card {
    pub front: String,
    pub back: String,
    pub notes: Option<String>,
    pub examples: Option<Vec<ExampleSentence>>,
    pub explanation: Option<String>,
    #[serde(default)]
    pub history: Option<Vec<ReviewHistory>>,
    #[serde(default)]
    pub next_review: Option<u64>,
    #[serde(default)]
    pub ease_factor: Option<u64>,
    #[serde(skip)]  // Don't deserialize from YAML
    pub file_path: Option<PathBuf>,
}

#[derive(Deserialize, Serialize)]
pub struct ExampleSentence {
    pub sentence: String,
    pub translation: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReviewHistory {
     date: u64, // UNIX timestamp
     remembered: bool,
 }

 impl Card {
    pub fn calculate_next_review(&mut self, current_time: u64, remembered: bool) -> Result<u64> {
        let random_factor = (0.8 + (rand::random::<f64>() * 0.4)) as u64;
        
        if remembered {
            self.ease_factor = Some((self.ease_factor.unwrap_or(60) as f64 * 1.5) as u64);
            self.next_review = Some(current_time + self.ease_factor.unwrap_or(0) + random_factor);
        } else {
            self.ease_factor = Some(60);
            self.next_review = Some(current_time + random_factor);
        }
        
        // Add review to history
        if self.history.is_none() {
            self.history = Some(Vec::new());
        }
        if let Some(history) = &mut self.history {
            history.push(ReviewHistory {
                date: current_time,
                remembered,
            });
        }

        // Write changes back to file
        if let Some(path) = &self.file_path {
            let yaml = serde_yaml::to_string(&self)?;
            fs::write(path, yaml)?;
        }
        
        Ok(self.next_review.expect("Could not set next review."))
    }

    pub fn initialize_review_data(&mut self) {
        if self.next_review.is_none() {
            self.next_review = Some(current_unix_time() as u64);
        }
        if self.ease_factor.is_none() {
            self.ease_factor = Some(60);
        }
        if self.history.is_none() {
            self.history = Some(Vec::new());
        }
    }
}