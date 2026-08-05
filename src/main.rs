use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::Command;
use std::io::Write;

fn main() {
    loop {
        let mut continue_1: bool = false;
        let home = env::home_dir().unwrap();
        let line = env::current_dir();
        let mut line1: String = String::new();
        let mut mingl: String = String::new();
        match line {
            Ok(line) => line1 = line.display().to_string().clone(),
            Err(_) => break,
        }
        let mut io: String = String::new();
        print!("PS {}>", line1);
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut io);
        mingl = io.trim().to_string().clone();
        if mingl.len() == 0 {continue;}
        let a: Vec<&str> = mingl.split_whitespace().collect();
        let mut a_copy = a.clone();
        let Operation1 : Operation = Operation{
            user_io : a_copy,
            home_path : &home,
        };
        match a[0].trim() {
            "切换目录" | "cd" => {
                Operation1.cd(&Operation1.user_io,continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            "目录下文件" | "ls" | "dir" => {
                Operation1.ls(continue_1);
                if continue_1 == true {
                    continue;
                }
            }
            "退出" | "exit" | "quit" => break,
            "当前目录" | "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
                continue;
            }
            "复制" | "cp" | "copy" => {
                Operation1.cp(&Operation1.user_io , &mingl , continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            "" => continue,
            "rm" | "删除" | "rmdir" | "remove" | "del" | "delete" => {
                Operation1.rm(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            }
            "echo" | "打印" | "print" | "printf" | "println" | "println!" => {
                Operation1.echo(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            "mkdir" | "mk" | "新建文件夹" | "新文件夹" | "创建目录" => {
                Operation1.mkdir(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            "touch" | "New-Item" | "ni" | "新建文件" | "新文件" | "创建文件" => {
                Operation1.touch(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            "type" | "文件内容" | "cat" => {
                Operation1.cat(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            },
            _ => {
                Operation1.x(&Operation1.user_io , continue_1);
                if continue_1 == true {
                    continue;
                }
            }
        }
    }
}

struct Operation<'operation>{
    user_io : Vec<&'operation str>,
    home_path: &'operation PathBuf,
}
impl<'operation>Operation<'operation>{
    fn cd (&self , user_io : &Vec<&str>,mut continue_1 : bool) {
        if user_io.len() == 1 {
            let mut  temp = env::home_dir();
            let mut temp1 = String::new();
            match &mut temp {
                Some(temp) => {
                },
                None => {}
            }
           match env::set_current_dir(&self.home_path) {
               Ok(_) => {},
               Err(q) => {
                   eprintln!("切换目录失败：{}", q);
                   continue_1 = true;
               },
           };
        } else {
            let full_path = user_io[1..].join(" ");
            match env::set_current_dir(Path::new(&full_path)) {
                Ok(_) => {}
                Err(q) => {
                    eprintln!("切换目录失败：{}", q);
                    continue_1 = true;
                }
            }
        }
    }

    fn ls(&self,mut continue_1 : bool) {
        let ls = fs::read_dir(".");
        match ls {
            Ok(e) => {
                println!("==========文件夹下文件:==========");
                for i in e {
                    let en = i.unwrap();
                    println!("{}", en.file_name().to_string_lossy());
                }
            }
            Err(_) => {eprintln!("无法读取文件夹，请检查权限和路径");
                continue_1 = true;
            }
        }
    }

    fn cp(&self , user_io : &Vec<&str> , mingl : &String , mut continue_1 : bool) {
        let s: Vec < &str > = mingl.split_whitespace().collect();
        if user_io.len() < 3 {
            eprintln!("命令\"{}\"无效!", s[0].to_string());
            continue_1 = true;
        }
        match fs::copy(s[1].to_string(), s[2].to_string()) {
            Ok(_) =>{
                if Path::new(s[1]).is_dir() {
                    eprintln!("你的复制对象\"{}\"是一个文件夹", s[1].to_string());
                }

                println!("{}已保存在{}", s[1].to_string(), s[2].to_string())
            },
            Err(_) => {
                println!("无法复制{}", s[1].to_string());
                continue_1 = true;
            }
        }
    }

    fn rm(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        if user_io.len() < 2 {
            eprintln!("命令\"{}\"无效!", user_io[0].to_string());
            continue_1 = true;
        }
        println!("您确定要删除{}吗?(Y/n)", user_io[1].to_string());
        let mut temp = String::new();
        let _ = io::stdin().read_line(&mut temp).expect("读取确认输入失败");
        if temp.trim() == "Y" {
            if Path::is_dir(std::path::Path::new(user_io[1])) {
                fs::remove_dir_all(user_io[1]).unwrap();
                println!("成功删除文件夹\"{}\",祝你好运!", user_io[1].to_string());
            } else if Path::is_file(std::path::Path::new(user_io[1])) {
                fs::remove_file(user_io[1]).unwrap();
                println!("成功删除文件\"{}\",祝你好运!", user_io[1].to_string());
            }
        }
    }

    fn echo(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            continue_1 = true;
        }
        let temp: String = user_io[1..].join(" ");
        println!("{:#}", temp.as_str());
    }

    fn mkdir(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            continue_1 = true;
        }
        if user_io.len() == 2 {
            match fs::create_dir(user_io[1]) {
                Ok(_) => {
                    println!("文件夹\"{}\"创建成功", user_io[1].to_string());
                },
                Err(_) => {
                    eprintln!("文件夹\"{}\"创建失败", user_io[1].to_string());
                    continue_1 = true;}
            };
        }
        if user_io.len() > 2 {
            let temp: usize = user_io.len();
            let mut m: usize = 1;
            while m < temp {
                match fs::create_dir(user_io[m].to_string()) {
                    Ok(_) => {
                        let _ = fs::create_dir(user_io[m].to_string());
                        println!("文件夹\"{}\"创建成功", user_io[m].to_string());
                    },
                    Err(_) => {
                        eprintln!("文件夹\"{}\"创建失败", user_io[m].to_string());
                        continue_1 = true;
                    }
                }
                let _ = env::set_current_dir(Path::new(&user_io[m]));
                m += 1;
            }
        }
    }

    fn touch(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        if user_io.len() < 2 {
            eprintln!("命令\"{}\"无效!", user_io[0].to_string());
            continue_1 = true;
        }
        if user_io[0] == "touch" || user_io[0] == "新建文件" {
            let _ = fs::File::create(user_io[1]);
        }
    }

    fn cat(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            continue_1 = true;
        }
        if user_io[1] == "nul" && user_io[2] == ">" {
            let _ = fs::File::create(user_io[3]);
        }
        let q = fs::read_to_string(user_io[1]);
        match &q {
            Ok(q) => { println!("{}", q);},
            Err(_) => {println!("\"{}\"无法打开", user_io[1].to_string());
            continue_1 = true;
            }
        }
    }

    fn x(&self , user_io : &Vec<&str> , mut continue_1 : bool) {
        let temp = &user_io[0];
            if std::path::Path::new(&temp).exists() {
                let q= Command::new("cmd").args(["/c", user_io[0]]).spawn();
                match &q {
                    Ok(q) => {},
                    Err(_) => {
                        eprintln!("Error:\"{}\"command not found!\n错误:命令\"{}\"不是命令或可执行文件!", user_io[0], user_io[0]);
                        continue_1 = true;
                    }
                }
        }
    }
}
