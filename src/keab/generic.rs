use clap::Error;
use image::imageops::sample_bilinear;
use turbojpeg::*;

pub struct GenericImage {
    filename : String,
    extension : String
}
