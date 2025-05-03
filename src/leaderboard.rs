// Module: leaderboard.rs
// Purpose: Builds stat-based leaderboards and ranks players.

use crate::stats::PlayerStats;
use std::collections::HashMap;

/// Builds sorted leaderboards for each stat category from player totals.
///
/// # Arguments
/// * `stats` - A map of player names to their aggregated `PlayerStats`.
///
/// # Returns
/// A tuple containing four vectors of `(player name, stat total)` pairs,
/// sorted in descending order for points, assists, rebounds, and defense.
pub fn build_leaderboards(
    stats: &HashMap<String, PlayerStats>
) -> (
    Vec<(String, f32)>, 
    Vec<(String, f32)>, 
    Vec<(String, f32)>, 
    Vec<(String, f32)>
) {
    
    // Initialize empty vectors to store leaderboard entries
    let mut points = Vec::new();
    let mut assists = Vec::new();
    let mut rebounds = Vec::new();
    let mut steals_blocks = Vec::new();

    // Iterate through each player's aggregated stats
    for (name, s) in stats {
        // Push player totals into each corresponding leaderboard category
        points.push((name.clone(), s.total_points));
        assists.push((name.clone(), s.total_assists));
        rebounds.push((name.clone(), s.total_rebounds));
        steals_blocks.push((name.clone(), s.total_steals + s.total_blocks));
    }

    // Sort each leaderboard in descending order by stat total
    points.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    assists.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    rebounds.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    steals_blocks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    (points, assists, rebounds, steals_blocks)
}

/// Finds a player's rank in a given leaderboard.
///
/// # Arguments
/// * `player` - The name of the player to search for.
/// * `leaderboard` - A sorted vector of `(player name, value)` tuples.
///
/// # Returns
/// A string indicating the player's rank or "Not ranked." if not found.
pub fn find_rank(player: &str, leaderboard: &[(String, f32)]) -> String {
    // Iterate through the leaderboard and compare names
    for (i, (name, _)) in leaderboard.iter().enumerate() {
        if name == player {
            // Return 1-based rank if found
            return format!("Rank {} in NBA", i + 1);
        }
    }
    // Player not found in leaderboard
    "Not ranked.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::PlayerStats;
    use std::collections::HashMap;

    #[test]
    fn test_build_leaderboards_sorts_correctly() {
        let mut map = HashMap::new();

        // Manually insert mock player stats for testing
        map.insert("A".to_string(), PlayerStats { total_points: 30.0, total_assists: 10.0, total_rebounds: 5.0, total_steals: 1.0, total_blocks: 1.0, games_played: 1, team: "BOS".to_string() });
        map.insert("B".to_string(), PlayerStats { total_points: 40.0, total_assists: 5.0, total_rebounds: 7.0, total_steals: 2.0, total_blocks: 0.0, games_played: 1, team: "LAL".to_string() });

        let (points, assists, _, defense) = build_leaderboards(&map);
        
        // Check if sorting is performed correctly per stat
        assert_eq!(points[0].0, "B");
        assert_eq!(assists[0].0, "A");
        assert_eq!(defense[0].0, "A");
    }

    #[test]
    fn test_find_rank_returns_correct_index() {
        let leaderboard = vec![
            ("LeBron".to_string(), 2000.0),
            ("Curry".to_string(), 1900.0),
            ("Durant".to_string(), 1800.0),
        ];
        // Check known rank
        assert_eq!(find_rank("Curry", &leaderboard), "Rank 2 in NBA");
        
        // Check unknown player
        assert_eq!(find_rank("Giannis", &leaderboard), "Not ranked.");
    }
}
