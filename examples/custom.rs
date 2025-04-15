fn main () {
    peroxide::widget! {
        CustomWidget  from peroxide::Button {
            x: u32,
            y: u32,
            width: i32,
            height: i32,
            id: i32,
        }
    };
    let btn = CustomWidget {
        x: 200,
        y: 50,
        width: 100,
        height: 30,
        id: 1,
    };
    peroxide::run_app(btn);

}