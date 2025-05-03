# CDSDS 210 Final Project

Final project for **CDSDS 210, Section B1**  
**Boston University – Spring 2025**  
**Author:** Sara Womack


## Project Overview

This Rust project analyzes player statistics from the NBA 2024–2025 season. It aggregates per-game performance data into season totals and allows the user to interactively explore players by team or view league-wide stat leaderboards. It also determines each player's "primary role" based on their highest statistical ranking.


## Features

- Loads and parses a dataset from CSV
- Aggregates total points, assists, rebounds, steals, and blocks
- Displays top 20 players by category (points, assists, rebounds, defense)
- Allows the user to select a team and then a player to view full season stats
- Assigns a primary player role based on leaderboard rank (Scorer, Playmaker, etc.)
- Includes modular design, tests, and clean documentation


## How to Run

cargo build
cargo run


## User Interaction

Once the program runs, users are prompted with two options:

Choose an option:
1. Select a player by team
2. View NBA leaderboards for a stat


### Option 1: Select a Player by Team

- The program asks the user to enter a team abbreviation (e.g., `BOS`, `LAL`, `OKC`).
- Input is **case-insensitive**, so `bos`, `Bos`, or `BOS` all work.
- The program then displays a list of players on that team, each numbered.
- The user selects a player by typing the number next to their name.
- It then shows the player’s full stat breakdown, leaderboard ranks, and assigned primary role.


### Option 2: View Leaderboards by Stat

- The program asks the user to enter a stat category:
  - `points`
  - `assists`
  - `rebounds`
  - `defense` (combined steals and blocks)
- It displays the top 20 players in the selected category, sorted in descending order.
- If the input is invalid, the program prints a helpful error message and exits gracefully.

The program runs in the terminal and includes input validation to handle typos or unsupported options.

### Note on Commit Timing:
Due to my unfamiliarity with GitHub workflows, I developed the full project locally before pushing any commits. 
I recreated three logical checkpoints by staging and committing components (e.g., data loading, leaderboard logic, 
write-up) separately to reflect project progress as best as possible. All code was developed and tested incrementally
prior to these commits.
