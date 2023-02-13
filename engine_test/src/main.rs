use rustygame::graphics::window::Window;
use rustygame::graphics::gl_wrapper::*;

use gl::types::*;
use std::mem;
use std::ptr;

fn main() {
    let mut window = Window::new(1080, 720, "rustygame_test");
    window.init_gl();

    let vertices : [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    let vao = rustygame::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    




    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3 , 0.5, 0.3 , 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        window.update();
    }
}

