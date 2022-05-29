#[derive(Serialize, Deserialize, Debug)]
pub struct dungeon {
    ///フロアの配列
    floors: [floor],
}
