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

    let mut cells = Vec::new();

    let num_cells_x = width as i32 / step_size as i32;
    let num_cells_y = height as i32 / step_size as i32;

    let mut generator = rand::thread_rng();

    for _x in 0..num_cells_x {
        let mut rows = Vec::new();

        for _y in 0..num_cells_y {
            let rand_num = generator.gen_range(0, 2);
            rows.push(rand_num != 0);
        }
        cells.push(rows);
    }

    println!(
        "The first element of the 2D array is {:?}",
        cells.get(0).unwrap().get(0).unwrap()
    );

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
    // TODO: a function to get the neightbours of a cell
    // get_neighbours(x, y)
    for i in 0..model.num_cells_x {
        let rows = &mut model.cells[i];

        for j in 0..model.num_cells_y {
            let rand_num = model.generator.gen_range(0, 2);
            rows[j] = rand_num != 0;
        }

        model.cells[i] = rows.to_vec();
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
    for (i, cell_row) in model.cells.iter().enumerate() {
        for (j, cell_value) in cell_row.iter().enumerate() {
            lib::draw_cell(i, j, cell_value, model, &canvas);
        }
    }

    canvas.background().color(WHITE);
    canvas.to_frame(app, &frame).unwrap();
}
