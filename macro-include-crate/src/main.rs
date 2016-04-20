macro_rules! prelude {
    () => {
        #[macro_use] extern crate hola;
    }
}

prelude!();

fn main() {
    hola!();
}
