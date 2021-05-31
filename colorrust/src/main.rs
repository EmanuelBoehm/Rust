use image::open;
use kmeans::*;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    
    let opts =  Opt::from_args();
    let verbose = opts.verbose;
    let img = open(&opts.img_path).unwrap().to_rgb8();
    let img_vec = img.to_vec().iter().map(|x| *x as f32).collect::<Vec<f32>>();
    let n = img_vec.len();
    
    let kmean = KMeans::new(img_vec, n/3, 3);
    if verbose >= 1 {
       // println!("Image \"{path}\" was loaded and has {img_length} Pixels to analyze.",path=&opts.img_path,img_length=n/3);
        println!("using kmeans to get representing colors")
    } 
    let centroids = kmean.kmeans_lloyd(10, 100000, KMeans::init_kmeanplusplus, &KMeansConfig::default()).centroids;
    let centroids_int = centroids.iter().map(|f| *f as u32).collect::<Vec<u32>>();

    println!("Centroids: {:?}", centroids_int);
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt,Debug)]
#[structopt(name = "colorrust")]
struct Opt {
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[structopt(short, long, default_value = "src/triangles.png",parse(from_os_str))]
    img_path: PathBuf,
    ///how many colors should be filtered from image
    #[structopt(short,long,default_value = "10")]
    color_count: u8,
    /// A level of verbosity, and can be used multiple times
    #[structopt(short, long, parse(from_occurrences))]
    verbose: i32,
}

