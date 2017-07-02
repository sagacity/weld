use components::component::*;

pub struct PanelBuilder {
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self { }
    }
}

impl Into<Component> for PanelBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Panel);
        c
    }
}

