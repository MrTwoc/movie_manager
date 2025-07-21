use std::{
    error::Error,
    io::{self, Write},
};

use rusqlite::Connection;

use crate::{
    db_services::{
        db_add_movie_to_db, db_delete_movie_from_db, db_find_movie, db_get_logged_in_role,
        db_get_users, db_list_movies, db_login_out, db_login_success, db_update_movie_in_db,
    },
    models::{Movie, Role},
};

// 处理登陆功能
pub fn handler_login(conn: &Connection, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user_info = db_get_users(&conn, username)?;
    // println!(
    //     "{} 的密码：{}，角色是: {}",
    //     user_info.username, user_info.password, user_info.role
    // );
    // 此处区分大小写
    println!("欢迎 {}!,请输入密码:", user_info.username);
    match rpassword::read_password() {
        Ok(password) => {
            if user_info.password == password {
                // login_success(&user_info.role)?;
                db_login_success(&conn, &user_info.username)?;
                println!("登陆成功-当前用户：{}.", user_info.username);
            } else {
                println!("密码错误!");
            }
        }
        Err(_) => {
            println!("登陆失败! 请检查用户名和密码是否正确.");
        }
    }
    Ok(())
}

pub fn handler_logout(conn: &Connection) {
    // logout();
    match db_login_out(&conn) {
        Ok(_) => {}
        Err(e) => {
            println!("注销登录失败: {}", e);
        }
    }
}

pub fn handler_list(conn: &Connection) -> Result<(), Box<dyn Error>> {
    match db_get_logged_in_role(&conn)? {
        Some(Role::Admin) | Some(Role::User) => {
            // let movies = services::read_form_json()?;
            // services::list_movies(&movies);
            db_list_movies(conn)?;
        }
        _ => {
            println!("查看失败,当前没有用户登录");
        }
    }
    Ok(())
}

pub fn handler_add(
    conn: &Connection,
    disc: usize,
    year: &str,
    title: &str,
    remark: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    match db_get_logged_in_role(&conn)? {
        Some(Role::Admin) => {
            let new_movie = Movie {
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            // 使用 if let 简化错误处理逻辑
            if let Err(e) = db_add_movie_to_db(&conn, &new_movie) {
                println!("电影添加失败: {}", e);
                return Ok(());
            }
        }
        _ => {
            println!("需要登陆 admin 用户才能添加电影");
        }
    }
    Ok(())
}

pub fn handler_delete(conn: &Connection, disc: &usize, title: &str) -> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = db_get_logged_in_role(&conn)? {
        if let Err(e) = db_delete_movie_from_db(conn, disc, title) {
            println!("电影删除失败: {}", e);
            return Ok(());
        }
    } else {
        println!("需要登陆 admin 用户才能删除电影");
    }
    Ok(())
}

pub fn handler_edit(
    conn: &Connection,
    old_disc: &usize,
    old_title: &str,
    // index: &usize,
) -> Result<(), Box<dyn Error>> {
    // 检查用户是否有权限编辑电影
    if let Some(Role::Admin) = db_get_logged_in_role(&conn)? {
        // 测试代码：
        // db_find_movie(&conn, disc, title)?;
        match db_find_movie(&conn, old_disc, old_title) {
            Ok(true) => {
                let mut movie = Movie {
                    disc: 1,
                    year: "".to_string(),
                    title: "".to_string(),
                    remark: None,
                };
                println!("找到电影-进入编辑状态");
                print!("请输入新的disc (当前: {}): ", movie.disc);
                // 刷新输出缓冲区，确保提示信息立即显示
                io::stdout().flush()?;

                let mut disc = String::new();
                io::stdin().read_line(&mut disc)?;
                let disc = disc.trim();
                if let Ok(disc) = disc.parse::<usize>() {
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
                if let Ok(year) = year.parse::<usize>() {
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
                // db_update_movie_in_db(&conn, &movie.disc, &movie.title, &movie)?;
                if let Err(e) = db_update_movie_in_db(&conn, old_disc, old_title, &movie) {
                    println!("电影编辑失败: {}", e);
                    return Ok(());
                }
            }
            Ok(false) => {
                println!("未找到电影");
            }
            Err(e) => {
                println!("查询电影失败: {}", e);
            }
        }
        // 读取电影列表
        // let mut movies = services::read_form_json()?;
        // // 查找指定碟片和索引的电影
        // if let Some(movie) = movies
        //     .iter_mut()
        //     // 如果碟片编号匹配
        //     .filter(|m| m.disc == *disc)
        //     // enumerate() 返回索引和电影元组
        //     .enumerate()
        //     // 查找指定索引的电影
        //     .find(|(i, _)| i == index)
        //     // map: 将找到的元组转换为电影对象
        //     .map(|(_, m)| m)
        // {
        // print!("请输入新的disc (当前: {}): ", movie.disc);
        // // 刷新输出缓冲区，确保提示信息立即显示
        // io::stdout().flush()?;

        // let mut disc = String::new();
        // io::stdin().read_line(&mut disc)?;
        // let disc = disc.trim();
        // if let Ok(disc) = disc.parse::<usize>() {
        //     movie.disc = disc;
        // } else {
        //     println!("无效的碟片编号，保持原值。");
        //     return Ok(());
        // }

        // print!("请输入新的电影年份 (当前: {}): ", movie.year);
        // io::stdout().flush()?;

        // let mut year = String::new();
        // io::stdin().read_line(&mut year)?;
        // let year = year.trim();
        // if let Ok(year) = year.parse::<usize>() {
        //     movie.year = year.to_string();
        // } else {
        //     println!("无效的年份，保持原值。");
        //     return Ok(());
        // }

        // print!("请输入新的电影标题 (当前: {}): ", movie.title);
        // io::stdout().flush()?;

        // let mut title = String::new();
        // io::stdin().read_line(&mut title)?;
        // let title = title.trim();
        // if !title.is_empty() {
        //     movie.title = title.to_string();
        // } else {
        //     println!("无效的电影标题，保持原值。");
        //     return Ok(());
        // }

        // print!("请输入新的电影备注: ");
        // io::stdout().flush()?;

        // let mut remark = String::new();
        // io::stdin().read_line(&mut remark)?;
        // let remark = remark.trim();
        // if !remark.is_empty() {
        //     movie.remark = remark.to_string().into();
        // } else {
        //     movie.remark = None;
        // }
        //     services::write_to_json(&movies)?;
        //     println!("电影编辑成功!");
        // }
    } else {
        println!("需要登陆 admin 用户才能编辑电影");
    }
    Ok(())
}
