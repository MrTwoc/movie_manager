/*
    课程地址：
    https://www.bilibili.com/video/BV1spJVzCEv8/?spm_id_from=333.1007.top_right_bar_window_custom_collection.content.click&vd_source=511b084e4bf87d71d725c5db0fb20b7f
*/

use std::{error::Error, process};

use movie_importer::read_text_to_json;
use rfd::FileDialog;

fn main() -> Result<(), Box<dyn Error>> {
    match FileDialog::new().add_filter("Text File", &["txt"])
    .set_title("select the DVD text file")
    .set_directory("C:/Users/Administrator/Desktop/")
    .pick_file() {
        Some(file_path) => {
            let file_path = read_text_to_json(&file_path)?;
            println!("Data save to: {file_path:?}");
            Ok(())
        }
        None => {
            eprintln!("File not selected");
            process::exit(-1);
        }
    }
}

