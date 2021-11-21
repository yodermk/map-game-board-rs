use map_game_board_rs::*;
use std::env::args;

#[derive(Copy, Clone, Default)]
struct TestType {
    x: i8
}

impl BoardSpot for TestType {
    fn svg(&self) -> String {
        return "<svg></svg>".to_string();
    }
}

fn main() {
    let filename = args().skip(1).next()
        .expect("Please pass a map file name as the command line argument.");

    let board :GameBoard<TestType> = GameBoard::load_from_file(filename)
        .expect("Can't load game board from that file.");

    for t in board.territories() {
        println!("{} -> {}", t.get_short_name(), t.get_name())
    }
}
