
#![feature(const_for)]
#![feature(core_intrinsics)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![allow(dead_code)]
#![macro_use]
use core::panic;
use std::{collections::HashSet};

extern crate lazy_static;
extern crate auto_ops;
mod init;
mod pgn;
mod gen_iter;
use init::{Board, Move};


fn main() {

    // let mut b: BaseBoard = BaseBoard::new(None);
    // b.reset_board();
    // println!("{:?}", b.board_fen(false));
    // b.set_board_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    // println!("{:?}", b.board_fen(false));
    // println!("{}", b.unicode(false, false, "."));
    // println!("rays: {:?}", rays()[0]);
    // let mut board = Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    // println!("{}", board.baseboard.unicode(false, false, "."));
    // board.push(Move::from_uci("D2D4"));
    // board.push(Move::from_uci("D7D5"));
    // board.push(Move::from_uci("C2C4"));
    // println!("{}", board.baseboard.unicode(false, false, "."));
    // println!("moves: {:#?}", board.generate_legal_moves(init::BB_ALL, init::BB_ALL).collect::<Vec<Move>>());
    use pgn::Node;
    let mut a = Node::new("hey");
    let mut b = a.add_variation(Move::from_uci("D2D4"), "comment", "starting", HashSet::new());
    let mut c = b.add_variation(Move::from_uci("D7D5"), "comment2", "starting2",HashSet::new());
    c = c.add_variation(Move::from_uci("C2C4"), "fdsa", "hdsa",HashSet::new());
    let board = c.board();
    println!("{}", board.baseboard.unicode(board.turn, false, "."));
}

// extern crate test;
// use test::Bencher;

// #[bench]
// fn bench_cp(b :&mut Bencher) {
//     test::black_box(
//          b.iter( || {
//             let a = crate::pgn::Node::new("hey");
//              (0..10000000).map(move |x| {
//                  &a.0;
//              })
//          })
//     )
// }
