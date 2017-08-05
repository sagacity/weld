extern crate weld;

use weld::model::*;

#[path = "common/utils.rs"]
#[macro_use]
mod utils;

use utils::TestRenderContext;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Container {}

fn container() -> Component {
    Component::new(Container {})
}

#[derive(Debug)]
struct Label {
    caption: String
}

fn label<S: Into<String>>(caption: S) -> Component {
    Component::new(Label { caption: caption.into() })
}

#[derive(Debug)]
struct Input {}

fn input() -> Component {
    Component::new(Input {})
}

enum InputEvent {
    TextChanged(&'static str)
}

impl Event for InputEvent {}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl_dummy_renderer!(Container);
impl_dummy_renderer!(Input);
impl_dummy_renderer!(Label);

#[derive(Clone, Debug)]
struct MyAppState {
    todos: Vec<String>,
}

impl State for MyAppState {
    fn build(&self) -> Component {
        let items: Vec<_> = self.todos.iter().enumerate().map(|(index, item)| {
            TodoItem { message: item.clone() }.build().name(format!("todo-{}", index))
        }).collect();

        container()
            .name("container")
            .child(
                input()
                    .name("input")
                    .on(Box::new(|state: Self, event| {
                        match *event {
                            InputEvent::TextChanged(str) => {
                                let mut new = state.clone();
                                new.todos.push(str.into());
                                Ok(new)
                            }
                        }
                    }))
            )
            .children(items)
    }
}

#[derive(Clone, Debug)]
struct TodoItem {
    message: String
}

impl State for TodoItem {
    fn build(&self) -> Component {
        label(self.message.clone())
    }
}

///
///
///

struct MyApp;

impl MyApp {
    fn new() -> MyAppState {
        MyAppState {
            todos: Vec::new()
        }
    }
}

#[test]
fn test_app() {
    let mut state = MyApp::new();
    assert_eq!(state.todos.len(), 0);

    let mut c = state.build();
    assert_eq!(c.inspect().children().len(), 1);

    state = c.find_by_name("input").unwrap().invoke(&state, InputEvent::TextChanged("foo")).unwrap();
    assert_eq!(state.todos.len(), 1);
    c = state.build();
    assert_eq!(c.inspect().children().len(), 2);
    assert_eq!(c.find_by_name("todo-0").is_some(), true);
    assert_eq!(c.find_by_name("todo-1").is_none(), true);

    state = c.find_by_name("input").unwrap().invoke(&state, InputEvent::TextChanged("bar")).unwrap();
    assert_eq!(state.todos.len(), 2);
    c = state.build();
    assert_eq!(c.inspect().children().len(), 3);
    assert_eq!(c.find_by_name("todo-0").is_some(), true);
    assert_eq!(c.find_by_name("todo-1").is_some(), true);

    let mut context = TestRenderContext::new(&c);
    context.render();
}