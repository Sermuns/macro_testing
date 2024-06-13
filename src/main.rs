use macroquad::prelude::*;

const BALL_ACCELERATION: f32 = 0.5;
const DAMPING: f32 = 0.95;
const GRAVITY: f32 = 0.1;
const FONT_SIZE : f32 = 25.0;
const BALL_RADIUS: f32 = 15.0;
const LINE_THICKNESS: f32 = 5.0;

#[macroquad::main("When the when the when the whn the")]
async fn main() {
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;
    let mut speed_x = 0.0;
    let mut speed_y = 0.0;
    let mut speed : f32 = 0.0;

    let mut top_speed : f32 = 0.0;

    let mut paused : bool = false;

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
    if is_key_pressed(KeyCode::P){
        *paused = !*paused;
    }
}

fn handle_movement(ball_x: &mut f32, ball_y: &mut f32, speed_x: &mut f32, speed_y: &mut f32, speed: &f32) {
    // apply speed
    *ball_x += *speed_x;
    *ball_y += *speed_y;

    // bounce against walls
    if *ball_x < 0.0 || *ball_x > screen_width() {
        *speed_x *= -1.0;
    }
    if *ball_y < 0.0 || *ball_y > screen_height() {
        *speed_y *= -1.0;
    }

    // clamp within screen
    *ball_x = ball_x.clamp(0.0, screen_width());
    *ball_y = ball_y.clamp(0.0, screen_height());

    #[cfg(not(target_arch = "wasm32"))]
    if is_key_pressed(KeyCode::Q) { // quit
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
    if is_key_down(KeyCode::D) {
        *speed_x += BALL_ACCELERATION;
    }
    if is_key_down(KeyCode::A) {
        *speed_x -= BALL_ACCELERATION;
    }
    if is_key_down(KeyCode::S) {
        *speed_y += BALL_ACCELERATION;
    }
    if is_key_down(KeyCode::W) {
        *speed_y -= BALL_ACCELERATION;
    }
}

fn draw_background(speed: f32, top_speed: f32) {
    clear_background(RED);
    draw_text("WASD to apply acceleration", 20.0, 40.0, FONT_SIZE, BLACK);
    draw_text("SPACEBAR to apply damping", 20.0, 60.0, FONT_SIZE, BLACK);
    draw_text("P to pause, R to reset", 20.0, 80.0, FONT_SIZE, BLACK);
    draw_text(&format!("Speed: {}", speed), 20.0, 120.0, FONT_SIZE, BLACK);
    draw_text(&format!("Top speed: {}", top_speed), 20.0, 140.0, FONT_SIZE, BLACK);
    draw_text("Hehe... you can't quit in the browser", 20.0, 180.0, FONT_SIZE, BLACK);
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
}