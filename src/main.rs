mod player;
use self::player::player::Player;
fn main() {
    let player = Player::instantiate_and_record(
        "/asset/player_data.json",
        "my player",
        0,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
    );
}
