/*
    指令说明
    .\movie.exe --help
    .\movie.exe login --username admin
    .\movie.exe list
    .\movie.exe add --disc 1 --year 2020 --title "The Shawshank Redemption" --remark "Best movie ever."
    .\movie.exe delete --disc 1 --index 0
    .\movie.exe edit --disc 1 --index 0
*/

use clap::{arg, Parser, Subcommand};
use movie::handler::{handler_add, handler_delete, handler_edit, handler_list, handler_login, handler_logout};


#[derive(Parser)]
#[command(
    version,
    about = "Movie App",
    long_about = "A simple movie application to manage your movie collection.",
)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>
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
        /// 电影在列表中的索引
        #[arg(short, long)]
        index: usize,
    },
    /// 编辑电影
    Edit {
        /// 电影的碟片编号
        #[arg(short, long)]
        disc: usize,
        /// 电影在列表中的索引
        #[arg(short, long)]
        index: usize,
    }
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handler_login(username)?,
        Some(Commands::Logout) => handler_logout(),
        Some(Commands::List) => handler_list()?,
        Some(Commands::Add {
            disc, 
            year, 
            title, 
            remark
        }) => handler_add(*disc, year, title, remark)?,
        Some(Commands::Delete { disc, index }) => handler_delete(disc, index)?,
        Some(Commands::Edit { disc, index }) => handler_edit(disc, index)?,
        
        _ => {println!("Please provide a valid command.");}
    }

    Ok(())
}
