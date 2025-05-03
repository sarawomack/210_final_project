// Module: stats.rs
// Purpose: Aggregates season stats from game logs and assigns primary player roles.

use crate::data::PlayerGame;
use std::collections::HashMap;

/// Holds total season stats for a player after aggregation.
/// Includes total points, assists, rebounds, defensive stats, games played, and team.
#[derive(Debug, Default)]
pub struct PlayerStats {
    pub total_points: f32,
    pub total_assists: f32,
    pub total_rebounds: f32,
    pub total_steals: f32,
    pub total_blocks: f32,
    pub games_played: usize,
    pub team: String,
}

/// Aggregates player statistics from a list of games.
/// Each player's stats are summed up, and the number of games played is counted.
/// The function returns a HashMap where the key is the player's name and the value is their aggregated stats.
///
/// # Arguments
/// * `players` - A slice of PlayerGame structs representing individual game statistics for players.
///
/// # Returns
/// A HashMap where the key is the player's name and the value is their aggregated PlayerStats.
pub fn aggregate_stats(players: &[PlayerGame]) -> HashMap<String, PlayerStats> {
    let mut stats: HashMap<String, PlayerStats> = HashMap::new();

    for game in players {
        // Insert a new PlayerStats entry if the player has not been seen before
        let entry = stats.entry(game.player_name.clone()).or_insert_with(|| PlayerStats {
            team: game.team.clone(),
            ..Default::default()
        });
        // Incrementally sum up each stat using unwrap_or to handle missing values
        entry.total_points += game.points.unwrap_or(0.0);
        entry.total_assists += game.assists.unwrap_or(0.0);
        entry.total_rebounds += game.rebounds.unwrap_or(0.0);
        entry.total_steals += game.steals.unwrap_or(0.0);
        entry.total_blocks += game.blocks.unwrap_or(0.0);
        entry.games_played += 1;
    }
    stats
}

/// Determines a player's primary role (e.g., Scorer, Playmaker) based on their best leaderboard rank.
///
/// # Arguments
/// * `player` - The player's name.
/// * `points_lb` - Leaderboard for total points.
/// * `assists_lb` - Leaderboard for total assists.
/// * `rebounds_lb` - Leaderboard for total rebounds.
/// * `steals_blocks_lb` - Leaderboard for combined steals and blocks.
///
/// # Returns
/// A string label for the player's role.
pub fn assign_primary_role(
    player: &str,
    points_lb: &[(String, f32)],
    assists_lb: &[(String, f32)],
    rebounds_lb: &[(String, f32)],
    steals_blocks_lb: &[(String, f32)]
) -> String {
    let mut best_rank = usize::MAX;
    let mut primary_role = "All-Around";

    // Compare ranks across each leaderboard, update if current stat is the best
    for (i, (name, _)) in points_lb.iter().enumerate() {
        if name == player && i < best_rank {
            best_rank = i;
            primary_role = "Scorer";
        }
    }
    for (i, (name, _)) in assists_lb.iter().enumerate() {
        if name == player && i < best_rank {
            best_rank = i;
            primary_role = "Playmaker";
        }
    }
    for (i, (name, _)) in rebounds_lb.iter().enumerate() {
        if name == player && i < best_rank {
            best_rank = i;
            primary_role = "Rebounder";
        }
    }
    for (i, (name, _)) in steals_blocks_lb.iter().enumerate() {
        if name == player && i < best_rank {
            best_rank = i;
            primary_role = "Defender";
        }
    }

    primary_role.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::PlayerGame;

    #[test]
    fn test_aggregate_stats_single_game() {
        let player = PlayerGame {
            player_name: "Test".to_string(),
            team: "BOS".to_string(),
            points: Some(10.0),
            assists: Some(5.0),
            rebounds: Some(7.0),
            steals: Some(1.0),
            blocks: Some(1.0),
        };

        let map = aggregate_stats(&[player]);
        let stats = map.get("Test").unwrap();

        // Ensure all stats are correctly tallied from a single game
        assert_eq!(stats.total_points, 10.0);
        assert_eq!(stats.total_assists, 5.0);
        assert_eq!(stats.total_rebounds, 7.0);
        assert_eq!(stats.total_steals, 1.0);
        assert_eq!(stats.total_blocks, 1.0);
        assert_eq!(stats.games_played, 1);
    }

    #[test]
    fn test_assign_primary_role_returns_expected() {
        let leaderboards = vec![
            vec![("A".to_string(), 100.0), ("B".to_string(), 90.0)], // Points
            vec![("C".to_string(), 100.0), ("D".to_string(), 90.0)], // Assists
            vec![("E".to_string(), 100.0), ("F".to_string(), 90.0)], // Rebounds
            vec![("G".to_string(), 100.0), ("H".to_string(), 90.0)], // Defense
        ];

        // Each player is tested to ensure they are assigned the correct role based on best leaderboard rank
        let role = assign_primary_role("B", &leaderboards[0], &leaderboards[1], &leaderboards[2], &leaderboards[3]);
        assert_eq!(role, "Scorer");

        let role = assign_primary_role("C", &leaderboards[0], &leaderboards[1], &leaderboards[2], &leaderboards[3]);
        assert_eq!(role, "Playmaker");

        let role = assign_primary_role("E", &leaderboards[0], &leaderboards[1], &leaderboards[2], &leaderboards[3]);
        assert_eq!(role, "Rebounder");

        let role = assign_primary_role("G", &leaderboards[0], &leaderboards[1], &leaderboards[2], &leaderboards[3]);
        assert_eq!(role, "Defender");
    }
}
