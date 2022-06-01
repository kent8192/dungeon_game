use super::event::Event;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Floor {
    ///階数
    floor_number: i32,
    ///総イベント数
    event_amount: u32,
    ///イベントの配列
    events: Vec<Event>,
}

impl Floor {
    pub fn instantiate(_floor_number: i32, _event_amount: u32, _events: Vec<Event>) -> Floor {
        let floor = Floor {
            floor_number: _floor_number,
            event_amount: _event_amount,
            events: _events,
        };
        return floor;
    }
}
