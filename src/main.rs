use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
};

use csv::Reader;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

// Assuming 32 starters
const QB_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 6,
    ss_max: 8,
    star_min: 12,
    star_max: 16,
};
// Assuming 32 starters + 32 3DRBs
const HB_LIMITS: DevLimits = DevLimits {
    xf_min: 2,
    xf_max: 4,
    ss_min: 6,
    ss_max: 10,
    star_min: 20,
    star_max: 30,
};
// Assuming 32 starters
const FB_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 0,
    ss_max: 2,
    star_min: 4,
    star_max: 6,
};
// Assuming about 100 starters
const WR_LIMITS: DevLimits = DevLimits {
    xf_min: 5,
    xf_max: 7,
    ss_min: 12,
    ss_max: 16,
    star_min: 40,
    star_max: 60,
};
// Assuming 32 starters and occasional 2/3 TE formations
const TE_LIMITS: DevLimits = DevLimits {
    xf_min: 1,
    xf_max: 3,
    ss_min: 4,
    ss_max: 8,
    star_min: 18,
    star_max: 25,
};
// Assuming 160 starters
const OL_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 12,
    ss_max: 16,
    star_min: 72,
    star_max: 95,
};
// Assuming 110-120 starters
const IDL_LIMITS: DevLimits = DevLimits {
    xf_min: 4,
    xf_max: 6,
    ss_min: 10,
    ss_max: 15,
    star_min: 50,
    star_max: 65,
};
// Assuming 64 starters
const EDGE_LIMITS: DevLimits = DevLimits {
    xf_min: 5,
    xf_max: 7,
    ss_min: 8,
    ss_max: 12,
    star_min: 25,
    star_max: 36,
};
// Assuming 110-120 starters
const LB_LIMITS: DevLimits = DevLimits {
    xf_min: 4,
    xf_max: 6,
    ss_min: 10,
    ss_max: 15,
    star_min: 50,
    star_max: 65,
};
// Assuming about 100 starters
const CB_LIMITS: DevLimits = DevLimits {
    xf_min: 5,
    xf_max: 7,
    ss_min: 12,
    ss_max: 16,
    star_min: 40,
    star_max: 60,
};
// Assuming 64 starters + a few used in BNOG and Slot
const S_LIMITS: DevLimits = DevLimits {
    xf_min: 3,
    xf_max: 5,
    ss_min: 7,
    ss_max: 10,
    star_min: 25,
    star_max: 36,
};
// Assuming 32 starters
const K_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 1,
    ss_max: 3,
    star_min: 8,
    star_max: 12,
};
// Assuming 32 starters
const P_LIMITS: DevLimits = DevLimits {
    xf_min: 0,
    xf_max: 0,
    ss_min: 1,
    ss_max: 3,
    star_min: 6,
    star_max: 10,
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
        };

        // Protect rookies who played at least 8 games
        for player in players
            .iter()
            .filter(|player| player.yearsPro <= 1)
            .filter(|player| {
                let games_played = passing_stats
                    .iter()
                    .filter(|stat| stat.player__fullName == player.fullName)
                    .map(|stat| stat.gamesPlayed)
                    .sum::<u8>()
                    .max(
                        rushing_stats
                            .iter()
                            .filter(|stat| stat.player__fullName == player.fullName)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        receiving_stats
                            .iter()
                            .filter(|stat| stat.player__fullName == player.fullName)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        defense_stats
                            .iter()
                            .filter(|stat| stat.player__fullName == player.fullName)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        kicking_stats
                            .iter()
                            .filter(|stat| stat.player__fullName == player.fullName)
                            .map(|stat| stat.gamesPlayed)
                            .sum(),
                    )
                    .max(
                        punting_stats
                            .iter()
                            .filter(|stat| stat.player__fullName == player.fullName)
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
                .find(|new| new.fullName == player.fullName)
                .map(|new| new.devTrait > player.devTrait)
                .unwrap_or(false)
        }) {
            protected_players.insert(player.fullName.clone());
            upgraded_players.insert(player.fullName.clone());
        }

        // Sort players according to their performance this season
        let players = players
            .into_iter()
            .sorted_unstable_by_key(|player| match pos {
                "QB" => (),
                "HB" | "FB" | "WR" | "TE" => (),
                "OL" => (),
                "IDL" | "EDGE" | "LB" | "CB" | "S" => (),
                "K" => (),
                "P" => (),
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
            eprintln!("Free Agents:");
        } else {
            eprintln!("{team}:");
        }
        for ((player, _, pos), (old, new)) in group
            .into_iter()
            .sorted_unstable_by_key(|((name, _, pos), _)| (pos.clone(), name.clone()))
        {
            eprintln!("{pos} {player}: {old:?} -> {new:?}");
        }
        eprintln!();
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
    pub id: u32,
    pub rosterId: u32,
    pub team: String,
    #[serde(deserialize_with = "from_str_bool")]
    pub isRetired: bool,
    pub age: u8,
    pub fullName: String,
    #[serde(deserialize_with = "from_str_bool")]
    pub isActive: bool,
    #[serde(deserialize_with = "from_str_bool")]
    pub isFreeAgent: bool,
    #[serde(deserialize_with = "from_str_bool")]
    pub isOnIR: bool,
    #[serde(deserialize_with = "from_str_bool")]
    pub isOnPracticeSquad: bool,
    pub position: String,
    pub playerBestOvr: u8,
    pub playerSchemeOvr: u8,
    pub scheme: u8,
    pub teamSchemeOvr: u32,
    pub weight: u32,
    pub yearsPro: u8,
    pub devTrait: u8,
}

#[derive(Deserialize)]
struct PassingData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
    pub gamesPlayed: u8,
    pub fantasy_points: f32,
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
struct ReceivingData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
    pub gamesPlayed: u8,
    pub fantasy_points: f32,
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
struct RushingData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
    pub gamesPlayed: u8,
    pub fantasy_points: f32,
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
struct DefenseData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
    pub gamesPlayed: u8,
    pub fantasy_points: f32,
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
struct KickingData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
    pub gamesPlayed: u8,
    pub fantasy_points: f32,
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
struct PuntingData {
    pub team__displayName: String,
    pub team__abbrName: String,
    pub player__rosterId: u32,
    pub player__fullName: String,
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