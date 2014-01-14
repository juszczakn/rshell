pub enum Token {
    CMDARG(~str), IF, THEN, END,
    OPEN_PAREN, CLOSE_PAREN,
    AND, OR, PIPE, PUSH
}

pub enum TokenList {
    TokenList(Token, ~TokenList),
    Null
}

pub fn match_token(token: &str) -> Option<Token> {
    match token {
        "(" => Some(OPEN_PAREN),
        ")" => Some(CLOSE_PAREN),
        "&&" => Some(AND),
        "||" => Some(OR),
        "|" => Some(PIPE),
        "&" => Some(PUSH),
        "if" => Some(IF),
        "then" => Some(THEN),
        "end" => Some(END),
        _ => Some(CMDARG(token.into_owned()))
    }
}
