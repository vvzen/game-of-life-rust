mod lib;

use nannou::prelude::*;
use rand::Rng;

// Turn on if you want to display the grid
const DRAW_GRID: bool = true;

// Increase the denominator if you want smaller cells
//pub const STEP_SIZE: usize = lib::GRID_SIZE / 2;
pub const STEP_SIZE: usize = 64;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn key_pressed(_app: &App, model: &mut lib::Model, key: Key) {
    println!("Key pressed: {:?}", key);

    match key {
        // Start
        Key::S => {
            println!("User pressed 'S' for 'Start'.");
            model.state = lib::AppState::Running;
        }
        // Clear
        Key::C => {
            model.current_stroke = Vec::new();
        }
        // Reset
        Key::R => {
            println!("User pressed 'R' for 'Reset'.");
            model.state = lib::AppState::Init;

            let mut rows = [lib::CellsRow {
                values: [lib::Cell { is_alive: false }; lib::GRID_SIZE],
            }; lib::GRID_SIZE];

            for x in 0..model.num_cells_x {
                let mut values = [lib::Cell { is_alive: false }; lib::GRID_SIZE];

                for y in 0..model.num_cells_y {
                    values[y as usize].is_alive = false;
                }
                let row = lib::CellsRow { values };

                rows[x as usize] = row;
            }
            let cells = lib::Cells { rows };
            model.cells = cells;
        }
        _ => {}
    }
}

fn mouse_pressed(_app: &App, model: &mut lib::Model, _button: MouseButton) {
    //
    //println!("Mouse pressed");
    model.drawing_state = lib::DrawingState::Started;
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
                    //println!("Mouse moved to: {:?}", pos);

                    // Snap the point to the grid
                    //println!("");
                    let snapped = lib::snap_to_grid(pos, &model);
                    println!("Snapped {:?} to {:?}", pos, snapped);

                    // Offset it to draw it
                    let point = pt2(
                        snapped.x + lib::CELL_SIZE as f32 * 0.5,
                        snapped.y + lib::CELL_SIZE as f32 * 0.5,
                    );
                    println!("After offset: {:?}", point);

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
                    println!(
                        "Mapped {:?} to {:?}",
                        point,
                        pt2(cell_index_x as f32, cell_index_y as f32)
                    );
                    model.cells.rows[cell_index_x].values[cell_index_y].is_alive = true;

                    //model.current_stroke = Vec::new();
                    model.current_stroke.push(point);
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
    // Set up the window size
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

    // Create a GRID_SIZExGRID_SIZE grid on the stack
    let mut rows = [lib::CellsRow {
        values: [lib::Cell { is_alive: false }; lib::GRID_SIZE],
    }; lib::GRID_SIZE];

    let num_cells_x = width as i32 / lib::CELL_SIZE as i32;
    let num_cells_y = height as i32 / lib::CELL_SIZE as i32;

    // Initialize all of the cells
    let mut generator = rand::thread_rng();

    for x in 0..num_cells_x {
        let mut values = [lib::Cell { is_alive: false }; lib::GRID_SIZE];

        for y in 0..num_cells_y {
            values[y as usize].is_alive = false;
        }
        let row = lib::CellsRow { values };

        rows[x as usize] = row;
    }
    let cells = lib::Cells { rows };

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

    let model = lib::Model {
        lines,
        cells,
        cell_size: lib::CELL_SIZE,
        app_width: width,
        app_height: height,
        num_cells_x: num_cells_x as usize,
        num_cells_y: num_cells_y as usize,
        generator,
        state: lib::AppState::Init,
        drawing_state: lib::DrawingState::Void,
        current_stroke: Vec::new(),
        grid_points,
        last_mouse_pos: pt2(0.0, 0.0),
        closest_points: Vec::new(),
    };

    println!("Cell size is {}", lib::CELL_SIZE);

    let input_point = pt2(65.0, -63.0);
    let test_snap = lib::snap_to_grid(input_point, &model);
    println!(
        "Snapping {:?} to the grid produces {:?}",
        input_point, test_snap
    );

    model
}

fn game_of_life(model: &mut lib::Model) {
    let cells = model.cells;
    let rows = cells.rows;

    for i in 0..model.num_cells_x {
        let mut row = rows[i];

        for j in 0..model.num_cells_y {
            // Find neighbours
            let neighbours_indices = lib::get_neighbours_indices(i, j, cells);

            let mut alive_neighbours = Vec::new();
            let mut dead_neighbours = Vec::new();

            for cell_index in neighbours_indices {
                let cell = rows[cell_index.x].values[cell_index.y];
                if cell.is_alive {
                    alive_neighbours.push(cell);
                } else {
                    dead_neighbours.push(cell);
                }
            }

            // Do the game of life..

            // 1. Any live cell with two or three live neighbours survives
            // 2. Any dead cell with three live neighbours becomes a live cell
            // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
            if row.values[j].is_alive {
                match alive_neighbours.len() {
                    2 | 3 => row.values[j] = lib::Cell { is_alive: true },
                    _ => row.values[j] = lib::Cell { is_alive: false },
                }
            } else {
                match alive_neighbours.len() {
                    3 => row.values[j] = lib::Cell { is_alive: true },
                    _ => row.values[j] = lib::Cell { is_alive: false },
                }
            }
        }

        model.cells.rows[i] = lib::CellsRow { values: row.values };
    }
}

fn update(_app: &App, model: &mut lib::Model, _update: Update) {
    // Do the game of life only when needed
    match model.state {
        lib::AppState::Running => game_of_life(model),
        lib::AppState::Init => {}
        _ => return,
    }
}

fn view(app: &App, model: &lib::Model, frame: Frame) {
    let canvas = app.draw();
    canvas.background().color(BLACK);

    match model.state {
        lib::AppState::Init => {
            // Draw the latest snapped point
            for pos in model.current_stroke.iter() {
                canvas
                    .quad()
                    .w(model.cell_size as f32)
                    .h(model.cell_size as f32)
                    .x_y(pos.x, pos.y)
                    .color(rgba(1.0, 1.0, 1.0, 1.0));
            }
        }
        lib::AppState::Running => {
            // Draw the cells
            for (i, cell_row) in model.cells.rows.iter().enumerate() {
                for (j, cell_value) in cell_row.values.iter().enumerate() {
                    lib::draw_cell(i, j, &cell_value.is_alive, model, &canvas);
                }
            }
        }
        _ => {}
    }

    // Draw a grid
    if DRAW_GRID {
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
