use data_bag::DataBag;
use events::Event;
use snowflake::ProcessUniqueId;
use layout::FlexStyle;
use std::fmt;

pub type ComponentId = ProcessUniqueId;

pub type EventHandler = Box<Fn(&Event) + 'static + Send>;

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter,
    Button
}

pub enum Configuration {
    Style(FlexStyle),
    Styles(Vec<FlexStyle>),
    Child(Component),
    Children(Vec<Component>),
    Event(Box<Fn(&Event) + 'static + Send>)
}

pub struct Component {
    id: ComponentId,
    component_type: Type,
    styles: Vec<FlexStyle>,
    children: Vec<Component>,
    event_handlers: Vec<EventHandler>,
    data_bag: DataBag,
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {{ id: {} }}", self.component_type, self.id)
    }
}

impl Component {
    pub fn new(t: Type, styles: Vec<FlexStyle>, children: Vec<Component>, event_handlers: Vec<EventHandler>) -> Component {
        Component {
            id: ProcessUniqueId::new(),
            component_type: t,
            styles: styles,
            children: children,
            event_handlers: event_handlers,
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
        for handler in self.event_handlers.iter() {
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

pub fn panel<C>(configurations: C) -> Component where C: IntoIterator<Item=Configuration> {
    let mut styles = Vec::new();
    let mut children = Vec::new();
    let mut event_handlers = Vec::new();

    for config in configurations {
        match config {
            Configuration::Style(s) => styles.push(s),
            Configuration::Styles(s) => styles.extend(s),
            Configuration::Child(c) => children.push(c),
            Configuration::Children(c) => children.extend(c),
            Configuration::Event(e) => event_handlers.push(e)
        }
    }

    Component::new(Type::Panel, styles, children, event_handlers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Configuration::*;

    #[test]
    fn can_create_configuration_tree() {
        let tree = panel(vec![Children(vec![
            panel(vec![]),
            panel(vec![])
        ])]);

        assert_eq!(tree.children().len(), 2);
    }
}