#![allow(non_snake_case)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
};

use csv::Reader;
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

const QB_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 6,
    ss_max: 8,
    star_min: 12,
    star_max: 20,
};
const HB_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 8,
    ss_max: 12,
    star_min: 30,
    star_max: 40,
};
const FB_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 0,
    ss_max: 2,
    star_min: 3,
    star_max: 5,
};
const WR_LIMITS: DevLimits = DevLimits {
    xf_min: 8,
    xf_max: 12,
    ss_min: 12,
    ss_max: 18,
    star_min: 45,
    star_max: 60,
};
const TE_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 6,
    ss_max: 8,
    star_min: 24,
    star_max: 32,
};
const OL_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 15,
    ss_max: 20,
    star_min: 75,
    star_max: 90,
};
const IDL_LIMITS: DevLimits = DevLimits {
    xf_min: 4,
    xf_max: 6,
    ss_min: 10,
    ss_max: 12,
    star_min: 36,
    star_max: 50,
};
const EDGE_LIMITS: DevLimits = DevLimits {
    xf_min: 4,
    xf_max: 6,
    ss_min: 6,
    ss_max: 10,
    star_min: 25,
    star_max: 40,
};
const LB_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 6,
    ss_max: 10,
    star_min: 24,
    star_max: 36,
};
const CB_LIMITS: DevLimits = DevLimits {
    xf_min: 3,
    xf_max: 6,
    ss_min: 10,
    ss_max: 16,
    star_min: 45,
    star_max: 60,
};
const S_LIMITS: DevLimits = DevLimits {
    xf_min: 3,
    xf_max: 5,
    ss_min: 8,
    ss_max: 12,
    star_min: 36,
    star_max: 50,
};
const K_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 1,
    ss_max: 3,
    star_min: 5,
    star_max: 8,
};
const P_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 1,
    ss_max: 2,
    star_min: 3,
    star_max: 6,
};

const THREE_FOUR_TEAMS: &[&str] = &[
    "Broncos",
    "Buccaneers",
    "Chargers",
    "Cowboys",
    "Dolphins",
    "Falcons",
    "Giants",
    "Jaguars",
    "Patriots",
    "Saints",
    "Texans",
    "Titans",
];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum DevTrait {
    XFactor = 3,
    Superstar = 2,
    Star = 1,
    Normal = 0,
}

