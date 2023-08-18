use image;
use turbojpeg;
use clap::Parser;
use rayon::prelude::*;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use std::path::Path;
use indicatif::ProgressBar;

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
    println!("༼ つ ◕_◕ ༽つ keab-ing.. ");
    // find all jpeg jpg and png's courtesy of  
    // https://programming-idioms.org/idiom/177/find-files-for-a-list-of-filename-extensions/6352/rust
    let all_img: Vec<PathBuf> = read_dir(&args.folder) 
    .unwrap()
    .filter_map(|f| f.ok())
    .filter(|f| match f.path().extension() {
        None => false,
        Some(ex) => ex == "jpeg" || ex == "jpg" || ex == "png"
   })
   .map(|f| f.path())
   .collect();

   let subsampling = get_subsamp(&args.subsamp);
   create_dir_all(format!("{}/compressed", &args.folder));

   let t1 = std::time::Instant::now();
   
   let total = all_img.len() as u64;
   let prog = ProgressBar::new(total);

   all_img.par_iter().for_each(|path| {
    prog.inc(1);
    keab_image(path.to_path_buf(), args.quality, subsampling)
   });
   prog.finish_with_message("compression done!\n");
   println!("compression done!, time: {:?}", t1.elapsed());

}

fn keab_image(image_path: PathBuf, quality: i32, subsamp: turbojpeg::Subsamp) {
    let img = image::open(&image_path).unwrap().into_rgb8();
    turbojpeg::compress_image(&img, quality, subsamp).unwrap();

    //maybe i will fix this idk lol
    let compresssed_path = format!("compressed/{}.jpeg", &image_path.file_stem().unwrap().to_str().unwrap()); 
    let newExt = format!("{}/{}", &image_path.parent().unwrap().to_str().unwrap() , &compresssed_path); 
    
    img.save(newExt).unwrap();
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
