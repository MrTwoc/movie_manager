use std::{error::Error, fs, path::PathBuf};

use regex::Regex;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Movie{
    disc: u32,  // 光盘编号
    year: String, // 年份
    title: String, // 标题
    remark: Option<String> // 备注
}

/// 读取文本文件并转换为JSON格式
pub fn read_text_to_json(file_path: &std::path::PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let txt = fs::read_to_string(file_path)?;
    
    //disc_no 用于记录光盘编号
    // 初始值为0，表示未设置光盘编号 
    let mut disc_no = 0u32;

    let disc_regex = Regex::new(r"^(\d+)\.$")?;
    let move_regex = Regex::new(r"^(\d{4})(.*?)(（儿童）)?$")?;
    let mut movies = Vec::new();

    // 逐行处理文本内容
    for line in txt.lines().map(str::trim).filter(|l|!l.is_empty()){
        if let Some(no) = disc_number(line,&disc_regex) {
            disc_no = no;
        } else {
            if let Some(movie) = parse_move(disc_no, line, &move_regex){
                movies.push(movie);
            }
        }
    }
    save_to_json(movies)
}

// 将电影列表保存为JSON文件
fn save_to_json(movies: Vec<Movie>) -> Result<PathBuf, Box<dyn Error>> {
    let json_str = serde_json::to_string_pretty(&movies)?;
    let path = FileDialog::new()
    .add_filter("JSON", &["json"])
    .set_title("Save as JSON file")
    .set_directory("C:/Users/Administrator/Desktop/")
    .save_file()
    .ok_or_else(|| "No save file selected".to_string())?;
    fs::write(&path, json_str)?;
    Ok(path)
}


// 从行中提取光盘编号
fn disc_number(line:&str, re:&Regex)->Option<u32> {
    re.captures(line)
    .map(|caps|caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
}

// 从行中解析电影信息
// 返回一个 Movie 结构体的 Option
fn parse_move(disc_no:u32, line:&str, re:&Regex)->Option<Movie>{
    re.captures(line).map(|caps| {
        println!("{caps:#?}");
        Movie {
            disc: disc_no, 
            year: caps.get(1).unwrap().as_str().trim().to_string(),
            title: caps.get(2).unwrap().as_str().trim().to_string(),
            remark: caps.get(3).map(|m|m.as_str().trim().to_string()),
        }
    })
}