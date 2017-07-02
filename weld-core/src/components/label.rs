use components::component::*;

#[derive(Debug, PartialEq)]
pub struct Caption {
    pub caption: &'static str,
}

pub trait Label {
    fn get_caption(&self) -> Option<&Caption>;
}

impl Label for Component {
    fn get_caption(&self) -> Option<&Caption> {
        self.data().get::<Caption>()
    }
}

pub struct LabelBuilder {
    caption: &'static str,
}

impl LabelBuilder {
    pub fn new() -> Self {
        Self { caption: "Untitled" }
    }

    pub fn caption(mut self, caption: &'static str) -> Self {
        self.caption = caption;
        self
    }
}

impl Into<Component> for LabelBuilder {
    fn into(self) -> Component {
        let mut c = Component::new(Type::Label);
        c.data_mut().put(Caption { caption: self.caption });
        c
    }
}

