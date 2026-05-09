#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopupCloseBehavior {
    #[default]
    CloseOnClick,
    CloseOnClickOutside,
    IgnoreClicks,
}

impl From<PopupCloseBehavior> for egui::PopupCloseBehavior {
    fn from(value: PopupCloseBehavior) -> Self {
        match value {
            PopupCloseBehavior::CloseOnClick => Self::CloseOnClick,
            PopupCloseBehavior::CloseOnClickOutside => Self::CloseOnClickOutside,
            PopupCloseBehavior::IgnoreClicks => Self::IgnoreClicks,
        }
    }
}
