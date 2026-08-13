use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::Command;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    ascii_picture();
    let home = env::home_dir().unwrap();
    let line = env::current_dir();
    loop {
        let mut line1: String = String::new();
        let mut input: String = String::new();
        let mut guandao: bool = false;
        match &line {
            Ok(line) => line1 = line.display().to_string().clone(),
            Err(_) => break,
        }
        let mut io: String = String::new();
        print!("AT {}>", line1);
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut io);
        input = io.trim().to_string().clone();
        if input.len() == 0 {
            continue;
        }
        let mut ming_line = input.clone();
        let a: Vec<&str> = input.split_whitespace().collect();
        let mut a_copy = a.clone();

        let operation1 : Operation = Operation{
            user_io : a_copy,
            home_path : &home,
        };
        let mut global_path_1 : GlobalPath = GlobalPath {
            home : env::home_dir().unwrap(),
            ming_l : ming_line.clone(),
            a,
        };
        //let mut  regedit_dict: HashMap<String, fn()> = HashMap::new();
        //regedit_dict.insert("cd".to_string() , operation1.cd);
        for i in &global_path_1.a {
            if i .to_string()== "|".to_string() {
                guandao = true;
            }
        }
        match global_path_1.a[0].trim() {
            "切换目录" | "cd" => {
                operation1.cd(&operation1.user_io)
            },
            "目录下文件" | "ls" | "dir" => {
                operation1.ls();
            },
            "退出" | "exit" | "quit" => break,
            "当前目录" | "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
                continue;
            }
            "复制" | "cp" | "copy" => {
                operation1.cp(&operation1.user_io , &input);
            },
            "" => continue,
            "rm" | "删除" | "rmdir" | "remove" | "del" | "delete" => {
                operation1.rm(&operation1.user_io);
            },
            "echo" | "打印" | "print" | "printf" | "println" | "println!" => {
                operation1.echo(&operation1.user_io);
            },
            "mkdir" | "mk" | "新建文件夹" | "新文件夹" | "创建目录" => {
                operation1.mkdir(&operation1.user_io);
            },
            "touch" | "New-Item" | "ni" | "新建文件" | "新文件" | "创建文件" | "type" => {
                operation1.touch(&operation1.user_io);
            },
             "文件内容" | "cat" => {
                operation1.cat(&operation1.user_io);
            }, 
            "grep" | "包含`" | "包括" => {
                operation1.grep(&operation1.user_io);
            },
            _ => {
                operation1.x(global_path_1.ming_l);
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
                   eprintln!("\x1b[31m切换目录失败：{}\x1b[0m", q);
                   return;
               },
           }
        } else {
            let full_path = user_io[1..].join(" ");
            match env::set_current_dir(Path::new(&full_path)) {
                Ok(_) => {}
                Err(q) => {
                    eprintln!("\x1b[31m切换目录失败：{}\x1b[0m", q);
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
                            eprintln!("\x1b[31m无法读取文件夹，请检查权限和路径\x1b[0m");
                            return Default::default();}
                    }
                }
                w
            }
            Err(_) => {
                eprintln!("\x1b[31m无法读取文件夹，请检查权限和路径\x1b[0m");
                Default::default()
            }
        }
    }

    fn cp(&self , user_io : &Vec<&str> , input : &String) {
        let s: Vec < &str > = input.split_whitespace().collect();
        if user_io.len() < 3 {
            eprintln!("\x1b[31m命令\"{}\"无效!\x1b[0m", s[0].to_string());
            return;
        }
        if Path::new(s[1]).is_dir() {
            eprintln!("\x1b[31m你的复制对象\"{}\"是一个文件夹\x1b[0m", s[1].to_string());
            return;
        }
        match fs::copy(s[1].to_string(), s[2].to_string()) {
            Ok(_) =>{
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
            eprintln!("\x1b[31m命令\"{}\"无效!\x1b[0m", user_io[0].to_string());
            return;
        }
        println!("\x1b[31m您确定要删除{}吗?(Y/n)", user_io[1].to_string());
        let mut temp = String::new();
        let _ = io::stdin().read_line(&mut temp);
        if temp.trim() == "Y" {
            let mut temp1: bool = true;
            if Path::is_dir(Path::new(user_io[1])) {
                match fs::remove_dir_all(user_io[1]) {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("\x1b[31m文件夹\"{}\"无法删除!\x1b[0m", user_io[1].to_string());
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
        println!("{}", temp);
    }

    fn mkdir(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0]);
            return;
        }
        for i in 1..user_io.len() {
            match fs::create_dir_all(user_io[i]) {
                Ok(_) => {
                    if user_io[1].contains("\\") {
                        println!("嵌套文件夹\"{}\"", user_io[1].to_string());
                    }
                    println!("文件夹\"{}\"创建成功", user_io[i].to_string());
                },
                Err(_) => {
                    eprintln!("\x1b[31m文件夹\"{}\"创建失败\x1b[0m", user_io[i].to_string());
                    return;}
            }
        }
    }

    fn touch(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            eprintln!("\x1b[31m命令\"{}\"无效!\x1b[0m", user_io[0]);
            return;
        }
        if user_io[0] == "touch" || user_io[0] == "新建文件" || user_io[0] == "ni"{
            let _ = fs::File::create(user_io[1]);
            println!("文件\"{}\"创建成功", user_io[1]);
        }
        if user_io.len() == 4 {
            if user_io[0] == "type" &&user_io[1] == "nul" && user_io[2] == ">" {
                let temp = fs::File::create(user_io[3]);
                match &temp {
                    Ok(_) => {
                        println!("文件\"{}\"创建成功", user_io[3]);
                    },
                    Err(_) => {
                        println!("文件\"{}\"无法创建", user_io[3].to_string());
                        return;
                    }
                }
            }
        }
    }

    fn cat(&self , user_io : &Vec<&str>) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        if Path::new(user_io[1]).is_file() {
            let q = fs::read_to_string(user_io[1]);
            match &q {
                Ok(q) => {
                    eprintln!("\x1b[31m{}\x1b[0m", q);
                },
                Err(_) => {
                    eprintln!("\x1b[31m\"{}\"无法打开\x1b[0m", user_io[1].to_string());
                    return;
                }
            }
            return;
        }
    }

    fn grep(&self , user_io : &Vec<&str>) -> Grep{
        if user_io.len() <= 2 {
            eprintln!("\x1b[0m命令无效\"{}\"!\x1b[0m", user_io[0].to_string());
            let grep_err: Grep = Grep{
                exit_code : 0 ,
                word : String::new() ,
            };
            grep_err
        }
        else {
            let mut back: String = String::new();
            let temp: String = user_io[2].to_string();
            let mut files: String = String::new();
            match fs::read_to_string(&temp) {
                Ok(q) => {
                    files = q;
                    let files_vec: Vec<String> = files.split("\n").map(|files_vec| files_vec.to_string()).collect();
                    for i in &files_vec {
                        if i.contains(&user_io[1].to_string()) {
                            back.push_str(i);
                            back.push_str("\n");
                        }
                    }
                    if back == String::new() {
                        let grep_new: Grep = Grep{
                            exit_code : 1 ,
                            word : String::new() ,
                        };
                        eprintln!("\x1b[31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
                        grep_new
                    }
                    else {
                        let grep_yes: Grep = Grep{
                            word : back,
                            exit_code : 0 ,
                        };
                        println!("{}", grep_yes.word);
                        grep_yes
                    }
                },
                Err(_) => {
                    eprintln!("\x1b[31m命令\"{}\"无效!\x1b[0m", user_io[0].to_string());
                    let grep_err: Grep = Grep{
                        exit_code : 0 ,
                        word : String::new() ,
                    };
                    eprintln!("\x1b[31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
                    grep_err
                }
            }
        }
    }

    fn x(&self , input : String) {
        let mut temp: String = String::new();
        for i in input.chars() {
            temp.push(i);
        }
        if Path::new(&temp).exists() {
            let _ = Command::new("cmd").args(["/c", temp.as_str()]).spawn();
        }
        else {
            eprintln!("\x1b[31m未找到命令:\"{}\",此命令无效!\x1b[0m", &input);
        }
    }
}