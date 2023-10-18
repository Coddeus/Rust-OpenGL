use std::{ffi::{CStr, CString}, ptr::null};

use gl::types::{GLuint, GLint, GLchar, GLenum};

/// An OpenGL Shader of the Graphics pipeline
pub struct Shader {
    id: GLuint,
}

impl Shader {
    /// Creates a shader out of a file path and a type
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }
    
        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
    
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
    
            let error = create_whitespace_cstring_with_len(len as usize);
    
            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            
            return Err(error.to_string_lossy().into_owned());
        }
    
        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/// An OpenGL program, to which shaders are attached
pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn set(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

/// Creates the shader program for Pianorium
pub fn create_program() -> Result<Program, &'static str> {
    let vert_shader =
        Shader::from_source(&CString::new(include_str!(".vert")).unwrap(), gl::VERTEX_SHADER).unwrap();

    let tess_ctrl_shader =
        Shader::from_source(&CString::new(include_str!(".tess_ctrl")).unwrap(), gl::TESS_CONTROL_SHADER).unwrap();

    let tess_eval_shader =
        Shader::from_source(&CString::new(include_str!(".tess_eval")).unwrap(), gl::TESS_EVALUATION_SHADER).unwrap();

    let geom_shader =
        Shader::from_source(&CString::new(include_str!(".geom")).unwrap(), gl::GEOMETRY_SHADER).unwrap();

    let frag_shader =
        Shader::from_source(&CString::new(include_str!(".frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, /*tess_ctrl_shader, tess_eval_shader,*/ geom_shader, frag_shader]).unwrap();

    Ok(shader_program)
}

/// OpenGL Vertex Buffer Object.
pub struct Vbo {
    pub id: GLuint,
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Vbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// OpenGL Index Buffer Object.
pub struct Ibo {
    pub id: GLuint,
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Ibo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ibo { id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// OpenGL Vertex Array Object.
pub struct Vao {
    pub id: GLuint,
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vao {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

/// OpenGL Shader Storage Buffer Object
pub struct Ssbo {
    pub id: GLuint,
}

impl Drop for Ssbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}
impl Ssbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ssbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// An OpenGL uniform, used by shaders for customization
pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: u32, name: &str) -> Result<Self, &'static str> {
        let cname: CString = CString::new(name).expect("CString::new failed");
        let location: GLint = unsafe { gl::GetUniformLocation(program, cname.as_ptr()) };
        if location==-1 { return Err("Couldn't get uniform location"); }
        Ok(Uniform { id: location })
    }
}