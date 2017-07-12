use components::component::*;
use yoga::{FlexStyle, StyleUnit};

pub struct PanelBuilder {
    styles: Vec<FlexStyle>
}

pub struct PanelSize {
    pub size: (StyleUnit, StyleUnit),
}

impl PanelBuilder {
    pub fn new(styles: Vec<FlexStyle>) -> Self {
        Self { styles }
    }
}

impl Into<Component> for PanelBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Panel, self.styles);
        c
    }
}

