use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::result::Result;
use std::marker::PhantomData;
use std::borrow::Borrow;
use std::fmt;
use layout::FlexStyle;
use snowflake::ProcessUniqueId;
use webrender::api::{LayoutRect, ColorF};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Event where Self: Sized + 'static {}

pub trait State where Self: Clone + 'static {
    fn build(&self) -> Component;
}

impl<S: State + Sized> From<S> for Component {
    fn from(state: S) -> Component {
        state.build()
    }
}

pub trait Renderer {
    fn id(&self) -> &'static str;
    fn render(&self, context: &mut RenderContext);
}

pub type ComponentId = ProcessUniqueId;

pub struct Component {
    id: ComponentId,
    name: Option<String>,
    renderer: Box<Renderer>,
    children: Vec<Component>,
    callbacks: HashMap<TypeId, Box<StateCallback>>,
    styles: Vec<FlexStyle>,
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Component {{ id: {} }}", self.id)
    }
}

impl Component {
    pub fn new<R: Renderer + 'static>(renderer: R) -> Component {
        Component {
            id: ProcessUniqueId::new(),
            name: None,
            renderer: Box::new(renderer),
            children: Vec::new(),
            callbacks: HashMap::new(),
            styles: Vec::new(),
        }
    }

    pub fn invoke<S: State, E: Event>(&self, state: &S, event: E) -> Result<S, InvocationError> {
        let callback = self.callbacks.get(&TypeId::of::<E>()).ok_or(InvocationError)?;
        callback.invoke(state, &event).and_then(|state_any| {
            Ok(state_any.downcast_ref::<S>().unwrap().clone())
        })
    }

    pub fn find_by_name<'a>(&'a self, name: &'static str) -> Option<&'a Component> {
        if let Some(ref self_name) = self.name {
            if self_name == name {
                return Some(self);
            }
        }

        for child in &self.children {
            if let Some(child) = child.find_by_name(name) {
                return Some(child);
            }
        }

        None
    }

    pub fn name<I: Into<String>>(mut self, name: I) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn on<S: State, E: Event>(mut self, handler: SyncStateHandler<S, E>) -> Self {
        let event_type = TypeId::of::<E>();
        self.callbacks.insert(event_type, Box::new(SyncStateCallback {
            state: PhantomData,
            handler,
        }));
        self
    }

    pub fn child<B: Into<Component>>(mut self, child: B) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children<B: Into<Component>>(mut self, children: Vec<B>) -> Self {
        for child in children {
            self.children.push(child.into());
        }
        self
    }

    pub fn style<B: Into<FlexStyle>>(mut self, style: B) -> Self {
        self.styles.push(style.into());
        self
    }

    pub fn styles<B: Into<FlexStyle>>(mut self, styles: Vec<B>) -> Self {
        for style in styles {
            self.styles.push(style.into());
        }
        self
    }

    pub fn inspect(&self) -> &InspectableComponent {
        self
    }
}

pub trait InspectableComponent {
    fn id(&self) -> &ComponentId;
    fn name(&self) -> &Option<String>;
    fn renderer(&self) -> &Renderer;
    fn children(&self) -> &Vec<Component>;
    fn styles(&self) -> &Vec<FlexStyle>;
}

impl InspectableComponent for Component {
    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn renderer(&self) -> &Renderer {
        self.renderer.borrow()
    }

    fn children(&self) -> &Vec<Component> {
        &self.children
    }

    fn styles(&self) -> &Vec<FlexStyle> {
        &self.styles
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

type SyncStateHandler<S, E> = Box<Fn(S, &E) -> Result<S, InvocationError>>;

#[derive(Debug)]
pub struct InvocationError;

trait StateCallback {
    fn invoke(&self, state: &Any, event: &Any) -> Result<Box<Any>, InvocationError>;
}

struct SyncStateCallback<S: State, E: Event> {
    state: PhantomData<S>,
    handler: SyncStateHandler<S, E>
}

impl<S: State, E: Event> StateCallback for SyncStateCallback<S, E> {
    fn invoke(&self, state: &Any, event: &Any) -> Result<Box<Any>, InvocationError> {
        let e = event.downcast_ref::<E>().ok_or(InvocationError)?;
        let s = state.downcast_ref::<S>().ok_or(InvocationError)?;
        let new_state = (self.handler)(s.clone(), e)?;
        Ok(Box::new(new_state))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum RenderElement {
    Rect(LayoutRect, ColorF)
}

pub trait RenderContext {
    fn render(&mut self);
    fn push(&mut self, e: RenderElement);
    fn next(&mut self);
    fn bounds(&self) -> LayoutRect;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
