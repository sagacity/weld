use data_bag::DataBag;

pub enum Type {
    Panel,
    Label
}

#[derive(Debug, PartialEq)]
pub struct Size {
    width: f64,
    height: f64,
}

pub struct Component {
    pub component_type: Type,
    data_bag: DataBag,
}

impl Component {
    pub ( crate ) fn new(t: Type) -> Component {
        Component { component_type: t, data_bag: DataBag::new() }
    }

    pub fn data(&self) -> &DataBag {
        &self.data_bag
    }

    pub fn data_mut(&mut self) -> &mut DataBag {
        &mut self.data_bag
    }
}
