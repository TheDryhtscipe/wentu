use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct STVRound {
    pub round_number: usize,
    pub vote_counts: HashMap<Uuid, usize>,
    pub eliminated: Option<Uuid>,
    pub quota: usize,
}

#[derive(Debug, Clone)]
pub struct STVResult {
    pub winner: Option<Uuid>,
    pub rounds: Vec<STVRound>,
    pub quota: usize,
}

/// Calculate Single Transferable Vote result for a single-winner election
/// (equivalent to Instant Runoff Voting).
///
/// Quota is recomputed each round against continuing (non-exhausted) ballots,
/// so a winner can be declared once they hold a majority of ballots that still
/// have an uneliminated preference. Elimination ties are broken by walking
/// backward through prior rounds (Scottish STV style); if still tied, the
/// candidate with the lowest UUID is eliminated as a deterministic fallback.
pub fn calculate_stv(
    voter_preferences: Vec<Vec<Uuid>>,
    date_options: Vec<Uuid>,
) -> STVResult {
    if voter_preferences.is_empty() || date_options.is_empty() {
        return STVResult {
            winner: None,
            rounds: vec![],
            quota: 0,
        };
    }

    let mut rounds: Vec<STVRound> = vec![];
    let mut eliminated: HashSet<Uuid> = HashSet::new();

    loop {
        let mut vote_counts: HashMap<Uuid, usize> = HashMap::new();
        for opt in &date_options {
            if !eliminated.contains(opt) {
                vote_counts.insert(*opt, 0);
            }
        }

        let mut continuing_ballots: usize = 0;
        for prefs in &voter_preferences {
            if let Some(&first_choice) = prefs.iter().find(|&&opt| !eliminated.contains(&opt)) {
                *vote_counts.entry(first_choice).or_insert(0) += 1;
                continuing_ballots += 1;
            }
        }

        let quota = if continuing_ballots == 0 {
            0
        } else {
            (continuing_ballots / 2) + 1
        };

        let winner_this_round = vote_counts
            .iter()
            .filter(|(_, &c)| c >= quota && quota > 0)
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
            .map(|(id, _)| *id);

        if let Some(winner) = winner_this_round {
            rounds.push(STVRound {
                round_number: rounds.len() + 1,
                vote_counts,
                eliminated: None,
                quota,
            });
            return STVResult {
                winner: Some(winner),
                rounds,
                quota,
            };
        }

        let remaining: Vec<Uuid> = date_options
            .iter()
            .copied()
            .filter(|opt| !eliminated.contains(opt))
            .collect();

        if remaining.len() <= 1 {
            let winner = remaining.first().copied();
            rounds.push(STVRound {
                round_number: rounds.len() + 1,
                vote_counts,
                eliminated: None,
                quota,
            });
            return STVResult {
                winner,
                rounds,
                quota,
            };
        }

        if continuing_ballots == 0 {
            rounds.push(STVRound {
                round_number: rounds.len() + 1,
                vote_counts,
                eliminated: None,
                quota,
            });
            return STVResult {
                winner: None,
                rounds,
                quota,
            };
        }

        let to_eliminate = pick_elimination(&remaining, &vote_counts, &rounds);

        rounds.push(STVRound {
            round_number: rounds.len() + 1,
            vote_counts,
            eliminated: Some(to_eliminate),
            quota,
        });
        eliminated.insert(to_eliminate);
    }
}

