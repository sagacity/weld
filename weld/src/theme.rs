use webrender_api::*;
use weld_core::component_tree::ComponentTree;
use cassowary::{Solver, Variable};
use cassowary::WeightedRelation::*;
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};
use std::collections::HashMap;

pub struct Theme;

impl Theme {
    pub fn new() -> Theme {
        Theme
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
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
        builder.push_rect(bounds, bounds, ColorF::new(1.0, 1.0, 1.0, 1.0));
    }
}