fn main() {
    let mut seed = [0u8; 32];
    seed.copy_from_slice("T4cwqWjlAaZonILlHIIvp5rwBmt6jwBl".as_bytes());
    let mut rng = StdRng::from_seed(seed);

    let total_xf_max = QB_LIMITS.xf_max
        + HB_LIMITS.xf_max
        + FB_LIMITS.xf_max
        + WR_LIMITS.xf_max
        + TE_LIMITS.xf_max
        + OL_LIMITS.xf_max
        + IDL_LIMITS.xf_max
        + EDGE_LIMITS.xf_max
        + LB_LIMITS.xf_max
        + CB_LIMITS.xf_max
        + S_LIMITS.xf_max
        + K_LIMITS.xf_max
        + P_LIMITS.xf_max;
    let total_xf_min = QB_LIMITS.xf_min
        + HB_LIMITS.xf_min
        + FB_LIMITS.xf_min
        + WR_LIMITS.xf_min
        + TE_LIMITS.xf_min
        + OL_LIMITS.xf_min
        + IDL_LIMITS.xf_min
        + EDGE_LIMITS.xf_min
        + LB_LIMITS.xf_min
        + CB_LIMITS.xf_min
        + S_LIMITS.xf_min
        + K_LIMITS.xf_min
        + P_LIMITS.xf_min;
    let total_ss_max = QB_LIMITS.ss_max
        + HB_LIMITS.ss_max
        + FB_LIMITS.ss_max
        + WR_LIMITS.ss_max
        + TE_LIMITS.ss_max
        + OL_LIMITS.ss_max
        + IDL_LIMITS.ss_max
        + EDGE_LIMITS.ss_max
        + LB_LIMITS.ss_max
        + CB_LIMITS.ss_max
        + S_LIMITS.ss_max
        + K_LIMITS.ss_max
        + P_LIMITS.ss_max
        - total_xf_max;
    let total_ss_min = QB_LIMITS.ss_min
        + HB_LIMITS.ss_min
        + FB_LIMITS.ss_min
        + WR_LIMITS.ss_min
        + TE_LIMITS.ss_min
        + OL_LIMITS.ss_min
        + IDL_LIMITS.ss_min
        + EDGE_LIMITS.ss_min
        + LB_LIMITS.ss_min
        + CB_LIMITS.ss_min
        + S_LIMITS.ss_min
        + K_LIMITS.ss_min
        + P_LIMITS.ss_min
        - total_xf_min;
    let total_star_max = QB_LIMITS.star_max
        + HB_LIMITS.star_max
        + FB_LIMITS.star_max
        + WR_LIMITS.star_max
        + TE_LIMITS.star_max
        + OL_LIMITS.star_max
        + IDL_LIMITS.star_max
        + EDGE_LIMITS.star_max
        + LB_LIMITS.star_max
        + CB_LIMITS.star_max
        + S_LIMITS.star_max
        + K_LIMITS.star_max
        + P_LIMITS.star_max
        - total_ss_max;
    let total_star_min = QB_LIMITS.star_min
        + HB_LIMITS.star_min
        + FB_LIMITS.star_min
        + WR_LIMITS.star_min
        + TE_LIMITS.star_min
        + OL_LIMITS.star_min
        + IDL_LIMITS.star_min
        + EDGE_LIMITS.star_min
        + LB_LIMITS.star_min
        + CB_LIMITS.star_min
        + S_LIMITS.star_min
        + K_LIMITS.star_min
        + P_LIMITS.star_min
        - total_ss_min;
    println!("Overall targets:");
    println!("XF: {total_xf_min}-{total_xf_max}");
    println!("SS: {total_ss_min}-{total_ss_max}");
    println!("Star: {total_star_min}-{total_star_max}");
    println!();

    let passing_stats: Vec<PassingData> = read_csv("data/neon_season/SFDL_passing.csv");
    let receiving_stats: Vec<ReceivingData> = read_csv("data/neon_season/SFDL_receiving.csv");
    let rushing_stats: Vec<RushingData> = read_csv("data/neon_season/SFDL_rushing.csv");
    let defense_stats: Vec<DefenseData> = read_csv("data/neon_season/SFDL_defense.csv");
    let kicking_stats: Vec<KickingData> = read_csv("data/neon_season/SFDL_kicking.csv");
    let punting_stats: Vec<PuntingData> = read_csv("data/neon_season/SFDL_punting.csv");
    let players_old: Vec<PlayerData> = read_csv("data/neon_players_old/SFDL_players.csv");
    let players_new: Vec<PlayerData> = read_csv("data/neon_players_new/SFDL_players.csv");

    // List of names
    let mut protected_players = HashSet::new();
    let mut upgraded_players = HashSet::new();
    // Map<(Name, Team, Position), (OldDev, NewDev)>
    let mut changed_players: HashMap<(String, String, String), (DevTrait, DevTrait)> =
        HashMap::new();
    for (pos, limits) in [
        ("QB", QB_LIMITS),
        ("HB", HB_LIMITS),
        ("FB", FB_LIMITS),
        ("WR", WR_LIMITS),
        ("TE", TE_LIMITS),
        ("OL", OL_LIMITS),
        ("IDL", IDL_LIMITS),
        ("EDGE", EDGE_LIMITS),
        ("LB", LB_LIMITS),
        ("CB", CB_LIMITS),
        ("S", S_LIMITS),
        ("K", K_LIMITS),
        ("P", P_LIMITS),
    ] {
        // All players at the position who can be considered for demotion
        let players = if pos == "IDL" {
            players_old
                .iter()
                .filter(|player| {
                    player.position == "DT"
                        || (["LE", "RE"].contains(&player.position.as_str())
                            && THREE_FOUR_TEAMS.contains(&player.team.as_str()))
                })
                .collect_vec()
        } else if pos == "EDGE" {
            players_old
                .iter()
                .filter(|player| {
                    (["LE", "RE"].contains(&player.position.as_str())
                        && !THREE_FOUR_TEAMS.contains(&player.team.as_str()))
                        || (["LOLB", "ROLB"].contains(&player.position.as_str())
                            && THREE_FOUR_TEAMS.contains(&player.team.as_str()))
                })
                .collect_vec()
        } else if pos == "LB" {
            players_old
                .iter()
                .filter(|player| {
                    (["LOLB", "ROLB"].contains(&player.position.as_str())
                        && !THREE_FOUR_TEAMS.contains(&player.team.as_str()))
                        || player.position == "MLB"
                })
                .collect_vec()
        } else if pos == "OL" {
            players_old
                .iter()
                .filter(|player| ["LG", "LT", "RG", "RT", "C"].contains(&player.position.as_str()))
                .collect_vec()
        } else {
            players_old
                .iter()
                .filter(|player| player.position == pos)
                .collect_vec()
        }
        .into_iter()
        .filter(|player| !player.isRetired)
        .collect_vec();

        // Protect rookies who played at least 8 games
        for player in players
            .iter()
            .filter(|player| player.yearsPro <= 1)
            .filter(|player| {
                let games_played = passing_stats
                    .iter()
                    .filter(|stat| stat.player__rosterId == player.rosterId)
                    .map(|stat| stat.gamesPlayed)
                    .sum::<u8>()
                    .max(
                        rushing_stats
                            .iter()
                            .filter(|stat| stat.player__rosterId == player.rosterId)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        receiving_stats
                            .iter()
                            .filter(|stat| stat.player__rosterId == player.rosterId)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        defense_stats
                            .iter()
                            .filter(|stat| stat.player__rosterId == player.rosterId)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        kicking_stats
                            .iter()
                            .filter(|stat| stat.player__rosterId == player.rosterId)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        punting_stats
                            .iter()
                            .filter(|stat| stat.player__rosterId == player.rosterId)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    );
                games_played >= 8
            })
        {
            protected_players.insert(player.fullName.clone());
        }

        // Protect players who just devved up
        for player in players_old.iter().filter(|player| {
            players_new
                .iter()
                .find(|new| new.rosterId == player.rosterId)
                .map(|new| new.devTrait > player.devTrait)
                .unwrap_or(false)
        }) {
            protected_players.insert(player.fullName.clone());
            upgraded_players.insert(player.fullName.clone());
        }

        // Sort players according to their performance this season
        let players = players
            .into_iter()
            .sorted_unstable_by(|player_a, player_b| match pos {
                "QB" => {
                    let a_pass_stats = passing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_pass_stats = passing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);
                    let a_rush_stats = rushing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_rush_stats = rushing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);

                    let score_a = calc_qb_score(player_a, a_pass_stats, a_rush_stats);
                    let score_b = calc_qb_score(player_b, b_pass_stats, b_rush_stats);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                "HB" | "FB" | "WR" | "TE" => {
                    let a_recv_stats = receiving_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_recv_stats = receiving_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);
                    let a_rush_stats = rushing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_rush_stats = rushing_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);

                    let score_a = calc_receiver_score(player_a, a_recv_stats, a_rush_stats);
                    let score_b = calc_receiver_score(player_b, b_recv_stats, b_rush_stats);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                "OL" => {
                    let score_a = calc_ol_score(player_a, &mut rng);
                    let score_b = calc_ol_score(player_b, &mut rng);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                "IDL" | "EDGE" | "LB" | "CB" | "S" => {
                    let a_stats = defense_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_stats = defense_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);

                    let score_a = calc_defense_score(player_a, a_stats);
                    let score_b = calc_defense_score(player_b, b_stats);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                "K" => {
                    let a_stats = kicking_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_stats = kicking_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);

                    let score_a = calc_kicker_score(player_a, a_stats);
                    let score_b = calc_kicker_score(player_b, b_stats);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                "P" => {
                    let a_stats = punting_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_a.rosterId);
                    let b_stats = punting_stats
                        .iter()
                        .find(|stat| stat.player__rosterId == player_b.rosterId);

                    let score_a = calc_punter_score(player_a, a_stats);
                    let score_b = calc_punter_score(player_b, b_stats);
                    if score_a > score_b {
                        Ordering::Less
                    } else if score_b > score_a {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                _ => unreachable!(),
            })
            .collect_vec();

        let xf_count = players
            .iter()
            .filter(|x| x.devTrait >= DevTrait::XFactor as u8)
            .count();
        if xf_count < limits.xf_min {
            let players = players
                .iter()
                .filter(|player| {
                    player.devTrait == DevTrait::Superstar as u8
                        && !upgraded_players.contains(&player.fullName)
                })
                .take(limits.xf_min - xf_count)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                upgraded_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::Superstar, DevTrait::XFactor),
                );
            }
        } else if xf_count > limits.xf_max {
            let players = players
                .iter()
                .filter(|player| {
                    player.devTrait == DevTrait::XFactor as u8
                        && !protected_players.contains(&player.fullName)
                })
                .rev()
                .take(xf_count - limits.xf_max)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::XFactor, DevTrait::Superstar),
                );
            }
        }

        let ss_count = players
            .iter()
            .filter(|x| x.devTrait >= DevTrait::Superstar as u8)
            .count();
        if ss_count < limits.ss_min {
            let players = players
                .iter()
                .filter(|player| {
                    player.devTrait == DevTrait::Star as u8
                        && !upgraded_players.contains(&player.fullName)
                })
                .take(limits.ss_min - ss_count)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                upgraded_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::Star, DevTrait::Superstar),
                );
            }
        } else if ss_count > limits.ss_max {
            let players = players
                .iter()
                .filter(|player| {
                    player.devTrait == DevTrait::Superstar as u8
                        && !protected_players.contains(&player.fullName)
                })
                .rev()
                .take(ss_count - limits.ss_max)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::Superstar, DevTrait::Star),
                );
            }
        }

        let star_count = players
            .iter()
            .filter(|x| x.devTrait >= DevTrait::Star as u8)
            .count();
        if star_count < limits.star_min {
            let players = players
                .iter()
                .filter(|player| player.devTrait == DevTrait::Normal as u8)
                .take(limits.star_min - star_count)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                upgraded_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::Normal, DevTrait::Star),
                );
            }
        } else if star_count > limits.star_max {
            let players = players
                .iter()
                .filter(|player| {
                    player.devTrait == DevTrait::Star as u8
                        && !protected_players.contains(&player.fullName)
                })
                .rev()
                .take(star_count - limits.star_max)
                .collect_vec();
            for player in players {
                protected_players.insert(player.fullName.clone());
                changed_players.insert(
                    (
                        player.fullName.clone(),
                        player.team.clone(),
                        player.position.clone(),
                    ),
                    (DevTrait::Star, DevTrait::Normal),
                );
            }
        }
    }

    for (team, group) in changed_players
        .into_iter()
        .sorted_unstable_by_key(|((_, team, _), _)| team.clone())
        .group_by(|((_, team, _), _)| team.clone())
        .into_iter()
    {
        if team.is_empty() {
            println!("Free Agents:");
        } else {
            println!("{team}:");
        }
        for ((player, _, pos), (old, new)) in group
            .into_iter()
            .sorted_unstable_by_key(|((name, _, pos), _)| (pos.clone(), name.clone()))
        {
            println!("{pos} {player}: {old:?} -> {new:?}");
        }
        println!();
    }
}

