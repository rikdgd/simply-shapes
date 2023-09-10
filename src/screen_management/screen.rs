use crate::shapes::color::Color;

pub trait Screen {
    fn start(&mut self) -> ();
    fn stop(&mut self) -> ();
    fn fill(&mut self, color: &Color) -> ();
}
