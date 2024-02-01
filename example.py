from dranik.api import DranikRuntime

def main(r: DranikRuntime) -> None:
    r.log("Hello, world!")
    r.debug("This is a debug message.")
    r.debug("The left stick is at", r.lstick_x, r.lstick_y)
    r.debug("The right stick is at", r.rstick_x, r.rstick_y)
    r.debug("The left trigger is at", r.ltrigger)
    r.debug("The right trigger is at", r.rtrigger)
    r.debug("The A button is", pressed(r.a))
    r.debug("The B button is", pressed(r.b))
    r.debug("The X button is", pressed(r.x))
    r.debug("The Y button is", pressed(r.y))
    r.debug("The left bumper is", pressed(r.lbumper))
    r.debug("The right bumper is", pressed(r.rbumper))
    r.debug("The D-Pad is at",
        pressed(r.dpad_up),
        pressed(r.dpad_down),
        pressed(r.dpad_left),
        pressed(r.dpad_right)
    )
    r.debug("The left stick is", pressed(r.lstick))
    r.debug("The right stick is", pressed(r.rstick))

def pressed(b: bool) -> str: return "pressed" if b else "not pressed"
