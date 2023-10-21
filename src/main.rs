mod winsdl;
use winsdl::Winsdl;
mod objects;
use objects::*;

use std::f32::consts::PI;

use sdl2::event::Event;

fn main() {
    let mut winsdl: Winsdl = Winsdl::new(1000, 1000).unwrap();
    unsafe { gl::Viewport(0, 0, 1000, 1000); }

    
    let program = create_program().unwrap();
    program.set();

    let (vertices, indices) = triangle_fan(100);
    println!("{:?}\n{:?}", vertices, indices);
    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {  }
            }
        }
        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

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