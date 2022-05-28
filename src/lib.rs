use nannou::prelude::*;
use std::cmp;

// NB: Don't let it grow too big or you'll get stack overflows at compile time :)
pub const GRID_SIZE: usize = 128;

pub const GRID_LINE_WEIGHT: f32 = 0.3;
pub const CELL_SIZE: usize = 16;

// Data structures
// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct CellIndex {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub is_alive: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct CellsRow {
    pub values: [Cell; GRID_SIZE],
}

#[derive(Debug, Copy, Clone)]
pub struct Cells {
    pub rows: [CellsRow; GRID_SIZE],
}

pub enum AppState {
    Init,
    Running,
    ShouldReset,
}

pub enum DrawingState {
    Started,
    Ended,
    Void,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub weight: f32,
}

pub struct Model {
    pub lines: Vec<Line>,
    pub cells: Cells,
    pub cell_size: usize,
    pub app_width: f32,
    pub app_height: f32,
    pub num_cells_x: usize,
    pub num_cells_y: usize,
    pub generator: rand::rngs::ThreadRng,
    pub state: AppState,
    pub drawing_state: DrawingState,
    pub current_stroke: Vec<Point2>,
    pub grid_points: Vec<Point2>,
    pub last_mouse_pos: Point2,
    pub closest_points: Vec<Point2>,
}

// Functions
// ----------------------------------------------------------------------------
pub fn get_neighbours_indices(x: usize, y: usize, cells: Cells) -> Vec<CellIndex> {
    let mut neighbours = Vec::new();
    let rows = cells.rows;

    // Top neighbours
    if y > 0 {
        let top_index = y - 1;
        let top_row = rows.get(top_index).unwrap();

        for i in -1..2 {
            let index = x as i32 + i;
            let neighbour = top_row.values.get(index as usize);
            match neighbour {
                Some(_x) => neighbours.push(CellIndex {
                    x: index as usize,
                    y: top_index,
                }),
                None => {}
            }
        }
    }

    // Bottom neighbours
    if y < rows.len() - 2 {
        let bottom_index = y + 1;
        let bottom_row = rows.get(bottom_index).unwrap();

        for i in -1..2 {
            let index = x as i32 + i;
            let neighbour = bottom_row.values.get(index as usize);
            match neighbour {
                Some(_x) => neighbours.push(CellIndex {
                    x: index as usize,
                    y: bottom_index,
                }),
                None => {}
            }
        }
    }

    // Left and right neighbours
    let c = rows.get(y);
    match c {
        Some(central_row) => {
            // Left
            let l_index = x - 1;
            let l_neighbour = central_row.values.get(l_index);
            match l_neighbour {
                Some(_l) => neighbours.push(CellIndex {
                    x: l_index as usize,
                    y,
                }),
                None => {}
            }

            // Right
            let r_index = x + 1;
            let r_neighbour = central_row.values.get(x + 1);
            match r_neighbour {
                Some(_r) => neighbours.push(CellIndex {
                    x: r_index as usize,
                    y,
                }),
                None => {}
            }
        }
        None => {}
    }

    neighbours
}

pub fn create_grid(app: &App, step_size: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();

    let window = app.window_rect();
    let width = window.w() as i32;
    let height = window.w() as i32;

    let start_w = -width / 2;
    let end_w = width / 2;

    let start_h = -height / 2;
    let end_h = -start_h;

    // This is stored per-line so that one day this could procedural
    // and different (eg: every N line, make a thicker one..)
    let current_weight = GRID_LINE_WEIGHT;

    // Horizontal lines
    for i in (start_h..end_h).step_by(step_size) {
        let current_y = i as f32;
        //println!("Adding line at {}", current_y);

        let line_props = Line {
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

        let line_props = Line {
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

pub fn draw_cell(x: usize, y: usize, alive: &bool, model: &Model, canvas: &Draw) {
    let size = model.cell_size as f32;

    // Convert from the 0,0 top-left system that I'm used to
    // (because I programmed in Processing/Openframeworks for ages),
    // to the 0,0-is-center system that nannou uses (which is the same
    // that OpenGL uses, afaik)
    let x_scaled = x as f32 * size;
    let y_scaled = y as f32 * size;
    let real_x = x_scaled - (model.app_width * 0.5) + (size * 0.5);
    let real_y = -y_scaled + (model.app_height * 0.5) - (size * 0.5);

    let color = if !alive { BLACK } else { WHITE };

    canvas
        .quad()
        .w(size)
        .h(size)
        .x_y(real_x, real_y)
        .color(color);
}

fn distance(a: &Point2, b: &Point2) -> f32 {
    let dist: f32 = ((b.x - a.x).pow(2.0) + (b.y - a.y).pow(2.0)).sqrt();

    dist
}

pub fn closest_n_points(in_point: Point2, points: &Vec<Point2>, n: usize) -> Vec<Point2> {
    //

    let mut distances = Vec::new();

    //println!("---------------------------------------------------");

    // Find the closest distance between the given point and
    // all of the points in the grid
    for pt in points.iter() {
        let dist = pt.distance(in_point);
        //println!("{:?} and {:?}, distance: {:?}", pt, in_point, dist);

        //let current_smallest = smallest_distances
        //    .iter()
        //    .fold(f32::INFINITY, |a, &b| a.min(b));

        distances.push((dist, pt));
    }

    // Sort by distance
    //distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //distances.sort_by_key(|k| k.0 as i64);
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    while distances.len() > n {
        distances.pop();
    }

    //println!("Closest distances: {:?}", distances);

    let mut closest_points = Vec::new();
    for (_dist, point) in distances {
        closest_points.push(pt2(point.x, point.y));
    }

    assert_eq!(closest_points.len(), n);
    //println!("Closest points: {:?}", closest_points);

    closest_points
}

pub fn snap_to_grid(in_point: Point2, model: &Model) -> Point2 {
    // Given a input point, find the closest point on the grid (by ceiling)

    let closest_points = closest_n_points(in_point, &model.grid_points, 4);

    // TODO: use a map()
    let closest_xa = cmp::min(closest_points[0].x as i32, closest_points[1].x as i32);
    let closest_xb = cmp::min(closest_points[2].x as i32, closest_points[3].x as i32);
    let closest_x = cmp::min(closest_xa, closest_xb);

    let closest_ya = cmp::min(closest_points[0].y as i32, closest_points[1].y as i32);
    let closest_yb = cmp::min(closest_points[2].y as i32, closest_points[3].y as i32);
    let closest_y = cmp::min(closest_ya, closest_yb);

    //let closest_point = pt2(0.0, 0.0);
    let closest_point = pt2(closest_x as f32, closest_y as f32);

    //println!("Closest point: {:?}", closest_point);
    closest_point
}
