use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monster {
    //ID
    id: String,
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
impl Monster {
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
        _id: String,
        _name: &str,
        _condition: u16,
        _hp: u32,
        _mp: u32,
        _atk: u32,
        _def: u32,
        _luk: u32,
        _lv: u32,
        _exp: u32,
    ) -> Monster {
        let monster = Monster {
            id: _id,
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
        return monster;
    }
}
