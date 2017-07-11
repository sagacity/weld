use components::component::*;
use yoga::StyleUnit;

pub struct PanelBuilder {
}

pub struct PanelSize {
    pub size: (StyleUnit, StyleUnit),
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

