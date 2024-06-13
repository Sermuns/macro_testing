use macroquad::prelude::*;
use rustfft::num_complex::Complex;

const DIRECTIONAL_ACCELERATION: f32 = 0.5;
const FORWARD_ACCELERATION: f32 = 0.001;
const DAMPING: f32 = 0.95;
const GRAVITY: f32 = 0.0;
const FONT_SIZE: f32 = 25.0;
const BALL_RADIUS: f32 = 15.0;
const LINE_THICKNESS: f32 = 5.0;

const FFT_ELEMENTS: usize = 100;

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
    let mut show_fft: bool = false;

    let mut previous_y_positions = vec![0.0; FFT_ELEMENTS];
    let mut fft_data = vec![0.0; FFT_ELEMENTS];

    loop {
        handle_states(&mut paused, &mut show_fft);
        if !paused {
            speed = calculate_speed(speed_x, speed_y);
            top_speed = f32::max(top_speed, speed);

            handle_movement(&mut ball_x, &mut ball_y, &mut speed_x, &mut speed_y, &speed);
            handle_damping(&mut speed_x, &mut speed_y, speed);
            handle_acceleration(&mut speed_x, &mut speed_y);

            // calculate FFT
            let y = ball_y;
            previous_y_positions.remove(0);
            previous_y_positions.push(y);
            let fft = rustfft::FftPlanner::new().plan_fft_forward(FFT_ELEMENTS);
            let mut input: Vec<Complex<f32>> = previous_y_positions
                .iter()
                .map(|&y| Complex::new(y, 0.0))
                .collect();
            let mut output = vec![Complex::new(0.0, 0.0); FFT_ELEMENTS];
            fft.process_outofplace_with_scratch(&mut input, &mut output, &mut []);
            fft_data = output.iter().map(|c| c.norm()).collect();
        }

        draw_ball_background(speed, top_speed);
        draw_ball_foreground(ball_x, ball_y, speed_x, speed_y);

        if show_fft {
            draw_fourier(&fft_data);
        }

        next_frame().await;
    }
}

/// Display the FFT data in a graph
fn draw_fourier(fft_data: &[f32]) {

    // Draw semi-transparent background
    let opacity = 0.6;
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, opacity));

    // Find the maximum value in the FFT data
    let max_value = fft_data.iter().cloned().fold(f32::MIN, f32::max);

    // Calculate the width of each bar in the graph and the scaling factor for the heights
    let width = screen_width() / fft_data.len() as f32;
    let height_scale = screen_height() / max_value;

    // Draw a rectangle for each value in the FFT data
    // Skip the first value, as it is the DC component
    // Skip the second half, symmetric for real input
    for (i, &value) in fft_data.iter().enumerate().skip(1).take(fft_data.len() / 2) {
        let rect_height = value * height_scale;
        let x = i as f32 * width;
        let y = screen_height() - rect_height;
        draw_rectangle(x, y, width, rect_height, BLUE);
    }
}

fn handle_states(paused: &mut bool, show_ball: &mut bool) {
    if is_key_pressed(KeyCode::P) {
        *paused = !*paused;
    }
    if is_key_pressed(KeyCode::T) {
        *show_ball = !*show_ball;
    }
}

fn handle_movement(
    ball_x: &mut f32,
    ball_y: &mut f32,
    speed_x: &mut f32,
    speed_y: &mut f32,
    speed: &f32,
) {
    // apply gravity
    *speed_y += GRAVITY;

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

fn draw_ball_background(speed: f32, top_speed: f32) {
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

fn draw_ball_foreground(ball_x: f32, ball_y: f32, speed_x: f32, speed_y: f32) {
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
