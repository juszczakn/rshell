mod tokens;

pub fn tokenizer(args: &[~str]) -> ~tokens::TokenList{
    let mut token_list = ~tokens::Null;
    for arg in args.iter() {
        let m = tokens::match_token(*arg);
        token_list = ~tokens::TokenList(m.unwrap(), token_list);

    }
    token_list
}

// Test
fn main() {
    let args = ~[~"asdf", ~"|", ~"fdsa", ~"&"];
    let mut list = tokenizer(args);
    let mut head;
    loop {
        match list {
            ~tokens::Null => break,
            ~tokens::TokenList(t, l) => { head = t; list = l }
        }
        match head {
            tokens::CMDARG(s) => println(s),
            tokens::PIPE => println("PIPE"),
            _ => println("dont care")
        }
    }
}
