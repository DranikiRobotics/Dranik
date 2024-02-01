#[derive(Debug)]
pub struct Runtime {
    pub(crate) telemetry: super::telemetry::Telemetry,
    pub(crate) gamepad: super::gamepad::Gamepad,
}

impl Runtime {
    // pub fn new() -> Self {
    //     Self {
    //         telemetry: super::telemetry::Telemetry,
    //         gamepad: super::gamepad::Gamepad,
    //     }
    // }
    // pub fn shadow(&self) -> Self {
    //     Self {
    //         telemetry: self.telemetry,
    //         gamepad: self.gamepad,
    //     }
    // }
}

/// Python bindings
#[doc(hidden)]
#[path = "pyruntime.rs"]
pub mod py;

macro_rules! i {
    ($method:ident $type:ty) => (
        #[inline] fn $method(&self) -> $type {
            self.gamepad.$method()
        }
    )
}

impl ::dranik_api::Telemetry for Runtime {
    #[inline]
    fn send(&self, message: ::dranik_api::TelemetryMessage) {
        self.telemetry.send(message);
    }
}

impl ::dranik_api::Gamepad for Runtime {
    i!(dpad_up bool);
    i!(dpad_down bool);
    i!(dpad_left bool);
    i!(dpad_right bool);
    i!(a bool);
    i!(b bool);
    i!(x bool);
    i!(y bool);
    i!(lstick bool);
    i!(rstick bool);
    i!(lstick_x f32);
    i!(lstick_y f32);
    i!(rstick_x f32);
    i!(rstick_y f32);
    i!(ltrigger f32);
    i!(rtrigger f32);
    i!(lbumper bool);
    i!(rbumper bool);
}

impl ::dranik_api::DranikRuntime for Runtime {}
