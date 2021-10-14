
use counter::Counter;
use regex::Regex;
use lazy_static::lazy_static;
use std::{cmp::max, collections::{HashMap, VecDeque}, fmt, fmt::Formatter, hash::Hash, intrinsics::{bitreverse, log2f64}, mem::MaybeUninit, ops::{self, BitOr, RangeBounds}, path::Iter, process::Output, result};


pub const FILE_NAMES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_NAMES: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const STARTING_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub type Color = bool;

pub const WHITE: bool = true;
pub const BLACK: bool = false;
pub const COLORS: [bool; 2] = [WHITE, BLACK];

type PieceType = u8;
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;
pub const PIECE_TYPES: [u8; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];
pub const PIECE_SYMBOLS: [Option<char>; 7] = [
    None,
    Some('p'),
    Some('n'),
    Some('b'),
    Some('r'),
    Some('q'),
    Some('k'),
];
lazy_static! {
    pub static ref PIECE_NAMES: [Option<&'static str>; 7] = {
        let names = [
            None,
            Some("pawn"),
            Some("knight"),
            Some("bishop"),
            Some("rook"),
            Some("queen"),
            Some("king"),
        ];
        names
    };
}
fn piece_symbol(piece_type: PieceType) -> Option<char> {
    PIECE_SYMBOLS[piece_type as usize]
}
fn piece_name(piece_name: PieceType) -> Option<&'static str> {
    PIECE_NAMES[piece_name as usize]
}
fn piece_type(piece_symbol: Option<char>) -> Option<u8> {
    match piece_symbol {
        Some('p') => Some(PAWN),
        Some('n') => Some(KNIGHT),
        Some('b') => Some(BISHOP),
        Some('r') => Some(ROOK),
        Some('q') => Some(QUEEN),
        Some('k') => Some(KING),
        _ => None,
    }
}
pub trait Boolean {
    fn bool(&self) -> bool;
}
#[derive(PartialEq)]
pub enum Status {
    VALID = 0,
    NO_WHITE_KING = 1 << 0,
    NO_BLACK_KING = 1 << 1,
    TOO_MANY_KINGS = 1 << 2,
    TOO_MANY_WHITE_PAWNS = 1 << 3,
    TOO_MANY_BLACK_PAWNS = 1 << 4,
    PAWNS_ON_BACKRANK = 1 << 5,
    TOO_MANY_WHITE_PIECES = 1 << 6,
    TOO_MANY_BLACK_PIECES = 1 << 7,
    BAD_CASTLING_RIGHTS = 1 << 8,
    INVALID_EP_SQUARE = 1 << 9,
    OPPOSITE_CHECK = 1 << 10,
    EMPTY = 1 << 11,
    RACE_CHECK = 1 << 12,
    RACE_OVER = 1 << 13,
    RACE_MATERIAL = 1 << 14,
    TOO_MANY_CHECKERS = 1 << 15,
    IMPOSSIBLE_CHECK = 1 << 16,
    INVALID_STATUS = 1 << 17
}
impl Status {
    fn to_enum(val: u32) -> Status{
        match val {
            0x00 => Status::VALID,
            0x01 => Status::NO_WHITE_KING,
            0x02 => Status::NO_BLACK_KING,
            0x04 => Status::TOO_MANY_KINGS,
            0x08 => Status::TOO_MANY_WHITE_PAWNS,
            0x10 => Status::TOO_MANY_BLACK_PAWNS,
            0x20 => Status::PAWNS_ON_BACKRANK,
            0x40 => Status::TOO_MANY_WHITE_PIECES,
            0x80 => Status::TOO_MANY_BLACK_PIECES,
            0x100 => Status::BAD_CASTLING_RIGHTS,
            0x200 => Status::INVALID_EP_SQUARE,
            0x400 => Status::OPPOSITE_CHECK,
            0x800 => Status::EMPTY,
            0x1000 => Status::RACE_CHECK,
            0x2000 => Status::RACE_OVER,
            0x4000 => Status::RACE_MATERIAL,
            0x8000 => Status::TOO_MANY_CHECKERS,
            0x10000 => Status::IMPOSSIBLE_CHECK,
            _ => Status::INVALID_STATUS
        }
    }
}
pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;
pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;
pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;
pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;
pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;
pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;
pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;
pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

pub const STATUS_VALID: u32 = Status::VALID as u32;
pub const STATUS_NO_WHITE_KING: u32 = Status::NO_WHITE_KING as u32;
pub const STATUS_NO_BLACK_KING: u32 = Status::NO_BLACK_KING as u32;
pub const STATUS_TOO_MANY_KINGS: u32 = Status::TOO_MANY_KINGS as u32;
pub const STATUS_TOO_MANY_WHITE_PAWNS: u32 = Status::TOO_MANY_WHITE_PAWNS as u32;
pub const STATUS_TOO_MANY_BLACK_PAWNS: u32 = Status::TOO_MANY_BLACK_PAWNS as u32;
pub const STATUS_PAWNS_ON_BACKRANK: u32 = Status::PAWNS_ON_BACKRANK as u32;
pub const STATUS_TOO_MANY_WHITE_PIECES: u32 = Status::TOO_MANY_WHITE_PIECES as u32;
pub const STATUS_TOO_MANY_BLACK_PIECES: u32 = Status::TOO_MANY_BLACK_PIECES as u32;
pub const STATUS_BAD_CASTLING_RIGHTS: u32 = Status::BAD_CASTLING_RIGHTS as u32;
pub const STATUS_INVALID_EP_SQUARE: u32 = Status::INVALID_EP_SQUARE as u32;
pub const STATUS_OPPOSITE_CHECK: u32 = Status::OPPOSITE_CHECK as u32;
pub const STATUS_EMPTY: u32 = Status::EMPTY as u32;
pub const STATUS_RACE_CHECK: u32 = Status::RACE_CHECK as u32;
pub const STATUS_RACE_OVER: u32 = Status::RACE_OVER as u32;
pub const STATUS_RACE_MATERIAL: u32 = Status::RACE_MATERIAL as u32;
pub const STATUS_TOO_MANY_CHECKERS: u32 = Status::TOO_MANY_CHECKERS as u32;
pub const STATUS_IMPOSSIBLE_CHECK: u32 = Status::IMPOSSIBLE_CHECK as u32;

type Square = u8;

pub fn unicode_piece_symbols(p: char) -> char {
    match p {
        'R' => '♖',
        'r' => '♜',
        'N' => '♘',
        'n' => '♞',
        'B' => '♗',
        'b' => '♝',
        'Q' => '♕',
        'q' => '♛',
        'K' => '♔',
        'k' => '♚',
        'P' => '♙',
        'p' => '♟',
        _ => '\x40',
    }
}
lazy_static! {
    pub static ref SQUARES: [u8; 64] = {
        let squares: [u8; 64] = (0..64)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("wrong sizeg");
        squares
    };
    pub static ref SQUARES_180: [u8; 64] = {
        let mut vec = Vec::new();
        for i in (0..8).rev() {
            vec.extend_from_slice(&SQUARES[i * 8..i * 8 + 8])
        }
        let squares = vec.try_into().expect("Wrong size");
        squares
    };
}
pub const SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];
pub const SQUARE_NAMES_180: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];
pub fn parse_square(name: &str) -> Square {
    let file: i32 = name.chars().nth(0).unwrap() as i32 - 'A' as i32;
    let rank: i32 = name.chars().nth(1).unwrap() as i32 - '1' as i32;
    assert!(file > -1 && file < 8);
    assert!(rank > -1 && rank < 8);
    return (rank * 8 + file) as Square;
}
pub fn square_name(square: Square) -> &'static str {
    SQUARE_NAMES[square as usize]
}
pub fn square(file_index: Square, rank_index: Square) -> Square {
    rank_index * 8 + file_index
}
pub fn square_file(square: Square) -> Square {
    square & 7
}
pub fn square_rank(square: Square) -> Square {
    square >> 3
}
pub fn square_distance(a: Square, b: Square) -> u8 {
    max(
        (square_file(a) as i8 - square_file(b) as i8).abs() as u8,
        (square_rank(a) as i8 - square_rank(b) as i8).abs() as u8,
    )
}
pub fn square_mirror(square: Square) -> Square {
    square ^ 0x38
}
enum Termination {
    CHECKMATE,
    STALEMATE,
    INSUFFICIENT_MATERIAL,
    SEVENTYFIVE_MOVES,
    FIVEFOLD_REPETITION,
    FIFTY_MOVES,
    THREEFOLD_REPETITION,
    VARIANT_WIN,
    VARIANT_LOSS,
    VARIANT_DRAW,
}
pub struct Outcome {
    termination: Termination,
    winner: Option<Color>
}
impl Outcome {
    fn result(&self) -> &str {
        if self.winner == None {"1/2-1/2"} else {if self.winner.unwrap() == WHITE {"1-0"} else {"0-1"}}
    }
}

type Bitboard = u64;
pub const BB_EMPTY: u64 = 0;
pub const BB_ALL: u64 = 0xffff_ffff_ffff_ffff;

lazy_static! {
    pub static ref BB_SQUARES: [u64; 64] = {
        let bb = (0..64)
            .map(|x| 1 << x)
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong sizef");
        bb
    };
}
pub const BB_A1: u64 = 1 << 0;
pub const BB_B1: u64 = 1 << 1;
pub const BB_C1: u64 = 1 << 2;
pub const BB_D1: u64 = 1 << 3;
pub const BB_E1: u64 = 1 << 4;
pub const BB_F1: u64 = 1 << 5;
pub const BB_G1: u64 = 1 << 6;
pub const BB_H1: u64 = 1 << 7;
pub const BB_A2: u64 = 1 << 8;
pub const BB_B2: u64 = 1 << 9;
pub const BB_C2: u64 = 1 << 10;
pub const BB_D2: u64 = 1 << 11;
pub const BB_E2: u64 = 1 << 12;
pub const BB_F2: u64 = 1 << 13;
pub const BB_G2: u64 = 1 << 14;
pub const BB_H2: u64 = 1 << 15;
pub const BB_A3: u64 = 1 << 16;
pub const BB_B3: u64 = 1 << 17;
pub const BB_C3: u64 = 1 << 18;
pub const BB_D3: u64 = 1 << 19;
pub const BB_E3: u64 = 1 << 20;
pub const BB_F3: u64 = 1 << 21;
pub const BB_G3: u64 = 1 << 22;
pub const BB_H3: u64 = 1 << 23;
pub const BB_A4: u64 = 1 << 24;
pub const BB_B4: u64 = 1 << 25;
pub const BB_C4: u64 = 1 << 26;
pub const BB_D4: u64 = 1 << 27;
pub const BB_E4: u64 = 1 << 28;
pub const BB_F4: u64 = 1 << 29;
pub const BB_G4: u64 = 1 << 30;
pub const BB_H4: u64 = 1 << 31;
pub const BB_A5: u64 = 1 << 32;
pub const BB_B5: u64 = 1 << 33;
pub const BB_C5: u64 = 1 << 34;
pub const BB_D5: u64 = 1 << 35;
pub const BB_E5: u64 = 1 << 36;
pub const BB_F5: u64 = 1 << 37;
pub const BB_G5: u64 = 1 << 38;
pub const BB_H5: u64 = 1 << 39;
pub const BB_A6: u64 = 1 << 40;
pub const BB_B6: u64 = 1 << 41;
pub const BB_C6: u64 = 1 << 42;
pub const BB_D6: u64 = 1 << 43;
pub const BB_E6: u64 = 1 << 44;
pub const BB_F6: u64 = 1 << 45;
pub const BB_G6: u64 = 1 << 46;
pub const BB_H6: u64 = 1 << 47;
pub const BB_A7: u64 = 1 << 48;
pub const BB_B7: u64 = 1 << 49;
pub const BB_C7: u64 = 1 << 50;
pub const BB_D7: u64 = 1 << 51;
pub const BB_E7: u64 = 1 << 52;
pub const BB_F7: u64 = 1 << 53;
pub const BB_G7: u64 = 1 << 54;
pub const BB_H7: u64 = 1 << 55;
pub const BB_A8: u64 = 1 << 56;
pub const BB_B8: u64 = 1 << 57;
pub const BB_C8: u64 = 1 << 58;
pub const BB_D8: u64 = 1 << 59;
pub const BB_E8: u64 = 1 << 60;
pub const BB_F8: u64 = 1 << 61;
pub const BB_G8: u64 = 1 << 62;
pub const BB_H8: u64 = 1 << 63;

