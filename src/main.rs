use std::{borrow::{Borrow, BorrowMut}, convert::TryInto, fmt, ops::Deref};

use fltk::{
    app, 
    button::Button, 
    frame::Frame, 
    prelude::*, 
    window::Window, 
    enums::{Color, Key, Shortcut},
    group::{Pack, PackType}
};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, buffer::PixelsMut};

//struct RgbTostring(image::Rgb<u8>);
//
//impl std::fmt::Display for RgbTostring{
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "(R:{}, G:{}, B:{})", self.0[0], self.0[1], self.0[2])
//    }
//}
//
//impl Deref for RgbTostring
//{
//    type Target = Rgb<u8>;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}

fn main() {

    let app = app::App::default();
    let mut win = Window::default()
        .with_size(1000, 800);
        
    
    win.end();
    win.show();
        
    let mut img = image::open("test.jpg").unwrap().into_rgb8();
    let w = img.width();
    let h = img.height();

    let mut frm = Frame::new(0,0,w as i32, h as i32, "");

    let raw_img = 
        unsafe {
            let t = img.borrow_mut().as_raw();
            fltk::image::RgbImage::from_data(t, w as i32, h as i32, fltk::enums::ColorDepth::Rgb8)
        }.unwrap();
    
    frm.set_image(Some(raw_img.copy()));
    while app.wait()
    {
        //bubblesort(&mut raw_img.copy(), &img, &mut frm, &mut win);
        
        frm.set_image(Some(raw_img.copy()));
        win.draw_child(&mut frm);
        
        for r in img.rows_mut()
        {
            let mut pixls:Vec<&mut Rgb<u8>> = r.collect();

            let mut i:usize = 0;
            let mut sorted = true;
            loop 
            {
                //println!("pixels {} and {}", to_string(pixls[i]), to_string(pixls[i+1]));
                if i == 0
                {
                    sorted = true;
                    frm.set_image(Some(raw_img.copy()));
                    win.draw_child(&mut frm);
                }
                let p1sum = pixls[i].0[0] as i32 + pixls[i].0[1] as i32 + pixls[i].0[2] as i32;
                let p2sum = pixls[i+1].0[0] as i32 + pixls[i+1].0[1] as i32 + pixls[i+1].0[2] as i32;
                if p1sum > p2sum
                {
                    //println!("swapping pixels {} and {}", to_string(r_vec[i]), to_string(r_vec[i+1]));
                    sorted = false;
                    swap_pixels(&mut pixls, i, i+1);
                    //println!("swapping pixels {} and {}", to_string(r_vec[i]), to_string(r_vec[i+1]));
                }
                else if p1sum == p2sum
                {
                    if pixls[i].0[0] > pixls[i+1].0[0]
                    {
                        //println!("swapping pixels {} and {}", to_string(pixls[i]), to_string(pixls[i+1]));
                        sorted = false;
                        swap_pixels(&mut pixls, i, i+1);
                    }
                }
                i += 1;
                if i == pixls.len() - 1
                {
                    i = 0;
                    if sorted{break;}
                }
            }
            //for p in r_vec
            //{
            //    print!("{} ", p.0[0]);
            //}
            //println!("----------------------------");
            frm.set_image(Some(raw_img.copy()));
            win.draw_child(&mut frm);
        }

        println!("Done");
        /*
        let mut pixls:Vec<&mut Rgb<u8>> = img.pixels_mut().collect();
        let mut i:usize = 0;
        let mut sorted = true;
        loop 
        {
            //println!("pixels {} and {}", to_string(r_vec[i]), to_string(r_vec[i+1]));
            if i == 0
            {
                sorted = true;
                frm.set_image(Some(raw_img.copy()));
                win.draw_child(&mut frm);
            }
            let p1sum = pixls[i].0[0] as i32 + pixls[i].0[1] as i32 + pixls[i].0[2] as i32;
            let p2sum = pixls[i+1].0[0] as i32 + pixls[i+1].0[1] as i32 + pixls[i+1].0[2] as i32;
            if p1sum > p2sum
            {
                //println!("swapping pixels {} and {}", to_string(r_vec[i]), to_string(r_vec[i+1]));
                sorted = false;
                swap_pixels(&mut pixls, i, i+1);
                //println!("swapping pixels {} and {}", to_string(r_vec[i]), to_string(r_vec[i+1]));
                
            }
            else if p1sum == p2sum
            {
                if pixls[i].0[0] > pixls[i+1].0[0]
                {
                    //println!("swapping pixels {} and {}", to_string(pixls[i]), to_string(pixls[i+1]));
                    sorted = false;
                    swap_pixels(&mut pixls, i, i+1);
                      
                }
            }
            i += 1;
            if i == pixls.len() - 1
            {
                i = 0;
                if sorted{break;}
            }
        }
        */
    }
    
    
    app.run().unwrap();
}


fn to_string(p: &Rgb<u8>) -> String {
    format!("(R:{}, G:{}, B:{})", p.0[0], p.0[1], p.0[2])
}

fn swap_pixels(vec: &mut Vec<&mut Rgb<u8>>, i: usize, j: usize)
{
    let temp = *vec[i];
    (*vec[i]).0[0] = vec[j].0[0];
    (*vec[i]).0[1] = vec[j].0[1];
    (*vec[i]).0[2] = vec[j].0[2];
    (*vec[j]).0[0] = temp.0[0];
    (*vec[j]).0[1] = temp.0[1];
    (*vec[j]).0[2] = temp.0[2];
}