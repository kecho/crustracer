mod ogl_display;

pub struct DisplayDesc
{
    pub w : u32,
    pub h : u32,
    pub window_title : String
}

pub struct DisplayContext<'a>
{
    pub width : u32,
    pub height : u32,
    pub sdl2 : &'a sdl2::Sdl,
    pub w : &'a sdl2::video::Window,
    pub event_pump : &'a sdl2::EventPump
}

pub trait ImgDisplay
{
    fn get_sdl2(&mut self) -> &sdl2::Sdl;
    fn get_sdl2_window(&self) -> &sdl2::video::Window;
    fn get_event_pump(&self) -> &sdl2::EventPump;

    fn run<DisplayCallbackT>(&mut self, display : &mut DisplayCallbackT)
        where DisplayCallbackT : DisplayCallbacks;
}

pub trait DisplayCallbacks
{
    fn on_event(&mut self, e : &sdl2::event::Event);
    fn on_cpu_render(&mut self, out_img : &mut Vec<u32>, dc : &DisplayContext);
    fn on_gl_render(&mut self, dc : &DisplayContext);
}

pub fn create_ogl_display(desc : DisplayDesc) -> impl ImgDisplay
{
    ogl_display::OglDisplay::new(desc)
}