use macroquad::prelude::*;
use ndarray::{Array1, arr2, arr1};


#[macroquad::main(window_conf)]
async fn main() {
    // Cache window settings for offsetting later
    let config = window_conf();

    // Distance from center in pixels
    let dist = 100.;

    let mut vertices = vec![
        Vertex {
            start_position: arr1(&[-1., 1., 1.]),
            position: arr1(&[-1., 1., 1.]),
        },
        Vertex {
            start_position: arr1(&[1., 1., 1.]),
            position: arr1(&[1., 1., 1.]),
        },
        Vertex {
            start_position: arr1(&[1., -1., 1.]),
            position: arr1(&[1., -1., 1.]),
        },
        Vertex {
            start_position: arr1(&[-1., -1., 1.]),
            position: arr1(&[-1., -1., 1.]),
        },
        Vertex {
            start_position: arr1(&[-1., 1., -1.]),
            position: arr1(&[-1., 1., -1.]),
        },
        Vertex {
            start_position: arr1(&[1., 1., -1.]),
            position: arr1(&[1., 1., -1.]),
        },
        Vertex {
            start_position: arr1(&[1., -1., -1.]),
            position: arr1(&[1., -1., -1.]),
        },
        Vertex {
            start_position: arr1(&[-1., -1., -1.]),
            position: arr1(&[-1., -1., -1.]),
        }
    ];

    let mut mouse_pos = mouse_position();

    // App loop
    loop {
        clear_background(BLACK);

        let rotation_speed = 0.01;

        let mut angle_x: f32 = 0.;
        let mut angle_y: f32 = 0.;

        let cur_mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let delta = (mouse_pos.0 - cur_mouse_pos.0, cur_mouse_pos.1 - mouse_pos.1);
            angle_x = delta.1 * rotation_speed * 0.25;
            angle_y = delta.0 * rotation_speed * 0.25;
        }
        mouse_pos = cur_mouse_pos;

        if is_key_down(KeyCode::Down) { angle_x = rotation_speed }
        if is_key_down(KeyCode::Up) { angle_x = -rotation_speed }
        if is_key_down(KeyCode::Right) { angle_y = rotation_speed }
        if is_key_down(KeyCode::Left) { angle_y = -rotation_speed }

        // Define rotation matrices
        let x_rot = arr2(&[
            [1., 0., 0.],
            [0., angle_x.cos(), -angle_x.sin()],
            [0., angle_x.sin(), angle_x.cos()]
        ]);
        let y_rot = arr2(&[
            [(angle_y * 2.).cos(), 0., (angle_y * 2.).sin()],
            [0., 1., 0.],
            [-(angle_y * 2.).sin(), 0., (angle_y * 2.).cos()]
        ]);

        // Iterate through each vertex twice to handle rotation and rendering
        let temp_verts = vertices.to_vec();
        for i in 0..temp_verts.len() {
            let mut vert = vertices[i].to_owned();

            // Matrix Multiplication for rotation
            // math, ew 🤢
            // jk, i love math
            vertices[i].position = vert.position.dot(&x_rot).dot(&y_rot);

            vert = vertices[i].to_owned();

            for j in 0..temp_verts.len() {
                if i as i32 == j as i32 { continue; }

                let other_vert = vertices[j].to_owned();

                // Fun way to determine whether or not to draw connecting line
                // Less concise and optimized but more enjoyable
                let mut diffs = 0;
                if vert.start_position[0] != other_vert.start_position[0] {
                    diffs += 1;
                }
                if vert.start_position[1] != other_vert.start_position[1] {
                    diffs += 1;
                }
                if vert.start_position[2] != other_vert.start_position[2] {
                    diffs += 1;
                }

                if diffs > 1 { continue }

                draw_line(
                    (vert.position[0] * ((vert.position[2] + 5.) * 0.25)) * dist
                        + (config.window_width / 2) as f32,
                    (vert.position[1] * ((vert.position[2] + 5.) * 0.25)) * dist
                        + (config.window_height / 2) as f32,
                    (other_vert.position[0] * ((other_vert.position[2] + 5.) * 0.25)) * dist
                        + (config.window_width / 2) as f32,
                    (other_vert.position[1] * ((other_vert.position[2] + 5.) * 0.25)) * dist
                        + (config.window_height / 2) as f32,
                    1.,
                    WHITE
                );
            }
        }

        for i in 0..vertices.len() {
            let vert = vertices[i].to_owned();

            draw_poly(
                (vert.position[0] * ((vert.position[2] + 5.) * 0.25)) * dist
                    + (config.window_width / 2) as f32,
                (vert.position[1] * ((vert.position[2] + 5.) * 0.25)) * dist
                    + (config.window_height / 2) as f32,
                10,
                5.,
                0.,
                RED
            );
        }

        next_frame().await;
    }
}


#[derive(Clone)]
struct Vertex {
    start_position: Array1<f32>,
    position: Array1<f32>
}

fn window_conf() -> Conf {
    Conf {
        window_width: 600,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
