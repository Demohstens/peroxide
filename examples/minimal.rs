use peroxide::Widget;

fn main() {
    struct RootWidget {}
    impl Widget for RootWidget {}
    peroxide::run_app("hello!");
}