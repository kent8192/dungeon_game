pub mod buttle;
pub mod item;
pub mod scenario;
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    /// 同一フロアで何番目のイベントか
    event_number: i32,
    /// イベントのタイプ
    event_type: EventType,
}
#[derive(Clone, Copy, Debug)]
pub enum EventType {
    //バトル
    Buttle,
    //シナリオ
    Scenario,
    //アイテム
    Item,
}
