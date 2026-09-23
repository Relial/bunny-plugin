use crate::Id;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub enum Order {
    Background,
    Middle,
    Foreground,
    Tooltip,
    Debug,
}

#[cfg(feature = "manager")]
impl From<Order> for egui::Order {
    #[inline]
    fn from(value: Order) -> Self {
        match value {
            Order::Background => Self::Background,
            Order::Middle => Self::Middle,
            Order::Foreground => Self::Foreground,
            Order::Tooltip => Self::Tooltip,
            Order::Debug => Self::Debug,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Order> for Order {
    #[inline]
    fn from(value: egui::Order) -> Self {
        match value {
            egui::Order::Background => Self::Background,
            egui::Order::Middle => Self::Middle,
            egui::Order::Foreground => Self::Foreground,
            egui::Order::Tooltip => Self::Tooltip,
            egui::Order::Debug => Self::Debug,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct LayerId {
    pub id: Id,
    pub order: Order,
}

impl LayerId {
    #[inline]
    pub fn new(order: Order, id: impl Into<Id>) -> Self {
        Self {
            order,
            id: id.into(),
        }
    }

    #[inline]
    pub fn debug() -> Self {
        Self {
            id: Id::new("debug"),
            order: Order::Debug,
        }
    }

    #[inline]
    pub fn background() -> Self {
        Self {
            id: Id::new("background"),
            order: Order::Background,
        }
    }
}

#[cfg(feature = "manager")]
impl From<LayerId> for egui::LayerId {
    #[inline]
    fn from(value: LayerId) -> Self {
        Self {
            order: value.order.into(),
            id: value.id.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::LayerId> for LayerId {
    #[inline]
    fn from(value: egui::LayerId) -> Self {
        Self {
            order: value.order.into(),
            id: value.id.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ShapeIdx(pub usize);

#[cfg(feature = "manager")]
impl From<ShapeIdx> for egui::layers::ShapeIdx {
    #[inline]
    fn from(value: ShapeIdx) -> Self {
        Self(value.0)
    }
}

#[cfg(feature = "manager")]
impl From<egui::layers::ShapeIdx> for ShapeIdx {
    #[inline]
    fn from(value: egui::layers::ShapeIdx) -> Self {
        Self(value.0)
    }
}
