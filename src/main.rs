use macroquad::prelude::*;

const BALL_ACCELERATION: f32 = 0.5;
const DAMPING: f32 = 0.95;
const GRAVITY: f32 = 0.1;

//
#[macroquad::main("When the when the when the whn the")]
async fn main() {
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;
    let mut speed_x = 0.0;
    let mut speed_y = 0.0;

    let mut paused : bool = false;
    let mut old_p : bool = false;

    loop {

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


        // pause
        if is_key_pressed(KeyCode::P){
            paused = !paused;
        }

        let speed = (speed_x.powi(2) + speed_y.powi(2)).sqrt();
        let speed_text = format!("Speed: {}", speed);
        if speed < 0.1 {
            speed_x = 0.0;
            speed_y = 0.0;
        } else {
            // apply gravity
            speed_y += GRAVITY;
        }

        // draw background
        clear_background(RED);
        draw_text("WASD to apply acceleration", 20.0, 40.0, 30.0, BLACK);
        draw_text("SPACEBAR to apply damping", 20.0, 60.0, 30.0, BLACK);
        draw_text(&speed_text, 20.0, 100.0, 30.0, BLACK);

        // apply brakes, frictio7
        if is_key_down(KeyCode::Space) {
            speed_x *= DAMPING;
            speed_y *= DAMPING;
            draw_circle(ball_x, ball_y, 20.0, GRAY);
        }

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


        if paused {
            next_frame().await;
            continue;
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

        next_frame().await;

    }
}
