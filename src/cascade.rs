//! What CascadeSelect cannot do in `.slint`: walk a tree. Ported from ely-core
//! `forms/cascade.rs` (`columns`, `trail_of`, `fit_trail`, `nodes`). Slint structs cannot nest,
//! so the tree arrives flat: each node names its parent's index, -1 for a root, and a level's
//! nodes keep their order in the list.

/// A flat node: its value, label and parent index.
#[derive(Clone, Debug, PartialEq)]
pub struct Node<'a> {
    pub value: &'a str,
    pub label: &'a str,
    pub parent: i32,
}

/// The indices of the nodes under `parent` (-1 for the roots), in order.
pub fn children(nodes: &[Node], parent: i32) -> Vec<usize> {
    nodes.iter().enumerate().filter(|(_, node)| node.parent == parent).map(|(ix, _)| ix).collect()
}

/// The columns a trail opens: the roots, then the children of each step with any. Each column
/// lists node indices.
pub fn columns(nodes: &[Node], trail: &[usize]) -> Vec<Vec<usize>> {
    let mut out = vec![children(nodes, -1)];
    for &ix in trail {
        let Some(&node) = out[out.len() - 1].get(ix) else {
            break;
        };
        let under = children(nodes, node as i32);
        if under.is_empty() {
            break;
        }
        out.push(under);
    }
    out
}

/// Row indices along `path`, as far as it matches.
pub fn trail_of(nodes: &[Node], path: &[&str]) -> Vec<usize> {
    let mut trail = Vec::new();
    let mut level = children(nodes, -1);
    for value in path {
        let Some(ix) = level.iter().position(|&node| nodes[node].value == *value) else {
            break;
        };
        trail.push(ix);
        level = children(nodes, level[ix] as i32);
    }
    trail
}

/// `trail` cut back to rows that still exist; never empty.
pub fn fit_trail(nodes: &[Node], trail: &[usize]) -> Vec<usize> {
    let mut fitted = Vec::new();
    let mut level = children(nodes, -1);
    for &ix in trail {
        let Some(&node) = level.get(ix) else {
            break;
        };
        fitted.push(ix);
        level = children(nodes, node as i32);
    }
    if fitted.is_empty() {
        fitted.push(0);
    }
    fitted
}

/// The node indices a trail passes through.
pub fn along(nodes: &[Node], trail: &[usize]) -> Vec<usize> {
    let mut level = children(nodes, -1);
    let mut out = Vec::new();
    for &ix in trail {
        let Some(&node) = level.get(ix) else {
            break;
        };
        out.push(node);
        level = children(nodes, node as i32);
    }
    out
}

/// `trail` cut to `level` steps, then `ix` appended unless negative.
pub fn step_to(trail: &[usize], level: usize, ix: i32) -> Vec<usize> {
    let mut out: Vec<usize> = trail.iter().copied().take(level).collect();
    if ix >= 0 {
        out.push(ix as usize);
    }
    out
}

