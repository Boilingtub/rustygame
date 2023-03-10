use rustygame::graphics::window::Window;
use rustygame::graphics::gl_wrapper::*;

use gl::types::*;
use std::mem;
use std::ptr;

fn main() {
    let mut window = Window::new(1080, 720, "rustygame_test");
    window.init_gl();

    let vertices : [f32; 15] = [
         0.5,  0.5, 0.0,
         0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5,  0.5, 0.0,
         1.0,  1.0, 0.0,
    ];

    let indices = [
        0, 1, 3,
        1, 2, 3,
        1, 4, 0
    ];

    let vao = rustygame::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);
    
    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();

    ibo.store_i32_data(&indices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );


    let index_attribute = VertexAttribute::new(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
     
    position_attribute.enable();
    index_attribute.enable();

    
    let shader_program = ShaderProgram::new("../assets/shaders/basic.vert" , "../assets/shaders/basic.frag");
    shader_program.bind();
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0 , 0.0, 0.0 , 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT , ptr::null());
        }
        window.update();
    }
}

