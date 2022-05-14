use serde::{Deserialize, Serialize};
use std::fs::{self, File};
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    //名前
    name: String,
    //状態異常
    condition: u16,
    //体力
    hp: u32,
    //魔力
    mp: u32,
    //攻撃力
    atk: u32,
    //防御力
    def: u32,
    //幸運値
    luk: u32,
    //レベル
    lv: u32,
    //経験値
    exp: u32,
}

impl Player {
    /// プレイヤーを作成&ファイルに記録
    /// * `<_datapath>` - <セーブデータファイルのパス>
    /// * `<_name>` - <プレイヤー名>
    /// * `<_condition>` - <状態異常>
    /// * `<_hp>` - <体力>
    /// * `<_mp>` - <魔力>
    /// * `<_atk>` - <攻撃力>
    /// * `<_def>` - <防御力>
    /// * `<_luk>` - <幸運値>
    /// * `<lv>` - <レベル>
    /// * `<exp>` - <経験値>
    pub fn instantiate_and_record(
        _datapath: &str,
        _name: &str,
        _condition: u16,
        _hp: u32,
        _mp: u32,
        _atk: u32,
        _def: u32,
        _luk: u32,
        _lv: u32,
        _exp: u32,
    ) -> Player {
        let player = Player {
            name: String::from(_name),
            condition: _condition,
            hp: _hp,
            mp: _mp,
            atk: _atk,
            def: _def,
            luk: _luk,
            lv: _lv,
            exp: _exp,
        };
        let json = serde_json::to_string(&player).unwrap();
        fs::write(_datapath, json).unwrap();
        return player;
    }
    ///プレイヤーをJSONファイルから作成
    /// * `<path>` - <JSOファイルのパス>
    pub fn from_json_file(path: &str) -> Player {
        let f = fs::read_to_string(path).expect("File not found");
        let player: Player = serde_json::from_str(&f).expect("convert filed");
        return player;
    }
}

enum StatusCondition {
    Poisoned = 0b1,
    Paralyzed = 0b10,
    Confused = 0b100,
    Stunned = 0b1000,
    Frozen = 0b10000,
    Burning = 0b100000,
    Dead = 0b1000000,
    Charmed = 0b10000000,
}
