use std::error::Error;

use movie::{
    models::{Role, User},
    services::read_form_json,
};
/*
    数据库相关代码，可用于代替service.rs文件
    1. 连接数据库
    2. 初始化数据库
    3. 创建数据库表 [用户表、电影表]
        用户表 [username、password、role]
        电影表 [disc、year、title、remark、user_id]
    4. 定义数据库操作函数 [注册用户、添加电影、查询电影、删除电影]
*/
use rusqlite::{Connection, Result};

pub fn create_db_tables(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化用户表，字段：id、用户名、密码、角色、登陆状态(离线还是在线),默认是离线
    // id是自增的，所以不需要指定初始值
    // role是枚举类型，需要转换为字符串
    // login_status是离线还是在线，初始值是离线，用户登陆后需要修改为在线
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL,
            login_status TEXT NOT NULL DEFAULT 'offline'
        )",
        (),
    )?;
    println!("用户数据表创建成功");

    // 初始化电影表,字段：id、碟片、年份、电影名、备注、用户id
    // id是自增的，所以不需要指定初始值
    // user_id是外键，需要关联到users表中的id字段
    // disc是碟片编号，year是年份，title是电影名，remark是备注，user_id是用户id
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY,
            disc INTEGER NOT NULL,
            year TEXT NOT NULL,
            title TEXT NOT NULL,
            remark TEXT,
            user_id INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        (),
    )?;
    println!("电影数据表创建成功");

    // 添加三个初始用户
    // 参数：用户名、密码、角色
    conn.execute(
        "INSERT INTO users (username, password, role) VALUES (?1, ?2, ?3)",
        ("admin", "admin", "admin"),
    )?;
    conn.execute(
        "INSERT INTO users (username, password, role) VALUES (?1, ?2, ?3)",
        ("user", "user", "user"),
    )?;
    conn.execute(
        "INSERT INTO users (username, password, role) VALUES (?1, ?2, ?3)",
        ("Dave", "Dave", "user"),
    )?;
    println!("初始用户添加成功");

    // 添加三部初始电影信息
    // 参数：编号、碟片、年份、电影名、备注、用户id
    conn.execute(
        "INSERT INTO movies (disc, year, title, remark, user_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        (1, 2019, "肖申克的救赎", "一部经典电影", 1),
    )?;
    println!("初始电影信息添加成功");
    // json_save_to_db(conn)?;
    println!("将json文件中的电影信息添加到数据库成功");
    if let Err(e) = json_save_to_db(conn) {
        eprintln!("Error fetching user: {}", e);
    }
    Ok(())
}

// 注册用户
pub fn _register_user(conn: &Connection, user: &User) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO users (username, password, role) VALUES (?1, ?2, ?3)",
        (&user.username, &user.password, &user.role.to_string()),
    )?;
    println!("用户注册成功");
    Ok(())
}

// 测试函数-列出所有用户
pub fn _list_users(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT username, password, role FROM users")?;

    let users = stmt.query_map([], |row| {
        let role_str: String = row.get(2)?;
        Ok(User {
            // id: row.get(0)?,
            username: row.get(0)?,
            password: row.get(1)?,
            role: match role_str.as_str() {
                "admin" => Role::Admin,
                "user" => Role::User,
                _ => Role::User,
            },
        })
    })?;

    // 遍历查询结果并打印
    for user in users {
        match user {
            Ok(user) => println!("{:?}", user),
            Err(e) => eprintln!("Error fetching user: {}", e),
        }
    }
    Ok(())
}

// 将json文件中所有电影信息转移到数据库
fn json_save_to_db(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let movies = read_form_json()?;
    for movie in movies {
        conn.execute(
            "INSERT INTO movies (disc, year, title, remark, user_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            (movie.disc, movie.year, movie.title, movie.remark, 1),
        )?;
    }
    Ok(())
}
