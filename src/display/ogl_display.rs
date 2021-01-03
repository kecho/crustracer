extern crate gl;
extern crate sdl2;
use std::ffi::CString;
use crate::display::{ImgDisplay, DisplayDesc, DisplayContext, DisplayCallbacks };

//--------------------------------------
//-------------Ogl Display--------------
//--------------------------------------
pub struct OglDisplay
{
    pub desc : DisplayDesc,
    window_manager : WindowManager,
    window : WindowData,
    ogl_data : OglData,
}

impl ImgDisplay for OglDisplay
{
    fn get_sdl2(&mut self) -> &sdl2::Sdl
    {
        &self.window_manager.sdl
    }
    fn get_sdl2_window(&self) -> &sdl2::video::Window
    {
        &self.window.window_data
    }

    fn get_event_pump(&self) -> &sdl2::EventPump
    {
        &self.window_manager.event_pump
    }

    fn run<DisplayCallbackT>(&mut self, display_cb : &mut DisplayCallbackT)
        where DisplayCallbackT : DisplayCallbacks
    {
        let mut img = vec![0u32; (self.desc.w*self.desc.h) as usize];
        'main_loop: loop
        {
            let window_sz = self.window.window_data.size();
            if window_sz.0 != self.desc.w || window_sz.1 != self.desc.h
            {
                img = vec![0u32; (window_sz.0*window_sz.1) as usize];
                self.ogl_data.texture = OglTexture::new(window_sz.0, window_sz.1);
                self.desc.w = window_sz.0;
                self.desc.h = window_sz.1;
            }

            for e in self.window_manager.event_pump.poll_iter()
            {
                display_cb.on_event(&e);
                match e
                {
                    sdl2::event::Event::Quit {..} => break 'main_loop,
                    _ => {},
                }
            }

            let dc = DisplayContext{ 
                width : self.desc.w, height : self.desc.h,
                sdl2 : &self.window_manager.sdl, w : &self.window.window_data, event_pump : &self.window_manager.event_pump };
            display_cb.on_cpu_render(&mut img, &dc);
            self.render(&img);
            display_cb.on_gl_render(&dc);
            self.window.window_data.gl_swap_window();
        }
    }
}


impl OglDisplay
{
    pub fn new(desc : DisplayDesc) -> OglDisplay
    {
        let window_manager = WindowManager::new();
        let window = window_manager.new_window(&desc);
        let program = create_gl_screen_program();   
        let texture = OglTexture::new(desc.w, desc.h);
        OglDisplay{
            desc : desc,
            window_manager : window_manager,
            window : window,
            ogl_data : OglData { program:program, texture:texture },
        }
    }

    pub fn render(&self, img : &Vec<u32>)
    {
        unsafe 
        {
            self.ogl_data.texture.update(self.desc.w, self.desc.h, 0, 0, img);
            gl::Viewport(0, 0, self.desc.w as i32, self.desc.h as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ActiveTexture(gl::TEXTURE0);

            gl::UseProgram(self.ogl_data.program.program);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.ogl_data.texture.tex_id);
            gl::Uniform1i(self.ogl_data.program.tex_uniform, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

//--------------------------------------
//-------------WindowData---------------
//--------------------------------------
struct WindowManager
{
    sdl : sdl2::Sdl,
    event_pump : sdl2::EventPump
}

struct WindowData
{
    window_data : sdl2::video::Window,
    _gl_context : sdl2::video::GLContext,
}

impl WindowManager
{
    fn new() -> WindowManager
    {
        let sdl2 = sdl2::init().unwrap();
        let event_pump = sdl2.event_pump().unwrap();
        WindowManager { sdl : sdl2,  event_pump : event_pump }
    }

    fn new_window(&self, display_desc : &DisplayDesc) -> WindowData
    {
        let video_subsystem = self.sdl.video().unwrap();
        let window = video_subsystem.window(&display_desc.window_title, display_desc.w, display_desc.h).opengl().resizable().build().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        WindowData { window_data : window, _gl_context : gl_context }
    }
}

//--------------------------------------
//-------------Ogl Private Data---------
//--------------------------------------
struct OglData
{
    program : OglProgram,
    texture : OglTexture
}
//--------------------------------------

//--------------------------------------
//-------------Ogl Texture--------------
//--------------------------------------
struct OglTexture
{
    tex_id : gl::types::GLuint
}

impl OglTexture
{
    fn new(w:u32, h:u32) -> OglTexture
    {
        let mut tex_id : gl::types::GLuint = 0;
        unsafe 
        {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        OglTexture { tex_id : tex_id }
    }

    fn update(&self, w:u32, h:u32, offset_x:u32, offset_y:u32, img : &Vec<u32>)
    {
        unsafe
        {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::TexSubImage2D(gl::TEXTURE_2D, offset_x as i32, offset_y as i32, 0i32, w as i32, h as i32, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const std::ffi::c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
impl Drop for OglTexture
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::DeleteTextures(1i32, &self.tex_id);
        }
    }
}

//--------------------------------------
//-------------OglProgram---------------
//--------------------------------------
struct OglProgram
{
    vertex_shader: gl::types::GLuint,
    pixel_shader: gl::types::GLuint,
    tex_uniform: gl::types::GLint,
    program : gl::types::GLuint,
}

fn create_gl_program(vertex_src : &str, pixel_src : &str, texture_uniform : &str) -> OglProgram
{
    let vid = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let pid = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let ppid = unsafe { gl::CreateProgram() };
    let cvertex_src = CString::new(vertex_src).unwrap();
    let cpixel_src = CString::new(pixel_src).unwrap();
    unsafe
    {
        gl::ShaderSource(vid, 1, &cvertex_src.as_ptr(), std::ptr::null());
        gl::CompileShader(vid);
        gl::ShaderSource(pid, 1, &cpixel_src.as_ptr(), std::ptr::null());
        gl::CompileShader(pid);
        gl::AttachShader(ppid, vid);
        gl::AttachShader(ppid, pid);
        gl::LinkProgram(ppid);
        gl::DetachShader(vid, vid);
        gl::DetachShader(pid, pid);
    }

    let ctexture_uniform = CString::new(texture_uniform).unwrap();
    let tex_uniform_id = unsafe { gl::GetUniformLocation(ppid, ctexture_uniform.as_ptr()) };
    return OglProgram {vertex_shader:vid, pixel_shader:pid, program:ppid, tex_uniform: tex_uniform_id };
}

fn create_gl_screen_program() -> OglProgram
{
    let vs_screen_src = "
        #version 330 core
        out vec2 pixel_uvs;
        void main()
        {
            float xV = -1.0 + float((gl_VertexID & 1) << 2);
            float yV = -1.0 + float((gl_VertexID & 2) << 1);
            vec2 p = vec2(xV, yV);
            gl_Position = vec4(p, 0.0, 1.0);
            pixel_uvs = p * 0.5 + 0.5;
        }
        ";

    let ps_screen_src = "
        #version 330 core
        uniform sampler2D u_frameTexture;
        in vec2 pixel_uvs;
        out vec4 Color;
        void main()
        {
            Color = vec4(texture(u_frameTexture, pixel_uvs).rgb, 1.0f);
        }
    ";

    create_gl_program(vs_screen_src, ps_screen_src, "u_frameTexture")
}

impl Drop for OglProgram
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.pixel_shader);
            gl::DeleteShader(self.program);
        }
    }
}

