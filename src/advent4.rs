use itertools::{iproduct, Itertools};
use regex::Regex;
use std::{
    fs::read_to_string,
    io::Error,
    ops::{Index, IndexMut},
    slice::RChunks,
};

const TARGET_XMAS: &str = "XMAS";
const TARGET_MAS: &str = "MAS";

fn read_lines_advent1(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| s.chars().collect())
        .collect()
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct BoardIndex {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct BoardIndexOffset {
    x: i32,
    y: i32,
}
impl BoardIndexOffset {
    const fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn negate(&self) -> BoardIndexOffset {
        Self { x: -self.x, y: -self.y }
    }
}

const OFFSETS: [BoardIndexOffset; 8] = [
    BoardIndexOffset::new(1, 0),
    BoardIndexOffset::new(-1, 0),
    BoardIndexOffset::new(0, 1),
    BoardIndexOffset::new(0, -1),
    BoardIndexOffset::new(1, 1),
    BoardIndexOffset::new(-1, 1),
    BoardIndexOffset::new(1, -1),
    BoardIndexOffset::new(-1, -1),
];

const OFFSETS_DIAGONALS: [BoardIndexOffset; 4] = [
    BoardIndexOffset::new(1, 1),
    BoardIndexOffset::new(-1, 1),
    BoardIndexOffset::new(1, -1),
    BoardIndexOffset::new(-1, -1),
];

impl BoardIndex {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y }
    }

    pub fn offset(&self, offset: &BoardIndexOffset) -> Option<BoardIndex> {
        let x_new: i32 = (self.x as i32) + offset.x;
        let y_new: i32 = (self.y as i32) + offset.y;

        if x_new >= 0 && y_new >= 0 {
            Some(Self {
                x: x_new.try_into().unwrap(),
                y: y_new.try_into().unwrap(),
            })
        } else {
            None
        }
    }
}

struct BoardIndexSequence {
    offset: BoardIndexOffset,
    next: Option<BoardIndex>,
    i: u32,
    length: u32,
}

impl BoardIndexSequence {
    fn new(start: BoardIndex, offset: BoardIndexOffset, length: u32) -> BoardIndexSequence {
        Self {
            offset: offset,
            next: Some(start),
            i: 0,
            length: length,
        }
    }
}

impl Iterator for BoardIndexSequence {
    type Item = BoardIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next;

        self.next = if let Some(idx) = n {
            if self.i < self.length - 1 {
                self.i += 1;
                idx.offset(&self.offset)
            } else {
                None
            }
        } else {
            None
        };

        n
    }
}

struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(b: Vec<Vec<char>>) -> Board {
        Self { board: b }
    }
}

impl Index<BoardIndex> for Board {
    type Output = char;

    fn index(&self, index: BoardIndex) -> &Self::Output {
        let y = index.y as usize;
        let x = index.x as usize;

        if self.board.len() - 1 < y || self.board[y].len() - 1 < x {
            &' '
        } else {
            &self.board[index.y as usize][index.x as usize]
        }
    }
}

impl Board {
    fn get_string(&self, index: BoardIndexSequence) -> String {
        index.map(|idx| self[idx]).join("")
    }

    fn get_rows(&self) -> u32 {
        self.board.len() as u32
    }

    fn get_columns(&self) -> u32 {
        self.board[0].len() as u32
    }
}

#[test]
pub fn advent4a_test() {
    let lines = read_lines_advent1("advent4a-test.txt");

    let board = Board::new(lines);

    let rows = 0..board.get_rows();
    let columns = 0..board.get_columns();

    let origins = iproduct!(rows, columns)
        .map(|(row, column)| BoardIndex::new(column, row));

    let count = iproduct!(origins, OFFSETS.into_iter())
        .map(|(origin, offset)| BoardIndexSequence::new(origin, offset, TARGET_XMAS.len() as u32))
        .map(|seq| board.get_string(seq))
        .filter(|value| value == TARGET_XMAS)
        .count();
    
    dbg!(count);
}

