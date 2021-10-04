use wasm_bindgen::prelude::*;
use logos::Logos;
use serde::{Serialize, Deserialize};
use std::cmp;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[error]
    InvalidToken,

    #[regex(r"\r?\n", priority = 3)]
    LineTerminator,
    #[regex(r"\s+")]
    Whitespace,

    /* Constants/Bindings */
    #[regex(r"[_$a-zA-Z][_$a-zA-Z0-9]*")]
    Identifier,
    #[regex(r"[0-9_\.]+", priority = 2)]
    NumericLiteral,

    #[token("false")]
    FalseKeyword,
    #[token("true")]
    TrueKeyword,
    #[token("null")]
    NullKeyword,

    //   /* Template nodes */
    // //   TemplateContinuation,
    // //   TemplateSpan,

    /* Punctuators */
    #[token("=>")]
    Arrow,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(".", priority = 3)]
    Period,
    #[token("...")]
    Ellipsis,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("?")]
    QuestionMark,
    #[token("'", priority = 1)]
    SingleQuote,
    #[token("\"", priority = 1)]
    DoubleQuote,
    #[token("</")]
    JSXClose,
    #[token("/>")]
    JSXAutoClose,

    /* Update operators */
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,

    /* Assign operators */
    #[token("=")]
    Assign,
    #[token("<<=")]
    ShiftLeftAssign,
    #[token(">>=")]
    ShiftRightAssign,
    #[token(">>>=")]
    LogicalShiftRightAssign,
    #[token("**=")]
    ExponentiateAssign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubtractAssign,
    #[token("*=")]
    MultiplyAssign,
    #[token("/=")]
    DivideAssign,
    #[token("%=")]
    ModuloAssign,
    #[token("^=")]
    BitwiseXorAssign,
    #[token("|=")]
    BitwiseOrAssign,
    #[token("&=")]
    BitwiseAndAssign,
    #[token("||=")]
    LogicalOrAssign,
    #[token("&&=")]
    LogicalAndAssign,
    #[token("??=")]
    CoalesceAssign,

    /* Unary/binary operators */
    #[token("typeof")]
    TypeofKeyword,
    #[token("delete")]
    DeleteKeyword,
    #[token("void")]
    VoidKeyword,
    #[token("!")]
    Negate,
    #[token("~")]
    Complement,
    #[token("+")]
    Add,
    #[token("-")]
    Subtract,
    #[token("in")]
    InKeyword,
    #[token("instanceof")]
    InstanceofKeyword,
    #[token("*")]
    Multiply,
    #[token("%")]
    Modulo,
    #[token("/")]
    Divide,
    #[token("**")]
    Exponentiate,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("===")]
    StrictEqual,
    #[token("!==")]
    StrictNotEqual,
    #[token("==")]
    LooseEqual,
    #[token("!=")]
    LooseNotEqual,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,

    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token(">>>")]
    LogicalShiftRight,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,

    /* Variable declaration kinds */
    #[token("var")]
    VarKeyword,
    #[token("let")]
    LetKeyword,
    #[token("const")]
    ConstKeyword,

    /* Other reserved words */
    #[token("break")]
    BreakKeyword,
    #[token("case")]
    CaseKeyword,
    #[token("catch")]
    CatchKeyword,
    #[token("class")]
    ClassKeyword,
    #[token("continue")]
    ContinueKeyword,
    #[token("debugger")]
    DebuggerKeyword,
    #[token("default")]
    DefaultKeyword,
    #[token("do")]
    DoKeyword,
    #[token("else")]
    ElseKeyword,
    #[token("export")]
    ExportKeyword,
    #[token("extends")]
    ExtendsKeyword,
    #[token("finally")]
    FinallyKeyword,
    #[token("for")]
    ForKeyword,
    #[token("function")]
    FunctionKeyword,
    #[token("if")]
    IfKeyword,
    #[token("import")]
    ImportKeyword,
    #[token("new")]
    NewKeyword,
    #[token("return")]
    ReturnKeyword,
    #[token("super")]
    SuperKeyword,
    #[token("switch")]
    SwitchKeyword,
    #[token("this")]
    ThisKeyword,
    #[token("throw")]
    ThrowKeyword,
    #[token("try")]
    TryKeyword,
    #[token("while")]
    WhileKeyword,
    #[token("with")]
    WithKeyword,

    /* Strict mode reserved words */
    #[token("implements")]
    ImplementsKeyword,
    #[token("interface")]
    InterfaceKeyword,
    #[token("package")]
    PackageKeyword,
    #[token("private")]
    PrivateKeyword,
    #[token("protected")]
    ProtectedKeyword,
    #[token("public")]
    PublicKeyword,
    #[token("static")]
    StaticKeyword,
    #[token("yield")]
    YieldKeyword,

    /* Strict mode reserved words */
    #[token("as")]
    AsKeyword,
    #[token("async")]
    AsyncKeyword,
    #[token("await")]
    AwaitKeyword,
    #[token("constructor")]
    ConstructorKeyword,
    #[token("get")]
    GetKeyword,
    #[token("set")]
    SetKeyword,
    #[token("from")]
    FromKeyword,
    #[token("of")]
    OfKeyword,
    #[token("enum")]
    EnumKeyword,
    #[token("eval")]
    Eval,
    #[token("arguments")]
    Arguments,

    //   EscapedReserved,
    //   EscapedFutureReserved,
    //   AnyIdentifier,

    // Stage #3 proposals
    #[token("#")]
    PrivateIdentifier,
    //   BigIntLiteral,
    #[token("??")]
    Coalesce,
    #[token("?.")]
    QuestionMarkPeriod,

    // Others
    //   CarriageReturn,
    //   PrivateField,
    //   Template,
    #[token("@")]
    Decorator,
    #[token("target")]
    Target,
    #[token("meta")]
    Meta,
    //   LineFeed,
    //   EscapedIdentifier,

    //   // JSX
    //   JSXText,
    Comment
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct LexerToken {
    value: Box<str>,
    token: Box<str>,
    range: Range,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Loc {
    offset: usize,
    line: usize,
    col: usize,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Range {
    start: Loc,
    end: Loc,
}


fn get_tokens(input: &str) -> Vec<LexerToken> {
    let text = input.trim_end();
    let mut lex = Token::lexer(text);
    let mut tokens = Vec::<LexerToken>::new();
    let mut offset = 0;
    let mut line = 1;
    let mut col = 1;
    let mut tok: std::option::Option<Token> = lex.next();

    while offset < text.len() {
        if offset == text.len() {
            break;
        }
        if offset != 0 {
            tok = lex.next();
        };

        if tok == Some(Token::LineTerminator) {
            let text = lex.slice();
            offset += text.chars().count();
            line += 1;
            col = 1;
            continue;
        }
        
        // Useful for debugging tokens
        // log(&format!("{:?}", tok));

        if tok == Some(Token::DoubleQuote) {
            for c in lex.remainder().chars() {
                if c == '\\' {
                    lex.bump(1);
                }
                if c != '"' {
                    lex.bump(1);
                } else {
                    lex.bump(1);
                    break;
                }
            }
        } else if tok == Some(Token::SingleQuote) {
            for c in lex.remainder().chars() {
                if c == '\\' {
                    lex.bump(1);
                }
                if c != '\'' {
                    lex.bump(1);
                } else {
                    lex.bump(1);
                    break;
                }
            }
        } else if tok == Some(Token::Divide) {
            let c0 = lex.remainder().chars().nth(0).unwrap();
            if c0 == '/' {
                tok = Some(Token::Comment);
                lex.bump(1);
                for c in lex.remainder().chars() {
                    if c == '\r' {
                        lex.bump(1);
                    }
                    if c != '\n' {
                        lex.bump(1);
                    } else {
                        break;
                    }
                }
            } else if c0 == '*' {
                tok = Some(Token::Comment);
                lex.bump(1);
                let mut maybe_end = false;
                for c in lex.remainder().chars() {
                    if c == '\\' {
                        lex.bump(1);
                    }
                    if c == '*' {
                        lex.bump(1);
                        maybe_end = true;
                    } else if maybe_end && c == '/' {
                        lex.bump(1);
                        break;
                    } else {
                        lex.bump(1);
                    }
                }
            }
        }

        let text = lex.slice();
        if text.trim() != "" {
            tokens.push(LexerToken{
                value: text.into(),
                token: format!("{:?}", tok.as_ref().unwrap()).as_str().into(),
                range: Range{
                    start: Loc{
                        col,
                        offset,
                        line,
                    },
                    end: Loc{
                        col: col + text.chars().count(),
                        offset: offset + text.chars().count(),
                        line: line,
                    }
                }
            });
        }
        offset += text.chars().count();
        col += text.chars().count();
    }
    
    return tokens;
}

#[wasm_bindgen]
pub fn lex(input: &str) -> JsValue {
    let tokens = get_tokens(input);
    return JsValue::from(serde_json::to_string(&tokens).unwrap());
}

#[wasm_bindgen(js_name = findRanges)]
pub fn find_ranges(input: &str, m: &str) -> JsValue {
    let mut matches = Vec::<Range>::new();
    let mut collector = Vec::<LexerToken>::new();
    let to_match = get_tokens(m);
    let tok_last = to_match.last().unwrap();
    let len = to_match.len() - 1;
    let tokens = get_tokens(input);

    for i in 0..tokens.len() {
        let tok = &tokens[i];
        if tok.value == tok_last.value && tok.token == tok_last.token {
            let mut tracked_pairs = Vec::<(&Box<str>, usize)>::new();
            let mut index = i.clone();
            let mut j = len;
            'inner: loop {
                if index < len {
                    break;
                }
                let t0 = &tokens[cmp::max(0, index - (len - j))];
                if t0.token.starts_with("Comment") {
                    index -= 1;
                    continue 'inner;
                }
                let t1 = &to_match[j];
                if t1.token.starts_with("Multiply") {
                    let t2 = &to_match[j + 1];
                    // log(&t2.value);
                    // log(&t2.token);
                }
                if t1.token.starts_with("Right") {
                    let index = (&tracked_pairs).into_iter().rposition(|&pair| pair.0 == &t1.token);
                    if index != None {
                        tracked_pairs[index.unwrap()].1 -= 1;
                    }
                }
                if t1.token.starts_with("Left") {
                    let index = (&tracked_pairs).into_iter().rposition(|&pair| pair.0 == &t1.token);
                    if index != None {
                        tracked_pairs[index.unwrap()].1 += 1;
                    } else {
                        tracked_pairs.push((&t1.token, 1));
                    }
                    log(&serde_json::to_string_pretty(&tracked_pairs).unwrap());
                }
                if t0.value == t1.value && t0.token == t1.token {
                    collector.push(t0.clone());
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                    continue 'inner;
                }
                break;
            }
            if collector.len() == to_match.len() {
                let range = Range{
                    start: collector.last().unwrap().range.start.clone(),
                    end: collector.first().unwrap().range.end.clone(),
                };
                matches.push(range);
            }
            if collector.len() > 0 {
                collector.clear();
            }
        }
    }
    return JsValue::from(serde_json::to_string(&matches).unwrap());
}
