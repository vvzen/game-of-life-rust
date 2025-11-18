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
            println!("User pressed 'G' to toggle the grid.");
            model.should_draw_grid = !model.should_draw_grid;
        }
        // Clear
        Key::C => {
            println!("User pressed 'C' to clear the cells.");
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
    println!("Mouse pressed: {button:?}");
    model.drawing_state = lib::DrawingState::Started;
    match button {
        MouseButton::Left => model.should_draw_white = true,
        MouseButton::Right => model.should_draw_white = false,
        _ => {}
    }
}

fn mouse_moved(_app: &App, model: &mut lib::Model, pos: Point2) {
    model.last_mouse_pos = pt2(pos.x, pos.y);

    let closest_points = lib::closest_n_points(pos, &model.grid_points, 4);
    model.closest_points = closest_points;

    if let lib::AppState::Init = model.state {
        // Start drawing
        if let lib::DrawingState::Started = model.drawing_state {
            // Snap the point to the grid
            let snapped = lib::snap_to_grid(pos, model);

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
    }
}

fn mouse_released(_app: &App, model: &mut lib::Model, _button: MouseButton) {
    //
    //println!("Mouse released");

    if let lib::DrawingState::Started = model.drawing_state {
        model.drawing_state = lib::DrawingState::Ended;
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

    let window_rect = app.window_rect();
    let width = window_rect.w();
    let height = window_rect.h();

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

    // Create the lines that make up the grid
    let lines = lib::create_grid(app, lib::CELL_SIZE);

    println!("Canvas size is {width}x{height}");
    println!("Cell size is {}", lib::CELL_SIZE);

    println!("INSTRUCTIONS:");
    println!("Draw cells with the mouse left (alive) or right (dead) button");
    println!("Press 'G' to toggle the grid view.");
    println!("Press 'S' to start the simulation.");
    println!("Press 'R' to reset the simulation.");
    println!("Press 'C' to clear all cells (set all cells to dead).");

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
        generations: 0,
    }
}

fn update(app: &App, model: &mut lib::Model, _update: Update) {
    if app.elapsed_frames() % 5 != 0 {
        return;
    }

    // Do the game of life only when needed
    if let lib::AppState::Running = model.state {
        lib::game_of_life(model);
        model.generations += 1;
        println!("Generation: {}", model.generations);
    }
}

fn view(app: &App, model: &lib::Model, frame: Frame) {
    let canvas = app.draw();
    canvas.background().color(BLACK);

    if app.elapsed_frames() % 5 != 0 {
        return;
    }

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
    }

    canvas.to_frame(app, &frame).unwrap();
}
