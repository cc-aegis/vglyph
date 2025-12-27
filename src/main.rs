use vglyph::compile;

// TODO: run cli from here

fn main() {
    let input = include_str!("../res/beta.glyph");
    let map = compile(input);

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