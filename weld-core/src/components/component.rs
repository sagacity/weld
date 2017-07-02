use data_bag::DataBag;

#[derive(Debug, PartialEq)]
pub struct Size {
    width: f64,
    height: f64,
}

pub struct Component {
    pub(crate) data_bag: DataBag,
}

impl Component {
    pub(crate) fn new() -> Component {
        Component { data_bag: DataBag::new() }
    }
}
