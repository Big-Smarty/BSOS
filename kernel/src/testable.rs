use crate::{serial_print, serial_println};

pub trait Testable {
    fn run(&self) -> ();
}
