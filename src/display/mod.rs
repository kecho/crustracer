mod ogl_display;

pub struct DisplayDesc
{
    pub w : u32,
    pub h : u32,
    pub window_title : String
}

pub trait ImgDisplay
{
    fn run<DrawT>(&mut self, draw_img_cb : DrawT) where
        DrawT : FnMut(&mut Vec<u32>);
}

pub fn create_ogl_display(desc : DisplayDesc) -> impl ImgDisplay
{
    ogl_display::OglDisplay::new(desc)
}