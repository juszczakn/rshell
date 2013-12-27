use std::os::env;
use std::io;
use std::io::buffered::BufferedReader;
use std::run::{Process, ProcessOptions, ProcessOutput};

/* Compiles and tested with rust-0.9pre.
A very basic shell program which allows for changing directories */

fn get_working_dir(old_dir: bool) -> ~str {
    let elts: ~[(~str,~str)] = env();
    
    for elt in elts.iter() {
        let (ref s1, ref s2): (~str,~str) = *elt;
        if old_dir {
            if *s1 == ~"OLDPWD" {
                return s2.clone()
            }
        } else {
            if *s1 == ~"PWD" {
                return s2.clone()
            }
        }
    }
    return ~""
}

fn set_working_dir(d: &str) {
    if d.len() == 0 {
        return
    }
    let old_pwd: &str = get_working_dir(false);
    let mut new_pwd: Path = Path::new(old_pwd);

    // 45 == '-', go to old_pwd
    if d[0] == 45 {
        new_pwd = Path::new(get_working_dir(true));
        // 47 == '/', absolute path
    } else if d[0] == 47 {
        new_pwd = Path::new(d);
    } else {
        new_pwd.push(d);
    }

    let n: Option<&str> = new_pwd.as_str();
    if n.is_some() {
        std::os::setenv(&"PWD", n.unwrap());
        std::os::setenv(&"OLDPWD", old_pwd);
    }
}

fn create_cmd(cmd: &str) -> ~[~str]{
    let mut cmd_arr: ~[~str] = ~[];

    for word in cmd.words() {
        let s: ~str = word.into_owned();
        cmd_arr.push(s);
    }
    cmd_arr
}

fn create_process(s: &str) -> Option<Process> {
    let line: ~[~str] = create_cmd(s);
    if line.len() == 0 {
        return None
    }

    let cmd: &str = line[0];
    let mut args: &[~str] = &[];
    if line.len() > 1 {
        args = line.slice(1, line.len());
    }
    
    if cmd == "cd" {
        let mut cd_dir: Path = Path::new("/home/nick");
        if args.len() > 0 {
            let mut new_path = args[0].clone();
            if new_path[0] == 45 {
                new_path = get_working_dir(true);
            }
            cd_dir = Path::new(new_path);
        }
        let ret_val: bool = std::os::change_dir(&cd_dir);

        let dir: Option<&str> = cd_dir.as_str();
        if ret_val == false {
            if dir.is_some() {
                println(format!("Error: '{}' not a valid path.",
                                dir.unwrap()));
            }
        } else {
            if dir.is_some() {
                set_working_dir(dir.unwrap());
            }
        }
        return None
    }

    let opts: ProcessOptions = ProcessOptions::new();
    let launch: Option<Process> = Process::new(cmd, args, opts);
    launch
}

fn handle_process(new_proc: ~Process) -> bool {
    let mut new_proc = new_proc;
    let proc_out: ProcessOutput = new_proc.finish_with_output();
    let output: &str = std::str::from_utf8(proc_out.output);
    let error: &str = std::str::from_utf8(proc_out.error);
    if output.len() != 0 {
        println(format!("{}", output));
    }
    if error.len() != 0 {
        println(format!("{}", error));
    }
    if !proc_out.status.success() {
        return false
    }
    true
}

fn read_stdin() {
    let mut reader = BufferedReader::new(io::stdin());
    while !reader.eof() {
        let line = reader.read_line();
        if line.is_some(){
            let line: ~str = line.unwrap();
            if line == ~"exit\n" {
                return
            }
            let new_proc: Option<Process> = create_process(line);
            if new_proc.is_some() {
                handle_process(~(new_proc.unwrap()));
            }
            print(format!("{}$> ", get_working_dir(false)));
            std::io::stdio::flush();
        }
    }
}

fn main() {
    print(format!("{}$> ", get_working_dir(false)));
    std::io::stdio::flush();
    read_stdin();
}
