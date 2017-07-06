use components::component::*;

pub struct PanelBuilder {
}

pub struct PanelSize {
    pub size: Size
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<Component> for PanelBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Panel);
        c
    }
}

