/// A trait that allows for reading from a gamepad.
/// 
/// Although this is a trait, it is not meant to be implemented by the user.
/// Instead, it is implemented by this crate.
pub trait Gamepad {
    /// Returns true if the dpad up button is pressed.
    fn dpad_up(&self) -> bool;
    /// Returns true if the dpad down button is pressed.
    fn dpad_down(&self) -> bool;
    /// Returns true if the dpad left button is pressed.
    fn dpad_left(&self) -> bool;
    /// Returns true if the dpad right button is pressed.
    fn dpad_right(&self) -> bool;
    /// Returns the x value of the left stick.
    fn lstick_x(&self) -> f32;
    /// Returns the y value of the left stick.
    fn lstick_y(&self) -> f32;
    /// Returns true if the left stick is pressed.
    fn lstick(&self) -> bool;
    /// Returns the x value of the right stick.
    fn rstick_x(&self) -> f32;
    /// Returns the y value of the right stick.
    fn rstick_y(&self) -> f32;
    /// Returns true if the right stick is pressed.
    fn rstick(&self) -> bool;
    /// Returns the value of the left trigger.
    fn ltrigger(&self) -> f32;
    /// Returns the value of the right trigger.
    fn rtrigger(&self) -> f32;
    /// Returns true if the a button is pressed.
    fn a(&self) -> bool;
    /// Returns true if the b button is pressed.
    fn b(&self) -> bool;
    /// Returns true if the x button is pressed.
    fn x(&self) -> bool;
    /// Returns true if the y button is pressed.
    fn y(&self) -> bool;
    /// Returns true if the left bumper is pressed.
    fn lbumper(&self) -> bool;
    /// Returns true if the right bumper is pressed.
    fn rbumper(&self) -> bool;
}
