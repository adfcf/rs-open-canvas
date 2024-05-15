use std::process::exit;
use glfw::{Action, Context, CursorMode, Glfw, Key, MouseButton, PWindow, SwapInterval};

use super::canvas::Canvas;
use super::gl;

static mut GLOBAL_GLFW_INITIALIZED: bool = false;

pub const ONE_SECOND: f64 = 1.0;

pub struct Config {
    title: String,
    aspect_ratio: f32,
    size: u32,
    resolution_scale: f32,
    swap_interval: SwapInterval,
}
pub struct Window {
    lib: Glfw,
    handle: PWindow,
    canvas_dimension: (u32, u32),
}

impl Config {
    pub fn new(title: String, aspect_ratio: f32, size: u32, resolution_scale: f32, swap_interval: SwapInterval) -> Self {
        Config {
            title,
            aspect_ratio,
            size,
            resolution_scale,
            swap_interval,
        }
    }
    pub fn default() -> Self {
        Config {
            title: String::from("Canvas"),
            aspect_ratio: 4.0 / 3.0,
            size: 300,
            resolution_scale: 1.0,
            swap_interval: SwapInterval::Sync(1),
        }
    }
}

impl Window {

    pub fn new(config: Config) -> Window {

        let height = config.size;
        let width = ((config.size as f32) * config.aspect_ratio) as u32;

        unsafe {
            if GLOBAL_GLFW_INITIALIZED {
                panic!("Only a single GLFW window can be created per program.");
            }
        }

        let glfw = glfw::init(glfw_callback);
        if let Err(_) = glfw {
            eprintln!("Error while initializing GLFW.");
            exit(0);
        }
        let mut glfw = glfw.unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let kit = glfw.with_primary_monitor(|this, m| {
            if m.is_none() {
                eprint!("Could not find a primary monitor.");
                exit(0);
            }
            let _vm = m.unwrap().get_video_mode().unwrap();
            this.create_window(
                width, height, 
                config.title.as_str(), 
                glfw::WindowMode::Windowed
            )
        });

        if let None = kit {
            eprintln!("Could create GLFW window.");
            exit(0);
        }
        let mut kit = kit.unwrap();

        // Loading OpenGL
        kit.0.make_current();
        gl::load_with(|h| kit.0.get_proc_address(h) as *const _);

        glfw.set_swap_interval(config.swap_interval);
        kit.0.set_key_callback(|this, key, _, _, _| { if key == Key::Escape { this.set_should_close(true); } });

        let canvas_dimension = (
            (width as f32 * config.resolution_scale) as u32, 
            (height as f32 * config.resolution_scale) as u32
        );
        // Creating OpenGL objects
        unsafe { 
            ogli::create_program();
            ogli::create_quad();
            ogli::create_tex(canvas_dimension.0, canvas_dimension.1);
            if let Some(e) = ogli::any_error() {
                println!("GL Error: {}", e);
            }
            GLOBAL_GLFW_INITIALIZED = true;
        }

        Window {
            lib: glfw,
            handle: kit.0,
            canvas_dimension,
        }

    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn poll_events(&mut self) {
        self.lib.poll_events();
    }

    pub fn swap_buffers(&mut self, canvas: &Canvas) {
        unsafe { 
            ogli::load_tex(&canvas); 
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            if let Some(e) = ogli::any_error() {
                eprintln!("GL Error: {}", e);
            }
        }
        self.handle.swap_buffers();
    }

    pub fn now(&self) -> f64 {
        self.lib.get_time()
    }

    pub fn get_canvas_dimension(&self) -> (u32, u32) {
        self.canvas_dimension
    }

    pub fn toggle_cursor_mode(&mut self) {
        let current_mode = self.handle.get_cursor_mode();
        self.handle.set_cursor_mode(if current_mode == CursorMode::Normal { CursorMode::Disabled } else { CursorMode::Normal });
    }   
    pub fn get_key_action(&self, key: Key) -> Action {
        self.handle.get_key(key)
    }
    pub fn get_mouse_button_action(&self, button: MouseButton) -> Action {
        self.handle.get_mouse_button(button)
    }
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        self.handle.get_cursor_pos()
    }

