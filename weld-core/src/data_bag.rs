use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataBag {
    data: HashMap<TypeId, Box<Any>>,
}

impl DataBag {
    pub fn new() -> DataBag {
        DataBag { data: HashMap::new() }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.data.get(&TypeId::of::<T>()) {
            Some(value) => {
                let any = value.as_ref();
                return any.downcast_ref::<T>();
            }
            None => None,
        }
    }

    pub fn put<T: 'static>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct Size {
        width: f64,
        height: f64,
    }

    #[derive(Debug, PartialEq)]
    pub struct Caption {
        caption: &'static str,
    }

    #[test]
    fn test_databag() {
        let mut bag = DataBag::new();

        assert_eq!(bag.get::<Size>(), None);
        assert_eq!(bag.get::<Caption>(), None);

        bag.put(Size {
            width: 3.0,
            height: 4.0,
        });

        assert_eq!(*bag.get::<Size>().unwrap(),
                   Size {
                       width: 3.0,
                       height: 4.0,
                   });

        bag.put(Caption { caption: "Foo" });

        bag.put(Size {
            width: 7.0,
            height: 8.0,
        });

        assert_eq!(*bag.get::<Caption>().unwrap(), Caption { caption: "Foo" });

        assert_eq!(*bag.get::<Size>().unwrap(),
                   Size {
                       width: 7.0,
                       height: 8.0,
                   });

    }
}