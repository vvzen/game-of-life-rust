mod lib;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn key_pressed(_app: &App, model: &mut lib::Model, key: Key) {
    //println!("Key pressed: {:?}", key);

    match key {
        // Start
        Key::S => {
            println!("User pressed 'S' for 'Start'.");
            model.state = lib::AppState::Running;
        }
        // Toggle grid
        Key::G => {
            model.should_draw_grid = !model.should_draw_grid;
        }
        // Clear
        Key::C => {
            model.cells.rows = lib::get_all_cells_as_dead();
        }
        // Reset
        Key::R => {
            println!("User pressed 'R' for 'Reset'.");

            model.current_stroke = Vec::new();
            model.state = lib::AppState::Init;

            let cells = lib::init_cells(model.num_cells_x, model.num_cells_y, true);
            model.cells = cells;
        }
        _ => {}
    }
}

fn mouse_pressed(_app: &App, model: &mut lib::Model, button: MouseButton) {
    //
    println!("Mouse pressed: {:?}", button);
    model.drawing_state = lib::DrawingState::Started;
    match button {
        MouseButton::Left => model.should_draw_white = true,
        MouseButton::Right => model.should_draw_white = false,
        _ => {}
    }
}

fn mouse_moved(_app: &App, model: &mut lib::Model, pos: Point2) {
    //
    model.last_mouse_pos = pt2(pos.x, pos.y);

    let closest_points = lib::closest_n_points(pos, &model.grid_points, 4);
    model.closest_points = closest_points;

    match model.state {
        lib::AppState::Init => {
            match model.drawing_state {
                // Start drawing
                lib::DrawingState::Started => {
                    // Snap the point to the grid
                    let snapped = lib::snap_to_grid(pos, &model);

                    // Discard clicks outside the target area
                    if snapped.x.abs() > model.app_width * 0.5 {
                        return;
                    }
                    if snapped.y.abs() > model.app_height * 0.5 {
                        return;
                    }

                    // Map the point to a cell in the grid
                    let cell_index_x = map_range(
                        snapped.x,
                        -model.app_width * 0.5,
                        model.app_width * 0.5,
                        0,
                        model.num_cells_x,
                    );
                    let cell_index_y = map_range(
                        snapped.y,
                        -model.app_height * 0.5,
                        model.app_height * 0.5,
                        model.num_cells_y - 1,
                        0,
                    );

                    let is_alive = model.should_draw_white;
                    model.cells.rows[cell_index_x].values[cell_index_y].is_alive = is_alive;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn mouse_released(_app: &App, model: &mut lib::Model, _button: MouseButton) {
    //
    //println!("Mouse released");

    match model.drawing_state {
        lib::DrawingState::Started => {
            model.drawing_state = lib::DrawingState::Ended;
        }
        _ => {}
    }
}

fn model(app: &App) -> lib::Model {
    // Set up the window
    app.new_window()
        .title("Game of Life")
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .mouse_released(mouse_released)
        .size(512, 512)
        .build()
        .unwrap();

    //app.main_window().set_resizable(false);

    let lines = lib::create_grid(app, lib::CELL_SIZE);
    println!("Created {} lines", lines.len());

    let window_rect = app.window_rect();

    let width = window_rect.w() as f32;
    let height = window_rect.h() as f32;

    let num_cells_x = width as i32 / lib::CELL_SIZE as i32;
    let num_cells_y = height as i32 / lib::CELL_SIZE as i32;

    // Initialize all of the cells
    let cells = lib::init_cells(num_cells_x as usize, num_cells_y as usize, true);

    // Calculate the integers that make up the grid
    let w = (width) as i32;
    let h = (height) as i32;

    let mut grid_points = Vec::new();

    for y in (-h..h).step_by(lib::CELL_SIZE) {
        for x in (-w..w).step_by(lib::CELL_SIZE) {
            grid_points.push(pt2(x as f32, y as f32));
        }
    }

    println!("Canvas size is {}x{}", width, height);
    println!("Cell size is {}", lib::CELL_SIZE);

    lib::Model {
        lines,
        cells,
        cell_size: lib::CELL_SIZE,
        app_width: width,
        app_height: height,
        num_cells_x: num_cells_x as usize,
        num_cells_y: num_cells_y as usize,
        state: lib::AppState::Init,
        should_draw_grid: false,
        should_draw_white: true,
        drawing_state: lib::DrawingState::Void,
        current_stroke: Vec::new(),
        grid_points,
        last_mouse_pos: pt2(0.0, 0.0),
        closest_points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut lib::Model, _update: Update) {
    // Do the game of life only when needed
    match model.state {
        lib::AppState::Running => lib::game_of_life(model),
        _ => return,
    }
}

fn view(app: &App, model: &lib::Model, frame: Frame) {
    let canvas = app.draw();
    canvas.background().color(BLACK);

    // Draw the cells
    for (i, cell_row) in model.cells.rows.iter().enumerate() {
        for (j, cell_value) in cell_row.values.iter().enumerate() {
            lib::draw_cell(i, j, &cell_value.is_alive, model, &canvas);
        }
    }

    // Draw the grid (if requested)
    if model.should_draw_grid {
        for line in model.lines.iter() {
            canvas
                .line()
                .start(pt2(line.start_x, line.start_y))
                .end(pt2(line.end_x, line.end_y))
                .weight(line.weight)
                .color(WHITE);
        }

        // Debugging: draw the coordinates too
        //for point in &model.grid_points {
        //    let text = format!("{},{}", point.x, point.y);

        //    canvas
        //        .text(&text)
        //        .font_size(16)
        //        .x_y(point.x, point.y)
        //        .color(RED);
        //}
    }

    // Debug: draw the current mouse position
    //let mouse_pos = format!("{},{}", model.last_mouse_pos.x, model.last_mouse_pos.y);

    //canvas
    //    .text(&mouse_pos)
    //    .font_size(16)
    //    .x_y(model.last_mouse_pos.x, model.last_mouse_pos.y)
    //    .color(RED);

    // Draw the current closest points to the mouse
    //for close_point in model.closest_points.iter() {
    //    canvas
    //        .ellipse()
    //        .w(32.0)
    //        .h(32.0)
    //        .x_y(close_point.x, close_point.y)
    //        .color(rgba(0.0, 0.0, 0.0, 0.5));
    //}

    canvas.to_frame(app, &frame).unwrap();
}
