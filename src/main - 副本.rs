use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::io::Write;
fn main() {
    loop {
        let line = env::current_dir();
        let mut line1: String = String::new();
        let mut mingl: String = String::new();
        let mut xiab: usize = 0;
        match line {
            Ok(line) => line1 = line.display().to_string().clone(),
            Err(_) => break,
        }
        let mut io: String = String::new();
        print!("PS {}>", line1);
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut io);
        mingl = io.trim().to_string().clone();
        if mingl.len() == 0 { continue; }
        let a: Vec<&str> = mingl.split_whitespace().collect();
        match a[0].trim() {
            "切换目录" | "cd" => {
                if a.len() == 1 {
                    let _ = env::set_current_dir(env::home_dir().unwrap());
                } else {
                    let full_path = a[1..].join(" ");
                    match env::set_current_dir(Path::new(&full_path)) {
                        Ok(_) => {}
                        Err(q) => {
                            eprintln!("切换目录失败：{}", q);
                        }
                    }
                }
            },
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
            "当前目录" | "pwd" => { println!("{}", env::current_dir().unwrap().display()); }
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
                println!("{}已保存在{}", a[1].to_string(), a[2].to_string());
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
                if a.len() == 2 {
                    match fs::create_dir(a[1]) {
                        Ok(_) => {
                            println!("文件夹\"{}\"创建成功", a[1].to_string());
                        },
                        Err(_) => { println!("文件夹\"{}\"创建失败", a[1].to_string()); }
                    };
                }
                if a.len() > 2 {
                    let q: usize = a.len();
                    let mut m: usize = 1;
                    while m < q {
                        match fs::create_dir(a[m].to_string()) {
                            Ok(_) => {
                                let _ = fs::create_dir(a[m].to_string());
                                println!("文件夹\"{}\"创建成功", a[m].to_string());},
                            Err(_) => println!("文件夹\"{}\"创建失败", a[m].to_string())
                        }
                        let _ = env::set_current_dir(Path::new(&a[m]));
                        m += 1;
                        }
                    }
                if a.len() > 3 {
                    if a[1] == "-h" {
                        let x: usize = a.len().try_into().unwrap();
                        let mut c: usize = 2;
                        let patt = env::current_dir();
                        while c < x {
                            match fs::create_dir(a[c].to_string()) {
                                Ok(_) => { println!("文件夹\"{}\"创建成功", a[c].to_string()); },
                                Err(_) => println!("文件夹\"{}\"创建失败", a[c].to_string())
                            }
                            let _ = env::set_current_dir(Path::new(&a[c]));
                            c += 1;
                            }
                        }
                    let _ = env::set_current_dir(env::home_dir().unwrap());
                }
            }
            "touch" | "New-Item" | "ni" | "新建文件" | "新文件" | "创建文件" => {
                if a.len() < 2 {
                    println!("命令\"{}\"无效!", a[0].to_string());
                    continue;
                }
                if a[0] == "touch" || a[0] == "新建文件" {
                    let _ = fs::File::create(a[1]);
                }

            }
            "type" | "文件内容" | "cat" => {
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
            _ => {
                let w = &a[0];
                let mlwj = fs::read_dir(".");
                if let mlwj = w {
                    if std::path::Path::new(&w).exists() {
                        let q= Command::new("cmd").args(["/c", &mingl]).spawn();
                        match &q {
                            Ok(_q) => {},
                            Err(_) => println!("Error:\"{}\"command not found!\n错误:命令\"{}\"不是命令或可执行文件!", mingl, mingl) }
                    }
                }
            }
        }
    }
}
