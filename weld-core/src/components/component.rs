use data_bag::DataBag;
use webrender_api::{LayoutSize, LayoutPixel, LayoutRect, LayoutPoint};
use euclid::{TypedSize2D, TypedSideOffsets2D};

#[derive(Debug)]
pub enum Type {
    Panel,
    Label,
    Splitter
}

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LayoutPercentage;

pub type PercentageSize = TypedSize2D<f32, LayoutPercentage>;
pub type PercentageSideOffsets = TypedSideOffsets2D<f32, LayoutPercentage>;
pub type LayoutSideOffsets = TypedSideOffsets2D<f32, LayoutPixel>;

#[derive(Debug)]
pub enum Size {
    Relative(PercentageSize),
    Absolute(LayoutSize)
}

impl Size {
    pub fn as_layout_rect(&self, bounds: &LayoutRect) -> LayoutRect {
        match *self {
            Size::Relative(percentage_size) => LayoutRect::new(bounds.origin, LayoutSize::new((percentage_size.width * bounds.size.width) / 100.0, (percentage_size.height * bounds.size.height) / 100.0)),
            Size::Absolute(absolute_size) => LayoutRect::new(bounds.origin, LayoutSize::new(absolute_size.width, absolute_size.height))
        }
    }
}

#[derive(Debug)]
pub enum Padding {
    Relative(PercentageSideOffsets),
    Absolute(LayoutSideOffsets)
}

pub struct Component {
    pub component_type: Type,
    pub size: Size,
    pub padding: Padding,
    data_bag: DataBag,
}

impl Component {
    pub ( crate ) fn new(t: Type) -> Component {
        Component {
            component_type: t,
            size: Size::Relative(PercentageSize::new(100.0, 100.0)),
            padding: Padding::Absolute(LayoutSideOffsets::new_all_same(0.0)),
            data_bag: DataBag::new()
        }
    }

    pub fn data(&self) -> &DataBag {
        &self.data_bag
    }

    pub fn data_mut(&mut self) -> &mut DataBag {
        &mut self.data_bag
    }
}
