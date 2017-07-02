use components::component::Component;

#[derive(Debug, PartialEq)]
pub struct Caption {
    pub caption: &'static str,
}

pub trait Label {
    fn get_caption(&self) -> Option<&Caption>;
}

impl Label for Component {
    fn get_caption(&self) -> Option<&Caption> {
        self.data_bag.get::<Caption>()
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
        let mut c = Component::new();
        c.data_bag.put(Caption { caption: self.caption });
        c
    }
}

