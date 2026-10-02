use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;

pub struct GlobalPath<'global_path>{
    pub home : PathBuf,
    pub ming_l : String,
    pub a : Vec<&'global_path str>,
}

#[derive(Clone)]
pub struct Operation<'operation>{
    pub user_io : Vec<&'operation str>,
    pub home_path: &'operation PathBuf,
}

impl<'operation>Operation<'operation>{
//内置命令
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
                        return ;
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
                    Ok(_) => {
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
            let _ = fs::File::create(user_io[1].to_string());
            println!("文件\"{}\"创建成功", user_io[1].to_string());
        }
        if user_io.len() == 4 {
            if user_io[0] == "type" &&user_io[1] == "nul" && user_io[2] == ">" {
                let temp = fs::File::create(user_io[3].to_string());
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

//外部命令    
    
pub fn ls(&self, _user_io: &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> Result<String, String> {
    let ls = fs::read_dir(".");
        let mut w : String;
    let mut temp: String = String::new();
        match ls {
            Ok(e) => {
                println!("==========文件夹下文件:==========");
                for i in e {
                    match i {
                        Ok(e) => {
                            println!("{}", e.path().display());
                            w = e.path().display().to_string();
                            let str_t: &str = &w.clone();
                            temp.push_str(str_t);
                            temp.push_str("\n");
                            temp.to_string();
                        }
                        Err(_) => {
                            eprintln!("\x1b[1;31m无法读取文件夹，请检查权限和路径\x1b[0m");
                            return Err("无法读取文件夹，请检查权限和路径".to_string())
                        }
                    }
                }
                Ok(temp)
            }
            Err(_) => {
                eprintln!("\x1b[1;31m无法读取文件夹，请检查权限和路径\x1b[0m");
                Err("无法读取文件夹，请检查权限和路径".to_string())
            }
        }
    }

    pub fn echo(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> Result<String, String> {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return Err("命令\"echo\"无效!".to_string());
        }
        let temp: String = user_io[1..].join(" ");
        println!("{}", temp);
        Ok(temp)
    }

    pub fn cat(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> Result<String, String> {
        if user_io.len() < 2 {
            println!("命令\"{}\"无效!", user_io[0].to_string());
            return Err(String::from("命令\"cat\"无效!"))
        }
        if Path::new(user_io[1]).is_file() {
            let q = fs::read_to_string(user_io[1]);
            match &q {
                Ok(q) => {
                    println!("{}", q);
                    Ok(String::from(q))
                },
                Err(_) => {
                    eprintln!("\x1b[1;31m\"{}\"无法打开\x1b[0m", user_io[1].to_string());
                   Err("无法打开指定文件".to_string())
                }
            }
        } else {
            Err("无法打开指定文件".to_string())
        }
    }

    pub fn grep(&self , user_io : &Vec<&str>, _input: &String , _work_path: &mut PathBuf) -> Result<String, String> {
        if user_io.len() <= 2 {
            eprintln!("\x1b[0m命令无效\"{}\"!\x1b[0m", user_io[0].to_string());
            Err("命令\"grep\"无效".to_string())
        }
        else {
            let mut back: String = String::new();
            let temp: String = user_io[2].to_string();
            let files: String;
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
                        eprintln!("\x1b[1;31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
                        Err("没有找到你指定的文字！".to_string())
                    }
                    else {
                        println!("{}", back);
                        Ok(back.to_string())
                    }
                },
                Err(_) => {
                    eprintln!("\x1b[1;31m没有找到\"{}\"\x1b[0m" , user_io[1].to_string());
                    Err("没有找到你指定的文件！".to_string())
                }
            }
        }
    }
//cmd命令
    pub fn x(&self , _user_io : &Vec<&str>, input: &String , _work_path: &mut PathBuf) {
            if let Err(cmd_err) = Command::new("cmd").args(["/c", input.as_str()]).spawn().unwrap().wait() {
                println!("\x1b[1;31m{}\x1b[0m", cmd_err);
            }
        }
}


pub fn cmd_list_pipe() -> HashMap<String, fn(&Operation ,&Vec<&str>, &String ,&mut PathBuf) ->Result<String, String> > {
    let mut cmd_list_pipe: HashMap<String, fn(&Operation, &Vec<&str>, &String, &mut PathBuf) -> Result<String, String>> = HashMap::new();
    cmd_list_pipe.insert("目录下文件".to_string(), |op, _user_io, input, _work_path| op.ls(_user_io, input, _work_path));cmd_list_pipe.insert("ls".to_string() , |op, _user_io, input, _work_path| op.ls(_user_io, input, _work_path));cmd_list_pipe.insert("dir".to_string() , |op, _user_io, input, _work_path| op.ls(_user_io, input, _work_path));
    cmd_list_pipe.insert("echo".to_string(), |op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));cmd_list_pipe.insert("打印".to_string() , |op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));cmd_list_pipe.insert("print".to_string() , |op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));cmd_list_pipe.insert("println".to_string() ,|op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));cmd_list_pipe.insert("printf".to_string() , |op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));cmd_list_pipe.insert("println!".to_string() , |op, _user_io, input, _work_path| op.echo(_user_io, input, _work_path));
    cmd_list_pipe.insert("cat".to_string() , |op, _user_io, input, _work_path| op.cat(_user_io, input, _work_path));cmd_list_pipe.insert("文件内容".to_string() ,|op, _user_io, input, _work_path| op.cat(_user_io, input, _work_path));
    cmd_list_pipe.insert("grep".to_string() , |op, _user_io, input, _work_path| op.grep(_user_io, input, _work_path)); cmd_list_pipe.insert("包含".to_string() , |op, _user_io, input, _work_path| op.grep(_user_io, input, _work_path)); cmd_list_pipe.insert("包括".to_string() ,|op, _user_io, input, _work_path| op.grep(_user_io, input, _work_path)); cmd_list_pipe.insert("Select-String".to_string() ,|op, _user_io, input, _work_path| op.grep(_user_io, input, _work_path)); cmd_list_pipe.insert("sls".to_string() ,|op, _user_io, input, _work_path| op.grep(_user_io, input, _work_path));
    cmd_list_pipe
}