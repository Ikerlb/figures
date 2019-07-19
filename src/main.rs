mod state;
mod imgutil;
mod rectangle;
mod util;
mod figure;
mod minimize;
use image::{imageops,FilterType};

use time::PreciseTime;
use clap::{Arg, App};
use state::State;
fn main() {
    let matches = App::new("figures")
                    .version("1.0")
                    .author("iker lissarrague. <ikerlb95@gmail.com>")
                    .about("approximate image with geometric figures")
                    .arg(Arg::with_name("inputFile")
                        .long("input")
                        .short("i")
                        .required(true)
                        .help("specify image to approximate")
                        .takes_value(true))
                    .arg(Arg::with_name("outputFile")
                        .long("output")
                        .short("o")
                        .required(true)
                        .help("specify output file")
                        .takes_value(true))
                    .arg(Arg::with_name("numberOfFigures")
                        .short("n")
                        .required(true)
                        .help("number of figures to approximate image")
                        .takes_value(true))
                    .arg(Arg::with_name("alpha")
                        .short("a")
                        .long("alpha")
                        .required(false)
                        .help("alpha value for the figures. 1-255")
                        .takes_value(true))
                    .arg(Arg::with_name("resize")
                        .short("r")
                        .long("resize")
                        .required(false)
                        .help("resize target image to this value")
                        .takes_value(true))
                    .arg(Arg::with_name("outputSize")
                        .short("s")
                        .long("size")
                        .required(false)
                        .help("size of output image")
                        .takes_value(true))
                .get_matches();

    let input_file=matches.value_of("inputFile").unwrap();
    let output_file=matches.value_of("outputFile").unwrap();
    let alpha:u8=matches.value_of("alpha")
                        .unwrap_or("127")
                        .parse::<u8>()
                        .unwrap();
    let resize:u32=matches.value_of("resize")
                        .unwrap_or("256")
                        .parse::<u32>()
                        .unwrap();
    let n=matches.value_of("numberOfFigures")
                 .unwrap()
                 .parse::<u32>()
                 .unwrap();
    let output_size=matches.value_of("size")
                           .unwrap_or("1024")
                           .parse::<u32>()
                           .unwrap();
    let t=image::open(input_file).unwrap().to_rgba();
    let target=imageops::resize(&t,resize,resize,FilterType::Nearest);
    let mut state=State::new(target,alpha);
    for i in 0..n{
        let start = PreciseTime::now();
        state.step();
        let end = PreciseTime::now();
        println!("Figure {} took {} seconds. {}% complete.",i+1,start.to(end),((i+1)*100)/n);
    }
    let img=imageops::resize(&state.current,output_size,output_size,FilterType::Nearest);
    img.save(output_file).unwrap();
}