#[test]
pub fn advent4a_run() {
    let lines = read_lines_advent1("advent4a-input.txt");

    let board = Board::new(lines);

    let rows = 0..board.get_rows();
    let columns = 0..board.get_columns();

    let origins = iproduct!(rows, columns)
        .map(|(row, column)| BoardIndex::new(column, row));

    let count = iproduct!(origins, OFFSETS.into_iter())
        .map(|(origin, offset)| BoardIndexSequence::new(origin, offset, TARGET_XMAS.len() as u32))
        .map(|seq| board.get_string(seq))
        .filter(|value| value == TARGET_XMAS)
        .count();
    
    dbg!(count);
}

pub fn advent4b(filename: &str) -> usize {
    let lines = read_lines_advent1(filename);

    let board = Board::new(lines);

    let rows = 1..board.get_rows() - 1;
    let columns = 1..board.get_columns() - 1;

    let origins = iproduct!(rows, columns)
        .map(|(row, column)| BoardIndex::new(column, row));

    let count = origins.map(|origin| {
        OFFSETS_DIAGONALS.into_iter()
            .map(|offset| {
                let modified_origin = origin.offset(&offset.negate());

                if let Some(modified_origin) = modified_origin {
                    Some(BoardIndexSequence::new(modified_origin, offset, TARGET_MAS.len() as u32))
                } else {
                    None
                }
            })
            .map(|seq| board.get_string(seq.unwrap()))
            .filter(|value| value == TARGET_MAS)
            .count()
    })
        .filter(|value| *value == 2)
        .count();

    count
}

#[test]
pub fn advent4b_test() {
    let filename = "advent4a-test.txt";

    assert_eq!(9, advent4b(filename));
}

#[test]
pub fn advent4b_run() {
    let filename = "advent4a-input.txt";

    assert_eq!(1864, advent4b(filename));
}


#[test]
pub fn board_load() {
    let lines = read_lines_advent1("advent4a-test.txt");

    dbg!(&lines);

    let board = Board::new(lines);

    let indexer = BoardIndexSequence::new(BoardIndex::new(0, 0), OFFSETS[0], 4);

    assert_eq!("MMMS", board.get_string(indexer));
}

#[test]
fn right_index_generator_produces_result() {
    let origin = BoardIndex::new(0, 0);
    let offset = OFFSETS[0];
    let mut indexer = BoardIndexSequence::new(origin, offset, 4);

    assert_eq!(indexer.next().unwrap(), BoardIndex::new(0, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(1, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(2, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(3, 0));
    assert!(indexer.next().is_none())
}

#[test]
fn left_index_generator_produces_result() {
    let origin = BoardIndex::new(4, 0);
    let offset = BoardIndexOffset::new(-1, 0);
    let mut indexer = BoardIndexSequence::new(origin, offset, 4);

    assert_eq!(indexer.next().unwrap(), BoardIndex::new(4, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(3, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(2, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(1, 0));
    assert!(indexer.next().is_none())
}

#[test]
fn left_index_generator_stops_early_at_0() {
    let origin = BoardIndex::new(2, 0);
    let offset = BoardIndexOffset::new(-1, 0);
    let mut indexer = BoardIndexSequence::new(origin, offset, 4);

    assert_eq!(indexer.next().unwrap(), BoardIndex::new(2, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(1, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(0, 0));
    assert!(indexer.next().is_none())
}

#[test]
fn down_right_index_generator_produces_result() {
    let origin = BoardIndex::new(0, 0);
    let offset = BoardIndexOffset::new(1, 1);
    let mut indexer = BoardIndexSequence::new(origin, offset, 4);

    assert_eq!(indexer.next().unwrap(), BoardIndex::new(0, 0));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(1, 1));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(2, 2));
    assert_eq!(indexer.next().unwrap(), BoardIndex::new(3, 3));
    assert!(indexer.next().is_none())
}
