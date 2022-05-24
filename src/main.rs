use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
struct LineProperties {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    weight: f32,
}

struct Model {
    lines: Vec<LineProperties>,
    app_height: f32,
    app_width: i32,
}

fn draw_grid(app: &App) -> Vec<LineProperties> {
    let mut lines = Vec::new();
    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();

    let window = app.window_rect();
    let width = window.w() as i32;
    let height = window.w() as i32;
    let step_size = 10;

    let start_w = -width / 2;
    let end_w = width / 2;

    let start_h = -height / 2;
    let end_h = height / 2;

    // Horizontal lines
    for i in (start_h..end_h).step_by(step_size) {
        let current_y = i as f32;

        let show_thicker_line = match horizontal_lines.len() % 10 {
            1 => true,
            _ => false,
        };

        let current_weight = if show_thicker_line { 2.0 } else { 0.5 } as f32;

        let line_props = LineProperties {
            start_x: start_w as f32,
            end_x: end_w as f32,
            start_y: current_y,
            end_y: current_y,
            weight: current_weight,
        };

        horizontal_lines.push(line_props);
        lines.push(line_props);
    }

    // Vertical lines
    for j in (start_w..end_w).step_by(step_size) {
        let current_x = j as f32;

        let show_thicker_line = match vertical_lines.len() % 10 {
            1 => true,
            _ => false,
        };
        let current_weight = if show_thicker_line { 2.0 } else { 0.5 } as f32;

        let line_props = LineProperties {
            start_x: current_x,
            end_x: current_x,
            start_y: start_h as f32,
            end_y: end_h as f32,
            weight: current_weight,
        };

        vertical_lines.push(line_props);
        lines.push(line_props);
    }

    return lines;
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(app: &App) -> Model {
    let lines = draw_grid(app);
    println!("Created {} lines", lines.len());

    let window = app.window_rect();
    let width = window.w() as i32;
    let height = window.h() as f32;

    Model {
        lines: lines,
        app_height: height,
        app_width: width,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let canvas = app.draw();

    for line in model.lines.iter() {
        //println!("Drawing line on x={}, and weight={}", line.x, line.weight);

        canvas
            .line()
            .start(pt2(line.start_x, line.start_y))
            .end(pt2(line.end_x, line.end_y))
            .weight(line.weight)
            .color(BLACK);
    }

    canvas.background().color(WHITE);
    canvas.to_frame(app, &frame).unwrap();
}
