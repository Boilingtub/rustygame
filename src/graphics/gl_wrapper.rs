use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::mem;

use std::io::Read;

use std::os::raw::*;
use std::ptr;

use gl::types::*;

use cgmath::*;

pub struct Vertex {
    x : f32,
    y : f32,
    z : f32,
}

pub struct Vao {
    id: gl::types::GLuint,
}
impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);   
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct BufferObject {
    id : gl::types::GLuint,
    r#type : gl::types::GLenum,
    usage: gl::types::GLenum,
}
impl BufferObject {
    pub fn new(r#type : gl::types::GLenum, usage : gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1 , &mut id);
        }
        BufferObject { id , r#type, usage }
    }
    
    pub fn bind(&self) {
        unsafe{
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe{
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self , data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            );
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            );
        }
    }
    
}

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(index: u32 , size: i32, r#type: GLenum , normalized: GLboolean,
               stride: GLsizei, pointer: *const c_void) -> VertexAttribute {
    
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct ShaderProgram {
    program_handel: u32,
    uniform_ids: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
    pub fn new(vertex_shader_path: &str , fragment_shader_path: &str) -> ShaderProgram {
        unsafe { 
            let vertex_shader = ShaderProgram::compile_shader(vertex_shader_path, gl::VERTEX_SHADER);
            let fragment_shader = ShaderProgram::compile_shader(fragment_shader_path, gl::FRAGMENT_SHADER);

            let program_handel = gl::CreateProgram();
            gl::AttachShader(program_handel, vertex_shader);
            gl::AttachShader(program_handel, fragment_shader);
            gl::LinkProgram(program_handel);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::ValidateProgram(program_handel);

            ShaderProgram {
                program_handel,
                uniform_ids: HashMap::new(),
            }
        }
    }

    pub fn compile_shader(shader_path : &str , shader_type : GLenum) -> GLuint {
        let mut shader_file = File::open(shader_path)
                .unwrap_or_else(|_| panic!("Failed to open {}", shader_path));
    
        let mut shader_source = String::new();
    
        shader_file.read_to_string(&mut shader_source)
                          .expect("Failed to read vertex shader");
    
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str = CString::new(shader_source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
            
            let mut result : i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
            if result == gl::FALSE.into() {
                let mut error_length : i32 = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_length);
                let mut error_message : i8 = 0;
                gl::GetShaderInfoLog(shader, error_length, &mut error_length, &mut error_message);

                println!("OpenGL : Shader Compilation Error !");
                println!("Error in : {}",shader_path);
                println!("{}",error_message);

                gl::DeleteShader(shader);
                return 0;

            }

            return shader;
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handel);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handel,
                CString::new(uniform_name).unwrap().as_ptr(),
            )
        };

        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }
}