    pub fn get_dimension(&self) -> (u32, u32) {
        let dimension = self.handle.get_framebuffer_size();
        (dimension.0 as u32, dimension.1 as u32)
    }

}

fn glfw_callback(error: glfw::Error, _: String) {
    println!("An error has occurred: {}", error);
    exit(0);
}

// OpenGL Interface
mod ogli {

    extern crate alloc;

    use crate::canvas::Canvas;

    use super::gl;

    pub unsafe fn create_quad() {

        let data: Vec<f32> = vec![ -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0 ];
    
        let mut vao_handle = 0;
        gl::GenVertexArrays(1, &mut vao_handle);
        gl::BindVertexArray(vao_handle);
    
        let mut vbo_handle = 0;
        gl::GenBuffers(1, &mut vbo_handle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_handle);
        
        gl::BufferData(gl::ARRAY_BUFFER, 8 * 4, data.as_ptr().cast(), gl::STATIC_DRAW);
    
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8, 0 as *const _);
        gl::EnableVertexAttribArray(0);
    
    }
    
    pub unsafe fn create_program() {
    
        let vss = r#"
            #version 330 core
    
            layout (location = 0) in vec2 position;
    
            out vec2 tex_mapping;
    
            void main() {
                gl_Position = vec4(position.xy, 0.0, 1.0);
                vec2 mapping = vec2(min(position.x + 1.0, 1.0), min(position.y + 1.0, 1.0));
                tex_mapping = vec2(1.0 - mapping.x, mapping.y);
            }
    
        "#;
        let fss = r#"
            #version 330 core
    
            in vec2 tex_mapping;
            out vec4 fragment_color;
    
            uniform sampler2D tex2d;
    
            void main() {
                fragment_color = texture(tex2d, tex_mapping);
            }
        "#;
    
        let vs_handle = gl::CreateShader(gl::VERTEX_SHADER);
        let fs_handle = gl::CreateShader(gl::FRAGMENT_SHADER);
    
        gl::ShaderSource(vs_handle, 1, &(vss.as_bytes().as_ptr().cast()), &(vss.len().try_into().unwrap()));
        gl::CompileShader(vs_handle);

        let mut success = 0;
        gl::GetShaderiv(vs_handle, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
              vs_handle,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            println!("=================> error vs: {}", String::from_utf8_lossy(&v));
        }

        gl::ShaderSource(fs_handle, 1, &(fss.as_bytes().as_ptr().cast()), &(fss.len().try_into().unwrap()));
        gl::CompileShader(fs_handle);
        
        let mut success = 0;
        gl::GetShaderiv(fs_handle, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
              vs_handle,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            println!("=================> error fs: {}", String::from_utf8_lossy(&v));
        }

        let program_handle = gl::CreateProgram();
        
        gl::AttachShader(program_handle, vs_handle);
        gl::AttachShader(program_handle, fs_handle);
        gl::LinkProgram(program_handle);
    
        gl::UseProgram(program_handle);
    
        

    }

    pub unsafe fn create_tex(width: u32, height: u32) -> u32 {

        let mut tex_handle = 0;
        gl::GenTextures(1, &mut tex_handle);
        gl::BindTexture(gl::TEXTURE_2D, tex_handle);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0 as *const _);

        gl::GenerateMipmap(gl::TEXTURE_2D);

        tex_handle

    }

    pub unsafe fn load_tex(canvas: &Canvas) {
        gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, canvas.get_width() as i32, canvas.get_height() as i32, gl::RGBA, gl::UNSIGNED_BYTE, canvas.raw().cast());
    }

    pub unsafe fn any_error() -> Option<u32> {
        let e = gl::GetError();
        if e == gl::NO_ERROR { None } else { Some(e) }
    }

}