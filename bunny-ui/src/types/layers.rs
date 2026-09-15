use egui::Id;

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
    pub order: Order,
    pub id: Id,
}

#[cfg(feature = "manager")]
impl From<egui::LayerId> for LayerId {
    #[inline]
    fn from(value: egui::LayerId) -> Self {
        Self {
            order: value.order.into(),
            id: value.id,
        }
    }
}
