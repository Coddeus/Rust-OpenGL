mod winsdl;

use std::time::Instant;

use winsdl::Winsdl;
use sdl2::event::Event;

fn main() {
    let mut winsdl: Winsdl = Winsdl::new(800, 600).unwrap();
    let start = Instant::now();
    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {  }
            }
        }
        unsafe {
            gl::ClearColor((start.elapsed().as_secs_f32().sin()+1.0)/2.0, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        winsdl.window.gl_swap_window();
    }
}
