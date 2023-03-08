use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Times Table Cardiod".to_owned(),
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let r = 240.0;
    let mut factor: f32 = 0.0;
    const TOTAL : f32  = 100.0;
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);
        draw_poly_lines(center_x, center_y, 100, r, 0., 1.0, WHITE);

        factor += 0.01;

        (0..TOTAL as u32).for_each(|i| {
            let a = get_vector(i as f32, TOTAL, r);
            let b = get_vector(i as f32 * factor, TOTAL, r);
            draw_translate(a, b, center_x, center_y);
        });

        let factor_text = format!("Factor: {:.2}", factor);
        draw_text(factor_text.as_str(), 20.0, 20.0, 30.0, WHITE);
        next_frame().await;
    }
}

struct Vector {
    x: f32,
    y: f32,
}

fn draw_translate(a: Vector, b: Vector, translate_x: f32, translate_y: f32) {
    let x1 = a.x + translate_x;
    let x2 = b.x + translate_x;
    let y1 = a.y + translate_y;
    let y2 = b.y + translate_y;

    draw_line(x1, y1, x2, y2, 1.0, WHITE);
}

fn map(value : f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (to_max - to_min) * (value - from_min) / (from_max  - from_min)
}

fn get_vector(index: f32, total: f32, radius: f32) -> Vector {
    let angle = map((index % total) as f32, 0.0, total, 0.0, 2.0 * std::f32::consts::PI);
    Vector { x: angle.cos() * radius, y: angle.sin() * radius }
}
