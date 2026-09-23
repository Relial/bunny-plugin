use abi_stable::std_types::ROption::{self, RNone, RSome};
use emath::{Pos2, Rect};

#[cfg(feature = "manager")]
use crate::closure::PluginClosure;
use crate::{
    Align, Id, LayerId, Layout, Order, RectAlign, Sense, UiStackInfo,
    containers::Frame,
    response::{BunnyInnerResponse, BunnyResponse},
    ui::BunnyUi,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum PopupAnchor {
    ParentRect(Rect),
    Pointer,
    PointerFixed,
    Position(Pos2),
}

impl From<Rect> for PopupAnchor {
    fn from(value: Rect) -> Self {
        Self::ParentRect(value)
    }
}

impl From<Pos2> for PopupAnchor {
    fn from(value: Pos2) -> Self {
        Self::Position(value)
    }
}

impl From<&BunnyResponse> for PopupAnchor {
    fn from(value: &BunnyResponse) -> Self {
        let rect = value.interact_rect();
        Self::ParentRect(rect)
    }
}

#[cfg(feature = "manager")]
impl From<PopupAnchor> for egui::PopupAnchor {
    fn from(value: PopupAnchor) -> Self {
        match value {
            PopupAnchor::ParentRect(rect) => Self::ParentRect(rect),
            PopupAnchor::Pointer => Self::Pointer,
            PopupAnchor::PointerFixed => Self::PointerFixed,
            PopupAnchor::Position(pos2) => Self::Position(pos2),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum PopupCloseBehavior {
    #[default]
    CloseOnClick,
    CloseOnClickOutside,
    IgnoreClicks,
}

#[cfg(feature = "manager")]
impl From<PopupCloseBehavior> for egui::PopupCloseBehavior {
    fn from(value: PopupCloseBehavior) -> Self {
        match value {
            PopupCloseBehavior::CloseOnClick => Self::CloseOnClick,
            PopupCloseBehavior::CloseOnClickOutside => Self::CloseOnClickOutside,
            PopupCloseBehavior::IgnoreClicks => Self::IgnoreClicks,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum SetOpenCommand {
    Bool(bool),
    Toggle,
}

#[cfg(feature = "manager")]
impl From<SetOpenCommand> for egui::SetOpenCommand {
    fn from(value: SetOpenCommand) -> Self {
        match value {
            SetOpenCommand::Bool(open) => Self::Bool(open),
            SetOpenCommand::Toggle => Self::Toggle,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum OpenKind<'a> {
    Open,
    Closed,
    Bool(&'a mut bool),
    Memory { set: ROption<SetOpenCommand> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum PopupKind {
    Popup,
    Tooltip,
    Menu,
}

impl PopupKind {
    pub fn order(self) -> Order {
        match self {
            Self::Tooltip => Order::Tooltip,
            Self::Menu | Self::Popup => Order::Foreground,
        }
    }
}

#[cfg(feature = "manager")]
impl From<PopupKind> for egui::PopupKind {
    fn from(value: PopupKind) -> Self {
        match value {
            PopupKind::Popup => Self::Popup,
            PopupKind::Tooltip => Self::Tooltip,
            PopupKind::Menu => Self::Menu,
        }
    }
}

#[repr(C)]
pub struct Popup<'a> {
    info: ROption<UiStackInfo>,
    frame: ROption<Frame>,
    open_kind: OpenKind<'a>,
    anchor: PopupAnchor,
    rect_align: RectAlign,
    layer_id: LayerId,
    layout: Layout,
    id: Id,
    width: ROption<f32>,
    close_behavior: PopupCloseBehavior,
    kind: PopupKind,
    gap: f32,
    sense: Sense,
    menu_style: bool,
}

impl<'a> Popup<'a> {
    pub fn new(id: impl Into<Id>, anchor: impl Into<PopupAnchor>, layer_id: LayerId) -> Self {
        Self {
            info: RNone,
            id: id.into(),
            anchor: anchor.into(),
            layer_id,
            rect_align: RectAlign::BOTTOM_START,
            open_kind: OpenKind::Open,
            close_behavior: PopupCloseBehavior::default(),
            kind: PopupKind::Popup,
            gap: 0.0,
            width: RNone,
            sense: Sense::click(),
            layout: Layout::default(),
            frame: RNone,
            menu_style: false,
        }
    }

    pub fn from_response(response: &BunnyResponse) -> Self {
        Self::new(
            Self::default_response_id(response),
            response,
            response.layer_id(),
        )
    }

    pub fn from_toggle_button_response(button_response: &BunnyResponse) -> Self {
        Self::from_response(button_response)
            .open_memory(button_response.clicked().then_some(SetOpenCommand::Toggle))
    }

    pub fn menu(button_response: &BunnyResponse) -> Self {
        Self::from_toggle_button_response(button_response)
            .kind(PopupKind::Menu)
            .layout(Layout::top_down_justified(Align::Min))
            .menu_style(true)
            .gap(0.0)
    }

    pub fn context_menu(response: &BunnyResponse) -> Self {
        Self::menu(response)
            .open_memory(if response.secondary_clicked() {
                RSome(SetOpenCommand::Bool(true))
            } else if response.clicked() {
                RSome(SetOpenCommand::Bool(false))
            } else {
                RNone
            })
            .at_pointer_fixed()
    }

    #[inline]
    pub fn kind(mut self, kind: PopupKind) -> Self {
        self.kind = kind;
        self
    }

    #[inline]
    pub fn info(mut self, info: UiStackInfo) -> Self {
        self.info = RSome(info);
        self
    }

    #[inline]
    pub fn align(mut self, position_align: RectAlign) -> Self {
        self.rect_align = position_align;
        self
    }

    #[inline]
    pub fn open(mut self, open: bool) -> Self {
        self.open_kind = if open {
            OpenKind::Open
        } else {
            OpenKind::Closed
        };
        self
    }

    #[inline]
    pub fn open_memory(mut self, set_state: impl Into<Option<SetOpenCommand>>) -> Self {
        self.open_kind = OpenKind::Memory {
            set: set_state.into().into(),
        };
        self
    }

    #[inline]
    pub fn open_bool(mut self, open: &'a mut bool) -> Self {
        self.open_kind = OpenKind::Bool(open);
        self
    }

    #[inline]
    pub fn close_behavior(mut self, close_behavior: PopupCloseBehavior) -> Self {
        self.close_behavior = close_behavior;
        self
    }

    #[inline]
    pub fn at_pointer(mut self) -> Self {
        self.anchor = PopupAnchor::Pointer;
        self
    }

    #[inline]
    pub fn at_pointer_fixed(mut self) -> Self {
        self.anchor = PopupAnchor::PointerFixed;
        self
    }

    #[inline]
    pub fn at_position(mut self, position: Pos2) -> Self {
        self.anchor = PopupAnchor::Position(position);
        self
    }

    #[inline]
    pub fn anchor(mut self, anchor: impl Into<PopupAnchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    #[inline]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = RSome(width);
        self
    }

    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    #[inline]
    pub fn menu_style(mut self, menu_style: bool) -> Self {
        self.menu_style = menu_style;
        self
    }

    #[inline]
    pub fn get_anchor(&self) -> PopupAnchor {
        self.anchor
    }

    #[inline]
    pub fn get_id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<R>> {
        ui.popup_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl Popup<'_> {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> ROption<BunnyResponse> {
        let Popup {
            info,
            frame,
            open_kind,
            anchor,
            rect_align,
            layer_id,
            layout,
            id,
            width,
            close_behavior,
            kind,
            gap,
            sense,
            menu_style,
        } = self;
        let mut popup = egui::Popup::new(id.into(), ui.ctx().clone(), anchor, layer_id.into())
            .align(rect_align.into())
            .kind(kind.into())
            .gap(gap)
            .sense(sense.into())
            .layout(layout.into())
            .close_behavior(close_behavior.into());
        if menu_style {
            popup = popup.style(egui::containers::menu::menu_style);
        }

        if let RSome(width) = width {
            popup = popup.width(width);
        }
        if let RSome(frame) = frame {
            popup = popup.frame(frame.into());
        }
        if let RSome(info) = info {
            popup = popup.info(info.into());
        }

        popup = match open_kind {
            OpenKind::Open => popup.open(true),
            OpenKind::Closed => popup.open(false),
            OpenKind::Bool(open) => popup.open_bool(open),
            OpenKind::Memory { set } => popup.open_memory(set.map(|s| s.into())),
        };

        let inner_response = popup.show(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });

        inner_response
            .map(|i| BunnyResponse::new(i.response))
            .into()
    }
}

impl Popup<'_> {
    pub fn default_response_id(response: &BunnyResponse) -> Id {
        response.id().with("popup")
    }
}