/// Wires the `Cascades` global, which every CascadeSelect asks. The host's root `.slint` file
/// exports `Cascades`, `CascadeNode` and `Choice`.
///
/// ```ignore
/// ely_slint::wire_cascades!(app);
/// ```
#[macro_export]
macro_rules! wire_cascades {
    ($app:expr) => {{
        use ::slint::Model;
        fn flat(nodes: &::slint::ModelRc<CascadeNode>) -> Vec<CascadeNode> {
            nodes.iter().collect()
        }
        fn view(nodes: &[CascadeNode]) -> Vec<$crate::cascade::Node<'_>> {
            nodes.iter().map(|n| $crate::cascade::Node { value: n.value.as_str(), label: n.label.as_str(), parent: n.parent }).collect()
        }
        fn ints(model: &::slint::ModelRc<i32>) -> Vec<usize> {
            model.iter().map(|ix| ix.max(0) as usize).collect()
        }
        fn model(trail: Vec<usize>) -> ::slint::ModelRc<i32> {
            ::slint::ModelRc::new(::slint::VecModel::from(trail.into_iter().map(|ix| ix as i32).collect::<Vec<_>>()))
        }
        let cascades = $app.global::<Cascades>();
        cascades.on_columns(|nodes, trail| {
            let nodes = flat(&nodes);
            let view = view(&nodes);
            let columns: Vec<::slint::ModelRc<Choice>> = $crate::cascade::columns(&view, &ints(&trail))
                .into_iter()
                .map(|column| {
                    let rows: Vec<Choice> = column
                        .into_iter()
                        .map(|ix| Choice {
                            value: nodes[ix].value.clone(),
                            label: nodes[ix].label.clone(),
                            branch: !$crate::cascade::children(&view, ix as i32).is_empty(),
                            ..Default::default()
                        })
                        .collect();
                    ::slint::ModelRc::new(::slint::VecModel::from(rows))
                })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(columns))
        });
        cascades.on_trail_of(|nodes, path| {
            let nodes = flat(&nodes);
            let path: Vec<::slint::SharedString> = path.iter().collect();
            let path: Vec<&str> = path.iter().map(|p| p.as_str()).collect();
            model($crate::cascade::trail_of(&view(&nodes), &path))
        });
        cascades.on_fit(|nodes, trail| {
            let nodes = flat(&nodes);
            model($crate::cascade::fit_trail(&view(&nodes), &ints(&trail)))
        });
        cascades.on_path(|nodes, trail| {
            let nodes = flat(&nodes);
            let values: Vec<::slint::SharedString> = $crate::cascade::along(&view(&nodes), &ints(&trail)).into_iter().map(|ix| nodes[ix].value.clone()).collect();
            ::slint::ModelRc::new(::slint::VecModel::from(values))
        });
        cascades.on_shown(|nodes, path| {
            let nodes = flat(&nodes);
            let view = view(&nodes);
            let path: Vec<::slint::SharedString> = path.iter().collect();
            let path: Vec<&str> = path.iter().map(|p| p.as_str()).collect();
            let trail = $crate::cascade::trail_of(&view, &path);
            $crate::cascade::along(&view, &trail).into_iter().map(|ix| nodes[ix].label.to_string()).collect::<Vec<_>>().join(" / ").into()
        });
        cascades.on_step_to(|trail, level, ix| model($crate::cascade::step_to(&ints(&trail), level.max(0) as usize, ix)));
        cascades.on_deepest(|nodes, trail| {
            let nodes = flat(&nodes);
            $crate::cascade::columns(&view(&nodes), &ints(&trail)).iter().map(Vec::len).max().unwrap_or(0) as i32
        });
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> Vec<Node<'static>> {
        let n = |value, label, parent| Node { value, label, parent };
        vec![
            n("asia", "Asia", -1),
            n("jp", "Japan", 0),
            n("tyo", "Tokyo", 1),
            n("osa", "Osaka", 1),
            n("eu", "Europe", -1),
            n("pt", "Portugal", 4),
        ]
    }

    #[test]
    fn a_trail_opens_one_column_per_parent() {
        let roots = world();
        assert_eq!(columns(&roots, &[0]).len(), 2);
        assert_eq!(columns(&roots, &[0, 0]).len(), 3);
        assert_eq!(columns(&roots, &[0, 0, 1]).len(), 3);
    }

    #[test]
    fn a_trail_is_cut_back_to_rows_that_exist() {
        let roots = world();
        assert_eq!(fit_trail(&roots, &[0, 0, 1]), [0, 0, 1]);
        assert_eq!(fit_trail(&roots, &[1, 5]), [1]);
        assert_eq!(fit_trail(&roots, &[9]), [0]);
        assert_eq!(fit_trail(&roots, &[]), [0]);
    }

    #[test]
    fn paths_and_trails_round_trip() {
        let roots = world();
        let trail = trail_of(&roots, &["asia", "jp", "osa"]);
        assert_eq!(trail, [0, 0, 1]);
        let labels: Vec<_> = along(&roots, &trail).into_iter().map(|ix| roots[ix].label).collect();
        assert_eq!(labels, ["Asia", "Japan", "Osaka"]);
        assert_eq!(trail_of(&roots, &["eu", "xx"]), [1]);
    }

    #[test]
    fn steps_cut_and_extend() {
        assert_eq!(step_to(&[0, 1, 2], 1, 3), [0, 3]);
        assert_eq!(step_to(&[0, 1], 2, 0), [0, 1, 0]);
        assert_eq!(step_to(&[0, 1], 1, -1), [0]);
    }
}