fn read_csv<T: DeserializeOwned>(filename: &str) -> Vec<T> {
    let file = BufReader::new(File::open(filename).unwrap());
    let mut reader = Reader::from_reader(file);
    reader.deserialize().map(|rec| rec.unwrap()).collect()
}

fn from_str_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    if s == "True" {
        Ok(true)
    } else if s == "False" {
        Ok(false)
    } else {
        panic!("Unexpected value for bool");
    }
}

const PASS_YARD_VALUE: f32 = 0.05;
const PASS_TD_VALUE: f32 = 4.0;
const INT_LOST_VALUE: f32 = -2.0;
const RUSH_YARD_VALUE: f32 = 0.1;
const RUSH_TD_VALUE: f32 = 4.0;
const FUMBLE_VALUE: f32 = -2.0;
const RECV_YARD_VALUE: f32 = 0.1;
const RECV_CATCH_VALUE: f32 = 0.25;
const RECV_DROP_VALUE: f32 = -0.05;
const RECV_TD_VALUE: f32 = 4.0;
const TACKLE_VALUE: f32 = 0.5;
const DEFLECTION_VALUE: f32 = 1.0;
const CATCH_ALLOWED_VALUE: f32 = -0.2;
const SACK_VALUE: f32 = 2.0;
const DEF_TD_VALUE: f32 = 6.0;
const FORCED_FUMBLE_VALUE: f32 = 2.0;
const FUMBLE_REC_VALUE: f32 = 2.0;
const INT_FORCED_VALUE: f32 = 3.0;
const INT_RETURN_YARD_VALUE: f32 = 0.05;
const SAFETY_VALUE: f32 = 3.0;
const FG_VALUE: f32 = 3.0;
const FG_50_PLUS_VALUE: f32 = 5.0;
const FG_MISS_VALUE: f32 = -1.0;
// This is positive to reduce the penalty from above.
const FG_50_PLUS_MISS_VALUE: f32 = 0.9;
const XP_VALUE: f32 = 1.0;
const XP_MISS_VALUE: f32 = -1.0;
const PUNT_AVG_YDS_VALUE: f32 = 0.02;

