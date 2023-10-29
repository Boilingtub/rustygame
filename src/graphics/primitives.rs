use crate::graphics::gl_wrapper::*;
use std::mem;
use std::ptr;
use gl::types::*;


struct Buffers {
    ibo : BufferObject,
    vbo : BufferObject,
    cbo : BufferObject,
}



struct Attributes{
    index_attribute : VertexAttribute,
    position_attribute : VertexAttribute,
    color_attribute : VertexAttribute,
}
impl Attributes{ 
    fn enable_all_attributes(&self) {
        self.position_attribute.enable();
        self.color_attribute.enable();
        self.index_attribute.enable();
    }
    
    fn disable_all_attributes(&self) {
        self.position_attribute.disable();
        self.color_attribute.disable();
        self.index_attribute.disable();
    }
}


#[derive(Clone)]
pub struct Vertex{
    pub position : [f32; 3],
    pub color : [f32; 4],
}
pub fn verticies_position_data(verticies : Vec<Vertex>) -> Vec<f32> {
    let mut position_data : Vec<f32> = vec![];
    for vertex in verticies {
        position_data.append(&mut vertex.position.to_vec());
    }
    
    return position_data
}
pub fn verticies_color_data(verticies : Vec<Vertex>) -> Vec<f32> {
    let mut color_data : Vec<f32> = vec![];
    for vertex in verticies {
        color_data.append(&mut vertex.color.to_vec());
    }

    return color_data
}



pub struct Triangle{
    name : String,
    shader_program : ShaderProgram, 
    vao : Vao,
    verticies : Vec<Vertex>,
    indicies : Vec<i32>,
    buffers : Buffers,
    attributes : Attributes,
}
impl Triangle{
    pub fn new(name : String, shader_program : ShaderProgram) -> Triangle {
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

        Triangle{
            name, 
            shader_program, 
            vao,
            verticies : verticies.to_vec(),
            indicies : indicies.to_vec(),
            buffers : Buffers{ibo , vbo , cbo},
            attributes : Attributes{index_attribute, position_attribute, color_attribute},
        }
    }

    pub fn render(&self) {
        self.shader_program.bind();
        self.vao.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indicies.len() as i32 ,gl::UNSIGNED_INT, ptr::null());
        }
    }
}







