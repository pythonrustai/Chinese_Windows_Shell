use std::{env, fs};
use std::path::PathBuf;
use std::io;
use std::io::Write;
use shell::{Operation, GlobalPath, cmd_list_pipe};

fn main() {
    let pipe_list = cmd_list_pipe();
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
        let mut ming_line = input.clone();
        let mut work_path: PathBuf = {
            env::current_dir().unwrap()
        };
        print!("AT {}@{}:{}> ", env::var("COMPUTERNAME").unwrap(), env::var("USERNAME").unwrap(), &work_path.display());
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut io);
        input = io.trim().to_string().clone();
        let a: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();
        let mut a_copy = a.clone();
        let a_copy_copy = a_copy.clone();
        let mut operation1: Operation = Operation{
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
        // if input.is_empty() {
        //     continue;
        // }
        if global_path_1.a.is_empty() {
            continue;
        }
        match &global_path_1.a[0].trim() {
            &"切换目录" | &"cd" => {
                operation1.cd(&operation1.user_io, &input ,&mut work_path);
            },
            &"目录下文件" | &"ls" | &"dir" => {
                operation1.ls(&operation1.user_io, &input ,&mut work_path);
            },
            &"退出" | &"exit" | &"quit" => break,
            &"当前目录" | &"pwd" => {
                println!("{}", env::current_dir().unwrap().display());
                continue;
            }
            &"复制" | &"cp" | &"copy" => {
                operation1.cp(&operation1.user_io , &input ,&mut work_path);
            },
            &"" => continue,
            &"rm" | &"删除" | &"rmdir" | &"remove" | &"del" | &"delete" => {
                operation1.rm(&operation1.user_io, &input ,&mut work_path);
            },
            &"echo" | &"打印" | &"print" | &"printf" | &"println" | &"println!" => {
                operation1.echo(&operation1.user_io , &input ,&mut work_path);
            },
            &"mkdir" | &"mk" | &"新建文件夹" | &"新文件夹" | &"创建目录" => {
                operation1.mkdir(&operation1.user_io , &input ,&mut work_path);
            },
            &"touch" | &"New-Item" | &"ni" | &"新建文件" | &"新文件" | &"创建文件" | &"type" => {
                operation1.touch(&operation1.user_io , &input ,&mut work_path);
            },
            &"文件内容" | &"cat" => {
                operation1.cat(&operation1.user_io ,&input ,&mut work_path);
            },
            &"grep" | &"包含" | &"包括" | &"Select-String" | &"sls" => {
                operation1.grep(&operation1.user_io, &input ,&mut work_path);
            },
            _ => {
                operation1.x(&operation1.user_io , &input ,&mut work_path);
            }
        }
        if guandao {
            let mut usi: usize = 0usize;
            let mut a_copy1 = Vec::new();
            let mut a_copy2 = Vec::new();
            for i in &a_copy_copy {
                usi+=1;
                if i == &"|" {
                    usi += 1;
                    a_copy1 = a_copy_copy[0..usi - 1usize].to_vec();
                    a_copy2 = a_copy_copy[usi..].to_vec();
                }
            }
            let pipe_ham = pipe_list.get(&a_copy1[0].to_string());
            match pipe_ham {
                Some(e) => {
                    let the_pipe1: Result<String, String> = e(&operation1 ,&a_copy1 ,&input ,&mut work_path);
                    match the_pipe1 {
                        Ok(t) => {
                            let env_temp: String = env::var("TEMP").unwrap();
                            let TEMP_TES: String = env_temp + "temptemptemp.txt";
                            fs::File::create(&TEMP_TES).unwrap();
                            let sss = t.clone();
                            fs::write(TEMP_TES, t).unwrap();
                            println!("{}", &sss);
                            let pipe_ham_2 = pipe_list.get(&a_copy2[0].to_string());
                            match pipe_ham_2 {
                                Some(e) => {
                                    let the_pipe1: Result<String, String> = e(&operation1,&a_copy2 ,&sss ,&mut work_path);
                                    match the_pipe1 {
                                        Ok(t) => {
                                            println!("{}", t);
                                        },
                                        Err(err) => {
                                            println!("第一个命令报错，原因：{}", err);
                                        }
                                    }
                                }
                                None => {
                                }
                            }
                        }
                        Err(err) => {
                            println!("第一个命令报错，原因：{}", err);
                        }
                    }
                },
                None => {
                },
            };
        }
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