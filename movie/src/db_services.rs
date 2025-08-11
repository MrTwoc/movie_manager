use std::error::Error;

use rusqlite::Connection;

use crate::models::{Movie, Role, User};

// 接收用户输入的用户名，查询数据库，找到返回User交给调用方判断密码，否则返回错误信息
pub fn db_get_users(conn: &Connection, username: &str) -> Result<User, Box<dyn Error>> {
    // 登录功能，接收用户输入的用户名，密码，查询数据库。
    // 若用户状态为在线，则提示用户已登录，否则验证用户名和密码是否正确.
    // 若用户名和密码正确则将登录状态设为online，否则返回错误信息
    match conn.prepare("SELECT username, password, role FROM users WHERE username = ?") {
        Ok(mut stmt) => {
            let user = stmt.query_row([username], |row| {
                let role_str: String = row.get(2)?;
                Ok(User {
                    username: row.get(0)?,
                    password: row.get(1)?,
                    role: match role_str.as_str() {
                        "admin" => Role::Admin,
                        "user" => Role::User,
                        _ => Role::User,
                    },
                })
            })?;
            Ok(user)
        }
        Err(e) => {
            println!("查询用户失败: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn db_login_success(conn: &Connection, username: &str) -> Result<(), Box<dyn Error>> {
    // 将所在用户的状态设为online
    match conn.prepare("UPDATE users SET login_status = 'online' WHERE username = ?") {
        Ok(mut stmt) => {
            stmt.execute([username])?;
            Ok(())
        }
        Err(e) => {
            println!("更新用户状态失败: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn db_login_out(conn: &Connection) -> Result<(), Box<dyn Error>> {
    // println!("正在注销登录");
    // 将所在用户的状态设为offline
    match conn.prepare("UPDATE users SET login_status = 'offline' WHERE login_status = 'online'") {
        Ok(mut stmt) => {
            stmt.execute([])?;
            println!("注销登录成功");
            Ok(())
        }
        Err(e) => {
            println!("更新用户状态失败: {}", e);
            Err(Box::new(e))
        }
    }
}

// 从数据库中读取当前登录用户的角色，替代get_logged_in_role()
pub fn db_get_logged_in_role(conn: &Connection) -> Result<Option<Role>, Box<dyn Error>> {
    let mut stmt =
        conn.prepare("SELECT username, password, role FROM users WHERE login_status = 'online'")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let role_str: String = row.get(2)?;
        let role = match role_str.as_str() {
            "admin" => Role::Admin,
            "user" => Role::User,
            _ => Role::User,
        };
        Ok(Some(role))
    } else {
        Ok(None)
    }
}

pub fn db_list_movies(conn: &Connection) -> Result<Vec<Movie>, Box<dyn Error>> {
    // 使用常量定义SQL语句
    const SQL: &str = "
        SELECT disc, year, title, remark 
        FROM movies
    ";

    let mut stmt = conn.prepare(SQL)?;

    // 使用列名代替索引，提高可读性和安全性
    let movies = stmt
        .query_map([], |row| {
            Ok(Movie {
                disc: row.get("disc")?,
                year: row.get("year")?,
                title: row.get("title")?,
                remark: row.get("remark")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?; // 统一收集错误
    // 打印查询结果
    for movie in &movies {
        println!("{:?}", movie);
    }

    Ok(movies)
}

pub fn db_add_movie_to_db(conn: &Connection, movie: &Movie) -> Result<(), Box<dyn Error>> {
    // 添加电影的user_id 默认先设为1，后续再修改
    let mut stmt = conn.prepare(
        "INSERT INTO movies (disc, year, title, remark, user_id) VALUES (?, ?, ?, ?, ?)",
    )?;
    stmt.execute((movie.disc, &movie.year, &movie.title, &movie.remark, 1))?;
    println!("db_save_to_db->电影信息添加成功");
    Ok(())
}

pub fn db_delete_movie_from_db(
    conn: &Connection,
    disc: &usize,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    // 因为disc不是全局唯一，所以根据disc和title删除电影信息
    let mut stmt = conn.prepare("DELETE FROM movies WHERE disc = ? AND title = ?")?;
    let rows_affected = stmt.execute((disc, &title))?;

    match rows_affected {
        0 => {
            println!(
                "db_delete_movie_from_db->电影信息删除失败:碟片-{}，电影:{}",
                disc, title
            );
        }
        _ => {
            println!(
                "db_delete_movie_from_db->电影信息删除成功:碟片-{}，电影:{}",
                disc, title
            );
        }
    }
    Ok(())
}

// 根据disc和title判断是否存在该电影
pub fn db_find_movie(conn: &Connection, disc: &usize, title: &str) -> Result<bool, Box<dyn Error>> {
    // 通过disc和title查找电影
    let mut stmt = conn.prepare("SELECT disc, title FROM movies WHERE disc = ? AND title = ?")?;
    let mut rows = stmt.query((disc, &title))?;
    if let Some(_) = rows.next()? {
        return Ok(true);
    }
    Ok(false)
}

// 根据disc和title更新电影信息
pub fn db_update_movie_in_db(
    conn: &Connection,
    old_disc: &usize,
    old_title: &str,
    movie: &Movie,
) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "UPDATE movies SET disc = ?, year = ?, title = ?, remark = ? WHERE disc = ? AND title = ?",
    )?;
    stmt.execute((
        &movie.disc,
        &movie.year,
        &movie.title,
        &movie.remark,
        old_disc,
        old_title,
    ))?;
    println!("db_update_movie_in_db->电影信息更新成功");
    Ok(())
}
