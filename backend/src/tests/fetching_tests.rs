#![allow(dead_code)]
use crate::models::datamodels::{Entry, XmlTag};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
#[test]
/// Tests our validate_entries function using consistent mock data.
pub fn test_validate_entries() {
    use crate::models::datamodels::SpRanked;
    use crate::stages::fetching::validate_entries;
    use std::collections::HashMap;

    let banned_users: Vec<String> = example("banned");
    // Construct some fake data
    let entries: Vec<Entry> = sp_entries();
    let data: XmlTag<Vec<Entry>> = XmlTag { value: entries };
    let map_json: Vec<SpRanked> = example("sp_ranked");
    let worst_score = map_json[map_json.len() - 1].map_data.score;
    let existing_hash: HashMap<String, (i32, i32)> = map_json
        .into_iter()
        .map(|rank: SpRanked| {
            (
                rank.map_data.profile_number.clone(),
                (rank.map_data.score, rank.rank),
            )
        })
        .collect();
    let (current_rank, not_cheated) =
        validate_entries(&data, existing_hash, banned_users, 47802, worst_score);
    assert_eq!(current_rank["76561198029488151"], 65);
    assert_eq!(current_rank["76561198039230536"], 3);
    assert_eq!(
        not_cheated[0].profile_number,
        "76561198039230536".to_string()
    );
    assert_eq!(
        not_cheated[1].profile_number,
        "76561198029488151".to_string()
    );
    assert_eq!(not_cheated[0].score, 1720);
    assert_eq!(not_cheated[1].score, 2100);
}

/// Deserializses a generic T into a vec of entries that implement Deserialize.
/// Used for parsing mock JSON data located in the path referenced below.
pub fn example<T: for<'de> serde::Deserialize<'de>>(id: &str) -> Vec<T> {
    let path_str = format!("./src/tests/examples/{}_example.json", id);
    let path = Path::new(&path_str);
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let res: Vec<T> = serde_json::from_str(&buff).unwrap();
    res
}

/// Constructs mock examples to use to test our validate_entries function works as intended.
pub fn sp_entries() -> Vec<Entry> {
    vec![
        // Xinera worse time (assume there's a manual submission)
        Entry {
            steam_id: XmlTag {
                value: "76561198054297641".to_string(),
            },
            score: XmlTag { value: 1858 },
        },
        // Daniel, same time
        Entry {
            steam_id: XmlTag {
                value: "76561198040982247".to_string(),
            },
            score: XmlTag { value: 1816 },
        },
        // Better score for Zypeh (beat wr)
        Entry {
            steam_id: XmlTag {
                value: "76561198039230536".to_string(),
            },
            score: XmlTag { value: 1720 },
        },
        // Better score for BreweryJake
        Entry {
            steam_id: XmlTag {
                value: "76561198029488151".to_string(),
            },
            score: XmlTag { value: 2100 },
        },
        // Same score for Mel
        Entry {
            steam_id: XmlTag {
                value: "76561198135023038".to_string(),
            },
            score: XmlTag { value: 2206 },
        },
    ]
}