pub const BB_CORNERS: u64 = BB_A1 | BB_H1 | BB_A8 | BB_H8;
pub const BB_CENTER: u64 = BB_D4 | BB_E4 | BB_D5 | BB_E5;

pub const BB_LIGHT_SQUARES: u64 = 0x55aa_55aa_55aa_55aa;
pub const BB_DARK_SQUARES: u64 = 0xaa55_aa55_aa55_aa55;

lazy_static! {
    pub static ref BB_FILES: [u64; 8] = {
        let bb_files = (0..8)
            .map(|x| 0x0101_0101_0101_0101 << x)
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong sized");
        bb_files
    };
    pub static ref BB_RANKS: [u64; 8] = {
        let bb_ranks = (0..8)
            .map(|x| 0xff << (8 * x))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong sizee");
        bb_ranks
    };
}
pub const BB_FILE_A: u64 = 0x0101_0101_0101_0101 << 0;
pub const BB_FILE_B: u64 = 0x0101_0101_0101_0101 << 1;
pub const BB_FILE_C: u64 = 0x0101_0101_0101_0101 << 2;
pub const BB_FILE_D: u64 = 0x0101_0101_0101_0101 << 3;
pub const BB_FILE_E: u64 = 0x0101_0101_0101_0101 << 4;
pub const BB_FILE_F: u64 = 0x0101_0101_0101_0101 << 5;
pub const BB_FILE_G: u64 = 0x0101_0101_0101_0101 << 6;
pub const BB_FILE_H: u64 = 0x0101_0101_0101_0101 << 7;

pub const BB_RANK_1: u64 = 0xff << (8 * 0);
pub const BB_RANK_2: u64 = 0xff << (8 * 1);
pub const BB_RANK_3: u64 = 0xff << (8 * 2);
pub const BB_RANK_4: u64 = 0xff << (8 * 3);
pub const BB_RANK_5: u64 = 0xff << (8 * 4);
pub const BB_RANK_6: u64 = 0xff << (8 * 5);
pub const BB_RANK_7: u64 = 0xff << (8 * 6);
pub const BB_RANK_8: u64 = 0xff << (8 * 7);

pub const BB_BACKRANKS: u64 = BB_RANK_1 | BB_RANK_8;
pub fn bit_length(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    unsafe { 1 + log2f64(x as f64) as u64 }
}
pub fn lsb(bb: Bitboard) -> u8 {
    (bit_length(bb & bb.wrapping_neg()) - 1) as u8
}
use crate::gen_iter;
pub fn scan_forward(bb: Bitboard) -> impl Iterator<Item = Square> {
    gen_iter!({
        let mut temp_bb = bb;
        let mut r = 0;
        while bb != 0 {
            r = temp_bb & temp_bb.wrapping_neg();
            yield (bit_length(r) - 1) as Square;
            temp_bb ^= r;
        }
    })
}
pub fn msb(bb: Bitboard) -> u8 {
    (bit_length(bb) - 1) as u8
}
pub fn scan_reversed(bb: Bitboard) -> impl Iterator<Item = Square> {
    gen_iter!({
        let mut temp_bb = bb;
        let mut r: u64 = 0;
        while temp_bb != 0 {
            let len = bit_length(temp_bb);
            r = if len != 0 {len - 1} else {0};
            yield r as Square;
            temp_bb ^= BB_SQUARES[r as usize];
        }
    })
}

fn popcount(bb: Bitboard) -> u32 {
    bb.count_ones()
}
pub fn flip_vertical(mut bb: Bitboard) -> Bitboard {
    bb = ((bb >> 8) & 0x00ff_00ff_00ff_00ff) | ((bb & 0x00ff_00ff_00ff_00ff) << 8);
    bb = ((bb >> 16) & 0x0000_ffff_0000_ffff) | ((bb & 0x0000_ffff_0000_ffff) << 16);
    bb = (bb >> 32) | ((bb & 0x0000_0000_ffff_ffff) << 32);
    return bb;
}
pub fn flip_horizontal(mut bb: Bitboard) -> Bitboard {
    bb = ((bb >> 1) & 0x5555_5555_5555_5555) | ((bb & 0x5555_5555_5555_5555) << 1);
    bb = ((bb >> 2) & 0x3333_3333_3333_3333) | ((bb & 0x3333_3333_3333_3333) << 2);
    bb = ((bb >> 4) & 0x0f0f_0f0f_0f0f_0f0f) | ((bb & 0x0f0f_0f0f_0f0f_0f0f) << 4);
    return bb;
}
pub fn flip_diagonal(mut bb: Bitboard) -> Bitboard {
    let mut t = (bb ^ (bb << 28)) & 0x0f0f_0f0f_0000_0000;
    bb = bb ^ (t ^ (t >> 28));
    t = (bb ^ (bb << 14)) & 0x3333_0000_3333_0000;
    bb = bb ^ (t ^ (t >> 14));
    t = (bb ^ (bb << 7)) & 0x5500_5500_5500_5500;
    bb = bb ^ (t ^ (t >> 7));
    return bb;
}
pub fn flip_anti_diagonal(mut bb: Bitboard) -> Bitboard {
    let mut t = bb ^ (bb << 36);
    bb = bb ^ ((t ^ (bb >> 36)) & 0xf0f0_f0f0_0f0f_0f0f);
    t = (bb ^ (bb << 18)) & 0xcccc_0000_cccc_0000;
    bb = bb ^ (t ^ (t >> 18));
    t = (bb ^ (bb << 9)) & 0xaa00_aa00_aa00_aa00;
    bb = bb ^ (t ^ (t >> 9));
    return bb;
}

pub fn shift_down(b: Bitboard) -> Bitboard {
    return b >> 8;
}
pub fn shift_2_down(b: Bitboard) -> Bitboard {
    return b >> 16;
}
pub fn shift_up(b: Bitboard) -> Bitboard {
    return (b << 8) & BB_ALL;
}
pub fn shift_2_up(b: Bitboard) -> Bitboard {
    return (b << 16) & BB_ALL;
}
pub fn shift_right(b: Bitboard) -> Bitboard {
    return (b << 1) & !BB_FILE_A & BB_ALL;
}
pub fn shift_2_right(b: Bitboard) -> Bitboard {
    return (b << 2) & !BB_FILE_A & !BB_FILE_B & BB_ALL;
}
pub fn shift_left(b: Bitboard) -> Bitboard {
    return (b >> 1) & !BB_FILE_H;
}
pub fn shift_2_left(b: Bitboard) -> Bitboard {
    return (b >> 2) & !BB_FILE_G & !BB_FILE_H;
}
pub fn shift_up_left(b: Bitboard) -> Bitboard {
    return (b << 7) & !BB_FILE_H & BB_ALL;
}
pub fn shift_up_right(b: Bitboard) -> Bitboard {
    return (b << 9) & !BB_FILE_A & BB_ALL;
}
pub fn shift_down_left(b: Bitboard) -> Bitboard {
    return (b >> 9) & !BB_FILE_H;
}
pub fn shift_down_right(b: Bitboard) -> Bitboard {
    return (b >> 7) & !BB_FILE_A;
}
pub fn any<T, B>(iter: T) -> bool where T: IntoIterator<Item = B>, B: Boolean {
    for i in iter {
        if i.bool() {
            return true;
        }
    }
    false
}
pub fn all<T, B>(iter: T) -> bool where T: IntoIterator<Item = B>, B: Boolean {
    for i in iter {
        if !i.bool() {
            return false;
        }
    }
    true
}
fn sliding_attacks<'a, I>(square: Square, occupied: Bitboard, deltas: I) -> Bitboard
where
    I: Iterator<Item = &'a i8>,
{
    let mut attacks = BB_EMPTY;

    for delta in deltas {
        let mut sq = square;

        loop {
            sq = sq.wrapping_add(*delta as u8);
            if !(/*0 <= sq && */sq < 64) || square_distance(sq, sq.wrapping_sub(*delta as u8)) > 2 {
                break;
            }
            attacks |= BB_SQUARES[sq as usize];

            if occupied & BB_SQUARES[sq as usize] != 0 {
                break;
            }
        }
    }
    return attacks;
}

fn step_attacks<'a, I>(square: Square, deltas: I) -> Bitboard
where
    I: Iterator<Item = &'a i8>,
{
    return sliding_attacks(square, BB_ALL, deltas);
}

