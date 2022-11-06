use std::{thread, time::Duration};

use game_of_life::{Board, Boundary};

fn main() {
    let mut board = Board::new(Boundary {
        height: 30,
        width: 30,
    });
    board.randomize();
    loop {
        println!("{}", board.pretty());
        clear_screen();

        thread::sleep(Duration::from_millis(100));
        board.next_gen();
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
