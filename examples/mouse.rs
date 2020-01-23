extern crate sysinputs;

use sysinputs::mouse::{click, MouseButton};

fn main() {
    click(MouseButton::Left);
}
