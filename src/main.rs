
use tgaimage::{TGAImage, TGAColor};

fn draw_line(ax :usize, ay:usize, bx:usize, by:usize,color : &TGAColor, image :&mut TGAImage)
{
    let steep = ax.abs_diff(bx) < ay.abs_diff(by);
    
    let (ax,ay, bx, by) = if steep 
    {
        (ay,ax,by,bx)
    }
    else 
    {
        (ax,ay,bx,by)    
    };

    let (ax,ay, bx, by) = if ax > bx 
    {
        (bx,by,ax,ay)
    } 
    else
    {
        (ax,ay,bx,by)
    };
    
    // let mut y = ay;
    for x in ax..=bx  
    {
        let t = (x - ax) as f64 / (bx-ax) as f64;
        let y = (ay as f64 + ((by - ay) as f64  * t)) as usize;
        if steep
        {
            image.set(
            y,
            x,
            color);
        }
        else 
        {
            image.set(
            x,
            y,
            color);
        };
        
    }
}


fn main()
{
    let white   : TGAColor = TGAColor::rgb(255,255,255);
    let green   : TGAColor = TGAColor::rgb(0,255,0);
    let red     : TGAColor = TGAColor::rgb(255,0,0);
    let blue    : TGAColor = TGAColor::rgb(0,0,255);

    let width : usize = 256;
    let height : usize = 256;

    let mut image : TGAImage = TGAImage::new(width,height,3);

    let ax : usize = 7;
    let ay : usize = 3;
    let bx : usize = 12;
    let by : usize = 37;
    let cx : usize = 62;
    let cy : usize = 53;

    draw_line(ax, ay, bx, by, &blue, &mut image);
    draw_line(bx, by, cx, cy, &red, &mut image);
    draw_line(ax, ay, cx, cy, &green, &mut image);
    draw_line(cx, cy, bx,by, &white, &mut image);

    image.write_tga_file("framebuffer.tga",false);
}