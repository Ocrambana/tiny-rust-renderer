use tgaimage::{TGAImage, TGAColor};


fn main()
{
    let white : TGAColor = TGAColor::rgb(255,255,255);
    let width : usize = 64;
    let height : usize = 64;

    let mut image : TGAImage = TGAImage::new(width,height,3);

    let ax : usize = 7;
    let ay : usize = 3;
    let bx : usize = 12;
    let by : usize = 37;
    let cx : usize = 62;
    let cy : usize = 53;

    image.set(ax,ay,&white);
    image.set(bx,by,&white);
    image.set(cx,cy,&white);

    image.write_tga_file("framebuffer.tga",false);
}