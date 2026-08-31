use std::{env, fs, io};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GlobalPath<'global_path>{
    pub home : PathBuf,
    pub ming_l : String,
    pub a : Vec<&'global_path str>,
}

pub struct Grep {
    pub exit_code : u8,
    pub word : String,
}

pub struct Operation<'operation>{
    pub user_io : Vec<&'operation str>,
    pub home_path: &'operation PathBuf,
}
impl<'operation>Operation<'operation>{
    pub fn cd (&self, user_io : &Vec<&str> ,_input: &String , work_path: &mut PathBuf) {
        if user_io.len() == 1 {
            let mut temp = env::home_dir();
            match &mut temp {
                Some(_) => {
                    match env::set_current_dir(&self.home_path) {
                    Ok(_) => {
                        *work_path = env::home_dir().unwrap();
                    },
                    Err(q) => {
                        eprintln!("\x1b[1;31m切换目录失败：{}\x1b[0m", q);
                        return;
                    },
                }
                },
                None => {
                    eprintln!("\x1b[1;31m切换目录失败：\x1b[0m");
                    return;}
            }

        } else {
            if user_io[1] == "~" {
                let mut temp = env::home_dir();
                match &mut temp {
                    Some(_) => {
                        *work_path = env::home_dir().unwrap();
                    },
                    None => {
                        eprintln!("\x1b[1;31m切换目录失败\x1b[0m",);
                        return;
                    }
                }
            }
            else {
                let full_path = user_io[1..].join(" ");
                match env::set_current_dir(Path::new(&full_path)) {
                    Ok(q) => {
                        *work_path = PathBuf::from(user_io[1..].join(" "));
                    },
                    Err(q) => {
                        eprintln!("\x1b[1;31m切换目录失败：{}\x1b[0m", q);
                        return;
                    }
                }
            }
        }
    }

pub fn ls(&self, _user_io: &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> String {
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
                            eprintln!("\x1b[1;31m无法读取文件夹，请检查权限和路径\x1b[0m");
                            return Default::default();}
                    }
                }
                w
            }
            Err(_) => {
                eprintln!("\x1b[1;31m无法读取文件夹，请检查权限和路径\x1b[0m");
                Default::default()
            }
        }
    }

    pub fn cp(&self , user_io : &Vec<&str> , input : &String , _work_path: &mut PathBuf) {
        let s: Vec <&str> = input.split_whitespace().collect();
        if user_io.len() < 3 {
            eprintln!("\x1b[1;31m命令\"{}\"无效!\x1b[0m", s[0].to_string());
            return;
        }
        if Path::new(s[1]).is_dir() {
            eprintln!("\x1b[1;31m你的复制对象\"{}\"是一个文件夹\x1b[0m", s[1].to_string());
            return;
        }
        match fs::copy(s[1].to_string(), s[2].to_string()) {
            Ok(_) =>{
                println!("\x1b[1;32m{}已保存在{}", s[1].to_string(), s[2].to_string())
            },
            Err(_) => {
                eprintln!("\x1b[1;31m无法复制{}", s[1].to_string());
                return;
            }
        }
    }

    pub fn rm(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
        if user_io.len() < 2 {
            eprintln!("\x1b[1;31m命令\"{}\"无效!\x1b[0m", user_io[0].to_string());
            return;
        }
        println!("\x1b[1;31m您确定要删除{}吗?(Y/n)", user_io[1].to_string());
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("");
        if temp.trim() == "Y" {
            let mut temp1: bool = true;
            if Path::is_dir(Path::new(user_io[1])) {
                match fs::remove_dir_all(user_io[1]) {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("\x1b[1;31m文件夹\"{}\"无法删除!\x1b[0m", user_io[1].to_string());
                        temp1 = false;
                    }
                }
                if temp1{
                    println!("\x1b[1;32m成功删除文件夹\"{}\",祝你好运!\x1b[0m", user_io[1].to_string());
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

    pub fn echo(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        let temp: String = user_io[1..].join(" ");
        println!("{}", temp);
    }

    pub fn mkdir(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
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
                    eprintln!("\x1b[1;31m文件夹\"{}\"创建失败\x1b[0m", user_io[i].to_string());
                    return;}
            }
        }
    }

    pub fn touch(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
        if user_io.len() < 2 {
            eprintln!("\x1b[1;31m命令\"{}\"无效!\x1b[0m", user_io[0]);
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

    pub fn cat(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return;
        }
        if Path::new(user_io[1]).is_file() {
            let q = fs::read_to_string(user_io[1]);
            match &q {
                Ok(q) => {
                    eprintln!("\x1b[1;31m{}\x1b[0m", q);
                },
                Err(_) => {
                    eprintln!("\x1b[1;31m\"{}\"无法打开\x1b[0m", user_io[1].to_string());
                    return;
                }
            }
            return;
        }
    }

    pub fn grep(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> Grep{
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
                        eprintln!("\x1b[1;31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
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
                    eprintln!("\x1b[1;31m命令\"{}\"无效!\x1b[0m", user_io[0].to_string());
                    let grep_err: Grep = Grep{
                        exit_code : 0 ,
                        word : String::new() ,
                    };
                    eprintln!("\x1b[1;31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
                    grep_err
                }
            }
        }
    }

    pub fn x(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) {
        let mut temp: String = String::new();
        for i in user_io[0].chars() {
            temp.push(i);
        }
        if Path::new(&temp).exists() {
            let _ = Command::new("cmd").args(["/c", temp.as_str()]).spawn();
        }
        else {
            eprintln!("\x1b[1;31m未找到命令:\"{}\",此命令无效!\x1b[0m", &user_io[0]);
        }
    }
}

fn cmd_list_pipe() -> HashMap<String, fn(&Operation ,&Vec<&str>, &String ,&mut PathBuf)> {
    let mut cmd_list_pipe: HashMap<String, fn(&Operation ,user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf)> = HashMap::new();
    cmd_list_pipe.insert("切换目录".to_string(), Operation::ls);
    cmd_list_pipe
}