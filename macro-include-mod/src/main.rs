macro_rules! prelude {
    () => {
        #[macro_use] mod hello;
    }
}

prelude!();

fn main() {
    hello!();
}
