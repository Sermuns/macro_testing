use macroquad::prelude::*;

const DIRECTIONAL_ACCELERATION: f32 = 0.5;
const FORWARD_ACCELERATION: f32 = 0.001;
const DAMPING: f32 = 0.95;
const GRAVITY: f32 = 0.1;
const FONT_SIZE: f32 = 25.0;
const BALL_RADIUS: f32 = 15.0;
const LINE_THICKNESS: f32 = 5.0;

const TITLE: &str = "When the when the when the whn the";

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;
    let mut speed_x = 0.0;
    let mut speed_y = 0.0;
    let mut speed: f32 = 0.0;

    let mut top_speed: f32 = 0.0;

    let mut paused: bool = false;

    loop {
        handle_pause(&mut paused);
        if !paused {
            speed = calculate_speed(speed_x, speed_y);
            top_speed = f32::max(top_speed, speed);

            handle_movement(&mut ball_x, &mut ball_y, &mut speed_x, &mut speed_y, &speed);
            handle_damping(&mut speed_x, &mut speed_y, speed);
            handle_acceleration(&mut speed_x, &mut speed_y);
        }

        draw_background(speed, top_speed);
        draw_foreground(ball_x, ball_y, speed_x, speed_y);

        next_frame().await;
    }
}

fn handle_pause(paused: &mut bool) {
    if is_key_pressed(KeyCode::P) {
        *paused = !*paused;
    }
}

fn handle_movement(
    ball_x: &mut f32,
    ball_y: &mut f32,
    speed_x: &mut f32,
    speed_y: &mut f32,
    speed: &f32,
) {
    // apply speed
    *ball_x += *speed_x;
    *ball_y += *speed_y;

    // bounce against walls
    if *ball_x < BALL_RADIUS || *ball_x > screen_width() - BALL_RADIUS {
        *speed_x *= -1.0;
    }
    if *ball_y < BALL_RADIUS || *ball_y > screen_height() - BALL_RADIUS {
        *speed_y *= -1.0;
    }

    // clamp within screen
    *ball_x = ball_x.clamp(BALL_RADIUS, screen_width() - BALL_RADIUS);
    *ball_y = ball_y.clamp(BALL_RADIUS, screen_height() - BALL_RADIUS);

    #[cfg(not(target_arch = "wasm32"))]
    if is_key_pressed(KeyCode::Q) {
        // quit
        std::process::exit(0);
    }

    // reset
    if is_key_pressed(KeyCode::R) {
        *ball_x = screen_width() / 2.0;
        *ball_y = screen_height() / 2.0;
        *speed_x = 0.0;
        *speed_y = 0.0;
    }

    if *speed < 0.1 {
        *speed_x = 0.0;
        *speed_y = 0.0;
    } else {
        // apply gravity
        *speed_y += GRAVITY;
    }
}

fn calculate_speed(speed_x: f32, speed_y: f32) -> f32 {
    (speed_x.powi(2) + speed_y.powi(2)).sqrt()
}

fn handle_damping(speed_x: &mut f32, speed_y: &mut f32, speed: f32) {
    if is_key_down(KeyCode::Space) && speed > 0.1 {
        *speed_x *= DAMPING;
        *speed_y *= DAMPING;
    }
}

fn handle_acceleration(speed_x: &mut f32, speed_y: &mut f32) {
    if is_key_down(KeyCode::F) {
        *speed_x *= 1.0 + FORWARD_ACCELERATION;
        *speed_y *= 1.0 + FORWARD_ACCELERATION;
    }

    if is_key_down(KeyCode::D) {
        *speed_x += DIRECTIONAL_ACCELERATION;
    }
    if is_key_down(KeyCode::A) {
        *speed_x -= DIRECTIONAL_ACCELERATION;
    }
    if is_key_down(KeyCode::S) {
        *speed_y += DIRECTIONAL_ACCELERATION;
    }
    if is_key_down(KeyCode::W) {
        *speed_y -= DIRECTIONAL_ACCELERATION;
    }
}

fn draw_background(speed: f32, top_speed: f32) {
    clear_background(RED);
    let text_lines = vec![
        "WASD to apply acceleration".to_string(),
        "SPACEBAR to apply damping".to_string(),
        "P to pause, R to reset".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        "Q to quit".to_string(),
        #[cfg(target_arch = "wasm32")]
        "...you can't quit in the browser".to_string(),

        "".to_string(),

        format!("Speed: {}", speed),
        format!("Top speed: {}", top_speed),

    ];

    for (i, line) in text_lines.iter().enumerate() {
        draw_text(&line, 20.0, 40.0 + i as f32 * FONT_SIZE, FONT_SIZE, BLACK);
    }
}

fn draw_foreground(ball_x: f32, ball_y: f32, speed_x: f32, speed_y: f32) {
    draw_circle(ball_x, ball_y, BALL_RADIUS, YELLOW);
    draw_line(
        ball_x,
        ball_y,
        ball_x + speed_x * 10.0,
        ball_y + speed_y * 10.0,
        LINE_THICKNESS,
        BLUE,
    );

    if is_key_down(KeyCode::Space) {
        draw_circle(ball_x, ball_y, BALL_RADIUS + 5.0, GRAY);
    }
    if is_key_down(KeyCode::F) {
        draw_circle(ball_x, ball_y, BALL_RADIUS + 5.0, GREEN);
    }
}
