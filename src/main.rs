mod keab;
use image;
use turbojpeg;
use std::{path::Path, default};
use glob::{glob_with, MatchOptions};
use keab::generic;
use clap::Parser;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct CliArguments {
    #[arg(long)]
    folder: String,

    #[arg(long)]
    quality: i32,

    #[arg(long)]
    subsamp: String 
}

fn main() {
    let args = CliArguments::parse();
    
    // find all jpeg jpg and png's courtesy of  
    // https://programming-idioms.org/idiom/177/find-files-for-a-list-of-filename-extensions/6352/rust
    let fuckshit: Vec<PathBuf> = fs::read_dir(args.folder) 
    .unwrap()
    .filter_map(|f| f.ok())
    .filter(|f| match f.path().extension() {
        None => false,
        Some(ex) => ex == "jpeg" || ex == "jpg" || ex == "png"
   })
   .map(|f| f.path())
   .collect();

   let subsampling = get_subsamp(&args.subsamp);

   let t1 = std::time::Instant::now();
   fuckshit.par_iter().for_each(|path| {
    keab_image(path.to_path_buf(), args.quality, subsampling)
   });
   println!("conversion done!: time {:?}", t1.elapsed());

}

fn keab_image(image_path: PathBuf, quality: i32, subsamp: turbojpeg::Subsamp) {
    let img = image::open(&image_path).unwrap().into_rgb8();
    turbojpeg::compress_image(&img, quality, subsamp).unwrap();
    img.save(image_path).unwrap();
} 

fn get_subsamp(subsampling: &str) -> turbojpeg::Subsamp{
    match subsampling.to_lowercase().as_str() {
        "422" => turbojpeg::Subsamp::Sub2x1,
        "420" => turbojpeg::Subsamp::Sub2x2,
        "gray" => turbojpeg::Subsamp::Gray,
        "411" => turbojpeg::Subsamp::Sub4x1,
        "440" => turbojpeg::Subsamp::Sub1x2,
        _ => turbojpeg::Subsamp::Sub2x2
    }
}