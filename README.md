![Build Status](https://img.shields.io/github/actions/workflow/status/pythonrustai/Chinese_Windows_Shell/rust.yml?label=Build)
# Chinese_Windows_Shell
## 简介
一个运行在Windows平台上的中文shell，旨在为中文开发者一个母语shell平台，本程序使用Rust编写

## 使用
exe在路径shell\target\debug\shell.exe ,双击运行即可

## 目前支持命令
目前此shell较为简陋，仅支持 cd,ls,pwd,copy,exit,echo,pwd等，后续我会逐步添加命令，以达到兼容CMD的效果

## 命令指南
cd: 输入"cd"或"切换目录" ，再空格，再输入路径。 

ls: 输入"ls"或"目录下文件"或"dir"打印当前路径下的所有文件和文件夹，以字母顺序打印。 

cp：输入"cp"或"copy"或"复制"，空格，加上源文件，空格，加上目标路径(末尾加上你的目标文件的路径)，注意：如果该目录有同名文件，会覆盖！ 

exit：输入"exit"或"quit"或"退出"，关闭命令行。 

echo：输入"echo"或"打印"，空格，加上要打印的内容，打印出要打印的内容。

pwd：输入"pwd"或"当前路径"，打印出当前路径。

remove:输入"del"或"rm"或"rmdir"或"remove"或"删除"，空格，加上路径，会自己识别文件还是文件夹，然后彻底删除，不进回收站！

type: 输入"type"或"文件内容"，空格，加上要打印的文件路径，会打印出该文件的内容。

mkdir: 输入"mkdir"或"创建目录"或"新建文件夹"新文件夹”或"创建目录"，空格，加上要创建的目录路径，会创建该目录。

touch: 输入"touch"或"创建文件"或"新建文件"或"创建文件"或"New-Item"或"type nul >"或"ni"或"新文件" ，空格，加上要创建的文件路径，会创建该文件。

（注：直接在命令行中输入可执行文件名或批处理文件名，即可执行）
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
It currently supports basic commands: `cd`, `ls`, `pwd`, `copy`, `exit`,`del`.
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

### 5. echo (Print Message)
Usage: Type `echo` (or Chinese alias: 打印) + space + message to print
Function: Print the specified message on the terminal.

### 6. pwd (Print Current Working Directory)
Usage: Type `pwd` (or Chinese alias: 当前路径)
Function: Print the current working directory path.
### 7.remove (Remove folder or files)
Usage: Type `del` / `rm` / `rmdir` / `remove` or the Chinese word 删除

Function: Followed by a space and a path.The program will automatically distinguish whether the target is a file or folder, then permanently delete it without sending items to the Recycle Bin!

### 8. type (Print File Content)
Usage: Type `type` or the Chinese word `文件内容` + space + file path

Function: Print the content of the specified file on the terminal.

### 9. mkdir (Create Directory)
Usage: Type `mkdir` or the Chinese word `创建目录` + space + directory path

Function: Create a new directory at the specified path.

### 10. touch (Create File)
Usage: Type `touch` or the Chinese word `创建文件` + space + file path

Function: Create a new file at the specified path.

(Notice: Directly input the executable file name or batch file name in the command line to execute it.)