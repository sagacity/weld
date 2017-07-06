use webrender_api::LayoutRect;
use component_tree::ComponentTree;
use components::component::{Component, Size};
use id_tree::{Children, NodeId};
use std::collections::HashMap;
use webrender_api::{LayoutSize, LayoutPoint};

pub trait Layout {
    fn determine_sizes(&self, tree: &ComponentTree, bounds: LayoutRect, result: &mut HashMap<i64, LayoutRect>);
}

pub struct CassowaryLayout;

impl CassowaryLayout {
    pub fn new() -> Box<Layout> {
        Box::new(CassowaryLayout {
        })
    }
}

impl Layout for CassowaryLayout {
    fn determine_sizes(&self, tree: &ComponentTree, bounds: LayoutRect, result: &mut HashMap<i64, LayoutRect>) {
        /*let mut nodes = tree.traverse_post_order();
        for node in nodes {
            println!("{:?}: {:?}", node.data().component_type, node.data().size);

            let mut computed_size: LayoutRect = match node.data().size {
                Size::Relative(percentage_size) => LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(0.0, 0.0)),
                Size::Absolute(layout_size) => LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(0.0, 0.0))
            };
            result.insert(node.data().id, computed_size);
        }*/
    }
}

/*
use cassowary::{Solver, Variable};
use cassowary::WeightedRelation::*;
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};

        let window_width = Variable::new();
        let window_height = Variable::new();
        let left = Variable::new();
        let right = Variable::new();
        let top = Variable::new();
        let bottom = Variable::new();

        let mut solver = Solver::new();
        solver.add_constraints(&[
            window_width | GE(REQUIRED) | 0.0,
            window_height | GE(REQUIRED) | 0.0,
            left | EQ(REQUIRED) | 15.0,
            right | EQ(REQUIRED) | window_width - 15.0,
            top | EQ(REQUIRED) | 15.0,
            bottom | EQ(REQUIRED) | window_height - 15.0,
            left | LE(REQUIRED) | right,
            top | LE(REQUIRED) | bottom,
        ]).unwrap();

        solver.add_edit_variable(window_width, STRONG).unwrap();
        solver.suggest_value(window_width, size.width as f64).unwrap();
        solver.add_edit_variable(window_height, STRONG).unwrap();
        solver.suggest_value(window_height, size.height as f64).unwrap();

        let changes = solver.fetch_changes();
        let mut values = HashMap::<Variable, f64>::new();
        for &(ref var, ref value) in changes {
            values.insert(*var, *value);
        }

        // dummy rect
        let bl = *(values.get(&left).unwrap_or(&0.0_f64)) as f32;
        let bt = *(values.get(&top).unwrap_or(&0.0_f64)) as f32;
        let br = *(values.get(&right).unwrap_or(&0.0_f64)) as f32;
        let bb = *(values.get(&bottom).unwrap_or(&0.0_f64)) as f32;
        let bounds = LayoutRect::new(LayoutPoint::new(bl, bt), LayoutSize::new(br - bl, bb - bt));
*/