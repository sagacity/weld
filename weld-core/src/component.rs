use data_bag::DataBag;
use events::Event;
use snowflake::ProcessUniqueId;
use layout::FlexStyle;
use std::fmt;

pub type ComponentId = ProcessUniqueId;

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter,
    Button
}

pub struct Component {
    id: ComponentId,
    component_type: Type,
    styles: Vec<FlexStyle>,
    children: Vec<Component>,
    event_handler: Option<Box<Fn(&Event) + 'static + Send>>,
    data_bag: DataBag,
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Component {{ id: {} }}", self.id)
    }
}

impl Component {
    pub fn new<S, C>(t: Type, styles: S, children: C, events: Option<Box<Fn(&Event) + 'static + Send>>) -> Component where S: IntoIterator<Item=FlexStyle>, C: IntoIterator<Item=Component> {
        let mut s = Vec::new();
        s.extend(styles);
        let mut c = Vec::new();
        c.extend(children);
        Component {
            id: ProcessUniqueId::new(),
            component_type: t,
            styles: s,
            children: c,
            event_handler: events,
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

    pub fn find<'a>(&'a self, id: &ComponentId) -> Option<&'a Component> {
        find_recursive(self, id)
    }

    pub fn handle(&self, event: &Event) {
        if let Some(ref handler) = self.event_handler {
            handler(event);
        }
    }
}

fn find_recursive<'a>(node: &'a Component, id: &ComponentId) -> Option<&'a Component> {
    if node.id() == id {
        return Some(node);
    }

    for child in node.children() {
        if let Some(found) = find_recursive(child, id) {
            return Some(found);
        }
    }

    None
}

pub fn panel<S, C>(styles: S, children: C) -> Component where S: IntoIterator<Item=FlexStyle>, C: IntoIterator<Item=Component> {
    Component::new(Type::Panel, styles, children, None)
}

pub fn panel2<S, C, E>(styles: S, children: C, events: E) -> Component where S: IntoIterator<Item=FlexStyle>, C: IntoIterator<Item=Component>, E: Fn(&Event) + Send + 'static {
    Component::new(Type::Panel, styles, children, Some(Box::new(events)))
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