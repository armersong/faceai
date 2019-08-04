#![allow(unused_must_use)]

extern crate faceai;
extern crate image;
extern crate env_logger;

use std::env;

const APP_ID:&str = "archive from https://ai.arcsoft.com.cn/product/arcface.html";
const APP_SDK_KEY:&str = "";

fn get_image_info(image_fn:&str) ->(u32, u32) {
    let mut img = image::open(image_fn).unwrap();
    let rgb = img.as_mut_rgb8().unwrap();
    (rgb.width(), rgb.height())
}

fn main() {
    env_logger::init();
    let mut args = env::args();
    if args.len() < 2 {
        println!("Usage {} <jpg picture> [<jpg picture>...] ", args.nth(0).unwrap());
        return;
    }
    let ai = faceai::FaceAiBuilder::new().app_id(APP_ID).app_secret(APP_SDK_KEY).build().unwrap();
    for (index, filename) in env::args().enumerate() {
        if index == 0  {
            continue;
        }
        // let filename = args.nth(i).unwrap();
        let mut img = image::open(filename.as_str()).unwrap();
        let (w, h) = get_image_info(filename.as_str()); 
        if w % 4 != 0 || w%4 != 0 {
            img = img.resize(w/4*4, h/4*4, image::imageops::FilterType::Nearest);
        }
        let img_rgb = img.as_mut_rgb8().unwrap();
        let data = (*img_rgb).as_mut_ptr();
        println!("width {} height {}", img_rgb.width(), img_rgb.height());
        ai.detect_rgb8(img_rgb.width(), img_rgb.height(), data).map(|faces| {
            let red = image::Rgb([255,0,0]);
            for face in faces.iter() {
                let rect = &face.position;
                //draw rectange
                for x in rect.left..rect.right+1 {
                    img_rgb.put_pixel(x as u32, rect.top as u32, red);
                }                
                for x in rect.left..rect.right+1 {
                    img_rgb.put_pixel(x as u32, rect.bottom as u32, red);
                }                
                for y in rect.top..rect.bottom+1 {
                    img_rgb.put_pixel(rect.left as u32, y as u32, red);
                }                
                for y in rect.top..rect.bottom+1 {
                    img_rgb.put_pixel(rect.right as u32, y as u32, red);
                }                
                ai.extract_feature_rgb8(img_rgb.width(), img_rgb.height(), data, face).map(|face_feature|{
                    println!("feature: {}", faceai::feature_to_string(face_feature.feature));
                });
            }
            image::save_buffer(format!("{}_face.jpg", filename), &*img_rgb, img_rgb.width(), img_rgb.height(),  image::ColorType::RGB(8));
        }).map_err(|err|{
            println!("face detect failed {}",err);
        });

    }
}
