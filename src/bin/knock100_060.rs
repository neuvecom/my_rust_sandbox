/*
中心座標を入力させ、横600縦400のウィンドウを開き、入力した座標に半径50の円を描くプログラム
ここからは、nannouがよさそう
--
Nannou で始める Rust / Creative-Coding 
https://zenn.dev/pvcresin/articles/4b9edacc87527a
--
cargo add nannou
*/

mod lib_knock100_get_point;
use crate::lib_knock100_get_point::get_point;
use nannou::prelude::*;

fn main() {
    let x = get_point("input center point x(-250.0〜250.0)".to_string());
    let y = get_point("input center point y(-150.0〜150.0)".to_string());
    println!("x: {}, y: {}", x, y);
    // nannou::sketch(view).run();
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    bg_color: Srgb<u8>,
    fg_color: Srgb<u8>,
    step_length: f32,
    start: Point2,
    end: Point2,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(600, 400).view(view).build().unwrap();
    Model {
        _window,
        bg_color: WHITE,
        fg_color: STEELBLUE,
        step_length: 10.0,
        start: pt2(0.0, 0.0),
        end: pt2(0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.start = model.end;
    let angle = random_range(0.0, 2.0 * PI);
    let vec = vec2(angle.cos(), angle.sin()) * model.step_length;
    model.end = model.start + vec;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(model.bg_color);
    }
    draw.line()
        .color(model.fg_color)
        .start(model.start)
        .end(model.end);
    draw.to_frame(app, &frame).unwrap();
}
