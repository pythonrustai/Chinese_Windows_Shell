use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::Command;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    ascii_picture();
    let mut regedit_ditc: HashMap<String, fn()> = HashMap::new();
    //regedit_ditc.insert("cd" , Operation1.cd());
    loop {
        let home = env::home_dir().unwrap();
        let line = env::current_dir();
        let mut line1: String = String::new();
        let mut mingl: String = String::new();
        let mut guandao: bool = false;
        match line {
            Ok(line) => line1 = line.display().to_string().clone(),
            Err(_) => break,
        }
        let mut io: String = String::new();
        print!("AT {}>", line1);
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut io);
        mingl = io.trim().to_string().clone();
        if mingl.len() == 0 {
            continue;
        }
        let mut ming_line = mingl.clone();
        let a: Vec<&str> = mingl.split_whitespace().collect();
        let mut a_copy = a.clone();

        let Operation1 : Operation = Operation{
            user_io : a_copy,
            home_path : &home,
        };
        let mut Global_Path_1 : GlobalPath = GlobalPath {
            home : env::home_dir().unwrap(),
            ming_l : ming_line.clone(),
            a,
            continue_1: false,
        };
        for i in &Global_Path_1.a {
            if i .to_string()== "|".to_string() {
                guandao = true;
            }
        }
        match Global_Path_1.a[0].trim() {
            "切换目录" | "cd" => {
                Operation1.cd(&Operation1.user_io)
            },
            "目录下文件" | "ls" | "dir" => {
                Operation1.ls();
            },
            "退出" | "exit" | "quit" => break,
            "当前目录" | "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
                continue;
            }
            "复制" | "cp" | "copy" => {
                Operation1.cp(&Operation1.user_io , &mingl);
            },
            "" => continue,
            "rm" | "删除" | "rmdir" | "remove" | "del" | "delete" => {
                Operation1.rm(&Operation1.user_io);
            },
            "echo" | "打印" | "print" | "printf" | "println" | "println!" => {
                Operation1.echo(&Operation1.user_io);
            },
            "mkdir" | "mk" | "新建文件夹" | "新文件夹" | "创建目录" => {
                Operation1.mkdir(&Operation1.user_io , &mut Global_Path_1);
            },
            "touch" | "New-Item" | "ni" | "新建文件" | "新文件" | "创建文件" | "type" => {
                Operation1.touch(&Operation1.user_io);
            },
             "文件内容" | "cat" => {
                Operation1.cat(&Operation1.user_io);
            },
                                                                                                    "grep" | "包含`" | "包括" => {
                Operation1.grep(&Operation1.user_io);
            },
            _ => {
                Operation1.x(Global_Path_1.ming_l);
            }
        }
        if guandao {}
    }
}

fn ascii_picture() {
    println!("     _      _   _   _____   _____ ");
    println!("    / \\    | \\ | | |_   _| | ____|");
    println!("   / _ \\   |  \\| |   | |   |  _|");
    println!("  / ___ \\  | |\\  |   | |   | |___");
    println!(" /_/   \\_\\ |_| \\_|   |_|   |_____|\n");
    println!("                    ____    _   _   _____   _       _     ");
    println!("                   / ___|  | | | | | ____| | |     | |    ");
    println!("                   \\___ \\  | |_| | |  _|   | |     | |");
    println!("                    ___) | |  _  | | |___  | |___  | |___");
    println!("                   |____/  |_| |_| |_____| |_____| |_____|\n");
}
struct GlobalPath<'global_path>{
    home : PathBuf,
    ming_l : String,
    a : Vec<&'global_path str>,
    continue_1: bool,
}

struct Grep {
    exit_code : u8,
    word : String,
}

struct Operation<'operation>{
    user_io : Vec<&'operation str>,
    home_path: &'operation PathBuf,
}
impl<'operation>Operation<'operation>{
    fn cd (&self , user_io : &Vec<&str>) {
        if user_io.len() == 1 {
            let mut  temp = env::home_dir();
            match &mut temp {
                Some(_) => {
                },
                None => {}
            }
           match env::set_current_dir(&self.home_path) {
               Ok(_) => {},
               Err(q) => {
                   eprintln!("切换目录失败：{}", q);
                   return;
               },
           }
        } else {
            let full_path = user_io[1..].join(" ");
            match env::set_current_dir(Path::new(&full_path)) {
                Ok(_) => {}
                Err(q) => {
                    eprintln!("切换目录失败：{}", q);
                    return;
                }
            }
        }
    }

    fn ls(&self) -> String {
        let ls = fs::read_dir(".");
        let mut w : String = String::new();
        match ls {
            Ok(e) => {
                println!("==========文件夹下文件:==========");
                for i in e {
                    match i {
                        Ok(e) => {
                            println!("{}", e.path().display());
                            w = e.path().display().to_string().clone();

                        }
                        Err(_) => {
                            eprintln!("无法读取文件夹，请检查权限和路径");
                            return Default::default();}
                    }
                }
                w
            }
            Err(_) => {
                eprintln!("无法读取文件夹，请检查权限和路径");
                Default::default()
            }
        }
    }

