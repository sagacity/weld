use data_bag::DataBag;
use snowflake::ProcessUniqueId;
use yoga;

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter,
    Button
}

pub struct Component {
    pub id: ProcessUniqueId,
    pub component_type: Type,
    pub styles: Vec<yoga::FlexStyle>,
    data_bag: DataBag,
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

impl Component {
    pub ( crate ) fn new(t: Type, styles: Vec<yoga::FlexStyle>) -> Component {
        Component {
            id: ProcessUniqueId::new(),
            component_type: t,
            styles: styles,
            data_bag: DataBag::new()
        }
    }

    pub fn data(&self) -> &DataBag {
        &self.data_bag
    }

    pub fn data_mut(&mut self) -> &mut DataBag {
        &mut self.data_bag
    }
}
