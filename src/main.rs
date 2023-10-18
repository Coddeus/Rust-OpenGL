mod winsdl;
use winsdl::Winsdl;
mod objects;
use objects::*;

use std::{time::Instant, f32::consts::PI};

use sdl2::event::{Event, WindowEvent};

fn main() {
    let mut winsdl: Winsdl = Winsdl::new(1000, 1000).unwrap();
    unsafe { gl::Viewport(0, 0, 1000, 1000); }

    // Write shaders
    // Create Program, VBO, IBO, VAO, SSBO and perform operations with them

    let program = create_program().unwrap();
    program.set();

    let (vertices, indices) = triangle_fan(24);
    println!("{:?}\n{:?}", vertices, indices);
    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    let u_time = Uniform::new(program.id, "u_time").unwrap();

    let start = Instant::now();
    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {gl::Viewport(0, 0, width, height);}
                    }
                }
                _ => {  }
            }
        }
        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Uniform1f(u_time.id, start.elapsed().as_secs_f32());
            gl::DrawElements(
                gl::TRIANGLES, 
                indices.len() as i32, 
                gl::UNSIGNED_INT, 
                0 as *const _
            )
        }
        winsdl.window.gl_swap_window();
    }
}

fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices = vec![
        0.0, 0.0,
        0.5, 0.0,
    ];
    let mut indices = vec![];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);

        indices.push(0);
        indices.push(m);
        indices.push(m+1);
    }

    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}