

use std::{cell::RefCell, collections::{HashSet, VecDeque}, rc::Rc};

use regex::Regex;
use crate::{gen_iter, init::{Move, Board, BaseBoard, Color}};
use lazy_static::lazy_static;
use std::ops::Index;


const NAG_NULL: u8 = 0;
const NAG_GOOD_MOVE: u8 = 1;
const NAG_MISTAKE: u8 = 2;
const NAG_BRILLIANT_MOVE: u8 = 3;
const NAG_BLUNDER: u8 = 4;
const NAG_SPECULATIVE_MOVE: u8 = 5;
const NAG_DUBIOUS_MOVE: u8 = 6;
const NAG_FORCED_MOVE: u8 = 7;
const NAG_SINGULAR_MOVE: u8 = 8;
const NAG_WORST_MOVE: u8 = 9;
const NAG_DRAWISH_POSITION: u8 = 10;
const NAG_QUIET_POSITION: u8 = 11;
const NAG_ACTIVE_POSITION: u8 = 12;
const NAG_UNCLEAR_POSITION: u8 = 13;
const NAG_WHITE_SLIGHT_ADVANTAGE: u8 = 14;
const NAG_BLACK_SLIGHT_ADVANTAGE: u8 = 15;
const NAG_WHITE_MODERATE_ADVANTAGE: u8 = 16;
const NAG_BLACK_MODERATE_ADVANTAGE: u8 = 17;
const NAG_WHITE_DECISIVE_ADVANTAGE: u8 = 18;
const NAG_BLACK_DECISIVE_ADVANTAGE: u8 = 19;
const NAG_WHITE_ZUGZWANG: u8 = 22;
const NAG_BLACK_ZUGZWANG: u8 = 23;
const NAG_WHITE_MODERATE_COUNTERPLAY: u8 = 132;
const NAG_BLACK_MODERATE_COUNTERPLAY: u8 = 133;
const NAG_WHITE_DECISIVE_COUNTERPLAY: u8 = 134;
const NAG_BLACK_DECISIVE_COUNTERPLAY: u8 = 135;
const NAG_WHITE_MODERATE_TIME_PRESSURE: u8 = 136;
const NAG_BLACK_MODERATE_TIME_PRESSURE: u8 = 137;
const NAG_WHITE_SEVERE_TIME_PRESSURE: u8 = 138;
const NAG_BLACK_SEVERE_TIME_PRESSURE: u8 = 139;
const NAG_NOVELTY: u8 = 146;

