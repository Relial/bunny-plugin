pub trait GlamExt {
    fn to_ui_vec(self) -> emath::Vec2;
    fn to_ui_pos(self) -> emath::Pos2;
}

impl GlamExt for glam::Vec2 {
    #[inline]
    fn to_ui_vec(self) -> emath::Vec2 {
        let glam::Vec2 { x, y } = self;
        emath::Vec2 { x, y }
    }

    #[inline]
    fn to_ui_pos(self) -> emath::Pos2 {
        let glam::Vec2 { x, y } = self;
        emath::Pos2 { x, y }
    }
}

pub trait UiExt {
    fn to_glam_vec(self) -> glam::Vec2;
}

impl UiExt for emath::Vec2 {
    #[inline]
    fn to_glam_vec(self) -> glam::Vec2 {
        let emath::Vec2 { x, y } = self;
        glam::Vec2 { x, y }
    }
}

impl UiExt for emath::Pos2 {
    #[inline]
    fn to_glam_vec(self) -> glam::Vec2 {
        let emath::Pos2 { x, y } = self;
        glam::Vec2 { x, y }
    }
}
