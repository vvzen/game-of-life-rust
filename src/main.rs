mod lib;

use nannou::prelude::*;
use rand::Rng;

fn main() {
    //nannou::app(model).update(update).simple_window(view).run();
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> lib::Model {
    // Set up the window size
    app.new_window().size(512, 512).build().unwrap();
    app.main_window().set_resizable(false);

    let step_size = 32;

    let lines = lib::draw_grid(app, step_size);
    println!("Created {} lines", lines.len());

    let window = app.window_rect();
    //let width_i = window.w() as i32;
    //let height_i = window.h() as i32;
    let width = window.w() as f32;
    let height = window.h() as f32;

    let mut rows = Vec::new();

    let num_cells_x = width as i32 / step_size as i32;
    let num_cells_y = height as i32 / step_size as i32;

    let mut generator = rand::thread_rng();

    for _x in 0..num_cells_x {
        let mut values = Vec::new();

        for _y in 0..num_cells_y {
            let rand_num = generator.gen_range(0, 2);
            values.push(lib::Cell {
                is_alive: rand_num != 0,
            });
        }
        let row = lib::CellsRow { values };

        rows.push(row);
    }
    let cells = lib::Cells { rows };

    println!("Canvas size is {}x{}", width, height);
    //println!("The 2D array is {:?}", cells);

    lib::Model {
        lines,
        cells,
        app_width: width,
        app_height: height,
        step_size,
        num_cells_x: num_cells_x as usize,
        num_cells_y: num_cells_y as usize,
        generator,
    }
}

fn update(_app: &App, model: &mut lib::Model, _update: Update) {
    // Do the game of life..

    // 1. Any live cell with two or three live neighbours survives

    // 2. Any dead cell with three live neighbours becomes a live cell

    // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.

    let cells_copy = model.cells.clone();

    for i in 0..model.num_cells_x {
        let rows = &mut model.cells.rows[i];

        for j in 0..model.num_cells_y {
            //let rand_num = model.generator.gen_range(0, 2);
            //rows[j] = rand_num != 0;

            let neighbours_indices = lib::get_neighbours_indices(i, j, cells_copy);

            let mut alive_neighbours = Vec::new();
            let mut dead_neighbours = Vec::new();

            for cell_index in neighbours_indices {
                let cell = model.cells.rows[cell_index.x].values[cell_index.y];
                if cell.is_alive {
                    alive_neighbours.push(cell);
                } else {
                    dead_neighbours.push(cell);
                }
            }
        }

        model.cells.rows[i] = lib::CellsRow {
            values: rows.values.to_vec(),
        };
    }
}

fn view(app: &App, model: &lib::Model, frame: Frame) {
    let canvas = app.draw();

    // Draw the background grid
    for line in model.lines.iter() {
        canvas
            .line()
            .start(pt2(line.start_x, line.start_y))
            .end(pt2(line.end_x, line.end_y))
            .weight(line.weight)
            .color(BLACK);
    }

    // Draw the cells
    for (i, cell_row) in model.cells.rows.iter().enumerate() {
        for (j, cell_value) in cell_row.values.iter().enumerate() {
            lib::draw_cell(i, j, &cell_value.is_alive, model, &canvas);
        }
    }

    canvas.background().color(WHITE);
    canvas.to_frame(app, &frame).unwrap();
}
