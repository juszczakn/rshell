use std::os::env;
use std::io::buffered::BufferedReader;
use std::io;

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

fn match_cmd(cmd: &~str) -> ~[~str]{
    let mut cmd_arr: ~[~str] = ~[];

    if cmd.slice(0,2) == "cd" {
        cmd_arr.push(cmd.slice(0, 2).clone());
        cmd_arr.push(cmd.slice(3, cmd_arr.len()).clone());
        //println("done");
    }
    cmd_arr
}

fn read_stdin() {
    let mut reader = BufferedReader::new(io::stdin());
    while !reader.eof() {
        let line = reader.read_line();
        match line {
            Some(ref s) => {
                let cmd = match_cmd(s);
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
