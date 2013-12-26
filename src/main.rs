use std::os::env;
use std::io::buffered::BufferedReader;
use std::io;
use std::run::Process;
use std::run::ProcessOptions;
use std::run::ProcessOutput;
use std::path::posix::Path;

fn get_working_dir() -> ~str {
    let elts: ~[(~str,~str)] = env();
    
    for elt in elts.iter() {
        let (ref s1, ref s2): (~str,~str) = *elt;
        if *s1 == ~"PWD" {
            return s2.clone()
        }
    }
    return ~""
}

fn set_working_dir(d: &str) {
    let mut new_pwd: ~str = get_working_dir();
    let old_pwd: &str = new_pwd;

    
    std::os::setenv(&"PWD", d);
    std::os::setenv(&"OLDPWD", old_pwd);
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
        let mut cd_dir: Path = Path::new("~/");
        if args.len() > 0 {
            cd_dir = Path::new(args[0].clone());
        }
        let ret_val: bool = std::os::change_dir(&cd_dir);
        if ret_val == false {
            println(format!("Error: '{}' is not a valid path.", args[0]));
        } else {
            set_working_dir(args[0]);
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
        match line {
            Some(ref s) => {
                let new_proc: Option<Process> = create_process(*s);
                if new_proc.is_some() {
                    handle_process(~(new_proc.unwrap()));
                }
                print(format!("{}$> ", get_working_dir()));
                std::io::stdio::flush();
            },
            None => println("None")
        }
    }
}

fn main() {
    print(format!("{}$> ", get_working_dir()));
    std::io::stdio::flush();
    read_stdin();
}
