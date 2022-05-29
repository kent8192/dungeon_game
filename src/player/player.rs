use num::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
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
    /// プレイヤーの状態(mut)を取得
    pub fn get_mut_condition(&mut self) -> &mut u16 {
        let val = &mut self.condition;
        return val;
    }
    /// プレイヤーの状態(imut)を取得
    pub fn get_imut_condition(&self) -> &u16 {
        let val = &self.condition;
        return val;
    }
}
/// 状態異常
#[derive(Clone, Copy, Debug, EnumIter, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
impl FromPrimitive for StatusEffect {
    fn from_u16(n: u16) -> Option<Self> {
        let checker = StatusEffect::iter();
        let result: Vec<StatusEffect> = checker.filter(|e| *e as u16 == n).collect();
        if result.len() == 1 {
            return Option::Some(result[0]);
        } else {
            return Option::None;
        }
    }
    fn from_i64(n: i64) -> Option<Self> {
        let checker = StatusEffect::iter();
        let result: Vec<StatusEffect> = checker.filter(|e| *e as u16 == n as u16).collect();
        if result.len() == 1 {
            return Option::Some(result[0]);
        } else {
            return Option::None;
        }
    }
    fn from_u64(n: u64) -> Option<Self> {
        let checker = StatusEffect::iter();
        let result: Vec<StatusEffect> = checker.filter(|e| *e as u16 == n as u16).collect();
        if result.len() == 1 {
            return Option::Some(result[0]);
        } else {
            return Option::None;
        }
    }
}

impl StatusEffect {
    /// 状態異常(単数)を付与
    pub fn attach_status_effect(target: &mut u16, cond: StatusEffect) {
        *target |= cond as u16;
    }
    /// 状態異常(複数)を付与
    pub fn attach_status_effect_list(target: &mut u16, conds: Vec<StatusEffect>) {
        for cond in conds {
            StatusEffect::attach_status_effect(target, cond);
        }
    }

    /// 状態異常(単数)を回復
    pub fn remove_status_effect(target: &mut u16, cond: StatusEffect) {
        *target &= !(cond as u16);
    }
    /// 状態異常(複数)を回復
    pub fn remove_status_effect_list(target: &mut u16, conds: Vec<StatusEffect>) {
        for cond in conds {
            StatusEffect::remove_status_effect(target, cond);
        }
    }

    /// 状態異常(単数)か確認
    pub fn is_status_effect(target: &u16, cond: StatusEffect) -> bool {
        let judge = target & (cond as u16);
        return judge == cond as u16;
    }

    /// 状態異常(複数)か確認
    pub fn is_status_effect_list(
        target: &u16,
        conds: Vec<StatusEffect>,
    ) -> HashMap<StatusEffect, bool> {
        let mut map: HashMap<StatusEffect, bool> = HashMap::new();
        for cond in conds {
            let judge = StatusEffect::is_status_effect(target, cond);
            map.insert(cond, judge);
        }
        return map;
    }

    /// なんの状態異常か確認
    pub fn get_status_effect_list(target: &u16) -> Vec<StatusEffect> {
        let checker = StatusEffect::iter();
        let array: Vec<StatusEffect> = checker
            .filter(|e| StatusEffect::is_status_effect(target, *e))
            .collect();
        return array;
    }
}
