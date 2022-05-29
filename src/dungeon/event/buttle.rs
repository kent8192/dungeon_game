use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
pub struct Buttle {
    monsters: [Monster],
    fields: [FieldUnit],
}
impl Buttle {
    pub fn generate_monster(&self) {
        for (field, monster) in (self.fields, self.monsters) {
            field.monster = monster;
        }
    }
    pub fn from_json_file(path: &str) -> Buttle {
        let json = fs::read_to_string(path).expect("File not found");
        let buttle: Buttle = serde_json::from_str(&json).expect("convert filed");
        return buttle;
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Field;
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldUnit {
    monster: Monster,
}
impl FieldUnit {
    pub fn generate_field_unit(&self) {}
}
