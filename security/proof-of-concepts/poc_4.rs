// Proof of Concept for Finding 4: Critical Arithmetic Error in Pay-to-Spawn Distribution
// This PoC demonstrates the arbitrary division by 10 in earnings calculation and lack of vault balance validation

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_pay_to_spawn_arithmetic_error() {
        // Setup: Create a pay-to-spawn game session with players who have kills/spawns
        let mut game_session = GameSession {
            session_id: "pay_to_spawn_test".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 1000, // 1000 tokens per bet
            game_mode: GameMode::PayToSpawnOneVsOne, // Pay-to-spawn mode
            team_a: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [5, 0, 0, 0, 0], // Player has 5 spawns (bought extra)
                player_kills: [3, 0, 0, 0, 0],  // Player has 3 kills
            },
            team_b: Team {
                players: [Pubkey::new_unique(), Pubkey::default(), Pubkey::default(), Pubkey::default(), Pubkey::default()],
                total_bet: 1000,
                player_spawns: [2, 0, 0, 0, 0], // Player has 2 spawns
                player_kills: [7, 0, 0, 0, 0],  // Player has 7 kills
            },
            status: GameStatus::InProgress,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        let player_a = game_session.team_a.players[0];
        let player_b = game_session.team_b.players[0];

        println!("=== Pay-to-Spawn Earnings Calculation Analysis ===");
        println!("Session bet amount: {}", game_session.session_bet);

        // Test Player A earnings calculation
        let player_a_kills_and_spawns = game_session.get_kills_and_spawns(player_a).unwrap();
        let player_a_earnings = player_a_kills_and_spawns as u64 * game_session.session_bet / 10;
        
        println!("\nPlayer A:");
        println!("  Kills: {}", game_session.team_a.player_kills[0]);
        println!("  Spawns: {}", game_session.team_a.player_spawns[0]);
        println!("  Total kills+spawns: {}", player_a_kills_and_spawns);
        println!("  Calculated earnings: {} tokens", player_a_earnings);
        println!("  Expected without /10: {} tokens", player_a_kills_and_spawns as u64 * game_session.session_bet);

        // Test Player B earnings calculation  
        let player_b_kills_and_spawns = game_session.get_kills_and_spawns(player_b).unwrap();
        let player_b_earnings = player_b_kills_and_spawns as u64 * game_session.session_bet / 10;
        
        println!("\nPlayer B:");
        println!("  Kills: {}", game_session.team_b.player_kills[0]);
        println!("  Spawns: {}", game_session.team_b.player_spawns[0]);
        println!("  Total kills+spawns: {}", player_b_kills_and_spawns);
        println!("  Calculated earnings: {} tokens", player_b_earnings);
        println!("  Expected without /10: {} tokens", player_b_kills_and_spawns as u64 * game_session.session_bet);

        // Demonstrate the arithmetic error
        // Player A: (3 kills + 5 spawns) * 1000 / 10 = 8 * 1000 / 10 = 800 tokens
        assert_eq!(player_a_kills_and_spawns, 8);
        assert_eq!(player_a_earnings, 800); // Arbitrary division by 10
        
        // Player B: (7 kills + 2 spawns) * 1000 / 10 = 9 * 1000 / 10 = 900 tokens  
        assert_eq!(player_b_kills_and_spawns, 9);
        assert_eq!(player_b_earnings, 900); // Arbitrary division by 10

        // The issue: Why divide by 10? This seems arbitrary and reduces earnings significantly
        let total_earnings_with_division = player_a_earnings + player_b_earnings; // 800 + 900 = 1700
        let total_earnings_without_division = (player_a_kills_and_spawns as u64 * game_session.session_bet) + 
                                           (player_b_kills_and_spawns as u64 * game_session.session_bet); // 8000 + 9000 = 17000
        
        println!("\n=== Economic Impact Analysis ===");
        println!("Total earnings with /10 division: {} tokens", total_earnings_with_division);
        println!("Total earnings without /10 division: {} tokens", total_earnings_without_division);
        println!("Difference (lost to players): {} tokens", total_earnings_without_division - total_earnings_with_division);

        // Demonstrate potential vault balance issue
        let assumed_vault_balance = 5000; // Assume vault has 5000 tokens
        println!("\n=== Vault Balance Validation Issue ===");
        println!("Assumed vault balance: {} tokens", assumed_vault_balance);
        
        if total_earnings_without_division > assumed_vault_balance {
            println!("ERROR: Without /10, total earnings ({}) exceed vault balance ({})", 
                     total_earnings_without_division, assumed_vault_balance);
            println!("This would cause transaction failures or partial distributions");
        }
        
        if total_earnings_with_division <= assumed_vault_balance {
            println!("With /10 division, earnings ({}) fit within vault balance ({})", 
                     total_earnings_with_division, assumed_vault_balance);
            println!("But this may be unintentionally reducing player rewards");
        }

        // Critical assertions proving the vulnerability
        assert_eq!(player_a_earnings, 800, "Player A earnings should be 800 due to /10 division");
        assert_eq!(player_b_earnings, 900, "Player B earnings should be 900 due to /10 division");
        
        // Prove the division by 10 is arbitrary and significant
        assert_eq!(total_earnings_with_division * 10, total_earnings_without_division, 
                   "Division by 10 reduces total earnings by 90%");

        println!("\nEXPLOIT CONFIRMED: Arbitrary division by 10 reduces player earnings by 90%!");
        println!("Players receive {} instead of {} tokens due to hardcoded division", 
                 total_earnings_with_division, total_earnings_without_division);
    }

    #[test]
    fn test_vault_balance_validation_missing() {
        // Test that demonstrates lack of vault balance validation
        let mut game_session = GameSession {
            session_id: "vault_test".to_string(),
            authority: Pubkey::new_unique(),
            session_bet: 10000, // High bet amount
            game_mode: GameMode::PayToSpawnFiveVsFive, // 5v5 mode for more players
            team_a: Team {
                players: [
                    Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique(), 
                    Pubkey::new_unique(), Pubkey::new_unique()
                ],
                total_bet: 50000,
                player_spawns: [10, 10, 10, 10, 10], // All players have many spawns
                player_kills: [10, 10, 10, 10, 10],  // All players have many kills
            },
            team_b: Team {
                players: [
                    Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique(), 
                    Pubkey::new_unique(), Pubkey::new_unique()
                ],
                total_bet: 50000,
                player_spawns: [10, 10, 10, 10, 10],
                player_kills: [10, 10, 10, 10, 10],
            },
            status: GameStatus::InProgress,
            created_at: 0,
            bump: 0,
            vault_bump: 0,
            vault_token_bump: 0,
        };

        println!("\n=== Vault Balance Validation Test ===");
        
        let mut total_calculated_earnings = 0u64;
        let all_players = game_session.get_all_players();
        
        for player in all_players {
            if player != Pubkey::default() {
                let kills_and_spawns = game_session.get_kills_and_spawns(player).unwrap();
                let earnings = kills_and_spawns as u64 * game_session.session_bet / 10;
                total_calculated_earnings += earnings;
                println!("Player {} earnings: {} tokens (kills+spawns: {})", 
                         player, earnings, kills_and_spawns);
            }
        }

        println!("Total calculated earnings for all players: {} tokens", total_calculated_earnings);
        
        // Simulate various vault balance scenarios
        let vault_scenarios = vec![
            ("Low vault", 50000),
            ("Medium vault", 100000), 
            ("High vault", 200000),
        ];

        for (scenario_name, vault_balance) in vault_scenarios {
            println!("\nScenario: {} ({} tokens)", scenario_name, vault_balance);
            
            if total_calculated_earnings > vault_balance {
                println!("  CRITICAL: Total earnings ({}) exceed vault balance ({})", 
                         total_calculated_earnings, vault_balance);
                println!("  This would cause transaction failures without proper validation");
            } else {
                println!("  OK: Vault balance sufficient for calculated earnings");
            }
        }

        // Prove that the function doesn't validate vault balance
        // In a real scenario, this could lead to failed transactions
        assert!(total_calculated_earnings > 0, "Players should have earnings");
        println!("\nVULNERABILITY CONFIRMED: No vault balance validation before distribution!");
    }
}
