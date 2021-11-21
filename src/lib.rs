use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};
use std::iter::{Enumerate, Map};
use std::slice::Iter;

/// Must be implemented by the primary type used for the data stored in a game territory.
pub trait BoardSpot {
    fn svg(&self) -> String;
}

/// Represent a single territory on the map
/// T is a game-specific type for whatever can be stored on a given territory
pub struct MapTerritory<T>
where T: Clone + BoardSpot + Default {
    name: String,  // long name to display when there's room
    name_short: String, // abbreviated name for when there's less room
    connections_to: Vec<usize>, // where we can go from here, indexes into GameBoard::territories
    connections_to_secondary: Vec<usize>, // Alternative connections, allows games to be more flexible
    current: T,  // What is the state of this territory right now
    // The following are for Risk-like games, but other games can use this data as they see fit
    auto_deploy: Option<i16>, // do we automatically put additional troops here if held at beginning of turn?
    init_neutral: Option<i16>, // will this start as neutral instead of being dealt out, if so with how many armies?
    revert_neutral: Option<i16>, // if held at beginning of turn, revert this this number neutral armies
}

impl<T: Clone + BoardSpot + Default> MapTerritory<T> {
    /// Set basic info for a territory
    pub fn territory_builder(name: String, name_short: String) -> MapTerritory<T> {
        MapTerritory {
            name,
            name_short,
            connections_to: Vec::new(),
            connections_to_secondary: Vec::new(),
            current: Default::default(),
            auto_deploy: None,
            init_neutral: None,
            revert_neutral: None
        }
    }

