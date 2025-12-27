mod compiler;

use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::parser::character::Annotations;
use crate::parser::Parser;


type Line = ((f32, f32), (f32, f32));
type Rect = (f32, f32, f32, f32);

pub fn compile(src: &str) -> HashMap<String, Vec<Line>> {
    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer.peekable());
    let mut map: HashMap<String, (Vec<Line>, Annotations)> = HashMap::new();

    while let Some(mut next) = parser.next() {
        let lines = next.syntax_tree.compile(&map, (0.0, 0.0, 1.0, 1.0));
        map.insert(next.names.remove(0), (lines, next.annotations));
    }

    // TODO: apply standalone bounds
    map.into_iter().map(|(k, (v, _))| (k, v)).collect()
}

fn interpolate(progress: f32, dots: &[(f32, f32)]) -> Option<(f32, f32)> {
    match dots {
        [] => None,
        [xy] => Some(*xy),
        dots => {
            let dots: Vec<_> = (1..dots.len())
                .map(|idx| (
                    dots[idx-1].0 * (1.0 - progress) + dots[idx].0 * progress,
                    dots[idx-1].1 * (1.0 - progress) + dots[idx].1 * progress,
                ))
                .collect();
            interpolate(progress, &dots)
        }
    }
}


fn compile_smooth_line(dots: &[(f32, f32)]) -> Vec<Line> {
    if let [start, end] = dots {
        return vec![(*start, *end)];
    }

    const LINE_STEPS: usize = 20;
    (0..LINE_STEPS + 1)
        .flat_map(|idx| interpolate(idx as f32 / LINE_STEPS as f32, dots))
        .collect::<Vec<_>>()
        .windows(2)
        .map(|xyxy| (xyxy[0], xyxy[1]))
        .collect()
}

/*
def step(m: float, points: [(float, float)]):
    if len(points) == 1:
        return points[0]
    else:
        result = []
        for idx in range(len(points)-1):
            result.append((points[idx][0] * (1-m) + points[idx+1][0] * m, points[idx][1] * (1-m) + points[idx+1][1] * m))
        return step(m, result)

def round_line(steps: [(float, float)]) -> [(float, float)]:
    result = []
    for i in range(101):
        result.append(step(i/100, steps))
    return result
 */