/// Applies a multiplier to a score based on a players age and possibly other
/// stuff in the future
fn calc_player_multiplier(player: &PlayerData) -> f32 {
    if player.age <= 24 || player.yearsPro <= 2 {
        return 1.2;
    }
    if player.age <= 26 {
        return 1.1;
    }
    if player.age <= 29 {
        return 1.0;
    }
    // A gradual slope that results in 20% loss at age 35, 40% loss at age 40, to a
    // max of 50%.
    (1.0 + (player.age - 30) as f32 * -0.04).max(0.5)
}

fn calc_qb_score(
    player: &PlayerData,
    pass_stats: Option<&PassingData>,
    rush_stats: Option<&RushingData>,
) -> f32 {
    calc_player_multiplier(player)
        * (pass_stats
            .map(|stat| {
                stat.passTotalYds as f32 * PASS_YARD_VALUE
                    + stat.passTotalTDs as f32 * PASS_TD_VALUE
                    + stat.passTotalInts as f32 * INT_LOST_VALUE
            })
            .unwrap_or(0.0)
            + rush_stats
                .map(|stat| {
                    stat.rushTotalYds as f32 * RUSH_YARD_VALUE
                        + stat.rushTotalTDs as f32 * RUSH_TD_VALUE
                        + stat.rushTotalFum as f32 * FUMBLE_VALUE
                })
                .unwrap_or(0.0))
}

