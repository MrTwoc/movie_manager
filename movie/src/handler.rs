

use std::{error::Error, io::{self, Write}};


use crate::{models::{Movie, Role}, services::{self, get_logged_in_role, get_users, login_success, logout}};

pub fn handler_login(username: &str) -> Result<(),Box<dyn std::error::Error>> {
    
    if let Some(user) = get_users().iter().find(|u|u.username.eq_ignore_ascii_case(username)) {
        // 此处区分大小写
        println!("Welcome {}!,Please input your password:", user.username);
        match rpassword::read_password()  {
            Ok(password)=> {
                if user.password == password {
                    login_success(&user.role)?;
                    println!("Login successful! You are logged in as {}.", user.username);
                } else {
                    println!("Incorrect password for user {}!", user.username);
                }
            }
            Err(_) => {
                println!("Failed to read password. Please try again.");
            }
        }
    } else {
        println!("User {} not found!", username);
    }
    Ok(())

}

pub fn handler_logout() {
    logout();
    println!("Logging out...");
}

pub fn handler_list() -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(_) => {
            let movies = services::read_form_json()?;
            services::list_movies(&movies);
        }
        None => {
            println!("查看失败，需要登陆")
        }
    }
    Ok(())
}

pub fn handler_add(
    disc: usize, 
    year: &str, 
    title: &str, 
    remark: &Option<String>
) -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(Role::Admin) => {
            let mut movies = services::read_form_json()?;
            let new_movie = Movie {
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_movie);
            services::write_to_json(&movies)?;
            println!("电影添加成功!");
        }
        _ => {
            println!("需要登陆 admin 用户才能添加电影");
        }
    }
    Ok(())
}

pub fn handler_delete(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()? {
        let movies = services::read_form_json()?;
        if let Some(movie) = movies
        .iter()
        .filter(|m| m.disc == *disc)
        .enumerate()
        .find(|(i, _)| i == index)
        .map(|(_, m)| m.clone())
        {
            let left_movies = movies
                .into_iter()
                .filter(|m| *m != movie)
                .collect::<Vec<Movie>>();
            services::write_to_json(&left_movies)?;
            println!("电影 {} 删除成功!", movie.title);
        }

    } else {
        println!("需要登陆 admin 用户才能删除电影");
    }
    Ok(())
}

pub fn handler_edit(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    // 检查用户是否有权限编辑电影
    if let Some(Role::Admin) = get_logged_in_role()? {
        // 读取电影列表
        let mut movies = services::read_form_json()?;
        // 查找指定碟片和索引的电影
        if let Some(movie) = movies
        .iter_mut()
        // 如果碟片编号匹配
        .filter(|m| m.disc == *disc)
        // enumerate() 返回索引和电影元组
        .enumerate()
        // 查找指定索引的电影
        .find(|(i, _)| i == index)
        // map: 将找到的元组转换为电影对象
        .map(|(_, m)| m)
        {
            print!("请输入新的disc (当前: {}): ", movie.disc);
            // 刷新输出缓冲区，确保提示信息立即显示
            io::stdout().flush()?;

            let mut disc = String::new();
            io::stdin().read_line(&mut disc)?;
            let disc = disc.trim();
            if let Ok(disc) = disc.parse::<usize>(){
                movie.disc = disc;
            } else {
                println!("无效的碟片编号，保持原值。");
                return Ok(());
            }

            print!("请输入新的电影年份 (当前: {}): ", movie.year);
            io::stdout().flush()?;

            let mut year = String::new();
            io::stdin().read_line(&mut year)?;
            let year = year.trim();
            if let Ok(year) = year.parse::<usize>(){
                movie.year = year.to_string();
            } else {
                println!("无效的年份，保持原值。");
                return Ok(());
            }

            print!("请输入新的电影标题 (当前: {}): ", movie.title);
            io::stdout().flush()?;

            let mut title = String::new();
            io::stdin().read_line(&mut title)?;
            let title = title.trim();
            if !title.is_empty() {
                movie.title = title.to_string();
            } else {
                println!("无效的电影标题，保持原值。");
                return Ok(());
            }

            print!("请输入新的电影备注: ");
            io::stdout().flush()?;

            let mut remark = String::new();
            io::stdin().read_line(&mut remark)?;
            let remark = remark.trim();
            if !remark.is_empty() {
                movie.remark = remark.to_string().into();
            } else {
                movie.remark = None;
            }
            services::write_to_json(&movies)?;
            println!("电影编辑成功!");
        }
    } else {
        println!("需要登陆 admin 用户才能编辑电影");
    }
    Ok(())
}