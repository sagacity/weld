use weld::model::*;

#[macro_export]
macro_rules! impl_dummy_renderer {
    ($component_name:ident) => {
        impl Renderer for $component_name {
            fn id(&self) -> &'static str {
                stringify!($component_name)
            }

            fn render(&self, ctx: &mut RenderContext) {
                println!("Rendering: {:?}", self);
                ctx.next();
            }
        }
    };
}

pub struct TestRenderContext<'a> {
    component: &'a InspectableComponent,
    elements: Vec<(String, String)>
}

impl<'a> TestRenderContext<'a> {
    pub fn new(component: &'a Component) -> TestRenderContext<'a> {
        TestRenderContext {
            component: component.inspect(),
            elements: Vec::new()
        }
    }

    pub fn elements(&self) -> &Vec<(String, String)> {
        &self.elements
    }
}

impl<'a> RenderContext for TestRenderContext<'a> {
    fn render(&mut self) {
        let component_name: String = self.component.name().clone().unwrap_or_else(|| "Unknown".into());
        let renderer_id: String = self.component.renderer().id().into();
        self.elements.push((component_name, renderer_id));
        self.component.renderer().render(self);
    }

    fn push(&mut self, _: RenderElement) {
    }

    fn next(&mut self) {
        for child in self.component.children().iter() {
            let mut child_context = TestRenderContext::new(child);
            child_context.render();
            self.elements.extend(child_context.elements);
        }
    }
}

