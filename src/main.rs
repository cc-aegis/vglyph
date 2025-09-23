#![feature(box_patterns)]

mod compiler;
pub mod lexer;
pub mod parser;

fn main() {
    let input = include_str!("../res/beta.glyph");
    let map = compiler::compile(input);

    dbg!(&map);
}







/*
tree := horizontal + horizontal + ..
horizontal := vertical | vertical | ..
vertical := shrink / shrink / ..
shrink := ..&&&inner
inner := value or value { tree }
value := arrow or (tree) or [tree] or [[tree]] or ident
arrow := <line> or <block>
line := xy [[-> or ~>] xy]*
block := xy => xy

 */