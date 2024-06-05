mod input;
mod processing;

use std::fs;
use std::io::{Read, Write};
use std::path;
use clap::Parser;


fn main() {
    let args = input::Cli::parse();

    // validate source
    let source = args.source;
    let dest = args.dest.join("out");
    fs::create_dir_all(&dest).expect("could not create output dir");

    // copy all files to tmp dir
    let files = input::get_files(source);


    for (i, file) in files
        .path()
        .read_dir()
        .expect("failed to iter files in tmp dir")
        .enumerate()
    {
        if let Ok(file) = file {
            let mut data = fs::File::open(file.path()).expect("failed to open file");
            let mut buffer = Vec::new();
            data.read_to_end(&mut buffer)
                .expect("failed to load file content to ram");

            let mut file_info = processing::FileInfo {
                count: i,
                data: buffer,
                images: vec![],
                webm: vec![],
                additional: vec![],
            };

            processing::extract_video(&mut file_info);
            processing::extract_aditional(&mut file_info);
            processing::extract_images(&mut file_info);

            for block in file_info.additional.iter(){
                let utf =  String::from_utf8_lossy(block);
                println!("found text: {}", utf);

            }
            println!("-----\nNew file");

            // save_data(&dest, file_info);
        }
    }
}

fn save_data(output: &path::PathBuf, file_info: processing::FileInfo) {

    let file = output.join(format!("file_{}.webm", file_info.count));
    let mut file = fs::File::create(file).unwrap();
    file.write_all(&file_info.webm).unwrap();

    for (i, img) in file_info.images.iter().enumerate(){
        let file = output.join(format!("file_{}_{}.jfif", file_info.count, i));
        // println!("writing file {}", file.display());

        let mut file = fs::File::create(file).expect("Could not create image file");
        file.write_all(img).unwrap()
    }

}
