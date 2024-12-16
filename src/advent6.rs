use itertools::{iproduct, Itertools};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::read_to_string,
    io::Error,
    ops::{Index, IndexMut},
    slice::RChunks,
};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_offset(&self) -> Position {
        match self {
            Direction::Up => Position::new_i32(0, -1),
            Direction::Down => Position::new_i32(0, 1),
            Direction::Left => Position::new_i32(-1, 0),
            Direction::Right => Position::new_i32(1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MazeCell {
    Open,
    Blocked,
    Guard(Direction),
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }

    fn new_i32(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    fn add(&self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct GuardState {
    pub position: Position,
    pub facing: Direction,
}

impl MazeCell {
    fn from_char(c: char) -> MazeCell {
        match c {
            '.' => MazeCell::Open,
            '#' => MazeCell::Blocked,
            '^' => MazeCell::Guard(Direction::Up),
            'v' => MazeCell::Guard(Direction::Down),
            '>' => MazeCell::Guard(Direction::Right),
            '<' => MazeCell::Guard(Direction::Left),
            _ => {
                panic!("Unknown maze cell character: {}", c)
            }
        }
    }
}

type Maze = Vec<Vec<MazeCell>>;

fn is_position_valid(m: &Maze, p: &Position) -> bool {
    if p.x < 0 || p.y < 0 || p.y as usize >= m.len() || p.x as usize >= m[0].len() {
        false
    } else {
        true
    }
}

fn read_maze_lines(filename: &str) -> Maze {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| s.chars().into_iter().map(MazeCell::from_char).collect())
        .collect()
}

fn find_guard(m: &Maze) -> GuardState {
    for (y, row) in m.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if let MazeCell::Guard(d) = cell {
                return GuardState {
                    position: Position::new(x, y),
                    facing: *d,
                };
            }
        }
    }

    panic!("Guard not found")
}

fn move_guard_single(m: &mut Maze, guard: &GuardState) -> Option<GuardState> {
    if !is_position_valid(m, &guard.position) {
        //Guard is already off the board, they cannot move further
        return None;
    }

    let offset = guard.facing.move_offset();

    let provisional_position = guard.position.add(offset);

    if !is_position_valid(m, &provisional_position) {
        //Position is off the board, return it.
        return Some(GuardState {
            position: provisional_position,
            facing: guard.facing,
        });
    }

    match m[provisional_position.y as usize][provisional_position.x as usize] {
        MazeCell::Open | MazeCell::Guard(_) => Some(GuardState {
            position: provisional_position,
            facing: guard.facing,
        }),
        MazeCell::Blocked => None,
    }
}

fn move_guard_until_stop(
    m: &mut Maze,
    guard_states: &mut Vec<GuardState>,
) -> (Vec<GuardState>, u32) {
    let mut moves = 0;

    while let Some(new_state) = move_guard_single(m, &guard_states.last().unwrap()) {
        if !guard_states
            .into_iter()
            .any(|s| s.position == new_state.position)
        {
            moves += 1;
        }

        guard_states.push(new_state);
    }

    (guard_states.to_vec(), moves)
}

fn useful_barrier_location(
    m: &mut Maze,
    guard_states: &Vec<GuardState>,
    extrapolated_states: &Vec<GuardState>,
    new_state: &GuardState,
) -> Option<Position> {
    //Evaluate the position to the right for the presence of a previous position, facing rotated 90 from current
    //If so, a barrier directly ahead would cause a loop
    let rotated_facing = new_state.facing.rotate();
    let possible_previous = GuardState {
        position: new_state.position.add(rotated_facing.move_offset()),
        facing: rotated_facing,
    };

    if is_position_valid(m, &possible_previous.position)
        && (guard_states.contains(&possible_previous) || extrapolated_states.contains(&possible_previous))
    {
        //In this case, see if we can place a barrier ahead of ourselves
        let barrier_position = new_state.position.add(new_state.facing.move_offset());

        if is_position_valid(m, &barrier_position) {
            return Some(barrier_position);
        }
    }

    return None;
}

fn extrapolate_backwards(m: &mut Maze, guard_states: &Vec<GuardState>, extrapolated_states: &Vec<GuardState>) -> Vec<GuardState> {
    //Extrapolate the newest state backwards until there is a barrier.  Note that the facing must remain the same.
    let last_state = guard_states.last().unwrap();
    let offset = last_state.facing.move_offset();
    let reverse_offset = Position::new_i32(-offset.x, -offset.y);
    let mut states: Vec<GuardState> = vec![];

    let mut potential_position = last_state.position.add(reverse_offset);

    while is_position_valid(m, &potential_position) {
        match m[potential_position.y as usize][potential_position.x as usize] {
            MazeCell::Blocked => {
                break;
            }
            _ => {
                let extrapolated_state = GuardState {
                    position: potential_position,
                    facing: last_state.facing,
                };

                if !guard_states.contains(&extrapolated_state) && !extrapolated_states.contains(&extrapolated_state) {
                    states.push(extrapolated_state);
                } else {
                    break;
                }

                potential_position = potential_position.add(reverse_offset);
            }
        }
    }

    states
}

fn move_guard_until_exit(m: &mut Maze, guard: GuardState) -> (Vec<GuardState>, u32, Vec<Position>) {
    let mut barrier_locations: Vec<Position> = vec![];
    let mut guard_states = vec![guard];
    let mut moves = 0;

    let mut extrapolated_states: Vec<GuardState> = vec![];

    extrapolated_states.append(&mut extrapolate_backwards(m, &guard_states, &extrapolated_states));

    while is_position_valid(m, &guard_states.last().unwrap().position) {
        while let Some(new_state) = move_guard_single(m, &guard_states.last().unwrap()) {
            if !guard_states
                .clone()
                .into_iter()
                .any(|s| s.position == new_state.position)
            {
                moves += 1;
            }

            if let Some(barrier_position) = useful_barrier_location(m, &guard_states, &extrapolated_states, &new_state) {
                barrier_locations.push(barrier_position);
            }

            guard_states.push(new_state);
        }

        if let Some(last_state) = guard_states.last() {
            if is_position_valid(m, &guard_states.last().unwrap().position) {
                let rotated_state = GuardState {
                    position: last_state.position,
                    facing: last_state.facing.rotate(),
                };

                if let Some(barrier_position) = useful_barrier_location(m, &guard_states, &extrapolated_states, &rotated_state) {
                    barrier_locations.push(barrier_position);
                }

                guard_states.push(rotated_state);

                extrapolated_states.append(&mut extrapolate_backwards(m, &guard_states, &extrapolated_states));
            }
        } else {
            panic!("No guard positions");
        }
    }

    (guard_states, moves, barrier_locations)
}

#[test]
fn input_move_guard_to_exit() {
    let mut maze = read_maze_lines("advent6a-input.txt");

    let guard_state = find_guard(&maze);

    let (guard_states, moves, barrier_locations) = move_guard_until_exit(&mut maze, guard_state);

    let terminal_state = guard_states.last().unwrap();

    assert_eq!(508, barrier_locations.len());
    assert_eq!(5329, moves);
    assert_eq!(130, terminal_state.position.y);
    assert_eq!(9, terminal_state.position.x);
    assert_eq!(Direction::Down, terminal_state.facing);
}

#[test]
fn test_move_guard_to_exit() {
    let mut maze = read_maze_lines("advent6a-test.txt");

    let guard_state = find_guard(&maze);

    let (guard_states, moves, barrier_locations) = move_guard_until_exit(&mut maze, guard_state);

    let terminal_state = guard_states.last().unwrap();

    assert_eq!(41, moves);
    assert_eq!(10, terminal_state.position.y);
    assert_eq!(7, terminal_state.position.x);
    assert_eq!(Direction::Down, terminal_state.facing);

    assert!(barrier_locations.contains(&Position::new(3, 6)));
    assert!(barrier_locations.contains(&Position::new(6, 7)));
    assert!(barrier_locations.contains(&Position::new(7, 7)));
    assert!(barrier_locations.contains(&Position::new(1, 8))); // Failed
    assert!(barrier_locations.contains(&Position::new(3, 8))); // Failed
    assert!(barrier_locations.contains(&Position::new(7, 9)));

    assert_eq!(6, barrier_locations.len());
}

#[test]
fn test_move_guard_to_stop_exit() {
    let mut maze = read_maze_lines("advent6a-test.txt");

    let guard_state = GuardState {
        position: Position::new_i32(3, 2),
        facing: Direction::Up,
    };

    let (guard_states, moves, barrier_locations) = move_guard_until_exit(&mut maze, guard_state);

    let terminal_state = guard_states.last().unwrap();

    assert_eq!(3, moves);
    assert_eq!(-1, terminal_state.position.y);
    assert_eq!(3, terminal_state.position.x);
    assert_eq!(Direction::Up, terminal_state.facing);
}

#[test]
fn test_move_guard_to_stop() {
    let mut maze = read_maze_lines("advent6a-test.txt");

    let guard_state = find_guard(&maze);

    let (guard_states, moves, barrier_locations) = move_guard_until_exit(&mut maze, guard_state);

    let terminal_state = guard_states.last().unwrap();

    assert_eq!(5, moves);
    assert_eq!(1, terminal_state.position.y);
    assert_eq!(4, terminal_state.position.x);
    assert_eq!(Direction::Up, terminal_state.facing);
}

#[test]
fn test_move_guard_single() {
    let mut maze = read_maze_lines("advent6a-test.txt");

    let guard_state = find_guard(&maze);

    let guard_state = move_guard_single(&mut maze, &guard_state);

    if let (Some(guard_state)) = guard_state {
        assert_eq!(5, guard_state.position.y);
        assert_eq!(4, guard_state.position.x);
        assert_eq!(Direction::Up, guard_state.facing);
    } else {
        assert!(false);
    }
}

#[test]
fn test_find_guard() {
    let maze = read_maze_lines("advent6a-test.txt");

    let guard_state = find_guard(&maze);

    assert_eq!(6, guard_state.position.y);
    assert_eq!(4, guard_state.position.x);
    assert_eq!(Direction::Up, guard_state.facing);
}

#[test]
fn read_test_maze_lines() {
    let maze = read_maze_lines("advent6a-test.txt");

    assert_eq!(10, maze.len());
    assert_eq!(10, maze[0].len());
    assert_eq!(MazeCell::Guard(Direction::Up), maze[6][4]);
    assert_eq!(MazeCell::Open, maze[2][2]);
    assert_eq!(MazeCell::Blocked, maze[0][4]);
}
