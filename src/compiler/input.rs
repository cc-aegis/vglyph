use std::collections::HashMap;
use crate::parser::character::Character;
use crate::parser::SyntaxTree;

impl Character {
    pub fn get_input(&self, reference: &HashMap<String, Vec<String>>) -> Vec<String> {
        if self.is_radical {
            vec![self.names[0].clone()]
        } else {
            self.syntax_tree.get_input(reference)
        }
    }
}

impl SyntaxTree {
    fn get_input(&self, reference: &HashMap<String, Vec<String>>) -> Vec<String> {
        use SyntaxTree as T;
        match self {
            T::Ident(ident) => {
                // TODO: get rid of unwrap
                reference.get(ident).unwrap().clone()
            }
            T::Lines(_) => vec![String::from("stroke")],
            T::Bounds(_, _) => Vec::new(),
            T::Plus(parts) | T::Horizontal(parts) | T::Vertical(parts) => {
                parts.iter().flat_map(|tree| tree.get_input(reference)).collect()
            },
            T::Inner(outer, inner) => {
                [outer.get_input(reference), inner.get_input(reference)].concat()
            }
            T::Shrink(_, tree) | T::Square(tree) | T::HalfwaySquare(tree) => {
                tree.get_input(reference)
            },
        }
    }
}