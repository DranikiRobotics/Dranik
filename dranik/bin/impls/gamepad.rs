#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Gamepad {
    pub(crate) dpad_up: bool,
    pub(crate) dpad_down: bool,
    pub(crate) dpad_left: bool,
    pub(crate) dpad_right: bool,
    pub(crate) lstick_x: f32,
    pub(crate) lstick_y: f32,
    pub(crate) lstick: bool,
    pub(crate) rstick_x: f32,
    pub(crate) rstick_y: f32,
    pub(crate) rstick: bool,
    pub(crate) ltrigger: f32,
    pub(crate) rtrigger: f32,
    pub(crate) a: bool,
    pub(crate) b: bool,
    pub(crate) x: bool,
    pub(crate) y: bool,
    pub(crate) lbumper: bool,
    pub(crate) rbumper: bool,
}

macro_rules! i {
    ($name:ident $type:ty) => (
        #[inline] fn $name(&self) -> $type {
            self.$name
        }
    )
}

impl ::dranik_api::Gamepad for Gamepad {
    i!(dpad_up bool);
    i!(dpad_down bool);
    i!(dpad_left bool);
    i!(dpad_right bool);
    i!(lstick_x f32);
    i!(lstick_y f32);
    i!(lstick bool);
    i!(rstick_x f32);
    i!(rstick_y f32);
    i!(rstick bool);
    i!(ltrigger f32);
    i!(rtrigger f32);
    i!(a bool);
    i!(b bool);
    i!(x bool);
    i!(y bool);
    i!(lbumper bool);
    i!(rbumper bool);
}
