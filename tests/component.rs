extern crate weld;
extern crate webrender;

use weld::model::*;

#[path="common/utils.rs"]
#[macro_use]
mod utils;

use utils::TestRenderContext;

#[derive(Debug)]
struct ComA {}

fn com_a() -> Component {
    Component::new(ComA {})
}

#[derive(Debug)]
struct ComB {}

fn com_b() -> Component {
    Component::new(ComB {})
}

impl_dummy_renderer!(ComA);
impl_dummy_renderer!(ComB);

#[derive(Clone, Debug)]
struct MyAppState {
    counter: u32,
    child: ChildState,
}

#[derive(Debug)]
enum MyAppEvent {
    Pressed,
}

impl Event for MyAppEvent {}

struct MyApp;

impl MyApp {
    fn new() -> MyAppState {
        MyAppState {
            counter: 0,
            child: ChildState { counter: 0 }
        }
    }
}

impl State for MyAppState {
    fn build(&self) -> Component {
        com_a()
            .name("parent")
            .on(Box::new(|state: Self, event| {
                match *event {
                    MyAppEvent::Pressed => {
                        Ok(MyAppState {
                            counter: state.counter + 1,
                            child: state.child
                        })
                    }
                }
            }))
            .child(com_b().name("child1"))
            .child(
                self.child.build()
                    .name("child2")
                    .on(Box::new(|state: Self, event| {
                        match *event {
                            MyAppEvent::Pressed => {
                                Ok(MyAppState {
                                    counter: state.counter,
                                    child: ChildState { counter: state.child.counter + 1 }
                                })
                            }
                        }
                    }))
            )
    }
}

#[derive(Clone, Debug)]
struct ChildState {
    counter: u32
}

impl State for ChildState {
    fn build(&self) -> Component {
        com_a()
    }
}

#[test]
fn test_invoke() {
    let state = MyApp::new();
    let component = state.build();
    let new_state = component.invoke(&state, MyAppEvent::Pressed)
        .and_then(|state| state.build().find_by_name("child2").unwrap().invoke(&state, MyAppEvent::Pressed))
        .unwrap();
    assert_eq!(new_state.counter, 1);
    assert_eq!(new_state.child.counter, 1);
}

#[test]
fn test_rendering() {
    let state = MyApp::new();
    let component = state.build();

    let mut context = TestRenderContext::new(&component);
    context.render();
    assert_eq!(context.elements(), &vec![
        ("parent".into(), "ComA".into()),
        ("child1".into(), "ComB".into()),
        ("child2".into(), "ComA".into()),
    ]);
}
