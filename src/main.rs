mod display;
mod camera;
use crate::display::{ ImgDisplay, DisplayContext, DisplayCallbacks };

extern crate imgui;
extern crate imgui_sdl2;
extern crate imgui_opengl_renderer;


struct Displayer
{
    pub imgui_sdl2 : imgui_sdl2::ImguiSdl2,
    pub imgui : imgui::Context,
    pub imgui_renderer : imgui_opengl_renderer::Renderer,
}

impl DisplayCallbacks for Displayer
{
    fn on_event(&mut self, e : &sdl2::event::Event)
    {
        self.imgui_sdl2.handle_event(&mut self.imgui, e);
    }

    fn on_cpu_render(&mut self, out_img : &mut Vec<u32>, dc : &DisplayContext)
    {
        for y in 0..dc.height
        {
            for x in 0..dc.width
            {
                let index = (y * dc.width + x) as usize;
                let is_white = match y & 0x1 
                {
                    0 => x & 0x1,
                    1 => !(x & 0x1),
                    _ => continue
                };

                if is_white == 1
                {
                    out_img[index] = 0xffffffffu32;
                }
                else
                {
                    out_img[index] = 0x0u32;
                }
            }
        }
    }

    fn on_gl_render(&mut self, dc : &DisplayContext)
    {
        self.imgui_sdl2.prepare_frame(self.imgui.io_mut(), &dc.w, &dc.event_pump.mouse_state());
        let ui = self.imgui.frame();
        ui.show_demo_window(&mut true);
        self.imgui_sdl2.prepare_render(&ui, &dc.w);
        self.imgui_renderer.render(ui);
    }
}

fn main()
{
    let display_desc = display::DisplayDesc {w: 1500, h: 400, window_title: "crustracer".to_owned()};
    let mut display = display::create_ogl_display(display_desc);
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| display.get_sdl2().video().unwrap().gl_get_proc_address(s) as _);
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, display.get_sdl2_window());
    let mut d = Displayer { imgui_sdl2 : imgui_sdl2, imgui : imgui, imgui_renderer : renderer };
    display.run(&mut d);
}
