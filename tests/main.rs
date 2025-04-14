#[cfg(test)]
mod tests {
    use peroxide::*;

    use crate::*;

    #[test]
    fn run_basic_window() {
        struct BaseWidgeet {}
        impl Widget for BaseWidgeet{};

        let w = BaseWidgeet{};
        run_app(w).unwrap();
    }

    fn draw_single_color() {
        
    }
}


