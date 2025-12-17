use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct STVRound {
    pub round_number: usize,
    pub vote_counts: HashMap<Uuid, usize>,
    pub eliminated: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct STVResult {
    pub winner: Option<Uuid>,
    pub rounds: Vec<STVRound>,
    pub quota: usize,
}

/// Calculate Single Transferable Vote result
/// For Wentu, we treat it as choosing 1 winner (the most preferred date)
pub fn calculate_stv(
    voter_preferences: Vec<Vec<Uuid>>, // Each inner vec is voter's ordered preferences
    date_options: Vec<Uuid>,
) -> STVResult {
    if voter_preferences.is_empty() {
        return STVResult {
            winner: None,
            rounds: vec![],
            quota: 0,
        };
    }

    let total_votes = voter_preferences.len();
    let quota = (total_votes / 2) + 1; // Majority for 1 seat

    let mut rounds = vec![];
    let mut eliminated: HashSet<Uuid> = HashSet::new();
    let current_preferences = voter_preferences;

    loop {
        // Count first preferences
        let mut vote_counts: HashMap<Uuid, usize> = HashMap::new();
        for prefs in &current_preferences {
            // Find first non-eliminated preference
            if let Some(&first_choice) = prefs.iter().find(|&&opt| !eliminated.contains(&opt)) {
                *vote_counts.entry(first_choice).or_insert(0) += 1;
            }
        }

        // Check for majority
        if let Some((&winner, &count)) = vote_counts.iter().max_by_key(|&(_, &c)| c) {
            if count >= quota {
                rounds.push(STVRound {
                    round_number: rounds.len() + 1,
                    vote_counts: vote_counts.clone(),
                    eliminated: None,
                });
                return STVResult {
                    winner: Some(winner),
                    rounds,
                    quota,
                };
            }
        }

        // All candidates eliminated (shouldn't happen) or only one remains
        if eliminated.len() >= date_options.len() - 1 {
            let remaining = date_options
                .iter()
                .find(|opt| !eliminated.contains(opt))
                .copied();
            rounds.push(STVRound {
                round_number: rounds.len() + 1,
                vote_counts: vote_counts.clone(),
                eliminated: None,
            });
            return STVResult {
                winner: remaining,
                rounds,
                quota,
            };
        }

        // Eliminate lowest-voted candidate
        if let Some((&to_eliminate, _)) = vote_counts
            .iter()
            .filter(|(&opt, _)| !eliminated.contains(&opt))
            .min_by_key(|&(_, &c)| c)
        {
            eliminated.insert(to_eliminate);
            rounds.push(STVRound {
                round_number: rounds.len() + 1,
                vote_counts: vote_counts.clone(),
                eliminated: Some(to_eliminate),
            });
        } else {
            break;
        }
    }

    STVResult {
        winner: None,
        rounds,
        quota,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_voter() {
        let uuid1 = Uuid::nil();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();

        let preferences = vec![vec![uuid1, uuid2, uuid3]];
        let options = vec![uuid1, uuid2, uuid3];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(uuid1));
        assert_eq!(result.quota, 1);
    }

    #[test]
    fn test_clear_majority() {
        let uuid1 = Uuid::nil();
        let uuid2 = Uuid::new_v4();

        let preferences = vec![
            vec![uuid1, uuid2],
            vec![uuid1, uuid2],
            vec![uuid2, uuid1],
        ];
        let options = vec![uuid1, uuid2];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(uuid1));
    }

    #[test]
    fn test_elimination() {
        let uuid1 = Uuid::nil();
        let uuid2 = Uuid::new_v4();
        let uuid3 = Uuid::new_v4();

        let preferences = vec![
            vec![uuid1, uuid2, uuid3],
            vec![uuid1, uuid3, uuid2],
            vec![uuid2, uuid1, uuid3],
            vec![uuid3, uuid2, uuid1],
        ];
        let options = vec![uuid1, uuid2, uuid3];

        let result = calculate_stv(preferences, options);
        assert!(result.winner.is_some());
        assert!(!result.rounds.is_empty());
    }
}
