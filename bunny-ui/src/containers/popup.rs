use abi_stable::std_types::{
    RArc, RBox, RHashMap,
    ROption::{self, RNone, RSome},
};
use egui::{Context, Id, Pos2, Rect, Sense, Ui};
use rapidhash::fast::RandomState;

use crate::{
    align::Align, containers::frame::Frame, elements::{Container, UiContainer}, input_state::PointerState, layout::Layout, paint::paintlist::Order, rect_align::RectAlign, response::{InnerResponse, Response}, ui::BunnyUi,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupAnchor {
    ParentRect(Rect),
    Pointer,
    PointerFixed,
    Position(Pos2),
}

impl PopupAnchor {
    pub fn rect(self, popup_id: Id, ctx: &Context) -> Option<Rect> {
        match self {
            PopupAnchor::ParentRect(rect) => Some(rect),
            PopupAnchor::Pointer => ctx.pointer_hover_pos().map(Rect::from_pos),
            PopupAnchor::PointerFixed => Popup::position_of_id(ctx, popup_id).map(Rect::from_pos),
            PopupAnchor::Position(pos2) => Some(Rect::from_pos(pos2)),
        }
    }
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

impl From<&Response> for PopupAnchor {
    fn from(value: &Response) -> Self {
        let rect = value.interact_rect;
        Self::ParentRect(rect)
    }
}

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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetOpenCommand {
    Bool(bool),
    Toggle,
}

impl From<SetOpenCommand> for egui::SetOpenCommand {
    fn from(value: SetOpenCommand) -> Self {
        match value {
            SetOpenCommand::Bool(open) => Self::Bool(open),
            SetOpenCommand::Toggle => Self::Toggle,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum OpenKind<'a> {
    Open,
    Closed,
    Bool(&'a mut bool),
    Memory { set: ROption<SetOpenCommand> },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
struct PopupClick {
    popup_interact_rect: Rect,
    click_pos: Pos2,
}

impl Default for PopupClick {
    fn default() -> Self {
        Self {
            popup_interact_rect: Rect::ZERO,
            click_pos: Pos2::ZERO,
        }
    }
}

#[repr(C)]
pub struct Popup<'a> {
    frame: ROption<Frame>,
    click: ROption<PopupClick>,
    open_kind: OpenKind<'a>,
    anchor: PopupAnchor,
    rect_align: RectAlign,
    layout: Layout,
    id: Id,
    pub(crate) width: ROption<f32>,
    close_behavior: PopupCloseBehavior,
    kind: PopupKind,
    gap: f32,
    sense: Sense,
    menu_style: bool,
}

impl<'a> Popup<'a> {
    pub fn new(id: Id, anchor: impl Into<PopupAnchor>) -> Self {
        Self {
            id,
            anchor: anchor.into(),
            rect_align: RectAlign::BOTTOM_START,
            open_kind: OpenKind::Open,
            close_behavior: PopupCloseBehavior::default(),
            click: RNone,
            kind: PopupKind::Popup,
            gap: 0.0,
            width: RNone,
            sense: Sense::click(),
            layout: Layout::default(),
            frame: RNone,
            menu_style: false,
        }
    }

    pub fn from_response(response: &Response) -> Self {
        Self::new(Self::default_response_id(response), response)
    }

    pub fn from_toggle_button_response(button_response: &Response) -> Self {
        Self::from_response(button_response)
            .open_memory(button_response.clicked().then_some(SetOpenCommand::Toggle))
    }

    pub fn menu(button_response: &Response) -> Self {
        Self::from_toggle_button_response(button_response)
            .kind(PopupKind::Menu)
            .layout(Layout::top_down_justified(Align::Min))
            .menu_style(true)
            .gap(0.0)
    }

    pub fn context_menu(response: &Response) -> Self {
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
    pub fn open_memory(mut self, set_state: impl Into<ROption<SetOpenCommand>>) -> Self {
        self.open_kind = OpenKind::Memory {
            set: set_state.into(),
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
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    #[inline]
    pub fn menu_style(mut self, menu_style: bool) -> Self {
        self.menu_style = menu_style;
        self
    }

    pub fn get_anchor(&self) -> PopupAnchor {
        self.anchor
    }

    pub fn get_anchor_rect(&self, ctx: &Context) -> Option<Rect> {
        self.anchor.rect(self.id, ctx)
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn is_open(&self, ctx: &Context) -> bool {
        #[allow(deprecated)]
        match &self.open_kind {
            OpenKind::Open => true,
            OpenKind::Closed => false,
            OpenKind::Bool(open) => **open,
            OpenKind::Memory { .. } => ctx.memory(|mem| mem.is_popup_open(self.id)),
        }
    }

    pub fn show<R>(
        mut self,
        ui: &mut BunnyUi<'a>,
        content: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = content(&mut new);
        let response = ui.response(self.id);
        match self.close_behavior {
            PopupCloseBehavior::CloseOnClick => {
                if ui.input(|i| i.pointer.any_click()) {
                    self.click = RSome(PopupClick::default());
                }
            }
            PopupCloseBehavior::CloseOnClickOutside => {
                if let Some(resp) = response
                    && let RSome(interact_pos) = ui.input(|i| {
                        if i.pointer.any_click() {
                            i.pointer.interact_pos()
                        } else {
                            RNone
                        }
                    })
                {
                    self.click = RSome(PopupClick {
                        popup_interact_rect: resp.interact_rect,
                        click_pos: interact_pos,
                    })
                }
            }
            PopupCloseBehavior::IgnoreClicks => {}
        };
        let inner = InnerResponse::new(ret, response.cloned().unwrap_or_default());
        ui.add_component(
            self.id,
            Container::Popup(RBox::new(PopupComponent {
                contents: new,
                popup: self,
            })),
        );
        inner
    }

    pub(crate) fn egui(self, ui: &mut Ui, id: Id) -> egui::Popup<'a> {
        let mut popup = egui::Popup::new(id, ui.ctx().clone(), self.anchor, ui.layer_id())
            .align(self.rect_align.into())
            .kind(self.kind.into())
            .gap(self.gap)
            .sense(self.sense)
            .layout(self.layout.into())
            .close_behavior(egui::PopupCloseBehavior::IgnoreClicks); // Must be handled manually because our interactions are 1 frame behind egui's

        let was_open_last_frame = ui.read_response(id).is_some();
        let close_click = was_open_last_frame && self.click.is_some();
        let pointer_elsewhere = if let RSome(click) = self.click {
            !click.popup_interact_rect.contains(click.click_pos)
        } else {
            false
        };
        let closed_by_click = match self.close_behavior {
            PopupCloseBehavior::CloseOnClick => close_click,
            PopupCloseBehavior::CloseOnClickOutside => close_click && pointer_elsewhere,
            PopupCloseBehavior::IgnoreClicks => false,
        };

        popup = if closed_by_click {
            popup.open(false)
        } else {
            match self.open_kind {
                OpenKind::Open => popup.open(true),
                OpenKind::Closed => popup.open(false),
                OpenKind::Bool(open) => popup.open_bool(open),
                OpenKind::Memory { set } => popup.open_memory(set.map(|s| s.into())),
            }
        };

        if let RSome(width) = self.width {
            popup = popup.width(width);
        }
        if let RSome(frame) = self.frame {
            popup = popup.frame(frame.into());
        }

        popup
    }
}

#[allow(deprecated)]
impl Popup<'_> {
    pub fn default_response_id(response: &Response) -> Id {
        response.id.with("popup")
    }

    pub fn is_id_open(ctx: &Context, popup_id: Id) -> bool {
        ctx.memory(|mem| mem.is_popup_open(popup_id))
    }

    pub fn position_of_id(ctx: &Context, popup_id: Id) -> Option<Pos2> {
        ctx.memory(|mem| mem.popup_position(popup_id))
    }
}

#[repr(C)]
pub struct PopupComponent<'a> {
    contents: BunnyUi<'a>,
    popup: Popup<'a>,
}

impl UiContainer for PopupComponent<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        pointer_state: RArc<PointerState>,
        id: Id,
    ) -> Response {
        let popup = self.popup.egui(ui, id);

        let inner = popup.show(|ui| {
            self.contents.ui(ui, responses, pointer_state.clone());
        });
        if let Some(inner) = inner {
            Response::new(id, inner.response, pointer_state)
        } else {
            Response::empty(id, pointer_state)
        }
    }
}
