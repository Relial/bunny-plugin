use abi_stable::std_types::ROption::{self, RSome};

use crate::containers::Frame;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum UiKind {
    Window,
    CentralPanel,
    LeftPanel,
    RightPanel,
    TopPanel,
    BottomPanel,
    Modal,
    Frame,
    ScrollArea,
    Resize,
    Menu,
    Popup,
    Tooltip,
    Picker,
    TableCell,
    GenericArea,
    Collapsible,
}

#[cfg(feature = "manager")]
impl From<UiKind> for egui::UiKind {
    fn from(value: UiKind) -> Self {
        match value {
            UiKind::Window => Self::Window,
            UiKind::CentralPanel => Self::CentralPanel,
            UiKind::LeftPanel => Self::LeftPanel,
            UiKind::RightPanel => Self::RightPanel,
            UiKind::TopPanel => Self::TopPanel,
            UiKind::BottomPanel => Self::BottomPanel,
            UiKind::Modal => Self::Modal,
            UiKind::Frame => Self::Frame,
            UiKind::ScrollArea => Self::ScrollArea,
            UiKind::Resize => Self::Resize,
            UiKind::Menu => Self::Menu,
            UiKind::Popup => Self::Popup,
            UiKind::Tooltip => Self::Tooltip,
            UiKind::Picker => Self::Picker,
            UiKind::TableCell => Self::TableCell,
            UiKind::GenericArea => Self::GenericArea,
            UiKind::Collapsible => Self::Collapsible,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::UiKind> for UiKind {
    fn from(value: egui::UiKind) -> Self {
        match value {
            egui::UiKind::Window => Self::Window,
            egui::UiKind::CentralPanel => Self::CentralPanel,
            egui::UiKind::LeftPanel => Self::LeftPanel,
            egui::UiKind::RightPanel => Self::RightPanel,
            egui::UiKind::TopPanel => Self::TopPanel,
            egui::UiKind::BottomPanel => Self::BottomPanel,
            egui::UiKind::Modal => Self::Modal,
            egui::UiKind::Frame => Self::Frame,
            egui::UiKind::ScrollArea => Self::ScrollArea,
            egui::UiKind::Resize => Self::Resize,
            egui::UiKind::Menu => Self::Menu,
            egui::UiKind::Popup => Self::Popup,
            egui::UiKind::Tooltip => Self::Tooltip,
            egui::UiKind::Picker => Self::Picker,
            egui::UiKind::TableCell => Self::TableCell,
            egui::UiKind::GenericArea => Self::GenericArea,
            egui::UiKind::Collapsible => Self::Collapsible,
        }
    }
}

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct UiStackInfo {
    pub frame: Frame,
    pub kind: ROption<UiKind>,
}

impl UiStackInfo {
    #[inline]
    pub fn new(kind: UiKind) -> Self {
        Self {
            kind: RSome(kind),
            ..Default::default()
        }
    }
}

#[cfg(feature = "manager")]
impl From<UiStackInfo> for egui::UiStackInfo {
    fn from(value: UiStackInfo) -> Self {
        let UiStackInfo { kind, frame } = value;
        Self {
            frame: frame.into(),
            kind: kind.map(|u| u.into()).into_option(),
            ..Default::default()
        }
    }
}
