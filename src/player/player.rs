use serde::{Deserialize, Serialize};
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
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
    /// プレイヤーを作成
    /// * `<_name>` - <プレイヤー名>
    /// * `<_condition>` - <状態異常>
    /// * `<_hp>` - <体力>
    /// * `<_mp>` - <魔力>
    /// * `<_atk>` - <攻撃力>
    /// * `<_def>` - <防御力>
    /// * `<_luk>` - <幸運値>
    /// * `<lv>` - <レベル>
    /// * `<exp>` - <経験値>
    pub fn instantiate(
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
        return player;
    }
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
        let json = fs::read_to_string(path).expect("File not found");
        let player: Player = serde_json::from_str(&json).expect("convert filed");
        return player;
    }

    /// プレイヤー名を取得
    pub fn get_name(&self) -> String {
        let val = &self.name;
        return val.to_string();
    }
    /// プレイヤーの状態を取得
    pub fn get_condition(&self) -> &u16 {
        let val = &self.condition;
        return val;
    }

    /// 状態異常(単数)を付与
    pub fn attach_status_effect(&mut self, cond: StatusEffect) {
        self.condition |= cond as u16;
    }
    /// 状態異常(複数)を付与
    pub fn attach_status_effect_list(&mut self, conds: Vec<StatusEffect>) {
        for i in 0..conds.len() {
            self.condition |= conds[i] as u16;
        }
    }

    /// 状態異常(単数)を回復
    pub fn remove_status_effect(&mut self, cond: StatusEffect) {
        self.condition &= !(cond as u16);
    }
    /// 状態異常(複数)を回復
    pub fn remove_status_effect_vec(&mut self, conds: Vec<StatusEffect>) {
        for i in 0..conds.len() {
            self.condition &= !(conds[i] as u16);
        }
    }

    /// 状態異常(単数)か確認
    pub fn is_status_effect(&self, cond: StatusEffect) -> bool {
        let judge = self.condition & (cond as u16);
        return judge == cond as u16;
    }

    /// 状態異常(複数)か確認
    pub fn is_status_effect_vec(&self, conds: Vec<StatusEffect>) -> Vec<bool> {
        let mut array: Vec<bool> = vec![];
        for cond in conds {
            let judge = self.condition & cond as u16;
            array.push(judge == (cond as u16));
        }
        return array;
    }

    /// なんの状態異常か確認
    /// TODO:PlayerのインスタンスがなんのStatusConditionを持っているかVec<StatusCondition>を返す
    pub fn get_status_effect_list(&self) -> Vec<StatusEffect> {
        let checker = StatusEffect::iter();
        let array: Vec<StatusEffect> = checker.filter(|e| self.is_status_effect(*e)).collect();
        return array;
    }
}
/// 状態異常
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum StatusEffect {
    /// 毒
    Poisoned = 0b1,
    /// 麻痺
    Paralyzed = 0b10,
    /// 混乱
    Confused = 0b100,
    /// 気絶
    Stunned = 0b1000,
    /// 凍結
    Frozen = 0b10000,
    /// やけど
    Burning = 0b100000,
    /// 死亡
    Dead = 0b1000000,
    /// 魅了
    Charmed = 0b10000000,
    /// 異常なし
    NoParticular = 0b100000000,
}
