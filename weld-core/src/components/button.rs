use components::component::*;
use yoga::FlexStyle;

pub struct ButtonBuilder {
    styles: Vec<FlexStyle>,
}

impl ButtonBuilder {
    pub fn new(styles: Vec<FlexStyle>) -> Self {
        Self {
            styles
        }
    }
}

impl Into<Component> for ButtonBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Button, self.styles);
        c
    }
}

