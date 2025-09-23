
#[derive(Debug)]
pub enum SyntaxTree {
    Ident(String),
    Lines(Vec<Vec<(f32, f32)>>),
    Bounds((f32, f32), (f32, f32)),
    Plus(Vec<SyntaxTree>),
    Horizontal(Vec<SyntaxTree>),
    Vertical(Vec<SyntaxTree>),
    Inner(Box<SyntaxTree>, Box<SyntaxTree>),
    Shrink(usize, Box<SyntaxTree>),
    Square(Box<SyntaxTree>),
    HalfwaySquare(Box<SyntaxTree>),
}
