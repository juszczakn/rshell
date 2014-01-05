#[crate_id(name="rshell", vers="0.0.1", author="Nicholas Juszczak")];

/* This program is distributed under the MIT license.
Author: Nicholas Juszczak <juszczakn@gmail.com> */

use std::os::env;
use std::{io, task};
use std::io::buffered::BufferedReader;
use std::run::{Process, ProcessOptions, ProcessOutput};

/* Compiles and tested with rust-0.9pre.
A very basic shell program which allows for changing directories */

enum DirType {
    Home,
    Pwd,
    OldPwd
}

fn main() {
    print(format!("{}$> ", get_directory(Pwd)));
    std::io::stdio::flush();
    read_stdin();
    std::os::set_exit_status(0);
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
            create_process(line);
            print(format!("{}$> ", get_directory(Pwd)));
            std::io::stdio::flush();
        }
    }
}

/* Create a process to run given a line of input of 
the form <cmd> <params> */
fn create_process(s: &str) -> bool {
    let line: ~[~str] = create_cmd(s);
    if line.len() == 0 {
        return false
    }

    let cmd: ~str = line[0].clone();
    let mut args: ~[~str] = ~[];
    if line.len() > 1 {
        args = line.slice(1, line.len()).into_owned();
    }
    
    /* 'cd' must be handled in-process, as we want the current
    processes current-directory to change */
    if cmd == ~"cd" {
        let mut cd_dir: Path = Path::new(get_directory(Home));
        if args.len() > 0 {
            let new_path = match args[0][0] {
                45 => get_directory(OldPwd),
                _ => args[0].clone()
            };
            if new_path[0] != 126 {
                cd_dir = Path::new(new_path);
            }
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
        return false
    }

    let args = args;
    let result = do task::try {
        let opts: ProcessOptions = ProcessOptions::new();
        let launch: Option<Process> = Process::new(cmd, args, opts);
        handle_process(~(launch.unwrap()))
    };
    return result.is_ok()
}

/* Given a running process, check status and print stdout, stderr */
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

/* Creates an vec of strings given a line of input */
fn create_cmd(cmd: &str) -> ~[~str]{
    let mut cmd_arr: ~[~str] = ~[];

    for word in cmd.words() {
        let s: ~str = word.into_owned();
        cmd_arr.push(s);
    }
    cmd_arr
}

/* Get a given directory based on environment variables */
fn get_directory(d: DirType) -> ~str {
    let match_dir: ~str = match d {
        Home => ~"HOME",
        OldPwd => ~"OLDPWD",
        Pwd => ~"PWD"
    };

    let elts: ~[(~str,~str)] = env();
    for elt in elts.iter() {
        let (ref s1, ref s2): (~str,~str) = *elt;
        if *s1 == match_dir {
            return s2.clone()
        }
    }
    return ~""
}

/* Called when 'cd' is used as a command. */
fn set_working_dir(d: &str) {
    if d.len() == 0 {
        return
    }
    let old_pwd: &str = get_directory(Pwd);
    let mut new_pwd: Path = Path::new(old_pwd);

    // 45 == '-', go to old_pwd
    if d[0] == 45 {
        new_pwd = Path::new(get_directory(OldPwd));
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
