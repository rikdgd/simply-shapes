use crate::shapes::color::Color;

pub trait Screen {
    fn start(&mut self) -> ();
    fn stop(&mut self) -> ();
    fn fill(&mut self, color: &Color) -> ();
}


macro_rules! generate_loop {
    (main_loop $l:expr) => {
        let val: usize = $l;
        println!("The answer is: {}", val);
    }
}

pub(crate) use generate_loop;