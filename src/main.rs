use macroquad::prelude::*;

const BALL_ACCELERATION: f32 = 0.5;
const DAMPING: f32 = 0.95;
const GRAVITY: f32 = 0.1;
const FONT_SIZE : f32 = 25.0;

#[macroquad::main("When the when the when the whn the")]
async fn main() {
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;
    let mut speed_x = 0.0;
    let mut speed_y = 0.0;

    let mut top_speed : f32 = 0.0;

    let mut paused : bool = false;

    loop {
        // pause
        if is_key_pressed(KeyCode::P){
            paused = !paused;
        }

        // Check if the game is paused at the beginning of the loop
        if !paused {
            // apply speed
            ball_x += speed_x;
            ball_y += speed_y;

            // bounce against walls
            if ball_x < 0.0 || ball_x > screen_width() {
                speed_x *= -1.0;
            }
            if ball_y < 0.0 || ball_y > screen_height() {
                speed_y *= -1.0;
            }

            // clamp within screen
            ball_x = ball_x.clamp(0.0, screen_width());
            ball_y = ball_y.clamp(0.0, screen_height());

            #[cfg(not(target_arch = "wasm32"))]
            if is_key_pressed(KeyCode::Q) { // quit
                break;
            }

            // reset
            if is_key_pressed(KeyCode::R) {
                ball_x = screen_width() / 2.0;
                ball_y = screen_height() / 2.0;
                speed_x = 0.0;
                speed_y = 0.0;
            }

            let speed = (speed_x.powi(2) + speed_y.powi(2)).sqrt();
            if speed < 0.1 {
                speed_x = 0.0;
                speed_y = 0.0;
            } else {
                // apply gravity
                speed_y += GRAVITY;
            }

            top_speed = f32::max(top_speed, speed);

            // apply damping?
            if is_key_down(KeyCode::Space) {
                speed_x *= DAMPING;
                speed_y *= DAMPING;
            }

            if is_key_down(KeyCode::D) {
                speed_x += BALL_ACCELERATION;
            }
            if is_key_down(KeyCode::A) {
                speed_x -= BALL_ACCELERATION;
            }
            if is_key_down(KeyCode::S) {
                speed_y += BALL_ACCELERATION;
            }
            if is_key_down(KeyCode::W) {
                speed_y -= BALL_ACCELERATION;
            }
        }

        // draw background
        clear_background(RED);
        draw_text("WASD to apply acceleration", 20.0, 40.0, FONT_SIZE, BLACK);
        draw_text("SPACEBAR to apply damping", 20.0, 60.0, FONT_SIZE, BLACK);
        draw_text("P to pause, R to reset", 20.0, 80.0, FONT_SIZE, BLACK);
        draw_text(&format!("Speed: {}", (speed_x.powi(2) + speed_y.powi(2)).sqrt()), 20.0, 120.0, FONT_SIZE, BLACK);
        draw_text(&format!("Top speed: {}", top_speed), 20.0, 140.0, FONT_SIZE, BLACK);

        // draw foreground
        draw_circle(ball_x, ball_y, 15.0, YELLOW);
        draw_line(
            ball_x,
            ball_y,
            ball_x + speed_x * 10.0,
            ball_y + speed_y * 10.0,
            5.0,
            BLUE,
        );

        if is_key_down(KeyCode::Space) {
            draw_circle(ball_x, ball_y, 20.0, GRAY);
        }

        next_frame().await;
    }
}