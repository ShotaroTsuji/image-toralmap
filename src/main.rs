extern crate getopts;
extern crate image;

use std::path::{Path,PathBuf};
use image::RgbImage;
use getopts::Options;
use std::env;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Matrix {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl FromStr for Matrix {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.split(",").collect();
        let a_fromstr = nums[0].parse::<i64>()?;
        let b_fromstr = nums[1].parse::<i64>()?;
        let c_fromstr = nums[2].parse::<i64>()?;
        let d_fromstr = nums[3].parse::<i64>()?;
        Ok(Matrix { a: a_fromstr, b: b_fromstr, c: c_fromstr, d: d_fromstr })
    }
}

fn modulo(x: i64, y: i64) -> i64 {
    ((x % y) + y) % y
}

fn apply_toralmap(src: RgbImage, mat: &Matrix) -> RgbImage {
    let mut dest = RgbImage::new(src.width(), src.height());
    let w = src.width() as i64;
    let h = src.height() as i64;

    for (x, y, p) in src.enumerate_pixels() {
        let x = x as i64;
        let y = y as i64;
        let new_x = modulo((mat.a*x*h + mat.b*y*w) / h, w);
        let new_y = modulo((mat.c*x*h + mat.d*y*w) / w, h);
        dest.put_pixel(new_x as u32, new_y as u32, *p);
    }

    dest
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optopt("c", "", "set the number of iterations", "COUNT");
    opts.optopt("m", "", "set the elements of matrix", "MATRIX");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let output = matches.opt_str("o")
        .unwrap_or_else(|| {
            let inpath = Path::new(&input);
            let mut path = PathBuf::new();
            path.push("output");
            path.set_extension(inpath.extension().unwrap());
            String::from(path.to_str().unwrap())
        });
    let count = matches.opt_str("c").map_or(1, |s| s.parse::<u32>().unwrap());
    let matrix = matches.opt_str("m")
        .map_or(Matrix{ a: 2, b: 1, c: 1, d: 1 },
                |s| s.parse::<Matrix>().unwrap());

    println!("Input  : {}", input);
    println!("Output : {}", output);
    println!("count  : {}", count);
    println!("matrix : {:?}", matrix);

    let mut img = image::open(input).unwrap().to_rgb();
    println!("width = {}, height = {}", img.width(), img.height());

    for _ in 0..count {
        img = apply_toralmap(img, &matrix);
    }

    img.save(output).unwrap();
}
