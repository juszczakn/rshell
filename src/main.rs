use std::os::env;
use std::io::buffered::BufferedReader;
use std::io;
use std::run::Process;
use std::run::ProcessOptions;

fn get_working_dir() -> ~str {
    let elts: ~[(~str,~str)] = env();
    let mut working_dir = ~"";
    
    for elt in elts.iter() {
        let (ref s1, ref s2): (~str,~str) = *elt;
        if *s1 == ~"PWD" {
            working_dir = s2.clone();
        }
    }
    working_dir
}

fn get_path() -> ~str {
    let elts: ~[(~str,~str)] = env();
    for elt in elts.iter() {
        let (ref s1, ref s2): (~str, ~str) = *elt;
        if *s1 == ~"PATH" {
            return s2.clone()
        }
    }
    ~""
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
    let opts: ProcessOptions = ProcessOptions::new();
    let launch: Option<Process> = Process::new(cmd, args, opts);
    launch
}

fn handle_process(new_proc: &Option<Process>) {
    match new_proc {
        Some(p) => {
            
        }
        None => println("None")
    }
}

fn read_stdin() {
    let mut reader = BufferedReader::new(io::stdin());
    while !reader.eof() {
        let line = reader.read_line();
        match line {
            Some(ref s) => {
                println("New line");
                let new_proc: Option<Process> = create_process(*s);
                handle_process(&new_proc);
            },
            None => println("None")
        }
    }
}

fn main() {
    let working_dir = get_working_dir();
    print(format!("{}$ ", working_dir));
    std::io::stdio::flush();
    read_stdin();
}