fn calc_receiver_score(
    player: &PlayerData,
    recv_stats: Option<&ReceivingData>,
    rush_stats: Option<&RushingData>,
) -> f32 {
    calc_player_multiplier(player)
        * (recv_stats
            .map(|stat| {
                stat.recTotalYds as f32 * RECV_YARD_VALUE
                    + stat.recTotalTDs as f32 * RECV_TD_VALUE
                    + stat.recTotalCatches as f32 * RECV_CATCH_VALUE
                    + stat.recTotalDrops as f32 * RECV_DROP_VALUE
            })
            .unwrap_or(0.0)
            + rush_stats
                .map(|stat| {
                    stat.rushTotalYds as f32 * RUSH_YARD_VALUE
                        + stat.rushTotalTDs as f32 * RUSH_TD_VALUE
                        + stat.rushTotalFum as f32 * FUMBLE_VALUE
                })
                .unwrap_or(0.0))
}

/// The game doesn't give us stats for OL so we have to do something weird and
/// terrible. Something very madden-esque. Although we at least take into
/// account age here, and add in a random factor so it's not just the youngest,
/// best players getting it. But this still sucks. Blame Madden for not giving
/// us OL stats.
fn calc_ol_score(player: &PlayerData, rng: &mut StdRng) -> f32 {
    calc_player_multiplier(player) * player.playerBestOvr as f32 * rng.gen_range(0.9..1.1)
}

fn calc_defense_score(player: &PlayerData, stats: Option<&DefenseData>) -> f32 {
    calc_player_multiplier(player)
        * stats
            .map(|stat| {
                stat.defTotalTackles * TACKLE_VALUE
                    + stat.defTotalDeflections as f32 * DEFLECTION_VALUE
                    + stat.defTotalCatchAllowed as f32 * CATCH_ALLOWED_VALUE
                    + stat.defTotalSacks * SACK_VALUE
                    + stat.defTotalTDs as f32 * DEF_TD_VALUE
                    + stat.defTotalForcedFum as f32 * FORCED_FUMBLE_VALUE
                    + stat.defTotalFumRec as f32 * FUMBLE_REC_VALUE
                    + stat.defTotalInts as f32 * INT_FORCED_VALUE
                    + stat.defTotalIntReturnYds as f32 * INT_RETURN_YARD_VALUE
                    + stat.defTotalSafeties as f32 * SAFETY_VALUE
            })
            .unwrap_or(0.0)
}

fn calc_kicker_score(player: &PlayerData, stats: Option<&KickingData>) -> f32 {
    calc_player_multiplier(player)
        * stats
            .map(|stat| {
                let fg_miss = stat.fGTotalAtt - stat.fGTotalMade;
                let fg_50plus_miss = stat.fGTotal50PlusAtt - stat.fGTotal50PlusMade;
                let xp_miss = stat.xPTotalAtt - stat.xPTotalMade;
                stat.fGTotalMade as f32 * FG_VALUE
                    + stat.fGTotal50PlusMade as f32 * FG_50_PLUS_VALUE
                    + fg_miss as f32 * FG_MISS_VALUE
                    + fg_50plus_miss as f32 * FG_50_PLUS_MISS_VALUE
                    + stat.xPTotalMade as f32 * XP_VALUE
                    + xp_miss as f32 * XP_MISS_VALUE
            })
            .unwrap_or(0.0)
}