    pub fn get_short_name(&self) -> &String {
        &self.name_short
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

/// In Risk-like games, a bonus is given if all territories in a bonus are held. This allows that or
/// some flexibility, in that you can get partial credit for some regions.
pub enum RegionBonusType {
    BonusForAll(i16),  // Only bonus if all territories held
    GradiatedBonus(Vec<i16>) // Grant [x] bonus if x territories in region held
}

/// Define a bonus region, which a player receives a bonus for holding all or part of.
pub struct BonusRegion {
    name: String,
    name_short: String,
    bonus_type: RegionBonusType,
    territories: Vec<usize>  // Indexes into GameBoard::territories
}

/// All information for a board on which a game is played
pub struct GameBoard<T>
where T: Clone + BoardSpot + Default {
    _territories: Vec<MapTerritory<T>>,
    bonus_regions: Vec<BonusRegion>
}

impl<T: Clone + BoardSpot + Default> GameBoard<T> {
    pub fn load_from_file(filename: String) -> Result<GameBoard<T>, String> {
        let file = File::open(filename);
        let mut file = match file {
            Ok(f) => f,
            Err(e) => return Err("Can't open file".to_string())
        };
        let mut raw_yaml = String::new();
        let bytes_read = file.read_to_string(&mut raw_yaml).unwrap();
        let yaml_tree = yaml_rust::YamlLoader::load_from_str(&raw_yaml);
        let yaml_tree = match yaml_tree {
            Ok(ref t) => &t[0],  // first document in vec is the tree, I don't get why this is a vec at all.
            Err(e) => return Err("Invalid YAML tree".to_string())
        };

        let top_hash = match yaml_tree.as_hash() {
            Some(h) => h,
            None => return Err("Top YAML tree is not a hash.".to_string())
        };

        let mut board = GameBoard::<T> {
            _territories: vec![],
            bonus_regions: vec![]
        };

        // Loop through territories
        let territory_it = top_hash[&Yaml::from_str("territories")].as_vec();
        let territory_it = match territory_it {
            Some(v) => v,
            None => return Err("'territories' does not contain a list".to_string())
        };

        // first time through to build a hashmap of shortnames to index IDs
        let mut name_map : HashMap::<String, usize> = HashMap::with_capacity(8192);
        for (i, t) in territory_it.iter().enumerate() {
            let tert_hash = match t.as_hash() {
                Some(h) => h,
                None => return Err("'territory' item is not a dictionary".to_string())
            };
            let name = match tert_hash[&Yaml::from_str("name")].as_str() {
                Some(s) => s.clone(),
                None => return Err("Territory has no name.".to_string())
            };
            name_map.insert(name.to_string(),i);
        }

        // now build the actual map
        for t in territory_it.iter() {
            let tert_hash = match t.as_hash() {
                Some(h) => h,
                None => return Err("'territory' item is not a dictionary".to_string())
            };
            let name = match tert_hash[&Yaml::from_str("name")].as_str() {
                Some(s) => s.clone().to_string(),
                None => return Err("Territory has no name.".to_string())
            };
            let full_name = match tert_hash[&Yaml::from_str("fullName")].as_str() {
                Some(s) => s.clone().to_string(),
                None => name.clone()
            };
            let mut new_tert: MapTerritory<T> = MapTerritory::<T>::territory_builder(full_name, name);
            let can_attack = match tert_hash[&Yaml::from_str("canAttack")].as_vec() {
                Some(v) => v,
                None => return Err("canAttack must be a list".to_string())
            };
            for attack in can_attack.iter() {
                let attack_str = match attack.as_str() {
                    Some(s) => s.clone(),
                    None => return Err("Something in 'canAttack' is not a string".to_string())
                };
                let i = name_map.get(attack_str);
                match i {
                    Some(ii) => new_tert.connections_to.push(*ii),
                    None => return Err(format!("Not a short name: {}", attack_str))
                };
            }

            board._territories.push(new_tert);
        }

        // Loop through bonus regions
        let region_it = top_hash[&Yaml::from_str("regions")].as_vec();
        let region_it = match region_it {
            Some(v) => v,
            None => return Err("'regions' does not contain a list".to_string())
        };

        for r in region_it.iter() {
            let reg_hash = match r.as_hash() {
                Some(h) => h,
                None => return Err("'territory' item is not a dictionary".to_string())
            };
            let name = match reg_hash[&Yaml::from_str("name")].as_str() {
                Some(s) => s.clone().to_string(),
                None => return Err("Region has no name.".to_string())
            };
            let full_name = match reg_hash[&Yaml::from_str("fullName")].as_str() {
                Some(s) => s.clone().to_string(),
                None => name.clone()
            };

            let bonus_for_all = match reg_hash[&Yaml::from_str("bonusForAll")].as_i64() {
                Some(i) => i as i16,
                None => 0
            };

            let mut new_region = BonusRegion {
                name: full_name,
                name_short: name,
                territories: vec![],
                bonus_type: RegionBonusType::BonusForAll(bonus_for_all)  // todo gradiated type
            };

            let territory_names = match reg_hash[&Yaml::from_str("territories")].as_vec() {
                Some(v) => v,
                None => return Err("territories must be a list".to_string())
            };
            for t in territory_names.iter() {
                let attack_str = match t.as_str() {
                    Some(s) => s.clone(),
                    None => return Err("Something in 'territories' is not a string".to_string())
                };
                let i = name_map.get(attack_str);
                match i {
                    Some(ii) => new_region.territories.push(*ii),
                    None => return Err(format!("Not a short name: {}", attack_str))
                };
            }

            board.bonus_regions.push(new_region);
        }

        Ok(board)
    }

    pub fn territories(&self) -> Iter<MapTerritory<T>> {
        self._territories.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::BoardSpot;

    #[derive(Clone, Default, PartialEq, Debug)]
    struct IntThing {
        thing: i32
    }

    impl BoardSpot for IntThing {
        fn svg(&self) -> String {
            return "<svg></svg>".to_string();
        }
    }
    fn setup() {

    }
    #[test]
    fn default()
    {
        let ithing = IntThing { ..Default::default()};
        assert_eq!(ithing, IntThing{thing: 0});
    }
}
