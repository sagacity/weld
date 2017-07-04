use components::component::*;

pub struct SplitterBuilder {
}

impl SplitterBuilder {
    pub fn new() -> Self {
        Self { }
    }
}

impl Into<Component> for SplitterBuilder {
    fn into(self) -> Component {
        let c = Component::new(Type::Splitter);
        c
    }
}