/// Pick the candidate to eliminate this round.
///
/// Lowest current vote count wins; ties broken by walking backward through
/// prior rounds (fewer votes in the most recent prior round is eliminated),
/// then by lowest UUID as a deterministic final fallback.
fn pick_elimination(
    remaining: &[Uuid],
    vote_counts: &HashMap<Uuid, usize>,
    prior_rounds: &[STVRound],
) -> Uuid {
    let min_count = remaining
        .iter()
        .map(|id| vote_counts.get(id).copied().unwrap_or(0))
        .min()
        .unwrap_or(0);

    let tied: Vec<Uuid> = remaining
        .iter()
        .copied()
        .filter(|id| vote_counts.get(id).copied().unwrap_or(0) == min_count)
        .collect();

    if tied.len() == 1 {
        return tied[0];
    }

    for prior in prior_rounds.iter().rev() {
        let prior_min = tied
            .iter()
            .map(|id| prior.vote_counts.get(id).copied().unwrap_or(0))
            .min()
            .unwrap_or(0);

        let still_tied: Vec<Uuid> = tied
            .iter()
            .copied()
            .filter(|id| prior.vote_counts.get(id).copied().unwrap_or(0) == prior_min)
            .collect();

        if still_tied.len() == 1 {
            return still_tied[0];
        }

        if still_tied.len() < tied.len() {
            return *still_tied.iter().min().unwrap();
        }
    }

    *tied.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uuid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn test_single_voter() {
        let a = uuid(1);
        let b = uuid(2);
        let c = uuid(3);

        let preferences = vec![vec![a, b, c]];
        let options = vec![a, b, c];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(a));
        assert_eq!(result.quota, 1);
    }

    #[test]
    fn test_clear_majority() {
        let a = uuid(1);
        let b = uuid(2);

        let preferences = vec![vec![a, b], vec![a, b], vec![b, a]];
        let options = vec![a, b];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(a));
        assert_eq!(result.quota, 2);
    }

    #[test]
    fn test_elimination_and_transfer() {
        let a = uuid(1);
        let b = uuid(2);
        let c = uuid(3);

        // a: 2 first-prefs, b: 1, c: 1. Quota=3.
        // b and c tied at bottom; no prior rounds; UUID fallback eliminates
        // lowest (b=uuid 2). b voter's 2nd pref is a -> a=3, wins.
        let preferences = vec![
            vec![a, b, c],
            vec![a, b, c],
            vec![b, a, c],
            vec![c, a, b],
        ];
        let options = vec![a, b, c];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(a));
        assert_eq!(result.rounds.len(), 2);
        assert_eq!(result.rounds[0].eliminated, Some(b));
        assert_eq!(result.rounds[1].vote_counts.get(&a), Some(&3));
    }

    #[test]
    fn test_dynamic_quota_with_exhausted_ballots() {
        let a = uuid(1);
        let b = uuid(2);
        let c = uuid(3);

        // 5 voters, initial quota=3.
        // Round 1: a=2, b=2, c=1. c eliminated.
        // c voter ranked only [c], so their ballot is exhausted.
        // Round 2: 4 continuing ballots, quota=3. a=2, b=2. Tie.
        // Backward tie-break: prior round a=2, b=2 (still tied), then UUID -> b eliminated (higher).
        // Actually lowest UUID stays; highest UUID is eliminated? Let me re-check — we pick lowest UUID to eliminate.
        // a=uuid(1), b=uuid(2): a has lower UUID, so a is eliminated.
        // Round 3: only b remains, b wins.
        let preferences = vec![
            vec![a],
            vec![a],
            vec![b],
            vec![b],
            vec![c],
        ];
        let options = vec![a, b, c];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(b));
        assert!(result.rounds.iter().any(|r| r.quota < 3));
    }

    #[test]
    fn test_zero_vote_candidate_appears_in_counts() {
        let a = uuid(1);
        let b = uuid(2);
        let c = uuid(3);

        let preferences = vec![vec![a], vec![a]];
        let options = vec![a, b, c];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(a));
        assert_eq!(result.rounds[0].vote_counts.get(&b), Some(&0));
        assert_eq!(result.rounds[0].vote_counts.get(&c), Some(&0));
    }

    #[test]
    fn test_empty_ballots() {
        let a = uuid(1);
        let result = calculate_stv(vec![], vec![a]);
        assert_eq!(result.winner, None);
        assert_eq!(result.quota, 0);
    }

    #[test]
    fn test_backward_tie_break_resolves() {
        let a = uuid(10);
        let b = uuid(20);
        let c = uuid(30);
        let d = uuid(40);

        // Round 1: a=2, b=1, c=1, d=3. Quota=4. b,c tied at bottom.
        // No prior rounds -> UUID fallback eliminates lowest (b).
        // b voter's 2nd pref (c) transfers.
        // Round 2: a=2, c=2, d=3. a,c tied at bottom.
        // Backward tie-break: in round 1, a=2 vs c=1, so c had fewer -> c eliminated.
        // c voters (2) transfer to their next pref: a.
        // Round 3: a=4, d=3. Quota=4. a wins.
        let preferences = vec![
            vec![a, c, d],
            vec![a, c, d],
            vec![b, c, a],
            vec![c, a, d],
            vec![d, a, b],
            vec![d, a, b],
            vec![d, a, b],
        ];
        let options = vec![a, b, c, d];

        let result = calculate_stv(preferences, options);
        assert_eq!(result.winner, Some(a));
        assert_eq!(result.rounds[0].eliminated, Some(b));
        assert_eq!(result.rounds[1].eliminated, Some(c));
    }

    #[test]
    fn test_determinism() {
        let a = uuid(1);
        let b = uuid(2);
        let c = uuid(3);

        let prefs = vec![vec![a, b, c], vec![b, c, a], vec![c, a, b]];
        let options = vec![a, b, c];

        let r1 = calculate_stv(prefs.clone(), options.clone());
        let r2 = calculate_stv(prefs, options);
        assert_eq!(r1.winner, r2.winner);
        assert_eq!(r1.rounds.len(), r2.rounds.len());
        for (ra, rb) in r1.rounds.iter().zip(r2.rounds.iter()) {
            assert_eq!(ra.eliminated, rb.eliminated);
        }
    }
}
