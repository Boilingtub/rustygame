use rustygame::graphics::window::*;
use rustygame::graphics::gl_wrapper::*;
use rustygame::graphics::primitives::*;

use gl::types::*;
use std::ptr;
use std::mem;

fn main() {
    let mut window = Window::new(1080, 720, "rustygame_test");
    window.init_gl();

    let basic_shader = ShaderProgram::new("../assets/shaders/basic.vert" , "../assets/shaders/basic.frag");

   // let triangle = Triangle::new("triangle1".to_string() ,basic_shader);

       let verticies = [
            Vertex {position : [0.0,0.5,0.0] , color : [1.0 , 0.0 , 0.0 , 1.0]},
            Vertex {position : [-0.5,-0.5,0.0] , color : [0.0 ,1.0 ,0.0 ,1.0]},
            Vertex {position : [0.5,-0.5,0.0] , color : [0.0 ,0.0 ,1.0, 1.0]},
        ];
        let indicies = [
            0,1,2,
            //0, 1, 3,
            //1, 2, 3,
            //1, 4, 0
        ];

        println!("{:?}",verticies_color_data(verticies.to_vec()));

        let vao = Vao::new();
        vao.bind();
 
        let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER , gl::STATIC_DRAW);
        ibo.bind();
        ibo.store_i32_data(&indicies);
        
        let index_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat> as GLsizei,
            ptr::null(),
        );
        index_attribute.enable();
        

        let vbo = BufferObject::new(gl::ARRAY_BUFFER , gl::STATIC_DRAW); 
        vbo.bind();
        vbo.store_f32_data(&verticies_position_data(verticies.to_vec()));
        
        let position_attribute = VertexAttribute::new(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null()
        );
        position_attribute.enable();
        

        let cbo = BufferObject::new(gl::ARRAY_BUFFER , gl::STATIC_DRAW);
        cbo.bind();
        cbo.store_f32_data(&verticies_color_data(verticies.to_vec()));

        let color_attribute = VertexAttribute::new(
            2,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * mem::size_of::<GLfloat> as GLsizei,
            ptr::null()
        );
        color_attribute.enable();
        
        ibo.unbind();
        vbo.unbind();
        cbo.unbind();

        vao.unbind();

        unsafe {
            gl::ClearColor(0.0 , 0.0, 0.0 , 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
}