    fn cp(&self , user_io : &Vec<&str> , mingl : &String) {
        let s: Vec < &str > = mingl.split_whitespace().collect();
        if user_io.len() < 3 {
            eprintln!("命令\"{}\"无效!", s[0].to_string());
            return;
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
                return;
            }
        }
    }

    fn rm(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            eprintln!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        println!("您确定要删除{}吗?(Y/n)", user_io[1].to_string());
        let mut temp = String::new();
        let _ = io::stdin().read_line(&mut temp);
        if temp.trim() == "Y" {
            let mut temp1: bool = true;
            if Path::is_dir(Path::new(user_io[1])) {
                match fs::remove_dir_all(user_io[1]) {
                    Ok(_) => {},
                    Err(_) => {
                        println!("文件夹\"{}\"无法删除!", user_io[1].to_string());
                        temp1 = false;
                    }
                }
                if temp1{
                    println!("成功删除文件夹\"{}\",祝你好运!", user_io[1].to_string());
                }
            } else if Path::is_file(Path::new(user_io[1])) {
               match fs::remove_file(user_io[1]) {
                   Ok(_) =>{} ,
                   Err(_) => { println!("文件\"{}\"", user_io[1].to_string());
                       temp1 = false;
                   },
               }
                if temp1{
                    println!("成功删除文件\"{}\",祝你好运!", user_io[1].to_string());
                }
            } else {
                println!("\"{}\" 不存在或无法访问", user_io[1]);
            }
        }
    }

    fn echo(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        let temp: String = user_io[1..].join(" ");
        println!("{:#}", temp.as_str());
    }

    fn mkdir(&self , user_io : &Vec<&str> , GlobalPath_1 : &mut GlobalPath) {
        if user_io.len() >= 4 {
            if user_io[0] == "type" &&user_io[1] == "nul" && user_io[2] == ">" {
                let temp = fs::create_dir(user_io[3]);
                match &temp {
                    Ok(_) => {
                        println!("{}", user_io[3]);
                    },
                    Err(_) => {
                        println!("\"{}\"无法创建", user_io[3].to_string());
                        return;
                    }
                }
            }
        }
        let mut hhome = env::current_dir();
        let mut hhhome: PathBuf;
        match hhome {
            Ok(q) => { hhhome = q;},
            Err(_) => {
                eprintln!("警告：创建目录后无法回到原目录!会回到用户目录!");
                match env::home_dir() {
                    Some(q) => hhhome = q,
                    None => {
                        eprintln!("无法获取用户目录，操作取消");
                        return;
                    }
                };
            }
        };
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        if user_io.len() == 2 {
            match fs::create_dir(user_io[1]) {
                Ok(_) => {
                    println!("文件夹\"{}\"创建成功", user_io[1].to_string());
                },
                Err(_) => {
                    eprintln!("文件夹\"{}\"创建失败", user_io[1].to_string());
                    return;}
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
                        return;
                    }
                }
                let _ = env::set_current_dir(Path::new(&user_io[m]));
                m += 1;
            }
        }
        match env::set_current_dir(&hhhome) {
            Ok(_) => {GlobalPath_1.home = hhhome;}
            Err(q) => {
                eprintln!("无法回到上级目录：{}!", q);
                return;
            }
        }
    }

    fn touch(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            eprintln!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        if user_io[0] == "touch" || user_io[0] == "新建文件" {
            let _ = fs::File::create(user_io[1]);
        }
    }

    fn cat(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        let q = fs::read_to_string(user_io[2]);
        match &q {
            Ok(q) => { println!("{}", q);},
            Err(_) => {println!("\"{}\"无法打开", user_io[2].to_string());
            return;
            }
        }
        return;
    }

    fn grep(&self , user_io : &Vec<&str>) -> Grep{
        if user_io.len() <= 2 {
            eprintln!("命令无效!");
            let GrepErr: Grep = Grep{
                exit_code : 0 ,
                word : String::new() ,
            };
            GrepErr
        }
        else {
            let mut back: String = String::new();
            let temp: String = user_io[2].to_string();
            let mut files: String = String::new();
            match fs::read_to_string(&temp) {
                Ok(q) => {
                    files = q;
                    let ffiles: Vec<String> = files.split("\n").map(|ffiles| ffiles.to_string()).collect();
                    for i in &ffiles {
                        if i.contains(&user_io[1].to_string()) {
                            back.push_str(i);
                            back.push_str("\n");
                        }
                    }
                    if back == String::new() {
                        let GrepNew: Grep = Grep{
                            exit_code : 1 ,
                            word : String::new() ,
                        };
                        println!("没有找到\"{}\"" , user_io[1].to_string());
                        GrepNew
                    }
                    else {
                        let GrepYes: Grep = Grep{
                            word : back,
                            exit_code : 0 ,
                        };
                        println!("{}", GrepYes.word);
                        GrepYes
                    }
                },
                Err(_) => {
                    println!("命令\"{}\"无效!", user_io[0].to_string());
                    let GrepErr: Grep = Grep{
                        exit_code : 0 ,
                        word : String::new() ,
                    };
                    println!("没有找到\"{}\"" , user_io[1].to_string());
                    GrepErr
                }
            }
        }
    }

    fn x(&self , mingl : String) {
        let mut temp: String = String::new();
        for i in mingl.chars() {
            temp.push(i);
        }

            if Path::new(&temp).exists() {
                let _ = Command::new("cmd").args(["/c", temp.as_str()]).spawn();
        }
    }
}