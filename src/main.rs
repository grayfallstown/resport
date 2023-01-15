use image::*;
use clap::{Parser};


fn aspect_ratio(width: u32, height: u32) -> String {
    let gcd = gcd(width, height);
    format!("{}:{}", width / gcd, height / gcd)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}



#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "resport")]
#[command(about = "CLI tool that tells you an images dimensions, aspect-ratio or if it is landscape", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}


#[derive(Parser, Debug)]
enum Subcommands {
    #[clap(name = "dimensions", about = "prints 1920x1080")]
    Dimensions {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "width", about = "prints 1920")]
    Width {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "height", about = "prints 1080")]
    Height {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "aspect-ratio", about = "prints 16:9")]
    AspectRatio {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "orientation", about = "prints landscape")]
    Orientation {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "is-landscape", about = "prints true")]
    IsLandscape {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
    #[clap(name = "is-portrait", about = "prints false")]
    IsPortrait {
        #[arg(short = 'i', value_name = "IMAGE", value_hint = clap::ValueHint::FilePath)]
        image: std::path::PathBuf,
    },
}
fn main() {
    let args = Cli::parse();

    match args.command {
        Subcommands::Dimensions { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{}x{}", dimensions.0, dimensions.1);
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::Width { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{}", dimensions.0);
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::Height { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{}", dimensions.1);
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::AspectRatio { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{}", aspect_ratio(dimensions.0, dimensions.1));
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::Orientation { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    if dimensions.0 > dimensions.1 {
                        println!("landscape");
                    } else {
                        println!("portrait");
                    }
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::IsLandscape { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{:?}", dimensions.0 > dimensions.1);
                }
                Err(e) => println!("{}", e),
            }
        },
        Subcommands::IsPortrait { image } => {
            let image_res = open(image);
            match image_res {
                Ok(image) => {
                    let dimensions = image.dimensions();
                    println!("{:?}", dimensions.0 < dimensions.1);
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
