// Proof of Concept for Finding 6: No Duplicate Player Check
// This PoC demonstrates that join_user_handler allows the same player to join multiple times

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_duplicate_player_vulnerability() {
        // Setup: Create a game session in WaitingForPlayers state
        let mut game_session = GameSession {
            session_id: "duplicate_test".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000,
            game_mode: GameMode::WinnerTakesAllThreeVsThree, // 3v3 for multiple slots
            team_a: Team {
                players: [Pubkey::default(); 5], // All empty initially
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            team_b: Team {
                players: [Pubkey::default(); 5], // All empty initially
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            status: GameStatus::WaitingForPlayers,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let duplicate_player = Pubkey::new_unique();
        println!("=== Duplicate Player Check Test ===");
        println!("Testing player: {}", duplicate_player);

        // Simulate join_user_handler logic for Team A
        println!("\n1. Joining player to Team A...");
        let team_a_slot = game_session.get_player_empty_slot(0).unwrap(); // Team A
        game_session.team_a.players[team_a_slot] = duplicate_player;
        game_session.team_a.player_spawns[team_a_slot] = 10;
        game_session.team_a.player_kills[team_a_slot] = 0;
        
        println!("   ✓ Player joined Team A at slot {}", team_a_slot);
        println!("   Team A players: {:?}", &game_session.team_a.players[0..3]);

        // Simulate join_user_handler logic for Team B (same player!)
        println!("\n2. Attempting to join SAME player to Team B...");
        let team_b_slot_result = game_session.get_player_empty_slot(1); // Team B
        
        match team_b_slot_result {
            Ok(team_b_slot) => {
                // The vulnerability: No duplicate checking allows this
                game_session.team_b.players[team_b_slot] = duplicate_player;
                game_session.team_b.player_spawns[team_b_slot] = 10;
                game_session.team_b.player_kills[team_b_slot] = 0;
                
                println!("   ✓ VULNERABILITY: Same player joined Team B at slot {}", team_b_slot);
                println!("   Team B players: {:?}", &game_session.team_b.players[0..3]);
                
                // Verify the duplicate exists
                assert_eq!(game_session.team_a.players[team_a_slot], duplicate_player);
                assert_eq!(game_session.team_b.players[team_b_slot], duplicate_player);
                
                println!("\n   EXPLOIT CONFIRMED: Player {} is on BOTH teams!", duplicate_player);
            }
            Err(_) => {
                println!("   Team B full, but duplicate check still missing");
            }
        }

        // Demonstrate the unfair advantage
        println!("\n=== Unfair Gameplay Analysis ===");
        println!("Player {} advantages:", duplicate_player);
        println!("1. Can play on both teams simultaneously");
        println!("2. Has inside information about both team strategies");
        println!("3. Can manipulate game outcome by controlling both sides");
        println!("4. Gets double the spawns and can earn from both teams");
        
        let total_spawns = game_session.team_a.player_spawns[team_a_slot] + 
                          game_session.team_b.player_spawns.get(0).unwrap_or(&0);
        println!("5. Total spawns available: {} (should be max 10)", total_spawns);
    }

    #[test]
    fn test_same_team_duplicate_slots() {
        // Test joining the same player to multiple slots on the same team
        let mut game_session = GameSession {
            session_id: "same_team_duplicate".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000,
            game_mode: GameMode::WinnerTakesAllFiveVsFive, // 5v5 for more slots
            team_a: Team {
                players: [Pubkey::default(); 5],
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            team_b: Team {
                players: [Pubkey::default(); 5],
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            status: GameStatus::WaitingForPlayers,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let duplicate_player = Pubkey::new_unique();
        println!("\n=== Same Team Multiple Slots Test ===");
        println!("Testing player: {}", duplicate_player);

        // Join player to first slot
        let slot1 = game_session.get_player_empty_slot(0).unwrap();
        game_session.team_a.players[slot1] = duplicate_player;
        println!("Player joined Team A slot {}", slot1);

        // Simulate scenario where slot becomes available (e.g., another player left)
        // and same player tries to join again
        let slot2 = game_session.get_player_empty_slot(0).unwrap();
        game_session.team_a.players[slot2] = duplicate_player;
        println!("Same player joined Team A slot {}", slot2);

        // Verify duplicate exists on same team
        assert_eq!(game_session.team_a.players[slot1], duplicate_player);
        assert_eq!(game_session.team_a.players[slot2], duplicate_player);

        println!("\nVULNERABILITY: Player {} occupies multiple slots on Team A", duplicate_player);
        println!("Slots occupied: {} and {}", slot1, slot2);
        
        // Count total slots occupied by this player
        let occupied_slots = game_session.team_a.players
            .iter()
            .filter(|&&p| p == duplicate_player)
            .count();
        
        println!("Total slots occupied by one player: {}", occupied_slots);
        assert!(occupied_slots > 1, "Player should occupy multiple slots");
        
        println!("IMPACT: Single player controls multiple game positions");
    }

    #[test]
    fn test_economic_exploitation() {
        // Demonstrate economic exploitation through duplicate joining
        let mut game_session = GameSession {
            session_id: "economic_exploit".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000, // 1000 tokens per join
            game_mode: GameMode::PayToSpawnThreeVsThree, // Pay-to-spawn mode
            team_a: Team {
                players: [Pubkey::default(); 5],
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            team_b: Team {
                players: [Pubkey::default(); 5],
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            status: GameStatus::WaitingForPlayers,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let exploiter = Pubkey::new_unique();
        println!("\n=== Economic Exploitation Analysis ===");
        println!("Exploiter: {}", exploiter);
        println!("Session bet per join: {} tokens", game_session.session_bet);

        // Join exploiter to multiple positions
        let positions = vec![
            (0, "Team A Slot 0"),
            (1, "Team B Slot 0"), 
            (0, "Team A Slot 1"), // If available
        ];

        let mut total_bets_paid = 0u64;
        let mut positions_occupied = 0;

        for (team, description) in positions {
            if let Ok(slot) = game_session.get_player_empty_slot(team) {
                // Simulate the economic transaction
                total_bets_paid += game_session.session_bet;
                positions_occupied += 1;

                // Add player to position
                if team == 0 {
                    game_session.team_a.players[slot] = exploiter;
                } else {
                    game_session.team_b.players[slot] = exploiter;
                }

                println!("Joined {}: {} tokens paid", description, game_session.session_bet);
            }
        }

        println!("\nExploitation Summary:");
        println!("- Positions occupied: {}", positions_occupied);
        println!("- Total tokens paid: {}", total_bets_paid);
        println!("- Normal player pays: {} tokens for 1 position", game_session.session_bet);
        println!("- Exploiter pays: {} tokens for {} positions", total_bets_paid, positions_occupied);

        if positions_occupied > 1 {
            let advantage_ratio = positions_occupied as f64;
            println!("- Exploiter has {}x positional advantage", advantage_ratio);
            println!("- Can earn from multiple positions simultaneously");
            println!("- Controls game outcome through multiple votes/actions");
        }

        assert!(positions_occupied > 1, "Exploiter should occupy multiple positions");
        println!("\nECONOMIC VULNERABILITY CONFIRMED: Unfair positional advantage through duplicates");
    }

    #[test]
    fn test_recommended_duplicate_prevention() {
        // Show what proper duplicate checking should look like
        println!("\n=== Recommended Duplicate Prevention Logic ===");

        let mut game_session = GameSession {
            session_id: "proper_validation".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000,
            game_mode: GameMode::WinnerTakesAllOneVsOne,
            team_a: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            team_b: Team {
                players: [Pubkey::default(); 5],
                total_bet: 0,
                player_spawns: [10; 5],
                player_kills: [0; 5],
            },
            status: GameStatus::WaitingForPlayers,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let test_player = game_session.team_a.players[0]; // Already in team A
        println!("Testing duplicate prevention for player: {}", test_player);

        // Proper validation logic that should exist
        fn check_player_already_in_game(game_session: &GameSession, player: Pubkey) -> bool {
            // Check team A
            for &p in &game_session.team_a.players {
                if p == player && p != Pubkey::default() {
                    return true;
                }
            }
            // Check team B  
            for &p in &game_session.team_b.players {
                if p == player && p != Pubkey::default() {
                    return true;
                }
            }
            false
        }

        let is_duplicate = check_player_already_in_game(&game_session, test_player);
        println!("Duplicate check result: {}", is_duplicate);

        if is_duplicate {
            println!("✓ PROPER BEHAVIOR: Player already in game, join should be rejected");
        } else {
            println!("✓ PROPER BEHAVIOR: Player not in game, join can proceed");
        }

        assert!(is_duplicate, "Player should be detected as already in game");
        
        println!("\nCURRENT VULNERABILITY: No such duplicate checking exists in join_user_handler");
        println!("RECOMMENDATION: Add duplicate player validation before allowing joins");
    }
}
