// Module: data.rs
// Purpose: Defines the PlayerGame struct and loads player stats from a CSV file.

use serde::Deserialize;
use std::error::Error;

/// Represents a single game’s statistics for a player.
/// Parsed from a CSV file using serde. Fields are optional to handle missing data.
#[derive(Debug, Deserialize)]
pub struct PlayerGame {
    // Each field maps to a CSV column, renamed for serde compatibility.
    #[serde(rename = "Player")]
    pub player_name: String,
    #[serde(rename = "Tm")]
    pub team: String,
    #[serde(rename = "PTS")]
    pub points: Option<f32>,
    #[serde(rename = "AST")]
    pub assists: Option<f32>,
    #[serde(rename = "TRB")]
    pub rebounds: Option<f32>,
    #[serde(rename = "STL")]
    pub steals: Option<f32>,
    #[serde(rename = "BLK")]
    pub blocks: Option<f32>,
}

/// Loads player game data from a CSV file.
///
/// # Arguments
/// * `path` - The file path to the CSV file containing player stats.
///
/// # Returns
/// A `Result` containing a vector of `PlayerGame` structs if successful,
/// or an error if the file could not be read or parsed.
pub fn load_players(path: &str) -> Result<Vec<PlayerGame>, Box<dyn Error>> {
    // Build a CSV reader from the given path, using flexible parsing to allow row length variation.
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)?; // Returns early with an error if the file can't be opened

    let mut players = Vec::new();

    // Loop through each record in the CSV file
    for result in rdr.deserialize() {
        // Deserialize the current row into a PlayerGame struct
        // If there's a deserialization error, return early with that error
        let record: PlayerGame = result?;

        players.push(record); // Add each successfully parsed record to the players vector
    }
    Ok(players)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_players_valid_csv() {
        let path = "test_data.csv";
        let mut file = File::create(path).unwrap();

        // Write mock data into the test CSV file
        writeln!(file, "Player,Tm,PTS,AST,TRB,STL,BLK").unwrap();
        writeln!(file, "Test Player,BOS,20,5,8,2,1").unwrap();

        let result = load_players(path).unwrap();
        assert_eq!(result.len(), 1); // Ensure exactly one record is returned
        assert_eq!(result[0].player_name, "Test Player");
    }

    #[test]
    fn test_load_players_invalid_path() {
        let result = load_players("nonexistent.csv");
        assert!(result.is_err()); // Should return an error for missing file
    }
}