macro_rules! create_regex{
    ($name: ident, $s: tt) => {
        lazy_static!(
        pub static ref $name: Regex = {
            let regex = Regex::new($s);
            regex.unwrap()
        };
    );
    }
}
create_regex!(TAG_REGEX, r#"^\[([A-Za-z0-9_]+)\s+\"([^\r]*)\"\]\s*$"#);
create_regex!(TAG_NAME_REGEX, r"^[A-Za-z0-9_]+\Z");
create_regex!(MOVETEXT_REGEX, r"(?s)([NBKRQ]?[a-h]?[1-8]?[\-x]?[a-h][1-8](?:=?[nbrqkNBRQK])?|[PNBRQK]?@[a-h][1-8]|--|Z0|0000|@@@@|O-O(?:-O)?|0-0(?:-0)?)|(\{.*)|(;.*)|(\$[0-9]+)|(\()|(\))|(\*|1-0|0-1|1/2-1/2)|([\?!]{1,2})");
create_regex!(SKIP_MOVETEXT_REGEX, r";|\{|\}");
create_regex!(CLOCK_REGEX, r"\[%clk\s(\d+):(\d+):(\d+(?:\.\d*)?)\]");
create_regex!(EVAL_REGEX, r"\[%eval\s(?:\#([+-]?\d+)|([+-]?(?:\d{0,10}\.\d{1,2}|\d{1,10}\.?)))(?:,(\d+))?\]");
create_regex!(ARROWS_REGEX, r"\[%(?:csl|cal)\s([RGYB][a-h][1-8](?:[a-h][1-8])?(?:,[RGYB][a-h][1-8](?:[a-h][1-8])?)*)\]");

pub const TAG_ROASTER: [&str; 7] = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];
#[derive(PartialEq)]
pub struct NodeBase {
    pub is_root: bool,
    pub parent: Option<NodeRef>,
    pub m: Option<Move>,
    pub variations: VecDeque<NodeRef>,
    pub comment: String,
    pub starting_comment: String,
    pub nags: HashSet<u64>,
}
type NodeRef = Rc<RefCell<NodeBase>>;
#[derive(Debug)]
pub struct Node (
    pub NodeRef
);
impl std::fmt::Debug for NodeBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}",self.comment, self.starting_comment, self.variations)
    }
}
impl<'a> PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}
impl Node {
    pub fn new(comment: &str) -> Node {
        Node (
            Rc::new(RefCell::new(NodeBase {
                is_root: false,
                parent: None,
                m: None,
                variations: VecDeque::new(),
                comment: String::from(comment),
                starting_comment: String::new(),
                nags: HashSet::new()
            }))
        )
    }
    pub fn init(parent:NodeRef , m: Move, comment: &str, starting_comment: &str, nags: HashSet<u64>) -> NodeRef{
        let node = Node::new(comment).0;
        node.borrow_mut().m = Some(m);
        node.borrow_mut().starting_comment = String::from(starting_comment);
        parent.borrow_mut().variations.push_front(node.clone());
        node.borrow_mut().parent = Some(parent);
        node
    }
    pub fn ply(&self) -> u64 {
        return 5;
    }
    pub fn turn(&self) -> Color {
        self.ply() % 2 == 0
    }
    pub fn root(&self) -> NodeRef {
        let mut node = self.0.clone();
        loop {
            match node.clone().borrow().parent.clone() {
                None => {break;},
                Some(n) => { node = n} 
            };
        };
        node
    }
    pub fn game(&self) -> NodeRef {
        let root = self.root();
        assert!(root.borrow().is_root);
        root
    }
    pub fn end(self) -> NodeRef {
        let mut node =self.0;
        while !node.clone().borrow().variations.is_empty() {
            node = node.clone().borrow().variations[0].clone();
        }
        node
    }
    fn is_end(&self) -> bool {
        self.0.borrow().variations.is_empty()
    }
    fn starts_variation(&self) -> bool {
        if !self.0.borrow().parent.is_none() {
            return false;
        }
        if !self.0.borrow().parent.as_ref().unwrap().borrow().variations.is_empty() {
            return false;
        }
        self.0.borrow().parent.as_ref().unwrap().borrow().variations[0] == self.0
    }
    pub fn is_mainline(&self) -> bool {
        let mut node = self.0.clone();
        while let Some(parent) = node.clone().borrow().parent.clone() {
            if parent.borrow().variations.is_empty() || parent.borrow().variations[0] != node {
                return false;
            }
            node = parent;
        }
        true
    }
    pub fn is_main_variation(&self) -> bool {
        if let Some(parent) = self.0.borrow().parent.as_ref() {
            return parent.borrow().variations.is_empty() || parent.borrow().variations[0] == self.0;
        }
        true
    }
    pub fn variation(&self, m: MoveRepr) -> NodeRef {
        self.index(m).clone()
    }
    pub fn has_variation(&self, m: Move) -> bool {
        for variation in &self.0.borrow().variations {
            if variation.borrow().m.unwrap() == m {
                return true;
            }
        }
        false
    }
    pub fn has_variation_node(&self, node: NodeRef) -> bool {
        for variation in &self.0.borrow().variations {
            if *variation == node {
                return true;
            }
        }
        false
    }
    pub fn promote_to_main(&mut self, m: MoveRepr) {
        let variation = self.index(m);
        let index= self.0.borrow().variations.iter().position(|x| *x == variation).unwrap();
        let temp = self.0.borrow_mut().variations.remove(index).unwrap();
        self.0.borrow_mut().variations.push_front(temp);
    }
    pub fn promote(&mut self, m: MoveRepr) {
        let variation = self.index(m);
        let index= self.0.borrow().variations.iter().position(|x| *x == variation).unwrap();
        if index > 0 {
            self.0.borrow_mut().variations.swap(index - 1, index);
        }
    }
    pub fn demote(&mut self, m: MoveRepr) {
        let variation = self.index(m);
        let index= self.0.borrow().variations.iter().position(|x| *x == variation).unwrap();
        if index < self.0.borrow().variations.len() - 1 {
            self.0.borrow_mut().variations.swap(index + 1, index)
        }
    }
    pub fn remove_variation(&mut self, m: MoveRepr) {
        let index= self.0.borrow().variations.iter().position(|x| *x == self.variation(m.clone())).unwrap();
        self.0.borrow_mut().variations.remove(index);
    }
    pub fn add_variation(&mut self, m: Move, comment: &str, starting_comment: &str, nags: HashSet<u64>) -> Node {
        let n = Node::init(self.0.clone(), m, comment, starting_comment, nags);
        Node(n)
    }
    pub fn add_main_variation(&mut self, m: Move, comment: &str, nags: HashSet<u64>) -> Node {
        let val= self.0.borrow_mut().variations.pop_back().unwrap();
        self.0.borrow_mut().variations.push_front(val);
        let node = self.add_variation(m, comment, "", nags);
        node
    }
    pub fn next(&self) -> Option<NodeRef> {
        if !self.0.borrow().variations.is_empty() {Some(self.0.borrow().variations[0].clone())} else {None}
    }
    pub fn mainline(&self) -> Mainline<NodeRef> {
        Mainline {start: self.0.clone(), f: |node| node}
    }
    pub fn mainline_moves(&self) -> Mainline<Option<Move>> {
        Mainline {start: self.0.clone(), f: |node| node.borrow().m}
    }
    pub fn add_line<T>(&self, moves: T, comment: &str, starting_comment: &str, nags: HashSet<u64>) -> Node where T: IntoIterator<Item = Move> {
        let mut node = Node(self.0.clone()); 
        for m in moves {
            node = node.add_variation(m, "", starting_comment, HashSet::new());
        }
        node
    }
    pub fn board(&self) -> Board {
        let mut stack: Vec<Move> = Vec::new();
        let mut node = self.0.clone();
        while node.borrow().m.is_some() && node.borrow().parent.is_some() {
            stack.push(node.borrow().m.clone().unwrap());
            node = node.clone().borrow().parent.as_ref().unwrap().clone();
        }

        let mut board= Board::new(None);
        board.reset();
        while !stack.is_empty() {
            board.push(stack.pop().unwrap());
        }
        board
    }
    pub fn index(&self, m: MoveRepr) -> Rc<RefCell<NodeBase>> {
        match m {
            MoveRepr::Int(x) => {
                self.0.borrow().variations[x].clone()
            },
            MoveRepr::NodeRef(n) => {
                let mut retval = None;
                for variation in self.0.borrow().variations.clone() {
                    if variation == n {
                        retval = Some(variation);
                    }
                }
                assert!(retval.is_some());
                retval.unwrap()
            },
            MoveRepr::Move(m) => {
                let mut retval = None;
                for variation in self.0.borrow().variations.clone() {
                    if variation.borrow().m.unwrap() == m {
                        retval = Some(variation);
                    }
                }
                assert!(retval.is_some());
                retval.unwrap()
            }
        }
    }
}
#[derive(Clone)]
pub enum MoveRepr {
    Int(usize), Move(Move), NodeRef(NodeRef)
}
pub struct Mainline<T> {
    start: NodeRef,
    f: fn(n: NodeRef) ->  T,
}
impl<T> Mainline<T> {
    fn new(start: NodeRef, f: fn(n: NodeRef) -> T) -> Mainline<T>{
        Mainline{start, f}
    }
    fn bool(&self) -> bool {
        !self.start.borrow().variations.is_empty()
    }
    fn iter(&'_ self) -> impl Iterator<Item = T> + '_ {
        gen_iter!({
            let mut node = self.start.clone();
            while !node.borrow().variations.is_empty() {
                node = node.clone().borrow().variations[0].clone();
                yield (self.f)(node.clone());
            }
        })
        
    }
}