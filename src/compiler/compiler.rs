use std::collections::HashMap;
use crate::compiler::{compile_smooth_line, Line};
use crate::parser::character::Annotations;
use crate::parser::SyntaxTree;

impl SyntaxTree {
    fn weight(&self) -> f32 {
        match self {
            SyntaxTree::Shrink(shrinkage, _) => 1.0 / (*shrinkage + 1) as f32,
            _ => 1.0,
        }
    }

    pub fn compile(&self, reference: &HashMap<String, (Vec<Line>, Annotations)>, bounds: (f32, f32, f32, f32)) -> Vec<Line> {
        match self {
            SyntaxTree::Lines(lines) => {
                let lines = lines
                    .iter()
                    .flat_map(|line| compile_smooth_line(line))
                    .collect();
                fit_inside(lines, bounds)
            },
            SyntaxTree::Bounds(_, _) => {
                Vec::new()
            },
            SyntaxTree::Shrink(_, tree) => {
                tree.compile(reference, bounds)
            },
            SyntaxTree::Plus(trees) => {
                trees.iter()
                    .flat_map(|tree| tree.compile(reference, bounds))
                    .collect()
            },
            SyntaxTree::Square(tree) => {
                let w = bounds.2 - bounds.0;
                let h = bounds.3 - bounds.1;
                let bounds = if h > w {
                    let d = (h - w) / 2.0;
                    (bounds.0, bounds.1 + d, bounds.2, bounds.3 - d)
                } else {
                    let d = (w - h) / 2.0;
                    (bounds.0 + d, bounds.1, bounds.2 - d, bounds.3)
                };
                tree.compile(reference, bounds)
            },
            SyntaxTree::HalfwaySquare(tree) => {
                let w = bounds.2 - bounds.0;
                let h = bounds.3 - bounds.1;
                let bounds = if h > w {
                    let d = (h - w) / 4.0;
                    (bounds.0, bounds.1 + d, bounds.2, bounds.3 - d)
                } else {
                    let d = (w - h) / 4.0;
                    (bounds.0 + d, bounds.1, bounds.2 - d, bounds.3)
                };
                tree.compile(reference, bounds)
            },
            SyntaxTree::Horizontal(trees) => {
                let total_weight: f32 = trees
                    .iter()
                    .map(SyntaxTree::weight)
                    .sum();
                let mut result = Vec::new();
                let mut x: f32 = 0.0;
                for tree in trees {
                    let weight = tree.weight();
                    let width = weight / total_weight * (bounds.2 - bounds.0);
                    let bounds = (x, bounds.1, x + width, bounds.3);
                    result.append(&mut tree.compile(reference, bounds));
                    x += width;
                }
                result
            }
            SyntaxTree::Vertical(trees) => {
                let total_weight: f32 = trees
                    .iter()
                    .map(SyntaxTree::weight)
                    .sum();
                let mut result = Vec::new();
                let mut y: f32 = 0.0;
                for tree in trees {
                    let weight = tree.weight();
                    let height = weight / total_weight * (bounds.3 - bounds.1);
                    let bounds = (bounds.0, y, bounds.2, y + height);
                    result.append(&mut tree.compile(reference, bounds));
                    y += height;
                }
                result
            },
            SyntaxTree::Inner(box SyntaxTree::Bounds(start, end), tree) => {
                dbg!("todo: fit bounds inside current bounds");
                tree.compile(reference, (start.0, start.1, end.0, end.1))
            },
            SyntaxTree::Inner(box SyntaxTree::Ident(outer), inner) => {
                let (outer_lines, Annotations { inner: (outer_rect, inner_rect), .. }) = reference.get(outer).unwrap();
                let outer = fit_inside(outer_lines.clone(), (outer_rect.0.0, outer_rect.0.1, outer_rect.1.0, outer_rect.1.1));
                let inner = inner.compile(reference, (inner_rect.0.0, inner_rect.0.1, inner_rect.1.0, inner_rect.1.1));
                let combined = [outer, inner].concat();
                fit_inside(combined, bounds)
            },
            SyntaxTree::Ident(ident) => {
                dbg!("todo: remove unwrap");
                fit_inside(reference.get(ident).unwrap().0.clone(), bounds)
            },
            _ => todo!()
        }
    }
}

fn fit_inside(mut lines: Vec<((f32, f32), (f32, f32))>, bounds: (f32, f32, f32, f32)) -> Vec<((f32, f32), (f32, f32))> {
    let x = bounds.0;
    let y = bounds.1;
    let w = bounds.2 - bounds.0;
    let h = bounds.3 - bounds.1;
    lines.iter_mut()
        .for_each(|(from, to)| {
            from.0 = from.0 * w + x;
            from.1 = from.1 * h + y;
            to.0 = to.0 * w + x;
            to.1 = to.1 * h + y;
        });
    lines
}