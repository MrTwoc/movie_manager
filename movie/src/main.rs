/*
    指令说明
    调试指令
    cargo run --bin movie login --username admin
    cargo run --bin movie logout
    cargo run --bin movie list
    cargo run --bin movie add --disc 2025 --year 2025 --title "测试电影添加" --remark "测试备注"
    cargo run --bin movie delete --disc 2025 --index 0
    cargo run --bin movie edit --disc 2025 --title "测试电影编辑"
*/

use clap::{Parser, Subcommand, arg};
use movie::handler::{
    handler_add, handler_delete, handler_edit, handler_list, handler_login, handler_logout,
};
use rusqlite::{Connection, params};

use crate::sql_tools::create_db_tables;

#[derive(Parser)]
#[command(
    version,
    about = "Movie App",
    long_about = "一个简单的电影应用程序来管理您的电影收藏."
)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// 登录
    Login {
        /// The username of the user
        #[arg(short, long)]
        username: String,
    },
    /// 退出登录
    Logout,
    /// 列出所有电影
    List,
    /// 添加电影
    Add {
        /// 电影的碟片编号
        #[arg(short, long)]
        disc: usize,
        /// 电影的发行年份
        #[arg(short, long)]
        year: String,
        /// 电影的标题
        #[arg(short, long)]
        title: String,
        /// 电影的备注信息
        #[arg(short, long)]
        remark: Option<String>,
    },
    /// 删除电影
    Delete {
        /// 电影的碟片编号
        #[arg(short, long)]
        disc: usize,
        /// 电影在列表中的索引, 引入了数据库后，根据disc和title来删除电影
        // #[arg(short, long)]
        // index: usize,
        /// 电影的标题
        #[arg(short, long)]
        title: String,
    },
    /// 编辑电影
    Edit {
        /// 电影的碟片编号
        #[arg(short, long)]
        disc: usize,
        /// 电影在列表中的索引  引入了数据库后，根据disc和title来编辑电影
        // #[arg(short, long)]
        // index: usize,
        /// 电影的标题
        #[arg(short, long)]
        title: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库
    let conn = Connection::open("app.db3")?;
    conn.pragma_update(None, "journal_mode", &"WAL")?;
    if let Err(e) = conn_db(&conn) {
        println!("数据库连接失败: {}", e);
        return Err(e);
    }

    // 测试代码：获取登陆用户的角色
    // db_get_logged_in_user(&conn)?;
    // match db_get_logged_in_role(&conn) {
    //     Ok(user) => {
    //         println!("当前登录用户: {:?}", user);
    //     }
    //     Err(_) => {}
    // }

    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handler_login(&conn, username)?,
        Some(Commands::Logout) => handler_logout(&conn),
        Some(Commands::List) => handler_list(&conn)?,
        Some(Commands::Add {
            disc,
            year,
            title,
            remark,
        }) => handler_add(&conn, *disc, year, title, remark)?,
        Some(Commands::Delete { disc, title }) => handler_delete(&conn, disc, title)?,
        Some(Commands::Edit { disc, title }) => handler_edit(&conn, disc, title)?,

        _ => {
            println!("未知指令");
        }
    }

    Ok(())
}

mod sql_tools;
fn conn_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // 检查 users 表是否存在
    let table_exists: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'users')",
        params![],
        |row| row.get(0),
    )?;

    if !table_exists {
        // 若表不存在，创建初始数据表
        match create_db_tables(&conn) {
            Ok(_) => {
                // println!("初始数据表创建成功");
            }
            Err(e) => {
                println!("数据表创建失败: {}", e);
                return Err(e.into());
            }
        }
    }
    Ok(())
}
