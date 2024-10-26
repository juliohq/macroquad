//! Cross platform system time access and FPS counters.

use crate::{color::Color, get_context, text::draw_text};

/// Draws the current FPS on the screen.
pub fn draw_fps(x: f32, y: f32, font_size: f32, color: Color) {
    draw_text(&format!("FPS: {}", get_fps()), x, y, font_size, color);
}

/// Returns current FPS
pub fn get_fps() -> i32 {
    let context = get_context();

    (1. / context.frame_time) as i32
}

/// Draws the duration in seconds of the last frame drawn on the screen.
pub fn draw_frame_time(x: f32, y: f32, font_size: f32, color: Color) {
    draw_text(
        &format!("Frame Time: {}", get_frame_time() * 1000.),
        x,
        y,
        font_size,
        color,
    );
}

/// Returns duration in seconds of the last frame drawn
pub fn get_frame_time() -> f32 {
    let context = get_context();

    if crate::experimental::scene::in_fixed_update() {
        crate::experimental::scene::fixed_frame_time()
    } else {
        context.frame_time as f32
    }
}

/// Returns elapsed wall-clock time in seconds since start
///
/// Note that as real world time progresses during computation,
/// the value returned will change. Therefore if you want
/// your game logic update to happen at the same *in-game* time
/// for all game objects, you should call this function once
/// save the value and reuse it throughout your code.
pub fn get_time() -> f64 {
    let context = get_context();

    miniquad::date::now() - context.start_time
}
