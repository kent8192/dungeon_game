use super::floor::Floor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dungeon {
    ///フロアの配列
    floors: Vec<Floor>,
}
