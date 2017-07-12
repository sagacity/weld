use components::component::*;
use yoga::FlexStyle;

pub struct SplitterBuilder {
    styles: Vec<FlexStyle>
}

impl SplitterBuilder {
    pub fn new(styles: Vec<FlexStyle>) -> Self {
        Self { styles }
    }
}

impl Into<Component> for SplitterBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Splitter, self.styles);
        c
    }
}

