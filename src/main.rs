// Module: main.rs
// Purpose: This is the entry point for the program. It handles user interaction via the terminal
// and calls helper functions to either show NBA stat leaderboards or display player summaries by team.

mod data;
mod stats;
mod leaderboard;

use data::{PlayerGame, load_players};
use stats::{PlayerStats, aggregate_stats, assign_primary_role};
use leaderboard::{build_leaderboards, find_rank};

use std::collections::HashMap;
use std::io::{self, Write};

/// Launches the program and handles the initial user prompt to select an analysis mode.
fn main() {
    // Load player games
    let players = load_players("database_24_25.csv").expect("Failed to load players.");
    let player_stats = aggregate_stats(&players);

    // Build rankings
    let (points_lb, assists_lb, rebounds_lb, steals_blocks_lb) = build_leaderboards(&player_stats);
    
    println!("\n(NBA player stats are current through February 7, 2025.)");
    println!("(If a player was traded during the season, their stats are combined across teams.)\n");

    println!("Choose an option:");
    println!("1. Select a player by team");
    println!("2. View NBA leaderboards for a stat");

    // Flush output to ensure prompt appears before input
    let mut choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    // Match user input to the appropriate flow
    if choice == "1" {
        team_player_flow(&players, &player_stats, &points_lb, &assists_lb, &rebounds_lb, &steals_blocks_lb);
    } else if choice == "2" {
        leaderboard_flow(&points_lb, &assists_lb, &rebounds_lb, &steals_blocks_lb);
    } else {
        println!("Invalid choice. Exiting.");
    }
}

/// Guides the user to select a team, then displays a selected player’s full stats and role.
///
/// # Arguments
/// * `players` - Raw per-game player data.
/// * `player_stats` - Aggregated totals per player.
/// * `*_lb` - Sorted leaderboards for various stats.
fn team_player_flow(
    players: &[PlayerGame],
    player_stats: &HashMap<String, PlayerStats>,
    points_lb: &[(String, f32)],
    assists_lb: &[(String, f32)],
    rebounds_lb: &[(String, f32)],
    steals_blocks_lb: &[(String, f32)]
) {
    // Collect all team abbreviations
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

    // Prompt user to input a valid team abbreviation
    println!("Enter a team abbreviation (e.g., BOS, LAL, NYK):");
    let mut team_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut team_input).unwrap();
    let team_input = team_input.trim().to_uppercase();

    // Get all players for the selected team
    let mut team_players: Vec<String> = player_stats.iter()
        .filter(|(_, stats)| stats.team == team_input)
        .map(|(name, _)| name.clone())
        .collect();

    if team_players.is_empty() {
        println!("No players found for team {}", team_input);
        return;
    }

    team_players.sort();

    // Ask user to choose a player by number from the list
    println!("\nPlayers for {}:", team_input);
    for (i, player) in team_players.iter().enumerate() {
        println!("{}. {}", i + 1, player);
    }

    // Ask user for player number
    println!("\nEnter the number next to the player you want to view:");
    let mut number_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number_input).unwrap();
    let number_input: usize = match number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    // Validate selection
    if number_input == 0 || number_input > team_players.len() {
        println!("Invalid player number.");
        return;
    }

    let selected_player = &team_players[number_input - 1];

    println!("\nSelected: {}", selected_player);

    // Display full stat breakdown and assigned role
    if let Some(stats) = player_stats.get(selected_player) {
        println!("- Total Points: {} ({})", stats.total_points, find_rank(selected_player, &points_lb));
        println!("- Total Assists: {} ({})", stats.total_assists, find_rank(selected_player, &assists_lb));
        println!("- Total Rebounds: {} ({})", stats.total_rebounds, find_rank(selected_player, &rebounds_lb));
        println!("- Total Steals + Blocks: {} ({})", stats.total_steals + stats.total_blocks, find_rank(selected_player, &steals_blocks_lb));

        let primary_role = assign_primary_role(selected_player, &points_lb, &assists_lb, &rebounds_lb, &steals_blocks_lb);
        println!("Primary Role: {}", primary_role);
    } else {
        println!("Player not found.");
    }
}

/// Handles the flow for displaying top players in a selected stat category.
///
/// # Arguments
/// * `points_lb` - Leaderboard for total points.
/// * `assists_lb` - Leaderboard for total assists.
/// * `rebounds_lb` - Leaderboard for total rebounds.
/// * `steals_blocks_lb` - Leaderboard for combined steals and blocks.
fn leaderboard_flow(
    points_lb: &[(String, f32)],
    assists_lb: &[(String, f32)],
    rebounds_lb: &[(String, f32)],
    steals_blocks_lb: &[(String, f32)]
) {
    println!("\nChoose a stat to view leaderboard:");
    println!("points, assists, rebounds, defense");

    let mut stat_choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut stat_choice).unwrap();
    let stat_choice = stat_choice.trim().to_lowercase(); // Allow for case-insensitive input

    // Determine which leaderboard to display
    let leaderboard = match stat_choice.as_str() {
        "points" => points_lb,
        "assists" => assists_lb,
        "rebounds" => rebounds_lb,
        "defense" => steals_blocks_lb,
        _ => {
            println!("Invalid stat choice.");
            return;
        }
    };

    if stat_choice == "defense" {
        println!("\n(Defense = blocks + steals)");
    }

    // Display the top 20 players for the selected stat
    println!("\nTop players for {}:", stat_choice);
    for (i, (name, total)) in leaderboard.iter().take(20).enumerate() {
        println!("{}. {} - {}", i + 1, name, total);
    }
}


