mod display;
use crate::display::ImgDisplay;

fn main()
{
    let display_desc = display::DisplayDesc {w: 300, h: 300, window_title: "crustracer".to_owned()};
    let mut display = display::create_ogl_display(display_desc);
    let mut img = Vec::new();
    for x in 0..300
    {
        for y in 0..300
        {
            let is_white = match y & 0x1 
            {
                0 => x & 0x1,
                1 => !(x & 0x1),
                _ => continue
            };

            if is_white == 1
            {
                img.push(0xffffffffu32);
            }
            else
            {
                img.push(0x0u32);
            }
        }
    }

    display.run(|v|{
        *v = img.clone();
    });
}
