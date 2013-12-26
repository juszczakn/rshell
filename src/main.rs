use std::os::env;
use std::io::buffered::BufferedReader;
use std::io;
use std::io::process::ProcessConfig;

static DIRECTORY: &'static str = "Hello";

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

fn create_cmd(cmd: &str) -> ~[~str]{
    let mut cmd_arr: ~[~str] = ~[];
    for word in cmd.words() {
        let s: ~str = word.into_owned();
        cmd_arr.push(s);
    }
    cmd_arr
}

fn read_stdin() {
    let mut reader = BufferedReader::new(io::stdin());
    while !reader.eof() {
        let line = reader.read_line();
        match line {
            Some(ref s) => {
                let cmd = create_cmd(*s);
                let proc_config: ProcessConfig = ProcessConfig {
                    program = *cmd[0];
                };
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
