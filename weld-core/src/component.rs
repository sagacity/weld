use data_bag::DataBag;
use snowflake::ProcessUniqueId;
use layout::FlexStyle;

pub type ComponentId = ProcessUniqueId;

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter,
    Button
}

#[derive(Debug)]
pub struct Component {
    id: ComponentId,
    component_type: Type,
    styles: Vec<FlexStyle>,
    children: Vec<Component>,
    data_bag: DataBag,
}

impl Component {
    pub fn new<S, C>(t: Type, styles: S, children: C) -> Component where S: IntoIterator<Item=FlexStyle>, C: IntoIterator<Item=Component> {
        let mut s = Vec::new();
        s.extend(styles);
        let mut c = Vec::new();
        c.extend(children);
        Component {
            id: ProcessUniqueId::new(),
            component_type: t,
            styles: s,
            children: c,
            data_bag: DataBag::new(),
        }
    }

    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    pub fn data(&self) -> &DataBag {
        &self.data_bag
    }

    pub fn data_mut(&mut self) -> &mut DataBag {
        &mut self.data_bag
    }

    pub fn children(&self) -> &Vec<Component> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Component> {
        &mut self.children
    }

    pub fn styles(&self) -> &Vec<FlexStyle> {
        &self.styles
    }
}

pub fn panel<S, C>(styles: S, children: C) -> Component where S: IntoIterator<Item=FlexStyle>, C: IntoIterator<Item=Component> {
    Component::new(Type::Panel, styles, children)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tree() {
        let tree = panel(vec![], vec![
            panel(vec![], vec![]),
            panel(vec![], vec![]),
        ]);

        assert_eq!(tree.children().len(), 2);
    }
}