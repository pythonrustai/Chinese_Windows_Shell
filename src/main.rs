use std::env;
use std::fs;
use std::path::Path;
use std::io;
fn main() {
    loop {
        let mut line = env::current_dir();
        let mut line1: String = String::new();
        let mut mingl: String = String::new();
        let mut xiab: usize = 0;
        match line {
            Ok(line) => line1 = line.display().to_string().clone(),
            Err(_) => break,
        }
        let mut io: String = String::new();
        println!("PS {}>", line1);
        let _ = io::stdin().read_line(&mut io);
        mingl = io.trim().to_string().clone();
        if mingl.len() == 0 { continue }
        let a: Vec<&str> = mingl.split_whitespace().collect();
        match a[0].trim() {
            "切换目录" | "cd" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                xiab += 1;
                match env::set_current_dir(&a[xiab]) {
                    Ok(_) => {},
                    Err(_) => { println!("Error: 无法前往此路径 ") },
                }
            }
            "目录下文件" | "ls" | "dir" => {
                let ls = fs::read_dir(".");
                match ls {
                    Ok(e) => {
                        println!("==========文件夹下文件:==========");
                        for i in e {
                            let en = i.unwrap();
                            println!("{:#?}", en.file_name().to_string_lossy());
                        }
                    }
                    Err(_) => println!("无法读取文件夹，请检查权限和路径"),
                }
            }
            "退出" | "exit" | "quit" => break,
            "当前目录" | "pwd" => println!("{:?}", env::current_dir().unwrap()),
            "复制" | "cp" | "copy" => {
                if a.len() < 3 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                if a[1] == "con" {
                    let _ = fs::File::create(a[2]);
                }
                let a: Vec<&str> = mingl.split_whitespace().collect();
                let _ = fs::copy(a[1].to_string(), a[2].to_string());
                println!("{:#?}已保存在{:#?}", a[1].to_string(), a[2].to_string());
            },
            "" => continue,
            "rm" | "删除" | "rmdir" | "remove" | "del" | "delete" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                println!("您确定要删除{}吗?(Y/n)", a[1].to_string());
                let mut q = String::new();
                let _ = io::stdin().read_line(&mut q).expect("读取确认输入失败");
                if q.trim() == "Y" {
                    if Path::is_dir(std::path::Path::new(a[1])) {
                        fs::remove_dir_all(a[1]).unwrap();
                        println!("成功删除文件夹\"{}\",祝你好运!", a[1].to_string());
                    } else if Path::is_file(std::path::Path::new(a[1])) {
                        fs::remove_file(a[1]).unwrap();
                        println!("成功删除文件\"{}\",祝你好运!", a[1].to_string());
                    }
                } else if q.trim() == "n" {
                    continue;
                } else {
                    continue;
                }
            }
            "echo" | "打印" | "print" | "printf" | "println" | "println!" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                let q: String = a[1..].join(" ");
                println!("{:#?}", q);
            },
            "mkdir" | "mk" | "新建文件夹" | "新文件夹" | "创建目录" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                match fs::create_dir(a[1]) {
                    Ok(_) => { let _ = fs::create_dir(a[1]); },
                    Err(_) => { println!("文件夹\"{}\"创建", a[1].to_string()); }
                };
            },
            "touch" | "New-Item" | "ni" | "新建文件" | "新文件" | "创建文件" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                if a[0] == "touch" || a[0] == "新建文件" {
                    let _ = fs::File::create(a[1]);
                }

            },
            "type" | "文件内容" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue; }
                if a[1] == "nul" && a[2] == ">" {
                    let _ = fs::File::create(a[3]);
                }
                let q = fs::read_to_string(a[1]);
                match &q {
                    Ok(q) => { println!("{}", q);},
                    Err(_) => println!("\"{}\"无法打开", a[1].to_string())
                }

            }
            _ => println!("Error: command \"{}\" not found", mingl),
        }
    }
}
