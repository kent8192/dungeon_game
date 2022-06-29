use serde::{Deserialize, Serialize};
use std::fs;
//　バトルイベント
#[derive(Serialize, Deserialize, Debug)]
pub struct Buttle {
    monsters: [Monster],
    fields: [FieldUnit],
}
// バトルの実装
impl Buttle {
    // モンスターを生成（not pub）
    fn generate_monster(&self) {
        for (field, monster) in (self.fields, self.monsters) {
            field.monster = monster;
        }
    }
    // JSONファイルからバトルイベントを生成
    pub fn from_json_file(path: &str) -> Buttle {
        let json = fs::read_to_string(path).expect("File not found");
        let buttle: Buttle = serde_json::from_str(&json).expect("convert filed");
        return buttle;
    }
}
//バトルフィールド
#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    units: [FieldUnit],
}
// バトルフィールドのユニット
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldUnit {
    monster: Monster,
}
// バトルフィールドのユニットの実装
impl FieldUnit {
    // モンスターを割当
    pub fn generate_field_unit(_monster: Monster) {
        monster = _monster;
    }
}
