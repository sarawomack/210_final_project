// Module: main.rs
// Purpose: Entry point for the program. Loads NBA player data and allows browsing by team.

mod data;

use data::{PlayerGame, load_players};

use std::io::{self, Write};

/// Launches the program and handles the initial user prompt to select a team and view its players.
fn main() {
    // Load player games from CSV
    let players = load_players("database_24_25.csv").expect("Failed to load players.");

    println!("\n(NBA player stats are current through February 7, 2025.)");
    println!("(If a player was traded during the season, their stats are combined across teams.)\n");

    // Prompt user to view players by team
    team_player_flow(&players);
}

/// Guides the user to select a team and view a list of players.
///
/// # Arguments
/// * `players` - Raw per-game player data loaded from CSV.
fn team_player_flow(players: &[PlayerGame]) {
    // Collect all team abbreviations from player data
    let mut teams: Vec<String> = players.iter()
        .map(|p| p.team.clone())
        .collect();
    teams.sort();
    teams.dedup(); // Remove duplicate team codes

    println!("\nAvailable teams:");
    for team in &teams {
        print!("{} ", team);
    }
    println!("\n");

    // Prompt for team abbreviation input
    println!("Enter a team abbreviation (e.g., BOS, LAL, NYK):");
    let mut team_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut team_input).unwrap();
    let team_input = team_input.trim().to_uppercase();

    // Get all player names for the selected team
    let mut team_players: Vec<String> = players.iter()
        .filter(|p| p.team == team_input)
        .map(|p| p.player_name.clone())
        .collect();

    if team_players.is_empty() {
        println!("No players found for team {}", team_input);
        return;
    }

    team_players.sort();
    println!("\nPlayers for {}:", team_input);
    for (i, player) in team_players.iter().enumerate() {
        println!("{}. {}", i + 1, player);
    }
}
