# Movie App

## 简介

Movie App 是一个简单的电影应用程序，用于管理电影收藏。</br>
该应用程序提供了用户登录和退出登录的功能，支持管理员和普通用户两种角色。</br>
这是一个跟随课程学习的练习Demo的第一、二、三章</br>
[【课程地址】"Rust 实战：电影信息维护（命令行版）"](https://www.bilibili.com/video/BV1i1jJz3Eqf?spm_id_from=333.788.videopod.sections&vd_source=511b084e4bf87d71d725c5db0fb20b7f)</br>
### movie_importer 项目(第一部分)
视频第一章：将txt格式的电影信息整理为json格式
#### 简述
- 使用Rust标准库进行文件IO操作
- 正则表达式(regex crate)解析文本格式
- Serde框架实现JSON序列化
- rfd库实现文件对话框交互

#### 功能
- 识别文本中的碟片编号（如`24.`）
- 解析电影条目格式：`年份 标题（可选备注）`
- 自动过滤空行和无效内容
- 保存文件时支持GUI路径选择
### movie 项目(第二、三部分)
视频第二、第三章：实现用户登录和登出以及角色管理、增删改查功能
#### 简述
- 使用Rust标准库进行文件IO操作
- Serde框架实现JSON序列化
- 基于clap实现命令行参数解析
- 基于rpassword实现密码输入的安全处理
- 基于log库实现简单的日志记录

#### 功能
- 用户登录与退出：支持普通用户和管理员用户两种角色，密码通过rpassword隐藏输入。
- 电影信息查看：查看所有电影信息，包括碟片编号、年份、标题和备注。
- 电影信息增删改：管理员用户可以添加新电影、删除已有电影，普通用户不能进行该操作。
- 日志记录：记录用户登录登出操作以及关键错误信息。

## 项目结构
```plaintext
movie_manager/
├── Cargo.toml
├── movie/
│   ├── Cargo.toml
│   ├── Movies.json
│   ├── readme.md
│   └── src/
│       ├── handler.rs      # 处理用户登录和退出登录的逻辑
│       ├── main.rs         # 程序入口，处理命令行参数
│       ├── models.rs       # 定义用户和角色的数据结构
│       ├── services.rs     # 提供用户服务，如获取用户列表、登录、退出登录等
│       └── lib.rs          # 模块导出文件
├── movie_importer/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs          # 包含核心功能函数，如读取文本转JSON
│       ├── main.rs         # 程序入口，处理文件选择
│       └── DVDs.txt        # 包含电影信息的文本文件
└── readme.md
```
## 依赖
clap 4.5.40：用于解析命令行参数。</br>
rpassword 7.4.0：用于安全地读取用户输入的密码。</br>
serde 1.0.194：用于序列化和反序列化数据。</br>
serde_json 1.0.106：用于处理 JSON 数据。</br>
unicode-width 0.2.1：用于获取 Unicode 字符宽度。</br>

## 使用方法
```
cargo run -- help
登录：
cargo run -- login --username <your_username>
例如
cargo run -- login --username dave
登出
cargo run -- logout
```
可用用户
| 用户名 | 密码 | 角色 |
 |--------|-------|--------| 
 | admin | admin | Admin | 
 | Dave | Dave | User | 
 | user | user | User |