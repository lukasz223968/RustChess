

use std::{cell::RefCell, collections::{HashMap, HashSet, VecDeque}, io::Error, rc::Rc};
use std::fs::File;
use std::io::{self, prelude::*};

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
create_regex!(TAG_REGEX, r#"^\[([A-Za-z0-9_]+)\s+"([^\r]*)"\]\s*$"#);
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
pub struct Node (
    pub NodeRef
);
pub struct Game {
    pub root: Node,
    pub headers: Headers
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::new();
        let mut ptr = self.0.clone();
        while !ptr.borrow().variations.is_empty() {
            builder.push_str(&ptr.borrow().variations.index(0).borrow().m.unwrap().uci());
            builder.push(' ');
            ptr = ptr.clone().borrow().variations.index(0).clone();
        }
        write!(f, "Game: {}", builder)
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
    pub fn is_end(&self) -> bool {
        self.0.borrow().variations.is_empty()
    }
    pub fn starts_variation(&self) -> bool {
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
    fn _accept(&self, parent_board: &mut Board, mut visitor: &mut GameBuilder, sidelines: bool) {
        let mut stack = Vec::from([Rc::new(RefCell::new(AcceptFrame::new(self.0.clone(), false, sidelines)))]); 

        while !stack.is_empty() {
            let topref = stack.last().unwrap().clone();
            let mut top = topref.borrow_mut();

            if top.in_variation {
                top.in_variation = false;
                visitor.end_variation();
            }
            if top.state == "pre" {
                top.node.borrow().accept_node(parent_board, visitor);
                top.state = "variations".to_string();
            }    
            else if top.state == "variations" {
                let var_opt = top.variations.iter().next();

                if let Some(variation) = var_opt {
                    if visitor.begin_variation() == () {
                        stack.push(Rc::new(RefCell::new(AcceptFrame::new(variation.clone(), true, false))));
                    }
                    top.in_variation = true;
                }
                else {
                    if !top.node.borrow().variations.is_empty() {
                        parent_board.push(top.node.borrow().m.unwrap());
                        stack.push(Rc::new(RefCell::new(AcceptFrame::new(top.node.borrow().variations.front().unwrap().clone(), false, true))));
                        top.state = "post".to_string();
                    }
                    else { top.state = "end".to_string(); }
                }
            }
            else if top.state == "post" {
                parent_board.pop();
                top.state = "end".to_string();
            }
            else { stack.pop(); }

        }
    }
    pub fn accept(&mut self, mut visitor: GameBuilder) -> GameBuilder {
        let mut parent_board = Node(self.0.borrow().parent.as_ref().unwrap().clone()).board();
        self._accept(&mut parent_board, &mut visitor, false);
        visitor
    }
}
impl NodeBase {
    fn accept_node(&self, mut parent_board: &mut Board, mut visitor: &mut GameBuilder) {
        if !self.starting_comment.is_empty() {
            visitor.visit_comment(&self.starting_comment);
        }
        visitor.visit_move(&parent_board, self.m.unwrap());

        parent_board.push(self.m.unwrap());
        visitor.visit_board(&Node(self.parent.as_ref().unwrap().clone()).board());
        parent_board.pop();

        let mut nags =  self.nags.iter().collect::<Vec<&u64>>();
        nags.sort();
        for nag in nags {
            visitor.visit_nag(*nag);
        }
        if !self.comment.is_empty() {
            visitor.visit_comment(&self.comment);
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
#[derive(Debug)]
pub struct Headers{
    tag_roaster: HashMap<String, String>,
    others: HashMap<String, String>,
    data: HashMap<String, String>
}
impl Headers {
    pub fn new(data: Option<HashMap<String, String>>) -> Headers{ 
        let mut d = data;
        if d.is_none() {
            d = Some(HashMap::from([
                ("Event", "?"),
                ("Site", "?"),
                ("Round", "????.??.??"),
                ("White", "?"),
                ("Black", "?"),
                ("Result", "*")
            ].map(|x| (String::from(x.0), String::from(x.1)))))
        }
        Headers { tag_roaster: HashMap::new(), others: HashMap::new(), data: d.unwrap() }
    }
    pub fn set(&mut self, key: &str, value: &str){
        if TAG_ROASTER.contains(&key) {
            self.tag_roaster.insert(key.to_string(), value.to_string());
        }
        else if !TAG_NAME_REGEX.is_match(key) {
            panic!("non alphanumeric pgn header tag: {}", key);
        }
        else if value.contains("\n") || value.contains("\r") {
            panic!("line break in pgn header {}", value)
        }
        else {
            self.others.insert(key.to_string(), value.to_string());
        }
    }
    pub fn get(&self, key: &str) -> Option<&str>{
        if TAG_ROASTER.contains(&key){
            return Some(self.tag_roaster.get(key).unwrap())
        }
        if let Some(a) = self.others.get(key) { Some(a) } else { None }
    }
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        gen_iter!({
            for key in TAG_ROASTER {
                if self.tag_roaster.contains_key(key) {
                    yield key;
                }
            }
        })
    }
}
pub enum SkipType {
    SKIP = 0
}

struct AcceptFrame {
    state: String,
    node: NodeRef,
    in_variation: bool,
    variations: VecDeque<NodeRef>,

}
impl AcceptFrame{
    fn new(node: NodeRef, in_variation: bool, sidelines: bool) -> AcceptFrame {
        let mut frame = AcceptFrame { state: "pre".to_string(), node: node.clone(), in_variation, variations: VecDeque::new()};
        if sidelines {
            let mut slice = node.borrow().parent.as_ref().unwrap().borrow().variations.clone();
            slice.pop_front();
            frame.variations = slice; 
        }
        frame.in_variation = false;
        frame
    }
}
pub trait BaseVisitor {
    fn begin_game(&self) -> Option<SkipType>;
    fn begin_headers(&self) -> Option<Headers>;
    fn visit_header(&self, tagname: &str, tagvalue: &str);
    fn end_headers(&self) -> Option<SkipType>;
    fn parse_san(&self, board: Board, san: &str) -> Move;
    fn visit_move(&self, board: Board, m: Move);
    fn visit_board(&self, board: Board);
    fn visit_comment(&self, comment: &str);
    fn visit_nag(&self, nag: u64);
    fn begin_variation(&self) -> Option<SkipType>;
    fn end_variation(&self);
    fn visit_result(&self, result: &str);
    fn end_game(&self);
    fn result<T>(&self) -> T;
    fn handle_error(&self, error: &str);
}
pub struct GameBuilder {
    game: Game,
    variation_stack: Vec<NodeRef>,
    starting_comment: String,
    in_variation: bool
}
impl GameBuilder {
    fn new() -> GameBuilder {
        GameBuilder {
            game: Game{root: Node::new(""), headers: Headers::new(None)},
            variation_stack: Vec::new(),
            starting_comment: String::new(),
            in_variation: false
        }
    }
    fn begin_game(&mut self) -> Option<SkipType>{
        self.game = Game{root: Node::new(""), headers: Headers::new(None)};
        self.variation_stack = Vec::new();
        self.variation_stack.push(self.game.root.0.clone());
        self.starting_comment = String::new();
        self.in_variation = false;
        None
    }
    fn new_and_begin() -> GameBuilder {
        let game = Game{root: Node::new(""), headers: Headers::new(None)};
        let mut variation_stack = Vec::new();
        variation_stack.push(game.root.0.clone());
        let starting_comment = String::new();
        let in_variation = false;
        GameBuilder {
            game,
            variation_stack,
            starting_comment,
            in_variation
        }
    }
    fn begin_headers(&self) -> Option<&Headers>{
        Some(&self.game.headers)
    }
    fn visit_header(&mut self, tagname: &str, tagvalue: &str){
        self.game.headers.set(tagname, tagvalue)
    }
    // fn end_headers(&self) -> Option<SkipType>;
    // fn parse_san(&self, board: Board, san: &str) -> Move;
    fn visit_nag(&self, nag: u64) -> Option<SkipType> {
        self.variation_stack.last().unwrap().borrow_mut().nags.insert(nag);
        None
    }
    fn begin_variation(&mut self){
        if let Some(parent) = self.variation_stack.last().cloned().unwrap().borrow().parent.clone() {
            self.variation_stack.push(parent.clone());
            self.in_variation = false;
        }
        else {
            panic!("begin variation called, but root node on top of stack");
        }
    }
    fn end_variation(&mut self) {
        self.variation_stack.pop();
    }
    fn visit_result(&mut self, result: &str) {
        if let Some(key) = self.game.headers.get("Result"){
            self.game.headers.set("Result", result);
        }
    }
    fn visit_comment(&mut self, comment: &str){
        if self.in_variation || (self.variation_stack.last().unwrap().borrow().parent.is_some() && Node(self.variation_stack.last().unwrap().clone()).is_end()){
            self.starting_comment = String::from(self.variation_stack.last().unwrap().borrow().starting_comment.clone());
        }
        self.starting_comment.push('\n');
        self.starting_comment.push_str(comment.trim());

    }
    fn visit_move(&mut self, board: &Board, m: Move){
        let last_copy = self.variation_stack.pop().unwrap();
        self.variation_stack.push(Node(last_copy).add_variation(m, "", "", HashSet::new()).0);

        self.variation_stack.last().unwrap().borrow_mut().starting_comment = self.starting_comment.clone();
        self.starting_comment = "".to_string();
        self.in_variation = true;
    }
    fn visit_board(&self, board: &Board){}
    fn end_game(&self){}
    pub fn result(&self) -> &Game {
        &self.game
    }
    pub fn parse_san(&self, board: &Board, san: &str) -> Move {
        board.parse_san(san)
    }
    // fn handle_error(&self, error: &str);
}
pub struct BufReader {
    reader: io::BufReader<File>,
}

impl BufReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        Ok(Self { reader })
    }

    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> Option<io::Result<&'buf mut String>> {
        buffer.clear();

        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }
}
fn isspace(s: &str) -> bool {
    s.chars().all(|x| x.is_whitespace())
}
fn read_line_or_empty<'buf>(handle: &mut BufReader, buffer: &'buf mut String ) -> &'buf str {
    match handle.read_line(buffer) {Some(s) => {s.expect("error while reading line")}, None => {""}}
}
pub fn read_game(mut handle: BufReader) -> Result<Rc<RefCell<GameBuilder>> , std::io::Error> {
    let mut visitor = Rc::new(RefCell::new(GameBuilder::new_and_begin()));

    let mut found_game = false;
    let mut skipping_game = false;
    // let mut headers: Option<&Headers> = None;

    let mut buffer = String::new();

    let mut board: Board = Board::new(None);

    let mut line = handle.read_line(&mut buffer).expect("Couldnt read first line of the file")?.trim_start_matches("\u{feff}");

    while isspace(line) || line.starts_with("%") || line.starts_with(";") {
        line = handle.read_line(&mut buffer).expect("error while reading the line")?;
    }
    
    let mut consecutive_empty_lines = 0;

    while !line.is_empty() {
        if line.starts_with("%") || line.starts_with(";") {
            line = read_line_or_empty(&mut handle, &mut buffer);
            continue;
        }

        if consecutive_empty_lines < 1 && isspace(line) {
            consecutive_empty_lines += 1;
            line = read_line_or_empty(&mut handle, &mut buffer);
            continue;
        }

        if !found_game {
            found_game = true;
            skipping_game = false;

            // if !skipping_game {
            //     headers = Some(&visitor.borrow().game.headers);
            // }
        }
        if !line.starts_with("[") { break; }

        consecutive_empty_lines = 0;

        if !skipping_game {
            let tag_match = TAG_REGEX.captures(line);
            if let Some(tag) = tag_match {
                visitor.borrow_mut().visit_header(&tag[1], &tag[2]);
            }
        }
        line = read_line_or_empty(&mut handle, &mut buffer);

    }
    if !found_game { return Err(Error::new(io::ErrorKind::Other, "Empty game")); }

    if !skipping_game {
        skipping_game = false;
    }

    if !skipping_game {
        
        let temp_v = visitor.borrow();
        // let fen = match temp_v.game.headers.get("FEN") { Some(v) => {v.clone()}, None => {"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"}};
        // board = Board::new(Some(crate::init::STARTING_FEN));
        board.reset();
        //visitor.borrow_mut().visit_board(&board);
    }

    //if skipping_game { CODE }

    let skip_variation_depth = 0;

    while !line.is_empty() {
        let read_next_line = true;

        if line.starts_with("%") || line.starts_with(";"){
            line = read_line_or_empty(&mut handle, &mut buffer);
            continue;
        }
        if isspace(line) {
            visitor.borrow_mut().end_game();
            return Ok(visitor);
        }

        for re_match in MOVETEXT_REGEX.find_iter(line) {
            let token = re_match.as_str();

            if token.starts_with("{") {

            }
            else if ["1-0", "0-1", "1/2-1/2", "*"].contains(&token) {
                visitor.borrow_mut().visit_result(token);
            }
            else {
                let m = visitor.borrow().parse_san(&board, token);
                visitor.borrow_mut().visit_move(&board, m);
                board.push(m);
                visitor.borrow().visit_board(&board);
            }
        }
        if read_next_line {
            line = read_line_or_empty(&mut handle, &mut buffer);
        }
    }
    Ok(visitor)
}