fn calc_punter_score(player: &PlayerData, stats: Option<&PuntingData>) -> f32 {
    calc_player_multiplier(player)
        * stats
            .map(|stat| {
                if stat.gamesPlayed < 6 {
                    // Because this is based on averages, given that good teams punt less,
                    // we need to have a minimum number of games played to consider a punter for
                    // promotion. On all the other positions, the stats have a
                    // direct correlation with games played.
                    //
                    // Except OL. Because we have no stats. Not even number of games played.
                    return stat.gamesPlayed as f32 / 100.;
                }
                stat.puntAvgYdsPerAtt * PUNT_AVG_YDS_VALUE
                    + stat.puntsTotalIn20 as f32 / stat.puntTotalAtt as f32
            })
            .unwrap_or(0.0)
}

#[derive(Debug, Clone, Copy)]
struct DevLimits {
    pub xf_min: usize,
    pub xf_max: usize,
    pub ss_min: usize,
    pub ss_max: usize,
    pub star_min: usize,
    pub star_max: usize,
}

#[derive(Deserialize)]
struct PlayerData {
    pub rosterId: u32,
    pub team: String,
    #[serde(deserialize_with = "from_str_bool")]
    pub isRetired: bool,
    pub age: u8,
    pub fullName: String,
    pub position: String,
    pub playerBestOvr: u8,
    pub yearsPro: u8,
    pub devTrait: u8,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PassingData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub passTotalAtt: u32,
    pub passTotalComp: u32,
    pub passAvgCompPct: f32,
    pub passTotalInts: u32,
    pub passTotalLongest: i32,
    pub passerAvgRating: f32,
    pub passTotalSacks: u32,
    pub passTotalTDs: u32,
    pub passTotalYds: i32,
    pub passAvgYdsPerAtt: f32,
    pub passAvgYdsPerGame: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ReceivingData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub recTotalCatches: u32,
    pub recAvgCatchPct: f32,
    pub recTotalDrops: u32,
    pub recTotalLongest: u32,
    pub recTotalTDs: u32,
    pub recTotalYdsAfterCatch: i32,
    pub recTotalYds: i32,
    pub recAvgYacPerCatch: f32,
    pub recAvgYdsPerCatch: f32,
    pub recAvgYdsPerGame: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct RushingData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub rushTotalAtt: u32,
    pub rushTotalBrokenTackles: u32,
    pub rushTotalFum: u32,
    pub rushTotalLongest: i32,
    pub rushTotalTDs: u32,
    pub rushTotal20PlusYds: u32,
    pub rushTotalYdsAfterContact: i32,
    pub rushTotalYds: i32,
    pub rushAvgYdsAfterContact: f32,
    pub rushAvgYdsPerAtt: f32,
    pub rushAvgYdsPerGame: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct DefenseData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub defTotalCatchAllowed: u32,
    pub defTotalDeflections: u32,
    pub defTotalForcedFum: u32,
    pub defTotalFumRec: u32,
    pub defTotalInts: u32,
    pub defTotalIntReturnYds: u32,
    pub defTotalSacks: f32,
    pub defTotalSafeties: u32,
    pub defTotalTDs: u32,
    pub defTotalTackles: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct KickingData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub fGTotalAtt: u32,
    pub fGTotal50PlusAtt: u32,
    pub fGTotal50PlusMade: u32,
    pub fGTotalLongest: u32,
    pub fGTotalMade: u32,
    pub fGAvgCompPct: f32,
    pub kickoffTotalAtt: u32,
    pub kickoffTotalTBs: u32,
    pub xPTotalAtt: u32,
    pub xPTotalMade: u32,
    pub xPAvgCompPct: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PuntingData {
    pub player__rosterId: u32,
    pub gamesPlayed: u8,
    pub puntsTotalBlocked: u32,
    pub puntsTotalIn20: u32,
    pub puntTotalLongest: i32,
    pub puntTotalTBs: u32,
    pub puntAvgNetYdsPerAtt: f32,
    pub puntTotalNetYds: i32,
    pub puntTotalAtt: u32,
    pub puntAvgYdsPerAtt: f32,
    pub puntTotalYds: i32,
}
