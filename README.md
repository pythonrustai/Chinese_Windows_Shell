# Chinese_Windows_Shell
## 简介
一个运行在Windows平台上的中文shell，旨在为中文开发者一个母语shell平台，本程序使用Rust编写

## 使用
exe在路径shell\target\debug\shell.exe ,双击运行即可

## 目前支持命令
目前此shell较为简陋，仅支持 cd,ls,pwd,copy,exit等，后续我会逐步添加命令，以达到兼容CMD的效果

## 命令指南
cd: 输入"cd"或"切换目录" ，再空格，再输入路径。 

ls: 输入"ls"或"目录下文件"或"dir"打印当前路径下的所有文件和文件夹，以字母顺序打印。 

cp：输入"cp"或"copy"或"复制"，空格，加上源文件，空格，加上目标路径(末尾加上你的目标文件的路径)，注意：如果该目录有同名文件，会覆盖！ 

exit：输入"exit"或"quit"或"退出"，关闭命令行。 

remove:输入"del"或"rm"或"rmdir"或"remove"或"删除"，空格，加上路径，会自己识别文件还是文件夹，然后彻底删除，不进回收站！

# Chinese_Windows_Shell

## Introduction
A Chinese-language Shell running on the Windows platform.
It aims to provide Chinese developers with a native mother-tongue Shell interactive environment.
This project is entirely written in Rust.

## How to Use
The executable file path:
`shell/target/debug/shell.exe`
Double-click the `.exe` file to launch the program directly.

## Supported Commands (Current Status)
This Shell is still in the early development stage with limited features.
It currently supports basic commands: `cd`, `ls`, `pwd`, `copy`, `exit`.
More commands will be added iteratively in subsequent updates to achieve full compatibility with the official Windows CMD.

## Command Manual
### 1. cd (Change Working Directory)
Usage: Type `cd` (or Chinese alias: 切换目录) + space + target directory path

Function: Navigate to the specified folder path.

### 2. ls (List Directory Contents)
Usage: Type `ls` / `dir` (or Chinese alias: 目录下文件)

Function: List all files and folders inside the current directory, sorted alphabetically.

### 3. cp / copy (File Copy)
Usage: Type `cp` / `copy` (or Chinese alias: 复制) + space + source file path + space + destination path

Warning: If a file with the same name already exists in the destination directory, it will be overwritten without prompt.

### 4. exit (Quit Shell Program)
Usage: Type `exit` / `quit` (or Chinese alias: 退出)

Function: Terminate and close the Shell terminal window.

### 5.remove (Remove folder or files)
Usage: Type del, rm, rmdir, remove or the Chinese word 删除

Function: Followed by a space and a path.The program will automatically distinguish whether the target is a file or folder, then permanently delete it without sending items to the Recycle Bin!

