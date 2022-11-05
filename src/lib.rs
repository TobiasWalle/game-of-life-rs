use std::collections::HashMap;

struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<Position, Cell>
}

#[test]
fn test_scenario_1() {
    let grid_initial = Grid::from([
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 0],
    ]);
    let grid_expected = Grid::from([
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ]);

    let grid_actual = grid_initial.simulate_step();

    assert_eq!(grid_actual, grid_expected);
}
