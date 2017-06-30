#![allow(dead_code)]

pub enum Children<'a> {
    None,
    Single(&'a Box<Component>),
    Multiple(&'a Vec<Box<Component>>),
}

pub trait Component {
    fn get_type(&self) -> &'static str;

    fn get_children<'a>(&'a self) -> Children<'a> {
        Children::None
    }
}

pub struct Center {
    child: Box<Component>,
}

pub struct Button {
    caption: &'static str,
}

pub struct Label {
    text: &'static str,
}

pub struct Column {
    children: Vec<Box<Component>>,
}

impl Component for Button {
    fn get_type(&self) -> &'static str {
        "Button"
    }
}
impl Component for Label {
    fn get_type(&self) -> &'static str {
        "Label"
    }
}

impl Component for Center {
    fn get_type(&self) -> &'static str {
        "Center"
    }

    fn get_children<'a>(&'a self) -> Children<'a> {
        Children::Single(&self.child)
    }
}

impl Component for Column {
    fn get_type(&self) -> &'static str {
        "Column"
    }

    fn get_children<'a>(&'a self) -> Children<'a> {
        Children::Multiple(&self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let root = make_default();
        traverse(&root);
        panic!("-------------------------");
    }

    fn traverse(c: &Box<Component>) {
        println!("{}", c.get_type());
        {
            match c.get_children() {
                Children::Single(child) => traverse(child),
                Children::Multiple(children) => {
                    for child in children.iter() {
                        traverse(child);
                    }
                }
                Children::None => {}
            }
        }
    }

    fn make_default() -> Box<Component> {
        let button = Box::new(Button { caption: "Click me" });
        let label = Box::new(Label { text: "Hello" });

        Box::new(Center { child: Box::new(Column { children: vec![button, label] }) })
    }
}
