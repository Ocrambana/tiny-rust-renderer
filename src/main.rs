
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
    
    let mut y: f64 = ay as f64;
    let mut ierr : isize = 0;
    for x in ax..=bx  
    {
        if steep
        {
            image.set(
                y as usize,
                x,
                color);
        }
        else 
        {
            image.set(
                x,
                y as usize,
                color);
        };
        ierr = ierr + 2 * by.abs_diff(ay) as isize;
        y = y + ((if by > ay {1isize} else {-1isize}) * ((ierr > (bx - ax) as isize)) as isize) as f64;
        ierr = ierr - 2 * (bx-ax) as isize * ((ierr > (bx - ax) as isize)) as isize;
        
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