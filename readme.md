# Movie App

## 简介

Movie App 是一个简单的电影应用程序，用于管理电影收藏。</br>
该应用程序提供了用户登录和退出登录的功能，支持管理员和普通用户两种角色。</br>
这是一个跟随课程学习的练习Demo的第一、二、三章</br>
[【课程地址】"Rust 实战：电影信息维护（命令行版）"](https://www.bilibili.com/video/BV1i1jJz3Eqf?spm_id_from=333.788.videopod.sections&vd_source=511b084e4bf87d71d725c5db0fb20b7f)</br>
### movie_importer 项目(课程第一部分)
视频第一章：将txt格式的电影信息整理为json格式
#### 功能
- 识别文本中的碟片编号（如`24.`）
- 解析电影条目格式：`年份 标题（可选备注）`
- 自动过滤空行和无效内容
- 保存文件时支持GUI路径选择
### movie 项目(课程第二、三部分)
视频第二、三章：实现用户登录登出以及角色管理、json文件中电影信息的增删改查功能
#### 功能
- 用户登录与退出：支持普通用户和管理员用户两种角色，密码通过rpassword隐藏输入。
- 电影信息查看：查看所有电影信息，包括碟片编号、年份、标题和备注。
- 电影信息增删改：管理员用户可以添加新电影、删除已有电影，普通用户不能进行该操作。
- 日志记录：记录用户登录登出操作以及关键错误信息。

### 我对项目的扩展 / 修改
- 将电影信息和用户信息保存为sqlite数据库
- 通过数据库对电影信息进行增删改查
- 通过数据库对用户信息进行核对
## 使用方法
```
cargo run --bin movie login --username admin
cargo run --bin movie logout
cargo run --bin movie list
cargo run --bin movie add --disc 2025 --year 2025 --title "测试电影添加" --remark "测试备注"
cargo run --bin movie delete --disc 2025 --index 0
cargo run --bin movie edit --disc 2025 --title "测试电影编辑"
```
初始用户
| 用户名 | 密码 | 角色 |
 |--------|-------|--------| 
 | admin | admin | Admin | 
 | Dave | Dave | User | 
 | user | user | User |