lazy_static! {
    pub static ref BB_KNIGHT_ATTACKS: [Bitboard; 64] = {
        let bb = (0..64)
            .map(|x| step_attacks(x, [17, 15, 10, 6, -17, -15, -10, -6].iter()))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong sizea");
        bb
    };
    pub static ref BB_KING_ATTACKS: [Bitboard; 64] = {
        let bb = (0..64)
            .map(|x| step_attacks(x, [9, 8, 7, 1, -9, -8, -7, -1].iter()))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong sizeb");
        bb
    };
    pub static ref BB_PAWN_ATTACKS: [[Bitboard; 64]; 2] = {
        let mut bb: [[Bitboard; 64]; 2] = [[0; 64]; 2];
        let deltas = [[-7, -9], [7, 9]];
        for i in 0..2 {
            bb[i] = (0..64)
                .map(|x| step_attacks(x, deltas[i].iter()))
                .collect::<Vec<u64>>()
                .try_into()
                .expect("wrong sizec");
        }
        bb
    };
}
fn edges(square: Square) -> Bitboard {
    return (BB_RANK_1 | BB_RANK_8) & !BB_RANKS[square_rank(square) as usize]
        | (BB_FILE_A | BB_FILE_H) & !BB_FILES[square_file(square) as usize];
}
fn carry_rippler(mask: Bitboard) -> impl Iterator<Item = u64> {
    gen_iter!({
        let mut subset: i64 = BB_EMPTY as i64;
        loop {
            yield subset as u64;
            subset = ((subset - mask as i64) as u64 & mask) as i64;
            if subset == 0 {
                break;
            }
        }
    })
}
lazy_static! {
    pub static ref BB_DIAG_MASKS: [Bitboard; 64] = {
        let table = (0..64)
            .map(|x| sliding_attacks(x, 0, [-9, -7, 7, 9].iter()) & !edges(x))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong size");
        table
    };
    pub static ref BB_FILE_MASKS: [Bitboard; 64] = {
        let table = (0..64)
            .map(|x| sliding_attacks(x, 0, [-8, 8].iter()) & !edges(x))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong size");
        table
    };
    pub static ref BB_RANK_MASKS: [Bitboard; 64] = {
        let table = (0..64)
            .map(|x| sliding_attacks(x, 0, [-1, 1].iter()) & !edges(x))
            .collect::<Vec<u64>>()
            .try_into()
            .expect("wrong size");
        table
    };
    pub static ref BB_DIAG_ATTACKS: [HashMap<Bitboard, Bitboard>; 64] = {
        let mut table = Vec::new();
        for sq in 0..64 {
            let mut attacks = HashMap::new();
            let mask = BB_DIAG_MASKS[sq];
            for subset in carry_rippler(mask) {
                attacks.insert(
                    subset,
                    sliding_attacks(sq as Square, subset, [-9, -7, 7, 9].iter()),
                );
            }
            table.push(attacks);
        }
        table.try_into().expect("wrong size")
    };
    pub static ref BB_FILE_ATTACKS: [HashMap<Bitboard, Bitboard>; 64] = {
        let mut table = Vec::new();

        for sq in 0..64 {
            let mut attacks = HashMap::new();
            let mask = BB_FILE_MASKS[sq];
            for subset in carry_rippler(mask) {
                attacks.insert(
                    subset,
                    sliding_attacks(sq as Square, subset, [-8, 8].iter()),
                );
            }
            table.push(attacks);
        }
        table.try_into().expect("wrong size")
    };
    pub static ref BB_RANK_ATTACKS: [HashMap<Bitboard, Bitboard>; 64] = {
        let mut table = Vec::new();

        for sq in 0..64 {
            let mut attacks = HashMap::new();
            let mask = BB_RANK_MASKS[sq];
            for subset in carry_rippler(mask) {
                attacks.insert(
                    subset,
                    sliding_attacks(sq as Square, subset, [-1, 1].iter()),
                );
            }
            table.push(attacks);
        }
        table.try_into().expect("wrong size")
    };
}
fn rays() -> [[Bitboard; 64]; 64] {
    let mut rays = [[0; 64]; 64];
    for (a, bb_a) in BB_SQUARES.iter().enumerate() {
        let mut rays_row = [0; 64];
        for (b, bb_b) in BB_SQUARES.iter().enumerate() {
            if (BB_DIAG_ATTACKS[a][&0] & bb_b) != 0 {
                rays_row[b] = (BB_DIAG_ATTACKS[a][&0] & BB_DIAG_ATTACKS[b][&0]) | bb_a | bb_b;
            } else if (BB_RANK_ATTACKS[a][&0] & bb_b) != 0 {
                rays_row[b] = BB_RANK_ATTACKS[a][&0] | bb_a;
            } else if (BB_FILE_ATTACKS[a][&0] & bb_b) != 0 {
                rays_row[b] = BB_FILE_ATTACKS[a][&0] | bb_a;
            } else {
                rays_row[b] = BB_EMPTY;
            }
        }
        rays[a] = rays_row;
    }
    rays
}
lazy_static! {
    pub static ref BB_RAYS: [[Bitboard; 64]; 64] = {
        let mut rays = rays();
        rays
    };
    pub static ref SAN_REGEX: Regex = {
        let regex =
            Regex::new(r"^([NBKRQ])?([a-h])?([1-8])?[\-x]?([a-h][1-8])(=?[nbrqkNBRQK])?[\+#]?\Z");
        regex.unwrap()
    };
    pub static ref FEN_CASTLING_REGEX: Regex = {
        let regex = Regex::new(r"^(?:-|[KQABCDEFGH]{0,2}[kqabcdefgh]{0,2})\Z");
        regex.unwrap()
    };
}
fn ray(a: Square, b: Square) -> Bitboard {
    BB_RAYS[a as usize][b as usize]
}
fn between(a: Square, b: Square) -> Bitboard {
    let bb = BB_RAYS[a as usize][b as usize] & ((BB_ALL << a) ^ (BB_ALL << b));
    bb & (bb - 1)
}
#[derive(Hash, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}
impl Piece {
    fn symbol(&self) -> char {
        let symbol: char = if self.color {
            piece_symbol(self.piece_type).unwrap().to_ascii_uppercase()
        } else {
            piece_symbol(self.piece_type).unwrap()
        };
        return symbol;
    }
    fn unicode_symbol(&self) -> char {
        return unicode_piece_symbols(self.symbol());
    }
    fn unicode_symbol_inverted(&self) -> char {
        if self.symbol().is_ascii_uppercase() {
            return unicode_piece_symbols(self.symbol().to_ascii_lowercase());
        }
        return unicode_piece_symbols(self.symbol().to_ascii_uppercase());
    }
    // TODO: repr_svg
    fn from_symbol(symbol: char) -> Piece {
        Piece {
            piece_type: piece_type(Some(symbol.to_ascii_lowercase())).unwrap(),
            color: symbol.is_ascii_uppercase(),
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
pub struct Move {
    from_square: Square,
    to_square: Square,
    promotion: Option<PieceType>,
}
impl Boolean for Option<Square>{
    fn bool(&self) -> bool {
        *self != None
    }
}
impl Boolean for Square {
    fn bool(&self) -> bool {
        *self != 0
    }
}
impl Boolean for Bitboard {
    fn bool(&self) -> bool {
        *self != 0
    }
}
impl Boolean for Move {
    fn bool(&self) -> bool {
        self.from_square.bool() || self.from_square.bool() || self.promotion.bool()
    }
}
impl Boolean for bool {
    fn bool(&self) -> bool {
        *self
    }
}
impl Move {
    pub fn uci(&self) -> String {
        let mut result = String::new();
        if let Some(promotion) = self.promotion {
            result.push_str(SQUARE_NAMES[self.from_square as usize]);
            result.push_str(SQUARE_NAMES[self.to_square as usize]);
            result.push(piece_symbol(promotion).unwrap());
        } else if self.bool() {
            result.push_str(SQUARE_NAMES[self.from_square as usize]);
            result.push_str(SQUARE_NAMES[self.to_square as usize]);
        } else {
            result.push_str("0000")
        }
        result
    }
    pub fn xboard(&self) -> String {
        if self.bool() {
            return self.uci();
        }
        return String::from("@@@@");
    }
    pub fn from_uci(uci: &str) -> Move {
        if uci == "0000" {
            Move::null()
        } else if uci.len() == 4 || uci.len() == 5 {
            let from_square = parse_square(&uci[..2]);
            let to_square = parse_square(&uci[2..4]);
            let promotion = if uci.len() == 5 {
                Some(uci.chars().nth(4).unwrap() as u8)
            } else {
                None
            };
            assert!(from_square != to_square);
            Move {
                from_square: from_square,
                to_square: to_square,
                promotion: promotion,
            }
        } else {
            panic!("expected uci string to be of lenght 4 or 5: {}", uci);
        }
    }
    pub fn null() -> Move {
        Move {
            from_square: 0,
            to_square: 0,
            promotion: None,
        }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move.from_uci({})", self.uci())
    }
}
impl fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Move(\"{}\")", self.uci())
    }
}
#[derive(Clone, Copy)]
pub struct BaseBoard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub promoted: u64,
    pub occupied_co: [u64; 2],
    pub occupied: u64,
}
impl BaseBoard {
    pub fn new(board_fen: Option<&str>) -> BaseBoard {
        let mut b = BaseBoard {
            pawns: BB_EMPTY,
            knights: BB_EMPTY,
            bishops: BB_EMPTY,
            rooks: BB_EMPTY,
            queens: BB_EMPTY,
            kings: BB_EMPTY,
            promoted: BB_EMPTY,
            occupied_co: [BB_EMPTY, BB_EMPTY],
            occupied: BB_EMPTY,
        };
        match board_fen {
            Some(STARTING_BOARD_FEN) => b.reset_board(),
            None => b.clear_board(),
            Some(fen) => b.set_board_fen(fen),
        }
        b
    }
    pub fn reset_board(&mut self) {
        self.pawns = BB_RANK_2 | BB_RANK_7;
        self.knights = BB_B1 | BB_G1 | BB_B8 | BB_G8;
        self.bishops = BB_C1 | BB_F1 | BB_C8 | BB_F8;
        self.rooks = BB_CORNERS;
        self.queens = BB_D1 | BB_D8;
        self.kings = BB_E1 | BB_E8;

        self.promoted = BB_EMPTY;

        self.occupied_co[WHITE as usize] = BB_RANK_1 | BB_RANK_2;
        self.occupied_co[BLACK as usize] = BB_RANK_7 | BB_RANK_8;
        self.occupied = BB_RANK_1 | BB_RANK_2 | BB_RANK_7 | BB_RANK_8;
    }
    pub fn clear_board(&mut self) {
        self.pawns = BB_EMPTY;
        self.knights = BB_EMPTY;
        self.bishops = BB_EMPTY;
        self.rooks = BB_EMPTY;
        self.queens = BB_EMPTY;
        self.kings = BB_EMPTY;

        self.promoted = BB_EMPTY;

        self.occupied_co[WHITE as usize] = BB_EMPTY;
        self.occupied_co[BLACK as usize] = BB_EMPTY;
        self.occupied = BB_EMPTY;
    }
    pub fn pieces_mask(&self, piece_type: PieceType, color: Color) -> Bitboard {
        let bb = match piece_type {
            PAWN => self.pawns,
            KNIGHT => self.knights,
            BISHOP => self.bishops,
            ROOK => self.rooks,
            QUEEN => self.queens,
            KING => self.knights,
            _ => {
                panic!("expected PieceType got: {}", piece_type)
            }
        };
        return bb & self.occupied_co[color as usize];
    }
    pub fn pieces(&self, piece_type: PieceType, color: Color) -> SquareSet {
        SquareSet {
            mask: self.pieces_mask(piece_type, color),
        }
    }
    pub fn piece_type_at(&self, square: Square) -> Option<PieceType> {
        let mask = BB_SQUARES[square as usize];
        if !self.occupied & mask != 0 {
            return None;
        } else if self.pawns & mask != 0 {
            return Some(PAWN);
        } else if self.knights & mask != 0 {
            return Some(KNIGHT);
        } else if self.bishops & mask != 0 {
            return Some(BISHOP);
        } else if self.rooks & mask != 0 {
            return Some(ROOK);
        } else if self.queens & mask != 0 {
            return Some(QUEEN);
        } else {
            return Some(KING);
        }
    }
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let piece_type_option = self.piece_type_at(square);
        match piece_type_option {
            Some(piece_type) => {
                let mask = BB_SQUARES[square as usize];
                let color = self.occupied_co[WHITE as usize] & mask != 0;
                Some(Piece {
                    piece_type: piece_type,
                    color: color,
                })
            }
            None => None,
        }
    }
    pub fn color_at(&self, square: Square) -> Option<Color> {
        let mask = BB_SQUARES[square as usize];
        if self.occupied_co[WHITE as usize] & mask != 0 {
            return Some(WHITE);
        } else if self.occupied_co[BLACK as usize] & mask != 0 {
            return Some(BLACK);
        }
        None
    }
    pub fn king(&self, color: Color) -> Option<Square> {
        let king_mask = self.occupied_co[color as usize] & self.kings & !self.promoted;
        if king_mask != 0 {
            return Some(msb(king_mask) as Square);
        }
        None
    }
    pub fn attacks_mask(&self, square: Square) -> Bitboard {
        let bb_square = BB_SQUARES[square as usize];
        if bb_square & self.pawns != 0 {
            let color = bb_square & self.occupied_co[WHITE as usize] != 0;
            return BB_PAWN_ATTACKS[color as usize][square as usize];
        } else if bb_square & self.knights != 0 {
            return BB_KNIGHT_ATTACKS[square as usize];
        } else if bb_square & self.kings != 0 {
            return BB_KING_ATTACKS[square as usize];
        } else {
            let mut attacks = 0;
            if bb_square & self.bishops != 0 || bb_square & self.queens != 0 {
                attacks = BB_DIAG_ATTACKS[square as usize]
                    [&(BB_DIAG_MASKS[square as usize] & self.occupied)];
            }
            if bb_square & self.rooks != 0 || bb_square & self.queens != 0 {
                attacks |= BB_RANK_ATTACKS[square as usize]
                    [&(BB_RANK_MASKS[square as usize] & self.occupied)]
                    | BB_FILE_ATTACKS[square as usize]
                        [&(BB_FILE_MASKS[square as usize] & self.occupied)];
            }

            attacks
        }
    }
    pub fn attacks(self, square: Square) -> SquareSet {
        SquareSet {
            mask: self.attacks_mask(square),
        }
    }
    pub fn _attackers_mask(&self, color: Color, square: Square, occupied: Bitboard) -> Bitboard {
        let rank_pieces = BB_RANK_MASKS[square as usize] & occupied;
        let file_pieces = BB_FILE_MASKS[square as usize] & occupied;
        let diag_pieces = BB_DIAG_MASKS[square as usize] & occupied;

        let queens_and_rooks = self.queens | self.rooks;
        let queens_and_bishops = self.queens | self.bishops;

        let attackers = (BB_KING_ATTACKS[square as usize] & self.kings)
            | (BB_KING_ATTACKS[square as usize] & self.knights)
            | (BB_RANK_ATTACKS[square as usize][&rank_pieces] & queens_and_rooks)
            | (BB_FILE_ATTACKS[square as usize][&file_pieces] & queens_and_bishops)
            | (BB_DIAG_ATTACKS[square as usize][&diag_pieces] & queens_and_rooks)
            | (BB_PAWN_ATTACKS[(!color) as usize][square as usize] & self.pawns);

        attackers & self.occupied_co[color as usize]
    }
    pub fn attackers_mask(&self, color: Color, square: Square) -> Bitboard {
        self._attackers_mask(color, square, self.occupied)
    }
    pub fn is_attacked_by(&self, color: Color, square: Square) -> bool {
        self.attackers_mask(color, square) != 0
    }
    pub fn attackers(&self, color: Color, square: Square) -> SquareSet {
        SquareSet {
            mask: self.attackers_mask(color, square),
        }
    }
    pub fn pin_mask(&self, color: Color, square: Square) -> Bitboard {
        if self.king(color) == None {
            return BB_ALL;
        }
        let king = self.king(color).unwrap();

        let square_mask = BB_SQUARES[square as usize];

        let a: [(&[HashMap<Bitboard, Bitboard>; 64], u64); 3] = [
            (&BB_FILE_ATTACKS, self.rooks | self.queens),
            (&BB_RANK_ATTACKS, self.rooks | self.queens),
            (&BB_DIAG_ATTACKS, self.bishops | self.queens),
        ];

        for (attacks, sliders) in a {
            let rays = attacks[king as usize][&0];
            if rays & square_mask != 0 {
                let snipers = rays & sliders & self.occupied_co[(!color as usize)];
                for sniper in scan_reversed(snipers) {
                    if between(sniper as Square, king) & (self.occupied | square_mask)
                        == square_mask
                    {
                        return ray(king, sniper as Square);
                    }
                }
                break;
            }
        }
        BB_ALL
    }
    pub fn pin(&self, color: Color, square: Square) -> SquareSet {
        SquareSet {
            mask: self.pin_mask(color, square),
        }
    }
    pub fn is_pinned(self, color: Color, square: Square) -> bool {
        self.pin_mask(color, square) != BB_ALL
    }
    pub fn _remove_piece_at(&mut self, square: Square) -> Option<PieceType> {
        let piece_type = self.piece_type_at(square);
        let mask = BB_SQUARES[square as usize];

        match piece_type {
            Some(PAWN) => self.pawns ^= mask,
            Some(KNIGHT) => self.knights ^= mask,
            Some(BISHOP) => self.bishops ^= mask,
            Some(ROOK) => self.rooks ^= mask,
            Some(QUEEN) => self.queens ^= mask,
            Some(KING) => self.kings ^= mask,
            _ => return None,
        }
        self.occupied ^= mask;
        self.occupied_co[WHITE as usize] &= !mask;
        self.occupied_co[BLACK as usize] &= !mask;

        self.promoted &= !mask;

        return piece_type;
    }
    pub fn remove_piece_at(&mut self, square: Square) -> Option<Piece> {
        let color = self.occupied_co[WHITE as usize] & BB_SQUARES[square as usize] != 0;
        let piece_type = self._remove_piece_at(square);
        match piece_type {
            Some(piece) => Some(Piece {
                piece_type: piece,
                color: color,
            }),
            None => None,
        }
    }
    pub fn _set_piece_at(
        &mut self,
        square: Square,
        piece_type: PieceType,
        color: Color,
        promoted: bool,
    ) {
        self._remove_piece_at(square);

        let mask = BB_SQUARES[square as usize];

        match piece_type {
            PAWN => {
                self.pawns |= mask;
            }
            KNIGHT => {
                self.knights |= mask;
            }
            BISHOP => {
                self.bishops |= mask;
            }
            ROOK => {
                self.rooks |= mask;
            }
            QUEEN => {
                self.queens |= mask;
            }
            KING => {
                self.kings |= mask;
            }
            _ => return,
        }
        self.occupied ^= mask;
        self.occupied_co[color as usize] ^= mask;

        if promoted {
            self.promoted ^= mask;
        }
    }
    pub fn set_piece_at(&mut self, square: Square, piece: Option<Piece>, promoted: bool) {
        match piece {
            Some(p) => {
                self._set_piece_at(square, p.piece_type, p.color, promoted);
            }
            None => {
                self.remove_piece_at(square);
            }
        }
    }
    pub fn board_fen(self, promoted: bool) -> String {
        let mut builder: String = String::new();
        let mut empty = 0;

        for square in SQUARES_180.into_iter() {
            let piece = self.piece_at(square);
            match piece {
                None => {
                    empty += 1;
                }
                Some(p) => {
                    if empty != 0 {
                        builder.push_str(&empty.to_string());
                        empty = 0;
                    }
                    builder.push(p.symbol());
                    if promoted && BB_SQUARES[square as usize] & self.promoted != 0 {
                        builder.push('~');
                    }
                }
            }
            if BB_SQUARES[square as usize] & BB_FILE_H != 0 {
                if empty != 0 {
                    builder.push_str(&empty.to_string());
                    empty = 0;
                }
                if square != H1 {
                    builder.push('/');
                }
            }
        }
        builder
    }
    pub fn set_board_fen(&mut self, fen: &str) {
        let fen_trimmed = fen.trim();
        if fen_trimmed.contains(" ") {
            panic!("expected position part of fen, got multiple parts {}", fen);
        }
        let rows: Vec<&str> = fen.split("/").collect();
        if rows.len() != 8 {
            panic!("expected 8 rows in position part of fen {}", fen);
        }

        for row in rows {
            let mut field_sum = 0;
            let mut previuos_was_digit = false;
            let mut previous_was_piece = false;

            for c in row.chars() {
                if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {
                    if previuos_was_digit {
                        panic!("two subseqeunt digits in position part of fen {}", fen);
                    }
                    field_sum += (c as u8 - 0x30) as u64;
                    previuos_was_digit = true;
                    previous_was_piece = false;
                } else if c == '~' {
                    if !previous_was_piece {
                        panic!("'~' not after piece in position part of fen {}", fen);
                    }
                    previuos_was_digit = false;
                    previous_was_piece = false;
                } else if PIECE_SYMBOLS.contains(&Some(c.to_ascii_lowercase())) {
                    field_sum += 1;
                    previuos_was_digit = false;
                    previous_was_piece = true
                } else {
                    panic!("invlaid character in position part of fen {}", fen);
                }
            }
            if field_sum != 8 {
                panic!("expected 8 columns per row in position part of fen {}", fen);
            }
        }
        self.clear_board();
        let mut square_index = 0;
        for c in fen_trimmed.chars() {
            if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {
                square_index += (c as u8 - 0x30) as Square;
            } else if PIECE_SYMBOLS.contains(&Some(c.to_ascii_lowercase())) {
                let piece = Piece::from_symbol(c);
                self._set_piece_at(
                    SQUARES_180[square_index as usize] as Square,
                    piece.piece_type,
                    piece.color,
                    false,
                );
                square_index += 1;
            } else if c == '~' {
                self.promoted |= BB_SQUARES[SQUARES_180[(square_index - 1) as usize] as usize];
            }
        }
    }
    pub fn piece_map(&self, mask: Bitboard) -> HashMap<Square, Piece> {
        let mut result = HashMap::new();

        for square in scan_reversed(self.occupied & mask) {
            result.insert(square as Square, self.piece_at(square as Square).unwrap());
        }
        result
    }
    pub fn set_piece_map(&mut self, pieces: HashMap<Square, Piece>) {
        self.clear_board();
        for (square, piece) in pieces {
            self._set_piece_at(square, piece.piece_type, piece.color, false);
        }
    }
    // TODO: maybe implement chess960 stuff
    pub fn unicode(&self, invert_color: bool, borders: bool, empty_square: &str) -> String {
        let mut builder = String::new();
        for rank_index in (0..8).rev() {
            if borders {
                builder.push_str("  ");
                builder.push_str(&"-".repeat(17));
                builder.push('\n');
                builder.push(RANK_NAMES[rank_index as usize]);
                builder.push(' ');
            }
            for file_index in 0..8 {
                let square_index = square(file_index, rank_index);
                if borders {
                    builder.push('|');
                } else if file_index > 0 {
                    builder.push(' ');
                }

                let piece = self.piece_at(square_index);
                match piece {
                    Some(p) => builder.push(if invert_color {
                        p.unicode_symbol()
                    } else {
                        p.unicode_symbol_inverted()
                    }),
                    None => builder.push_str(empty_square),
                }
            }
            if borders {
                builder.push('|');
            }
            if borders || rank_index > 0 {
                builder.push('\n');
            }
        }
        if borders {
            builder.push_str("  ");
            builder.push_str(&"-".repeat(17));
            builder.push('\n');
            builder.push_str("   a b c d e f g h");
        }
        builder
    }
    // TODO: repr svg
    pub fn apply_transform(&mut self, f: fn(Bitboard) -> Bitboard) {
        self.pawns = f(self.pawns);
        self.knights = f(self.knights);
        self.bishops = f(self.bishops);
        self.rooks = f(self.rooks);
        self.queens = f(self.queens);
        self.kings = f(self.kings);

        self.occupied_co[WHITE as usize] = f(self.occupied_co[WHITE as usize]);
        self.occupied_co[BLACK as usize] = f(self.occupied_co[BLACK as usize]);
        self.occupied = f(self.occupied);
        self.promoted = f(self.promoted);
    }
    pub fn transform(&self, f: fn(Bitboard) -> Bitboard) -> BaseBoard {
        let mut board = self.clone();
        board.apply_transform(f);
        board
    }
    pub fn apply_mirror(&mut self) {
        self.apply_transform(flip_vertical);
        self.occupied_co[WHITE as usize] = self.occupied_co[BLACK as usize];
        self.occupied_co[BLACK as usize] = self.occupied_co[WHITE as usize];
    }
    pub fn mirror(&self) -> BaseBoard {
        let mut board = self.clone();
        board.apply_mirror();
        board
    }
}
impl PartialEq for BaseBoard {
    fn eq(&self, board: &Self) -> bool {
        self.occupied == board.occupied
            && self.occupied_co[WHITE as usize] == board.occupied_co[WHITE as usize]
            && self.pawns == board.pawns
            && self.knights == board.knights
            && self.bishops == board.bishops
            && self.rooks == board.rooks
            && self.queens == board.queens
            && self.kings == board.kings
    }
}
#[derive(Clone, Copy)]
pub struct BoardState {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    promoted: u64,
    occupied_w: u64,
    occupied_b: u64,
    occupied: u64,
    turn: Color,
    castling_rights: Bitboard,
    ep_square: Option<Square>,
    halfmove_clock: u64,
    fullmove_number: u64,
}
impl BoardState {
    fn new(board: Board) -> BoardState {
        BoardState {
            pawns: board.baseboard.pawns,
            knights: board.baseboard.knights,
            bishops: board.baseboard.bishops,
            rooks: board.baseboard.rooks,
            queens: board.baseboard.queens,
            kings: board.baseboard.kings,
            promoted: board.baseboard.promoted,
            occupied_w: board.baseboard.occupied_co[WHITE as usize],
            occupied_b: board.baseboard.occupied_co[BLACK as usize],
            occupied: board.baseboard.occupied,
            turn: board.turn,
            castling_rights: board.castling_rights,
            ep_square: board.ep_square,
            halfmove_clock: board.halfmove_clock,
            fullmove_number: board.fullmove_number,
        }
    }
    fn restore(&self, board: &mut Board) {
        board.baseboard.pawns = self.pawns;
        board.baseboard.knights = self.knights;
        board.baseboard.bishops = self.bishops;
        board.baseboard.rooks = self.rooks;
        board.baseboard.queens = self.queens;
        board.baseboard.kings = self.kings;

        board.baseboard.occupied_co[WHITE as usize] = self.occupied_w;
        board.baseboard.occupied_co[BLACK as usize] = self.occupied_b;
        board.baseboard.occupied = self.occupied;

        board.baseboard.promoted = self.promoted;

        board.turn = self.turn;
        board.castling_rights = self.castling_rights;
        board.ep_square = self.ep_square;
        board.halfmove_clock = self.halfmove_clock;
        board.fullmove_number = self.fullmove_number;
    }
}
pub struct Board {
    pub baseboard: BaseBoard,
    pub ep_square: Option<Square>,
    pub move_stack: Vec<Move>,
    pub stack: Vec<BoardState>,
    pub turn: Color,
    pub castling_rights: Bitboard,
    pub halfmove_clock: u64,
    pub fullmove_number: u64,
}
//"class" variables as inline functions
// impl Board {
//     #[inline]
//     fn turn() -> Color {}
// }

