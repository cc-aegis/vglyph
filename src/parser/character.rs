use crate::parser::SyntaxTree;

#[derive(Debug)]
pub struct Character {
    pub is_radical: bool,
    pub names: Vec<String>,
    pub syntax_tree: SyntaxTree,
    pub annotations: Annotations,
}

type Rect = ((f32, f32), (f32, f32));

#[derive(Debug)]
pub struct Annotations {
    pub standalone: Rect,
    pub inner: (Rect, Rect)
}

impl Annotations {
    pub fn new() -> Self {
        Annotations {
            standalone: ((0.0, 0.0), (1.0, 1.0)),
            inner: (((0.0, 0.0), (1.0, 1.0)), ((0.2, 0.2), (0.8, 0.8))),
        }
    }
}