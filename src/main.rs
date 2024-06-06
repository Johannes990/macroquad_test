use lib2D;


fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Testing macroquad...".to_owned(),
        window_height: 350,
        high_dpi: false,
        fullscreen: false,
        window_width: 500,
        window_resizable: true,
        icon: None,
        sample_count: 0,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        lib2D::background().await;
        lib2D::draw_circles().await;
        lib2D::show_screen_size().await;
        macroquad::prelude::next_frame().await;
    }
}