impl Board {
    pub fn new(fen: Option<&str>) -> Board {
        let baseboard = BaseBoard::new(fen);
        let mut board = Board {
            baseboard: baseboard,
            ep_square: None,
            move_stack: Vec::new(),
            stack: Vec::new(),
            turn: WHITE,
            castling_rights: BB_EMPTY,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        match fen {
            None => {
                board.clear();
            }
            Some(STARTING_FEN) => {
                board.reset();
            }
            _ => board.set_fen(fen.unwrap()),
        }
        board
    }
    pub fn reset(&mut self) {
        self.turn = WHITE;
        self.castling_rights = BB_CORNERS;
        self.ep_square = None;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;

        self.reset_board();
    }
    pub fn reset_board(&mut self) {
        self.baseboard.reset_board();
        self.clear_stack();
    }
    pub fn clear(&mut self) {
        self.turn = WHITE;
        self.castling_rights = BB_EMPTY;
        self.ep_square = None;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;

        self.clear_board();
    }
    pub fn copy(&mut self, copy_stack: bool) -> Board {
        let mut board = Board::new(None);
        board.baseboard = self.baseboard;
        board.ep_square = self.ep_square;
        board.castling_rights = self.castling_rights;
        board.turn = self.turn;
        board.fullmove_number = self.fullmove_number;
        board.halfmove_clock = self.halfmove_clock;
        if copy_stack {
            board.move_stack = self.move_stack.to_owned();
            board.stack = self.stack.to_owned(); 
        }
        board
    }
    pub fn clear_board(&mut self) {
        self.baseboard.clear_board();
        self.clear_stack();
    }
    pub fn clear_stack(&mut self) {
        self.move_stack.clear();
        self.stack.clear();
    }
    pub fn ply(&self) -> u64 {
        2 * (self.fullmove_number - 1) + (self.turn == BLACK) as u64
    }
    pub fn remove_piece_at(&mut self, square: Square) -> Option<Piece> {
        let piece = self.baseboard.remove_piece_at(square);
        self.clear_stack();
        piece
    }
    pub fn set_piece_at(&mut self, square: Square, piece: Option<Piece>, promoted: bool) {
        self.baseboard.set_piece_at(square, piece, promoted);
        self.clear_stack();
    }
    pub fn generate_pseudo_legal_moves(
        &self,
        from_mask: Bitboard,
        to_mask: Bitboard,
    ) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            let our_pieces = self.baseboard.occupied_co[self.turn as usize];
            let non_pawns = our_pieces & !self.baseboard.pawns & from_mask;
            for from_square in scan_reversed(non_pawns) {
                let moves =
                    self.baseboard.attacks_mask(from_square as Square) & !our_pieces & to_mask;
                for to_square in scan_reversed(moves) {
                    yield Move {
                        from_square: from_square as Square,
                        to_square: to_square as Square,
                        promotion: None,
                    }
                }
            }
            if from_mask & self.baseboard.kings != 0 {
                for castling_move in self.generate_castling_moves(from_mask, to_mask) {
                    yield castling_move
                }
            }
            let pawns =
                self.baseboard.pawns & self.baseboard.occupied_co[self.turn as usize] & from_mask;
            if pawns == 0 {
                return;
            }
            let capturers = pawns;
            for from_square in scan_reversed(capturers) {
                let targets = BB_PAWN_ATTACKS[self.turn as usize][from_square as usize]
                    & self.baseboard.occupied_co[!self.turn as usize]
                    & to_mask;
                for to_square in scan_reversed(targets) {
                    yield Move {
                        from_square: from_square as Square,
                        to_square: to_square as Square,
                        promotion: None,
                    };
                }
            }
            let mut single_moves = 0;
            let mut double_moves = 0;
            if self.turn == WHITE {
                single_moves = pawns << 8 & !self.baseboard.occupied;
                double_moves =
                    single_moves << 8 & !self.baseboard.occupied & (BB_RANK_3 | BB_RANK_4)
            } else {
                single_moves = pawns >> 8 & !self.baseboard.occupied;
                double_moves = pawns >> 8 & !self.baseboard.occupied & (BB_RANK_6 | BB_RANK_5);
            }
            single_moves &= to_mask;
            double_moves &= to_mask;

            for to_square in scan_reversed(single_moves) {
                let from_square = to_square.wrapping_add(if self.turn == BLACK {
                    8
                } else {
                    (8 as u8).wrapping_neg()
                });

                yield Move {
                    from_square: from_square as Square,
                    to_square: to_square as Square,
                    promotion: None,
                };
            }

            for to_square in scan_reversed(double_moves) {
                let from_square = to_square.wrapping_add(if self.turn == BLACK {
                    16
                } else {
                    (16 as u8).wrapping_neg()
                });
                yield Move {
                    from_square: from_square as Square,
                    to_square: to_square as Square,
                    promotion: None,
                }
            }
            if self.ep_square != None {
                for ep in self.generate_pseudo_legal_ep(from_mask, to_mask) {
                    yield ep;
                }
            }
        })
    }
    pub fn generate_pseudo_legal_ep(
        &self,
        from_mask: Bitboard,
        to_mask: Bitboard,
    ) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            if self.ep_square != None
                || (!BB_SQUARES[self.ep_square.unwrap() as usize] & to_mask) != 0
            {
                return;
            }
            if BB_SQUARES[self.ep_square.unwrap() as usize] & self.baseboard.occupied != 0 {
                return;
            }
            let capturers = self.baseboard.pawns
                & self.baseboard.occupied_co[self.turn as usize]
                & from_mask
                & BB_PAWN_ATTACKS[!self.turn as usize][self.ep_square.unwrap() as usize]
                & BB_RANKS[(if self.turn { 4 } else { 3 }) as usize];
            for capturer in scan_reversed(capturers) {
                yield Move {
                    from_square: capturer as Square,
                    to_square: self.ep_square.unwrap() as Square,
                    promotion: None,
                };
            }
        })
    }
    pub fn generate_pseudolegal_captures(
        &self,
        from_mask: Bitboard,
        to_mask: Bitboard,
    ) -> impl Iterator<Item = Move> + '_{
        self.generate_pseudo_legal_moves(
            from_mask,
            to_mask & self.baseboard.occupied_co[!self.turn as usize],
        )
        .chain(self.generate_pseudo_legal_ep(from_mask, to_mask))
    }
    pub fn checkers_mask(&self) -> Bitboard {
        match self.baseboard.king(self.turn) {
            Some(king) => self.baseboard.attackers_mask(!self.turn, king),
            None => BB_EMPTY,
        }
    }
    pub fn checkers(&self) -> SquareSet {
        SquareSet::new(self.checkers_mask())
    }
    pub fn is_check(&self) -> bool {
        self.checkers_mask() != 0
    }
    pub fn gives_check(&mut self, m: Move) -> bool {
        self.push(m);
        let retval = self.is_check();
        self.pop();
        retval
    }
    pub fn is_into_check(&self, m: Move) -> bool{
        let king = self.baseboard.king(self.turn);
        if king == None {
            return false;
        }
        let checkers = self.baseboard.attackers_mask(!self.turn, king.unwrap());
        if checkers != 0 && !(self.generate_evasions(king.unwrap(), checkers, BB_SQUARES[m.from_square as usize], BB_SQUARES[m.to_square as usize]).any(|x| x == m)){
            return true;
        }
        !self._is_safe(king.unwrap(), self._slider_blockers(king.unwrap()), m)
    }
    pub fn was_into_check(&self) -> bool {
        let king = self.baseboard.king(!self.turn);
        king != None && self.baseboard.is_attacked_by(self.turn, king.unwrap())
    }
    pub fn is_pseudo_legal(&self, m: Move) -> bool {
        if !m.bool() { return false; }
        
        let piece = self.baseboard.piece_type_at(m.from_square);

        if piece == None {
            return false;
        }

        let from_mask = BB_SQUARES[m.from_square as usize];
        let to_mask = BB_SQUARES[m.to_square as usize];

        if self.baseboard.occupied_co[self.turn as usize] & from_mask == 0 {
            return false;
        }

        if let Some(promo) = m.promotion{
            if piece.unwrap() != PAWN { return false; }
            if self.turn == WHITE && square_rank(m.to_square) != 7 { return false; }
            else if self.turn == BLACK && square_rank(m.to_square) != 0 { return false; }
        }

        if piece.unwrap() == KING {
            if self.generate_castling_moves(BB_ALL, BB_ALL).any(|x| x == m) {
                return true;
            }
        }

        if self.baseboard.occupied_co[self.turn as usize] & to_mask != 0 {
            return false;
        }

        if piece.unwrap() == PAWN {
            return self.generate_pseudo_legal_moves(from_mask, to_mask).any(|x| x == m);
        }

        self.baseboard.attacks_mask(m.from_square) & to_mask != 0
    }
    pub fn is_legal(&self, m: Move) -> bool {
        !self.is_variant_end() && self.is_pseudo_legal(m) && !self.is_into_check(m)
    }
    pub fn is_variant_end(&self) -> bool { false }
    pub fn is_variant_loss(&self) -> bool { false }
    pub fn is_variant_win(&self) -> bool { false }
    pub fn is_variant_draw(&self) -> bool { false }
    
    pub fn is_game_over(&mut self, claim_draw: bool) -> bool {
        if let Some(outcome) = self.outcome(claim_draw){ true } else { false }
    }
    pub fn result(&mut self, claim_draw: bool) -> String {
        if let Some(outcome) = self.outcome(claim_draw) { 
            let a = String::from(outcome.result());
            a
        } 
        else {String::from("*")}
    }
    pub fn outcome(&mut self, claim_draw: bool) -> Option<Outcome> {
        if self.is_variant_loss() {
            return Some(Outcome { termination: Termination::VARIANT_LOSS, winner: Some(!self.turn) }) 
        }
        if self.is_variant_win() {
            return Some(Outcome { termination: Termination::VARIANT_WIN, winner: Some(self.turn) }) 
        }
        if self.is_variant_draw() {
            return Some(Outcome { termination: Termination::VARIANT_DRAW, winner: None }) 
        }

        if self.is_checkmate() {
            return Some(Outcome { termination: Termination::CHECKMATE, winner: Some(!self.turn) }) 
        }
        if self.is_insufficient_material() {
            return Some(Outcome { termination: Termination::INSUFFICIENT_MATERIAL, winner: None }) 
        }
        if !any(self.generate_legal_moves(BB_ALL, BB_ALL)){
            return Some(Outcome { termination: Termination::STALEMATE, winner: None }) 
        }

        if self.is_seventyfive_moves() {
            return Some(Outcome { termination: Termination::SEVENTYFIVE_MOVES, winner: None}) 
        }
        if self.is_fivefold_repetition() {
            return Some(Outcome { termination: Termination::FIVEFOLD_REPETITION, winner: None}) 
        }

        if claim_draw {
            if self.can_claim_fifty_moves() {
                return Some(Outcome { termination: Termination::FIFTY_MOVES, winner: None }) 
            }
            if self.can_claim_threefold_repetition() {
                return Some(Outcome { termination: Termination::THREEFOLD_REPETITION, winner: None }) 
            }
        }
        None

    }
    pub fn is_checkmate(&self) -> bool {
        if !self.is_check() {
            return false;
        }
        !any(self.generate_legal_moves(BB_ALL, BB_ALL))
    }
    pub fn is_stalemate(&self) -> bool {
        if self.is_check() { return false }
        if self.is_variant_end() { return false }

        return !any(self.generate_legal_moves(BB_ALL, BB_ALL))
    }
    pub fn is_insufficient_material(&self) -> bool {
        return all(COLORS.map(|col| self.has_insufficient_material(col)));
    }
    pub fn has_insufficient_material(&self, color: Color) -> bool {
        if self.baseboard.occupied_co[color as usize] 
        & (self.baseboard.pawns | self.baseboard.rooks | self.baseboard.queens)
        != 0 {
            return false;
        }

        if self.baseboard.occupied_co[color as usize] & self.baseboard.knights != 0 {
            return popcount(self.baseboard.occupied_co[color as usize]) <= 2
            && self.baseboard.occupied_co[!color as usize] & !self.baseboard.kings
            & !self.baseboard.queens == 0
        }

        if self.baseboard.occupied_co[color as usize] & self.baseboard.bishops != 0 {
            let same_color = !self.baseboard.bishops & BB_DARK_SQUARES != 0
                || !self.baseboard.bishops & BB_LIGHT_SQUARES != 0;
            return same_color && !self.baseboard.pawns != 0 && !self.baseboard.knights != 0;
        }
        true
    }
    pub fn is_halfmoves(&self, n: u64) -> bool {
        self.halfmove_clock >= n && any(self.generate_legal_moves(BB_ALL, BB_ALL))
    }
    pub fn is_seventyfive_moves(&self) -> bool {
        self.is_halfmoves(150)
    }
    pub fn is_fivefold_repetition(&mut self) -> bool {
        self.is_repetition(5)
    }
    pub fn can_claim_draw(&mut self) -> bool {
        self.can_claim_fifty_moves() || self.can_claim_threefold_repetition()
    }
    pub fn is_fifty_moves(&self) -> bool {
        self.is_halfmoves(100)
    }
    pub fn can_claim_fifty_moves(&mut self) -> bool {
        if self.is_fifty_moves() {
            return true;
        }

        if self.halfmove_clock >= 99 {
            let moves = self.generate_legal_moves(BB_ALL, BB_ALL).collect::<Vec<Move>>();
            for m in moves {
                if !self.is_zeroing(m){
                    self.push(m);
                    let retval = self.is_fifty_moves();
                    self.pop();
                    return retval;
                }
            }
        }
        false
    }
    pub fn can_claim_threefold_repetition(&mut self) -> bool {
        let transposition_key = self.transposition_key();
        let mut transpositions = [[transposition_key]].iter().cloned().collect::<Counter<_>>();

        let mut switchyard: Vec<Move> = Vec::new();

        while !self.move_stack.is_empty() {
            let m = self.pop();
            switchyard.push(m);
            if self.is_irreversible(m){
                break;
            }
            let new_transposition = [[self.transposition_key()]].iter().cloned().collect::<Counter<_>>();
            transpositions.extend(&new_transposition);
        }

        while !switchyard.is_empty() {
            self.push(switchyard.pop().unwrap());
        }

        if transpositions[&[transposition_key]] != 0 {
            return true;
        }
        let moves = self.generate_castling_moves(BB_ALL, BB_ALL).collect::<Vec<Move>>();
        for m in moves {
            self.push(m);
            let retval = transpositions[&[self.transposition_key()]] >= 2;
            self.pop();
            return retval;
        }
        return false;

    }
    pub fn is_repetition(&mut self, c: u32) -> bool {
        let mut count = c;
        let mut maybe_repetitions = 1;

        for state in self.stack.iter().rev() {
            if state.occupied == self.baseboard.occupied {
                maybe_repetitions += 1;
                if maybe_repetitions >= count {
                    break;
                }
            }
        }
        if maybe_repetitions < count {
            return false;
        }
        let transposition_key = self.transposition_key();
        let mut retval = false;
        let mut switchyard = Vec::new();
        loop {
            if count <= 1 {
                retval = true;
                break;
            }
            if self.move_stack.len() < (count - 1) as usize {
                break;
            }

            let m = self.pop();
            switchyard.push(m);

            if self.is_irreversible(m) {
                break;
            }
            if self.transposition_key() == transposition_key {
                count -= 1;
            }
        }
        while !switchyard.is_empty() {
            self.push(switchyard.pop().unwrap());
        }
        retval
    }
    pub fn set_fen(&mut self, fen: &str) {
        let mut parts = fen.split(' ').collect::<VecDeque<&str>>();
        let mut board = "";
        if let Some(board_part) = parts.pop_front() {
            board = board_part
        } else {
            panic!("Empty fen");
        }

        let mut turn = WHITE;
        if let Some(turn_part) = parts.pop_front() {
            if turn_part == "w" {
                turn = WHITE;
            } else if turn_part == "b" {
                turn = BLACK;
            } else {
                panic!("expected 'w' or 'b' for turn part of fen")
            }
        } else {
            turn = WHITE
        }

        let mut castling = "";
        if let Some(castling_part) = parts.pop_front() {
            castling = castling_part;
            if !FEN_CASTLING_REGEX.is_match(castling_part) {
                panic!("invalid castling part in fen {}", fen);
            }
        } else {
            castling = "-";
        }

        let mut ep_square: Option<u8> = None;
        if let Some(ep_part) = parts.pop_front() {
            if ep_part.len() != 1 {
                panic!("invalid ep_part len {}", ep_part);
            }
            if let Some(sq) = ep_part.chars().nth(0) {
                if sq == '-' {
                    ep_square = None;
                } else if 0x30 <= sq as u8 && 0x38 > sq as u8 {
                    ep_square = Some(sq as u8 - 0x30);
                } else {
                    panic!("Invalid en passant square {}", sq);
                }
            } else {
                panic!("ep_part is not a char");
            }
        } else {
            ep_square = None
        }

        let mut halfmove_clock = 0;
        if let Some(halfmove_part) = parts.pop_front() {
            match halfmove_part.parse::<i64>() {
                Ok(n) => {
                    if n < 0 {
                        panic!("halfmove_clock cannot be negative");
                    }
                    halfmove_clock = n;
                }
                Err(e) => {
                    panic!("invalid halfmove clock in fen: {}", fen);
                }
            }
        } else {
            halfmove_clock = 0
        }

        let mut fullmove_number = 0;
        if let Some(fullmove_part) = parts.pop_front() {
            match fullmove_part.parse::<i64>() {
                Ok(n) => {
                    if n < 0 {
                        panic!("fullmove_number cannot be negative");
                    }
                    fullmove_number = max(n, 1);
                }
                Err(e) => {
                    panic!("invalid halfmove clock in fen: {}", fen);
                }
            }
        } else {
            fullmove_number = 1
        }

        if parts.len() != 0 {
            panic!("fen string has more parts than expected: {}", fen);
        }

        self.baseboard.set_board_fen(board);

        self.turn = turn;
        self._set_castling_fen(castling);
        self.ep_square = ep_square;
        self.halfmove_clock = halfmove_clock as u64;
        self.fullmove_number = fullmove_number as u64;
        self.clear_stack();
    }
    pub fn _set_castling_fen(&mut self, castling_fen: &str) {
        if castling_fen == "-" {
            self.castling_rights = BB_EMPTY;
            return;
        }
        if !FEN_CASTLING_REGEX.is_match(castling_fen) {
            panic!("invalid castling fen {}", castling_fen);
        }

        self.castling_rights = BB_EMPTY;

        for flag in castling_fen.chars().into_iter() {
            let color = if flag.is_ascii_uppercase() {
                WHITE
            } else {
                BLACK
            };
            let flag = flag.to_ascii_lowercase();
            let backrank = if color == WHITE { BB_RANK_1 } else { BB_RANK_8 };
            let rooks =
                self.baseboard.occupied_co[color as usize] & self.baseboard.rooks & backrank;
            let king = self.baseboard.king(color);

            if flag == 'q' {
                if king != None && lsb(rooks) < king.unwrap() {
                    self.castling_rights |= rooks & rooks.wrapping_neg();
                } else {
                    self.castling_rights |= BB_FILE_A & backrank;
                }
            } else if flag == 'k' {
                let rook = msb(rooks);
                if king != None && king.unwrap() < rook as u8 {
                    self.castling_rights |= BB_SQUARES[rook as usize];
                } else {
                    self.castling_rights |= BB_FILE_H & backrank;
                }
            } else {
                self.castling_rights |= BB_FILES[(flag as u8 - 0x30) as usize] & backrank;
            }
        }
    }
    pub fn set_castling_fen(&mut self, castling_fen: &str) {
        self._set_castling_fen(castling_fen);
        self.clear_stack();
    }
    pub fn board_state(&mut self) -> BoardState {
        BoardState::new(self.copy(false))
    }
    pub fn push(&mut self, m: Move) {
        let board_state = self.board_state();
        self.castling_rights = self.clean_castling_rights();
        self.move_stack.push(m);
        self.stack.push(board_state);

        let ep_square = self.ep_square;
        self.ep_square = None;

        self.halfmove_clock += 1;
        if self.turn == BLACK {
            self.fullmove_number += 1;
        }
        if !m.bool() {
            self.turn = !self.turn;
            return;
        }
        if self.is_zeroing(m) {
            self.halfmove_clock = 0;
        }
        let from_bb = BB_SQUARES[m.from_square as usize];
        let to_bb = BB_SQUARES[m.to_square as usize];

        let mut promoted = self.baseboard.promoted & from_bb != 0;
        let mut piece_type = match self.baseboard._remove_piece_at(m.from_square) {
            Some(p) => p,
            None => {
                panic!(
                    "push() expects move to be pseudo-legal, but got {} in {}",
                    m,
                    self.baseboard.board_fen(false)
                )
            }
        };
        let mut capture_square = m.to_square;
        let mut captured_piece_type = self.baseboard.piece_type_at(capture_square);

        self.castling_rights &= !to_bb & !from_bb;
        if piece_type == KING && !promoted {
            if self.turn == WHITE {
                self.castling_rights &= !BB_RANK_1;
            } else {
                self.castling_rights &= !BB_RANK_8;
            }
        } else if captured_piece_type == Some(KING) && !self.baseboard.promoted & to_bb != 0 {
            if self.turn == WHITE && square_rank(m.to_square) == 7 {
                self.castling_rights &= !BB_RANK_8;
            } else if self.turn == BLACK && square_rank(m.to_square) == 0 {
                self.castling_rights &= BB_RANK_1;
            }
        }

        if piece_type == PAWN {
            let diff = m.to_square as i16 - m.from_square as i16;

            if diff == 16 && square_rank(m.from_square) == 1 {
                self.ep_square = Some(m.from_square + 8);
            } else if diff == -16 && square_rank(m.from_square) == 6 {
                self.ep_square = Some(m.from_square - 8);
            } else if m.to_square == ep_square.unwrap()
                && (diff.abs() == 7 || diff.abs() == 0)
                && captured_piece_type != None
            {
                let down = if self.turn == WHITE {
                    (8 as u8).wrapping_neg()
                } else {
                    8
                };
                capture_square = ep_square.unwrap().wrapping_add(down);
                captured_piece_type =
                    Some(self.baseboard._remove_piece_at(capture_square).unwrap());
            }
        }

        if m.promotion != None {
            promoted = true;
            piece_type = m.promotion.unwrap();
        }
        let castling =
            piece_type == KING && ((self.baseboard.occupied_co[self.turn as usize] & to_bb) != 0);
        if castling {
            let a_side = square_file(m.to_square) < square_file(m.from_square);

            self.baseboard._remove_piece_at(m.from_square);
            self.baseboard._remove_piece_at(m.to_square);

            if a_side {
                self.baseboard._set_piece_at(
                    if self.turn == WHITE { C1 } else { C8 },
                    KING,
                    self.turn,
                    false,
                );
                self.baseboard._set_piece_at(
                    if self.turn == WHITE { D1 } else { D8 },
                    ROOK,
                    self.turn,
                    false,
                );
            } else {
                self.baseboard._set_piece_at(
                    if self.turn == WHITE { G1 } else { G8 },
                    KING,
                    self.turn,
                    false,
                );
                self.baseboard._set_piece_at(
                    if self.turn == WHITE { F1 } else { F8 },
                    ROOK,
                    self.turn,
                    false,
                );
            }
        }
        if !castling {
            let was_promoted = self.baseboard.promoted & to_bb != 0;
            self.baseboard
                ._set_piece_at(m.to_square, piece_type, self.turn, promoted);

            // Reserved for Crazyhouse .... supposedly
            // if captured_piece_type {
            //     self.push_capture
            // }
        }
        self.turn = !self.turn;
    }
    pub fn pop(&mut self) -> Move {
        let m = self.move_stack.pop();
        self.stack.pop().unwrap().restore(self);
        m.unwrap()
    }
    pub fn has_pseudo_legal_en_passant(&self) -> bool {
        self.ep_square.bool() && any(self.generate_pseudo_legal_ep(BB_ALL, BB_ALL)) 
    }
    pub fn has_legal_en_passant(&self) -> bool {
        self.ep_square.bool() && any(self.generate_legal_ep(BB_ALL, BB_ALL))
    }
    pub fn is_en_passant(&self, m: Move) -> bool {
        let diff = m.to_square as i16 - m.from_square as i16;
        if let Some(ep_square) = self.ep_square {
            return ep_square == m.to_square && self.baseboard.pawns & BB_SQUARES[m.from_square as usize] != 0
                && (diff == -7 || diff == 7 || diff == 9 || diff == -9) && self.baseboard.occupied & BB_SQUARES[m.to_square as usize] == 0;
        }
        else {
            return false
        }
    }
    pub fn is_capture(&self, m: Move) -> bool {
        let touched = BB_SQUARES[m.from_square as usize] ^ BB_SQUARES[m.to_square as usize];
        touched & self.baseboard.occupied_co[!self.turn as usize] != 0 || self.is_en_passant(m)
    }
    pub fn is_zeroing(&self, m: Move) -> bool {
        let touched = BB_SQUARES[m.from_square as usize] ^ BB_SQUARES[m.to_square as usize];
        touched & self.baseboard.pawns != 0
            || touched & self.baseboard.occupied_co[!self.turn as usize] != 0
    }
    pub fn reduces_castling_rights(&self, m: Move) -> bool {
        let cr = self.clean_castling_rights();
        let touched =  BB_SQUARES[m.from_square as usize] ^ BB_SQUARES[m.to_square as usize];

        touched & cr != 0 || cr & BB_RANK_1 != 0 
        && touched & self.baseboard.kings & self.baseboard.occupied_co[WHITE as usize] & !self.baseboard.promoted != 0
        || cr & BB_RANK_8 != 0
        && touched & self.baseboard.kings & self.baseboard.occupied_co[BLACK as usize] & !self.baseboard.promoted != 0
    }
    pub fn is_irreversible(&self, m: Move) -> bool {
        self.is_zeroing(m) || self.reduces_castling_rights(m) || self.has_legal_en_passant()
    }
    pub fn is_castling(&self, m: Move) -> bool {
        if self.baseboard.kings & BB_SQUARES[m.from_square as usize] != 0 {
            let diff = square_file(m.from_square) as i16 - square_file(m.to_square) as i16;
            return diff.abs() > 1
            || self.baseboard.rooks & self.baseboard.occupied_co[self.turn as usize] & BB_SQUARES[m.to_square as usize] != 0;
        }
        false
    }
    pub fn is_kingside_castling(&self, m: Move) -> bool {
        self.is_castling(m) && square_file(m.to_square) > square_file(m.from_square)
    }
    pub fn is_queenside_castling(&self, m: Move) -> bool {
        self.is_castling(m) && square_file(m.to_square) < square_file(m.from_square)
    }

    pub fn clean_castling_rights(&self) -> Bitboard {
        if !self.stack.is_empty() {
            return self.castling_rights;
        }
        let castling = (self.castling_rights & self.baseboard.rooks);
        let mut white_castling =
            ((castling & BB_RANK_1) & self.baseboard.occupied_co[WHITE as usize]);
        let mut black_castling =
            ((castling & BB_RANK_8) & self.baseboard.occupied_co[BLACK as usize]);
        white_castling &= (BB_A1 | BB_H1);
        black_castling &= (BB_A8 | BB_H8);
        if self.baseboard.occupied_co[WHITE as usize]
            & self.baseboard.kings
            & !self.baseboard.promoted
            & BB_E1
            == 0
        {
            white_castling = 0;
        }
        if self.baseboard.occupied_co[BLACK as usize]
            & self.baseboard.kings
            & !self.baseboard.promoted
            & BB_E8
            == 0
        {
            black_castling = 0;
        }
        return (white_castling | black_castling);
    }
    pub fn has_castling_rights(&self, color: Color) -> bool {
        "Checks if the given side has castling rights.";
        let backrank = if color == WHITE { BB_RANK_1 } else { BB_RANK_8 };
        return self.clean_castling_rights() & backrank != 0;
    }
    pub fn has_kingside_castling_rights(&self, color: Color) -> bool {
        "
        Checks if the given side has kingside (that is h-side in Chess960)
        castling rights.
        ";
        let backrank = if color == WHITE { BB_RANK_1 } else { BB_RANK_8 };
        let king_mask = ((self.baseboard.kings & self.baseboard.occupied_co[color as usize])
            & backrank)
            & !self.baseboard.promoted;
        if king_mask == 0 {
            return false;
        }
        let mut castling_rights = self.clean_castling_rights() & backrank;
        while castling_rights != 0 {
            let rook = castling_rights & castling_rights.wrapping_neg();
            if rook > king_mask {
                return true;
            }
            castling_rights = (castling_rights & (castling_rights - 1));
        }
        return false;
    }
    pub fn has_queenside_castling_rights(&self, color: Color) -> bool {
        "
        Checks if the given side has queenside (that is a-side in Chess960)
        castling rights.
        ";
        let backrank = if color == WHITE { BB_RANK_1 } else { BB_RANK_8 };
        let king_mask = self.baseboard.kings
            & self.baseboard.occupied_co[color as usize]
            & backrank
            & !self.baseboard.promoted;
        if king_mask == 0 {
            return false;
        }
        let mut castling_rights = self.clean_castling_rights() & backrank;
        while castling_rights != 0 {
            let rook = castling_rights & castling_rights.wrapping_neg();
            if rook < king_mask {
                return true;
            }
            castling_rights = castling_rights & (castling_rights - 1);
        }
        return false;
    }
    pub fn status(&self) -> Status {
        "
        Gets a bitmask of possible problems with the position.
    
        :data:`~chess.STATUS_VALID` if all basic validity requirements are met.
        This does not imply that the position is actually reachable with a
        series of legal moves from the starting position.
    
        Otherwise, bitwise combinations of:
        :data:`~chess.STATUS_NO_WHITE_KING`,
        :data:`~chess.STATUS_NO_BLACK_KING`,
        :data:`~chess.STATUS_TOO_MANY_KINGS`,
        :data:`~chess.STATUS_TOO_MANY_WHITE_PAWNS`,
        :data:`~chess.STATUS_TOO_MANY_BLACK_PAWNS`,
        :data:`~chess.STATUS_PAWNS_ON_BACKRANK`,
        :data:`~chess.STATUS_TOO_MANY_WHITE_PIECES`,
        :data:`~chess.STATUS_TOO_MANY_BLACK_PIECES`,
        :data:`~chess.STATUS_BAD_CASTLING_RIGHTS`,
        :data:`~chess.STATUS_INVALID_EP_SQUARE`,
        :data:`~chess.STATUS_OPPOSITE_CHECK`,
        :data:`~chess.STATUS_EMPTY`,
        :data:`~chess.STATUS_RACE_CHECK`,
        :data:`~chess.STATUS_RACE_OVER`,
        :data:`~chess.STATUS_RACE_MATERIAL`,
        :data:`~chess.STATUS_TOO_MANY_CHECKERS`,
        :data:`~chess.STATUS_IMPOSSIBLE_CHECK`.
        ";
        let mut errors = STATUS_VALID;
        if !self.baseboard.occupied != 0 {
            errors |= STATUS_EMPTY;
        }
        if self.baseboard.occupied_co[WHITE as usize] & self.baseboard.kings == 0 {
            errors |= STATUS_NO_WHITE_KING;
        }
        if self.baseboard.occupied_co[BLACK as usize] & self.baseboard.kings == 0{
            errors |= STATUS_NO_BLACK_KING;
        }
        if popcount(self.baseboard.occupied & self.baseboard.kings) > 2 {
            errors |= STATUS_TOO_MANY_KINGS;
        }
        if popcount(self.baseboard.occupied_co[WHITE as usize]) > 16 {
            errors |= STATUS_TOO_MANY_WHITE_PIECES;
        }
        if popcount(self.baseboard.occupied_co[BLACK as usize]) > 16 {
            errors |= STATUS_TOO_MANY_BLACK_PIECES;
        }
        if popcount(self.baseboard.occupied_co[WHITE as usize] & self.baseboard.pawns) > 8 {
            errors |= STATUS_TOO_MANY_WHITE_PAWNS;
        }
        if popcount((self.baseboard.occupied_co[BLACK as usize] & self.baseboard.pawns)) > 8 {
            errors |= STATUS_TOO_MANY_BLACK_PAWNS;
        }
        if self.baseboard.pawns & BB_BACKRANKS != 0 {
            errors |= STATUS_PAWNS_ON_BACKRANK;
        }
        if self.castling_rights != self.clean_castling_rights() {
            errors |= STATUS_BAD_CASTLING_RIGHTS;
        }
        let valid_ep_square = self._valid_ep_square();
        if self.ep_square != valid_ep_square {
            errors |= STATUS_INVALID_EP_SQUARE;
        }
        if self.was_into_check() {
            errors |= STATUS_OPPOSITE_CHECK;
        }
        let checkers = self.checkers_mask();
        let our_kings = self.baseboard.kings & self.baseboard.occupied_co[self.turn as usize] & !self.baseboard.promoted;
        if popcount(checkers) > 2 {
            errors |= STATUS_TOO_MANY_CHECKERS;
        } else {
            if popcount(checkers) == 2 && ray(lsb(checkers), msb(checkers)) & our_kings != 0 {
                errors |= STATUS_IMPOSSIBLE_CHECK;
            } else {
                if valid_ep_square != None
                    && any(scan_reversed(checkers)
                        .map(|checker| ray(checker, valid_ep_square.unwrap()) & our_kings)
                        .collect::<Vec<_>>())
                {
                    errors |= STATUS_IMPOSSIBLE_CHECK;
                }
            }
        }
        Status::to_enum(errors)
    }
    pub fn _valid_ep_square(&self) -> Option<Square> {
        if self.ep_square == None {
            return None;
        }
        let mut ep_rank = 0;
        let mut pawn_mask = 0;
        let mut seventh_rank_mask = 0;
        if self.turn == WHITE {
            ep_rank = 5;
            pawn_mask = shift_down(BB_SQUARES[self.ep_square.unwrap() as usize]);
            seventh_rank_mask = shift_up(BB_SQUARES[self.ep_square.unwrap() as usize]);
        } else {
            ep_rank = 2;
            pawn_mask = shift_up(BB_SQUARES[self.ep_square.unwrap() as usize]);
            seventh_rank_mask = shift_down(BB_SQUARES[self.ep_square.unwrap() as usize]);
        }
        if square_rank(self.ep_square.unwrap()) != ep_rank {
            return None;
        }
        if self.baseboard.pawns & self.baseboard.occupied_co[!self.turn as usize] & pawn_mask == 0 {
            return None;
        }
        if self.baseboard.occupied & BB_SQUARES[self.ep_square.unwrap() as usize] != 0 {
            return None;
        }
        if self.baseboard.occupied & seventh_rank_mask != 0 {
            return None;
        }
        return self.ep_square;
    }
    pub fn is_valid(&self) -> bool {
        "
        Checks some basic validity requirements.

        See :func:`~chess.Board.status()` for details.
        ";
        return self.status() == Status::VALID;
    }
    pub fn _ep_skewered(&self, king: Square, capturer: Square) -> bool {
        assert!(self.ep_square != None);
        let last_double = self.ep_square.unwrap()
            + if self.turn == WHITE {
                (8 as u8).wrapping_neg()
            } else {
                8
            };
        let occupancy = self.baseboard.occupied
            & !BB_SQUARES[last_double as usize]
            & !BB_SQUARES[capturer as usize]
            | BB_SQUARES[self.ep_square.unwrap() as usize];

        let horizontal_attackers = (self.baseboard.occupied_co[!self.turn as usize]
            & (self.baseboard.rooks | self.baseboard.queens));
        if BB_RANK_ATTACKS[king as usize][&(BB_RANK_MASKS[king as usize] & occupancy)]
            & horizontal_attackers
            != 0
        {
            return true;
        }
        let diagonal_attackers = self.baseboard.occupied_co[!self.turn as usize]
            & (self.baseboard.bishops | self.baseboard.queens);
        if (BB_DIAG_ATTACKS[king as usize][&(BB_DIAG_MASKS[king as usize] & occupancy)]
            & diagonal_attackers)
            != 0
        {
            return true;
        }
        return false;
    }
    pub fn _slider_blockers(&self, king: Square) -> Bitboard {
        let rooks_and_queens = self.baseboard.rooks | self.baseboard.queens;
        let bishops_and_queens = self.baseboard.bishops | self.baseboard.queens;
        let snipers = (BB_RANK_ATTACKS[king as usize][&0] & rooks_and_queens)
            | (BB_FILE_ATTACKS[king as usize][&0] & rooks_and_queens)
            | (BB_DIAG_ATTACKS[king as usize][&0] & bishops_and_queens);
        let mut blockers = 0;
        for sniper in scan_reversed(snipers & self.baseboard.occupied_co[!self.turn as usize]) {
            let b = (between(king, sniper as Square) & self.baseboard.occupied);
            if b != 0 && BB_SQUARES[msb(b) as usize] == b {
                blockers |= b;
            }
        }
        return blockers & self.baseboard.occupied_co[self.turn as usize];
    }
    pub fn _is_safe(&self, king: Square, blockers: Bitboard, m: Move) -> bool {
        if m.from_square == king {
            if self.is_castling(m) {
                return true;
            } else {
                return !self.baseboard.is_attacked_by(!self.turn, m.to_square);
            }
        } else {
            if self.is_en_passant(m) {
                return self.baseboard.pin_mask(self.turn, m.from_square)
                    & BB_SQUARES[m.to_square as usize] != 0
                    && !self._ep_skewered(king, m.from_square);
            } else {
                return (blockers & BB_SQUARES[m.from_square as usize]) == 0
                    || (ray(m.from_square, m.to_square) & BB_SQUARES[king as usize]) != 0;
            }
        }
    }
    pub fn generate_evasions(&self, king: Square, checkers: Bitboard, from_mask: Bitboard, to_mask: Bitboard) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            let sliders = checkers & (self.baseboard.bishops | self.baseboard.rooks | self.baseboard.queens);

            let mut attacked = 0;

            for checker in scan_reversed(sliders) {
                attacked |= ray(king, checker) & !BB_SQUARES[checker as usize];
            }

            if BB_SQUARES[king as usize] & from_mask != 0 {
                for to_square in scan_reversed(BB_KING_ATTACKS[king as usize] 
                    & !self.baseboard.occupied_co[self.turn as usize] & !attacked & to_mask){

                    yield Move {from_square: king, to_square: to_square, promotion: None };
                }
            }
            let checker = msb(checkers);
            if BB_SQUARES[checkers as usize] == checkers {
                let target = between(king, checker) | checkers;

                for m in self.generate_pseudo_legal_moves(!self.baseboard.kings & from_mask, target & to_mask) {
                    yield m;
                }

                if self.ep_square != None && !BB_SQUARES[self.ep_square.unwrap() as usize] & target != 0 {
                    let last_double = self.ep_square.unwrap().wrapping_add(if self.turn == WHITE {(8 as u8).wrapping_neg()} else {8});
                    if last_double == checker {
                        for m in self.generate_pseudo_legal_ep(from_mask, to_mask) {
                            yield m;
                        }
                    }
                }
            }
        })
        
    }
    pub fn generate_legal_moves(&self, from_mask: Bitboard, to_mask: Bitboard) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            if self.is_variant_end() { return }

            let king_mask = self.baseboard.kings & self.baseboard.occupied_co[self.turn as usize];

            if king_mask != 0 {
                let king = msb(king_mask);
                let blockers = self._slider_blockers(king);
                let checkers = self.baseboard.attackers_mask(!self.turn, king);
                if checkers != 0 {
                    for m in self.generate_evasions(king, checkers, from_mask, to_mask){
                        if self._is_safe(king, blockers, m) {
                            yield m;
                        }
                    }
                }
                else {
                    for m in self.generate_pseudo_legal_moves(from_mask, to_mask) {
                        if self._is_safe(king, blockers, m){
                            yield m;
                        }
                    }
                }
            }
            else {
                for m in self.generate_pseudo_legal_moves(from_mask, to_mask) {
                    yield m;
                }
            }
        })
    }
    pub fn generate_legal_ep(&self, from_mask: Bitboard, to_mask: Bitboard) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            if self.is_variant_end(){
                return
            }

            for m in self.generate_pseudo_legal_ep(from_mask, to_mask){
                if !self.is_into_check(m) {
                    yield m;
                }
            }
        })
    }
    pub fn generate_legal_captures(&self, from_mask: Bitboard, to_mask: Bitboard) -> impl Iterator<Item = Move> + '_{
        self.generate_legal_moves(from_mask, to_mask & self.baseboard.occupied_co[!self.turn as usize]).chain(
            self.generate_legal_ep(from_mask, to_mask)
        )
    }
    pub fn attacked_for_king(&self, path: Bitboard, occupied: Bitboard) -> bool {
        any(scan_reversed(path).map(|sq| self.baseboard._attackers_mask(!self.turn, sq, occupied)))
    }
    pub fn generate_castling_moves(&self, from_mask: Bitboard, to_mask: Bitboard) -> impl Iterator<Item = Move> + '_{
        gen_iter!({
            if self.is_variant_end(){
                return
            }

            let backrank = if self.turn == WHITE {BB_RANK_1} else {BB_RANK_8};
            let mut king = self.baseboard.occupied_co[self.turn as usize] 
                & self.baseboard.kings & !self.baseboard.promoted & backrank & from_mask;
            king &= king.wrapping_neg();

            if king == 0 {
                return
            }

            let bb_c = BB_FILE_C & backrank;
            let bb_d = BB_FILE_D & backrank;
            let bb_f = BB_FILE_F & backrank;
            let bb_g = BB_FILE_G & backrank;

            for candidate in scan_reversed(self.clean_castling_rights() & backrank & to_mask) {
                let rook = BB_SQUARES[candidate as usize];

                let a_side = rook < king;
                let king_to = if a_side {bb_c} else {bb_g};
                let rook_to = if a_side {bb_d} else {bb_f};

                let king_path = between(msb(king), msb(king_to));
                let rook_path = between(candidate, msb(rook_to));

                if ((self.baseboard.occupied ^ king ^ rook) & (king_path | rook_path | king_to | rook_to) != 0
                    || self.attacked_for_king(king_path | king, self.baseboard.occupied ^ king)
                    || self.attacked_for_king(king_to, self.baseboard.occupied ^ king ^ rook ^ rook_to)) {

                        yield Move{from_square: msb(king), to_square: candidate, promotion: None};
                    }
            }
        })
    }
    pub fn transposition_key(&self) -> Option<Transposition> {
        if self.has_legal_en_passant() {
            return None;
        }
        Some(Transposition{pawns: self.baseboard.pawns, knights: self.baseboard.knights, bishops: self.baseboard.bishops, rooks: self.baseboard.rooks,
        queens: self.baseboard.queens, kings: self.baseboard.kings, occupied_w: self.baseboard.occupied_co[WHITE as usize], 
        occupied_b: self.baseboard.occupied_co[BLACK as usize], turn: self.turn, clean_castling_rights: self.clean_castling_rights(), ep_square: self.ep_square})
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Transposition {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
    occupied_w: Bitboard,
    occupied_b: Bitboard,
    turn: bool,
    clean_castling_rights: Bitboard,
    ep_square: Option<u8>

}
pub struct LegalMoveGenerator {
    board: Board,
}
impl LegalMoveGenerator {
    fn new(board: Board) -> LegalMoveGenerator {
        LegalMoveGenerator { board: board }
    }
}
pub trait IntoSquareSet {
    fn into_square_set(&self) -> SquareSet;
}
macro_rules! add_into_square_set_for_arrays {
    ($a: ty) => {
        impl<const N: usize> IntoSquareSet for [$a; N] {
            fn into_square_set(&self) -> SquareSet {
                let mut mask = 0;
                for square in self.into_iter() {
                    mask |= BB_SQUARES[*square as usize];
                }
                SquareSet { mask: mask }
            }
        }
    };
}
macro_rules! add_into_square_set_for_iterables {
    ($a: ty) => {
        impl IntoSquareSet for $a {
            fn into_square_set(&self) -> SquareSet {
                let mut mask = 0;
                for square in self {
                    mask |= BB_SQUARES[*square as usize];
                }
                SquareSet { mask: mask }
            }
        }
    };
}
add_into_square_set_for_arrays!(Square);
add_into_square_set_for_arrays!(u64);
add_into_square_set_for_iterables!(Vec<u64>);
add_into_square_set_for_iterables!(Vec<u8>);
impl IntoSquareSet for Square {
    fn into_square_set(&self) -> SquareSet {
        SquareSet {
            mask: *self as u64 & BB_ALL,
        }
    }
}
impl IntoSquareSet for u64 {
    fn into_square_set(&self) -> SquareSet {
        SquareSet {
            mask: *self as u64 & BB_ALL,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SquareSet {
    mask: Bitboard,
}
impl SquareSet {
    fn new<I>(squares: I) -> SquareSet
    where
        I: IntoSquareSet,
    {
        squares.into_square_set()
    }
    fn bool(&self) -> bool {
        self.mask != 0
    }
    fn add(&mut self, square: Square) {
        self.mask |= BB_SQUARES[square as usize];
    }
    fn discard(&mut self, square: Square) {
        self.mask &= !BB_SQUARES[square as usize];
    }
    fn isdisjoint<T>(&self, other: T) -> bool
    where
        T: IntoSquareSet,
    {
        !(*self & other).bool()
    }
    fn issubset<T>(&self, other: T) -> bool
    where
        T: IntoSquareSet,
    {
        !(!*self & other).bool()
    }
    fn issuperset<T>(&self, other: T) -> bool
    where
        T: IntoSquareSet,
    {
        (self.mask & !other.into_square_set().mask) != 0
    }
    fn union<T>(&self, other: T) -> SquareSet
    where
        T: IntoSquareSet,
    {
        *self | other
    }
    fn intersection<T>(&self, other: T) -> SquareSet
    where
        T: IntoSquareSet,
    {
        *self & other
    }
    fn difference<T>(&self, other: T) -> SquareSet
    where
        T: IntoSquareSet,
    {
        *self - other
    }
    fn symmetric_difference<T>(&self, other: T) -> SquareSet
    where
        T: IntoSquareSet,
    {
        *self ^ other
    }
    fn symmetric_difference_update<T>(&mut self, others: Vec<T>)
    where
        T: IntoSquareSet,
    {
        for other in others {
            *self ^= other
        }
    }
    fn update<T>(&mut self, others: Vec<T>)
    where
        T: IntoSquareSet,
    {
        for other in others {
            *self |= other;
        }
    }
    fn interseciton_update<T>(&mut self, others: Vec<T>)
    where
        T: IntoSquareSet,
    {
        for other in others {
            *self &= other;
        }
    }
    fn remove(&mut self, square: Square) {
        let mask = BB_SQUARES[square as usize];
        if self.mask & mask != 0 {
            self.mask ^= mask;
        } else {
            panic!("Deleting non-existent square");
        }
    }
    fn pop(&mut self) -> Square {
        if self.mask == 0 {
            panic!("Pop from empty SquareSEt");
        }
        let square = lsb(self.mask);
        self.mask &= self.mask - 1;
        square as Square
    }
    fn clear(&mut self) {
        self.mask = BB_EMPTY
    }
    fn carry_rippler(&self) -> impl Iterator<Item = Bitboard> {
        carry_rippler(self.mask)
    }
    fn mirror(&self) -> SquareSet {
        SquareSet {
            mask: flip_vertical(self.mask),
        }
    }
    fn toarray(&self) -> [bool; 64] {
        let mut result = [false; 64];
        for square in self.into_iter() {
            result[square as usize] = true;
        }
        result
    }
}
impl Iterator for SquareSet {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        scan_forward(self.mask).next()
    }
}
macro_rules! overload_squareset_operator {
    ($big_name: tt, $func: item) => {
       impl<T> ops::$big_name<T> for SquareSet
       where
          T: IntoSquareSet
        {
            type Output = SquareSet;
            $func
        }
    };
}
macro_rules! overload_squareset_operator_assign {
    ($big_name: tt, $func: item) => {
       impl<T> ops::$big_name<T> for SquareSet
       where
          T: IntoSquareSet
        {
            $func
        }
    };
}
overload_squareset_operator!(
    BitOr,
    fn bitor(self, rhs: T) -> Self::Output {
        let mut r = rhs.into_square_set();
        r.mask = r.mask | self.mask;
        r
    }
);
overload_squareset_operator!(
    BitAnd,
    fn bitand(self, rhs: T) -> Self::Output {
        let mut r = rhs.into_square_set();
        r.mask = r.mask + self.mask;
        r
    }
);
overload_squareset_operator!(
    BitXor,
    fn bitxor(self, rhs: T) -> Self::Output {
        let mut r = rhs.into_square_set();
        r.mask = r.mask ^ self.mask;
        r
    }
);
overload_squareset_operator!(
    Sub,
    fn sub(self, rhs: T) -> Self::Output {
        let mut r = rhs.into_square_set();
        r.mask = self.mask & !r.mask;
        r
    }
);
overload_squareset_operator_assign!(
    BitOrAssign,
    fn bitor_assign(&mut self, rhs: T) {
        self.mask |= rhs.into_square_set().mask;
    }
);
overload_squareset_operator_assign!(
    BitAndAssign,
    fn bitand_assign(&mut self, rhs: T) {
        self.mask &= rhs.into_square_set().mask;
    }
);
overload_squareset_operator_assign!(
    SubAssign,
    fn sub_assign(&mut self, rhs: T) {
        self.mask &= !rhs.into_square_set().mask;
    }
);
overload_squareset_operator_assign!(
    BitXorAssign,
    fn bitxor_assign(&mut self, rhs: T) {
        self.mask ^= rhs.into_square_set().mask;
    }
);
impl std::ops::Not for SquareSet {
    type Output = SquareSet;
    fn not(self) -> Self::Output {
        SquareSet { mask: !self.mask }
    }
}
fn main() {
    // let mut b: BaseBoard = BaseBoard::new(None);
    // b.reset_board();
    // println!("{:?}", b.board_fen(false));
    // b.set_board_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    // println!("{:?}", b.board_fen(false));
    // println!("{}", b.unicode(false, false, "."));
    // println!("rays: {:?}", rays()[0]);
    let mut board = Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    println!("{}", board.baseboard.unicode(false, false, "."));
    board.push(Move::from_uci("D2D4"));
    board.push(Move::from_uci("D7D5"));
    board.push(Move::from_uci("C2C4"));
    println!("{}", board.baseboard.unicode(false, false, "."));
    println!("moves: {:#?}", board.generate_legal_moves(BB_ALL, BB_ALL).collect::<Vec<Move>>());
}
