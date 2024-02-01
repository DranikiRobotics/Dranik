"""
# Dranik Core API

This module provides the core API for Dranik. It is intended to be used by
developers to create their own Dranik operations.

Example:
```python
from dranik.api import DranikRuntime

def main(r: DranikRuntime) -> None:
    r.log("Hello, world!")
```
"""

class DranikRuntime():
    """
    The runtime object for Dranik. This object is passed to the main function
    
    Methods:
    - `log`: Log a message to the console
    - `debug`: Log a debug message to the console

    Properties:
    - `dpad_up`: The state of the D-Pad up button
    - `dpad_down`: The state of the D-Pad down button
    - `dpad_left`: The state of the D-Pad left button
    - `dpad_right`: The state of the D-Pad right button
    - `lstick_x`: The x-axis value of the left stick
    - `lstick_y`: The y-axis value of the left stick
    - `lstick`: Wether the left stick is pressed
    - `rstick_x`: The x-axis value of the right stick
    - `rstick_y`: The y-axis value of the right stick
    - `rstick`: Wether the right stick is pressed
    - `ltrigger`: The value of the left trigger
    - `rtrigger`: The value of the right trigger
    - `a`: Wether the A button is pressed
    - `b`: Wether the B button is pressed
    - `x`: Wether the X button is pressed
    - `y`: Wether the Y button is pressed
    - `lbumper`: Wether the left bumper is pressed
    - `rbumper`: Wether the right bumper is pressed
    """
    def log(self, *message: object) -> None: ...
    def debug(self, *message: object) -> None: ...
    @property
    def dpad_up(self) -> bool: ...
    @property
    def dpad_down(self) -> bool: ...
    @property
    def dpad_left(self) -> bool: ...
    @property
    def dpad_right(self) -> bool: ...
    @property
    def lstick_x(self) -> float: ...
    @property
    def lstick_y(self) -> float: ...
    @property
    def lstick(self) -> bool: ...
    @property
    def rstick_x(self) -> float: ...
    @property
    def rstick_y(self) -> float: ...
    @property
    def rstick(self) -> bool: ...
    @property
    def ltrigger(self) -> float: ...
    @property
    def rtrigger(self) -> float: ...
    @property
    def a(self) -> bool: ...
    @property
    def b(self) -> bool: ...
    @property
    def x(self) -> bool: ...
    @property
    def y(self) -> bool: ...
    @property
    def lbumper(self) -> bool: ...
    @property
    def rbumper(self) -> bool: ...
