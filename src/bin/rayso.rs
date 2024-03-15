use std::path::PathBuf;

use rayso_rs::*;
use structopt::StructOpt;

/// RaysoRS is a simple and fast downloader for the Rayso code snapshot service.
#[derive(StructOpt, Debug)]
#[structopt(name = "rayso")]
struct Opt {
    #[structopt(name = "FILE", help = "The file to upload to rayso.")]
    file: String,
    #[structopt(
        short,
        long,
        help = "Set whether dark mode is used.",
        parse(try_from_str),
        default_value = "true"
    )]
    dark_mode: bool,
    #[structopt(
        short,
        long,
        help = "Set whether a background is used.",
        parse(try_from_str),
        default_value = "true"
    )]
    background: bool,
    #[structopt(short, long, help = "Set the language, normally auto-detected.")]
    language: Option<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let input = std::fs::read_to_string(opt.file.clone()).unwrap();

    let mut config = RaysoConfigBuilder::default().code(&input);
    config = config.dark_mode(opt.dark_mode);
    config = config.background(opt.background);

    if let Some(language) = opt.language {
        config = config.language(&language);
    }

    config = config.title(
        PathBuf::from(opt.file.clone())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let config = config.build();

    let result = downloader::download(config);

    match result.await {
        Ok(_) => println!("Downloaded {}", opt.file),
        Err(e) => println!("Error: {}", e),
    }
}
