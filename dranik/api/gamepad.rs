//! Gamepad-related structs and traits

use crate::*;

/// A trait that allows for reading from a gamepad.
/// 
/// Although this is a trait, it is not meant to be implemented by the user.
/// Instead, it is implemented by this crate.
pub trait Gamepad: Clone + PartialEq + Send + Sync {
    /// Returns the state of the dpad
    ///
    /// Includes up, down, left, and right
    fn dpad(&self) -> Result<GamepadDpad>;
    /// Returns the state of the left stick
    ///
    /// Includes x, y, and pressed
    fn left_stick(&self) -> Result<GamepadStick>;
    /// Returns the state of the right stick
    ///
    /// Includes x, y, and pressed
    fn right_stick(&self) -> Result<GamepadStick>;
    /// Returns the state of the left trigger
    fn left_trigger(&self) -> Result<l2math::Float64>;
    /// Returns the state of the right trigger
    fn right_trigger(&self) -> Result<l2math::Float64>;
    /// Is the A button pressed?
    fn a(&self) -> Result<bool>;
    /// Is the B button pressed?
    fn b(&self) -> Result<bool>;
    /// Is the X button pressed?
    fn x(&self) -> Result<bool>;
    /// Is the Y button pressed?
    fn y(&self) -> Result<bool>;
    /// Is the left bumper pressed?
    fn left_bumper(&self) -> Result<bool>;
    /// Is the right bumper pressed?
    fn right_bumper(&self) -> Result<bool>;

    /// A non-standard 'back' button
    #[inline]
    fn back(&self) -> Result<bool> {
        Err(HardwareError::Other {
            message: "This gamepad does not have a 'back' button",
        })
    }
    /// A non-standard 'start' button
    #[inline]
    fn start(&self) -> Result<bool> {
        Err(HardwareError::Other {
            message: "This gamepad does not have a 'start' button",
        })
    }
}

/// A struct that holds the state of a gamepad stick
/// 
/// This is NOT a hardware component, but rather a struct
/// that is used as a wrapper for a sub-component of a gamepad.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct GamepadStick {
    /// How far the stick is pushed in the x direction (left/right)
    pub x: f64,
    /// How far the stick is pushed in the y direction (up/down)
    pub y: f64,
    /// Is the stick pressed down?
    pub pressed: bool,
}

impl GamepadStick {
    /// Creates a new `GamepadStick` with the given values
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    pub const fn new(x: f64, y: f64, pressed: bool) -> Self {
        Self { x, y, pressed }
    }
    /// Creates a new `GamepadStick` with all values set to their defaults
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    pub const fn default() -> Self {
        Self::new(0.0, 0.0, false)
    }
    /// Turns the x and y values of the stick into an angle
    ///
    /// This is useful for things like precision driving
    #[inline]
    pub fn angle(&self) -> libtrig::Angle2D {
        libtrig::prelude!();
        libtrig::Angle2D::from_radians(self.y.atan2(self.x))
    }
}

impl From<libtrig::Angle2D> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from(angle: libtrig::Angle2D) -> Self {
        libtrig::prelude!();
        Self::new(angle.cos(), angle.sin(), false)
    }
}

impl From<GamepadStick> for libtrig::Angle2D {
    #[inline]
    #[must_use = "This returns a new Angle2D"]
    fn from(stick: GamepadStick) -> Self {
        stick.angle()
    }
}

impl From<libtrig::Vec2D> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from(vec: libtrig::Vec2D) -> Self {
        Self::new(vec.x(), vec.y(), false)
    }
}

impl From<GamepadStick> for libtrig::Vec2D {
    #[inline]
    #[must_use = "This returns a new Vec2D"]
    fn from(stick: GamepadStick) -> Self {
        Self::new(stick.x, stick.y)
    }
}

impl From<(f64, f64, bool)> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from((x, y, pressed): (f64, f64, bool)) -> Self {
        Self::new(x, y, pressed)
    }
}

impl From<(f64, f64)> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y, false)
    }
}

impl Eq for GamepadStick {}

/// A struct that holds the state of a gamepad's Dpad
///
/// Includes up, down, left, and right.
/// 
/// This is NOT a hardware component, but rather a struct
/// that is used as a wrapper for a sub-component of a gamepad.
///
/// (Why is this not called GamepadDpad if it's more like a `+` symbol?)
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadDpad {
    /// Is the up button pressed?
    pub up: bool,
    /// Is the down button pressed?
    pub down: bool,
    /// Is the left button pressed?
    pub left: bool,
    /// Is the right button pressed?
    pub right: bool,
}

impl GamepadDpad {
    /// Creates a new `GamepadDpad` with the given values
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    pub const fn new(up: bool, down: bool, left: bool, right: bool) -> Self {
        Self { up, down, left, right }
    }
    /// Creates a new `GamepadDpad` with all values set to their defaults
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    pub const fn default() -> Self {
        Self::new(false, false, false, false)
    }
}

impl From<(bool, bool, bool, bool)> for GamepadDpad {
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    fn from((up, down, left, right): (bool, bool, bool, bool)) -> Self {
        Self::new(up, down, left, right)
    }
}

impl From<GamepadDpad> for (bool, bool, bool, bool) {
    #[inline]
    #[must_use = "This returns a new tuple"]
    fn from(dpad: GamepadDpad) -> Self {
        (dpad.up, dpad.down, dpad.left, dpad.right)
    }
}
