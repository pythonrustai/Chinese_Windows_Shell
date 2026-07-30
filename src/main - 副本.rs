use std::arch::x86_64::__m128;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
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
        println!("PS {:#?}>", line1);
        let _ = io::stdin().read_line(&mut io);
        mingl = io.trim().to_string().clone();
        let a: Vec<&str> = mingl.split(" ").collect();
        match a[0].trim() {
            "切换目录" | "cd" => {
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
                let mut a: Vec<&str> = mingl.split(" ").collect();
                     for i in &a {
                         let _ = fs::copy(a[1].to_string(), a[2].to_string());
                         println!("{:#?}已保存在{:#?}", a[1].to_string(), a[2].to_string());
                         }
            },
            "\n" => continue,
            "rm" | "删除" | "rmdir" | "remove" => {
                println!("您确定要删除{}吗?(Y/n)", a[1].to_string());
                let mut q = String::new();
                io::stdin().read_line(&mut q).expect("读取确认输入失败");;
                if q.trim() == "Y"{

                    match fs::remove_dir_all(a[1]){
                        Ok(_) => println!("\"{}\"删除成功", a[1].to_string()),
                        Err(_) => { println!("无法删除！");
                                    continue;
                        }
                    }
                }else if q.trim() =="n" {
                    continue;
                }
                else {
                    continue;
                }
            }
            _ => println!("Error: command \"{}\" not found", mingl),
        }
    }

}
