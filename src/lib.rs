pub fn hello() {
    println!("Hello World");
}

#[test]
#[rustfmt::skip]
fn test_die() {
    let board_initial = Board::new([
      [0, 0, 0],
      [0, 1, 0],
      [0, 0, 0],
    ]);
    let board_expected = Board::new([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);

    let board_actual = board_initial.next_gen();

    assert_eq!(board_expected, board_actual);
}
