use data_bag::DataBag;
use snowflake::ProcessUniqueId;
use yoga;

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter
}

pub struct Component {
    pub id: ProcessUniqueId,
    pub component_type: Type,
    pub styles: Vec<yoga::FlexStyle>,
    data_bag: DataBag,
}

impl Component {
    pub ( crate ) fn new(t: Type) -> Component {
        Component {
            id: ProcessUniqueId::new(),
            component_type: t,
            styles: Vec::new(),
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
