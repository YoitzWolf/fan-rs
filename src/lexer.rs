
#[derive(Debug, Clone, PartialEq)]
pub enum StringLiteralType {
    Char,
    String
}

#[derive(Debug, Clone)]
pub enum TokenFlag {
    Literal,
    StringLiteral(StringLiteralType),
    Operational,
    TokBrk
}

impl TokenFlag {
    pub fn is_literal(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}

type Token = (TokenFlag, String);
type TokenLine = Vec<Token>;

#[derive(Debug, Clone)]
pub enum LexError {
    UnknownError,
    InvalidCharLiteralValue,
    InvalidOperatorToken,
    LiteralEndNotFound
}

#[derive(Debug, Clone)]
pub enum BlockSymbol {
    BlockBracketOpen,
    BlockBracketClose,
    TupleBracketOpen,
    TupleBracketClose,
    // TemplateBracketOpen,
    // TemplateBracketClose,
    IndexBracketOpen,
    IndexBracketClose
}

impl BlockSymbol {
    pub fn try_from<'a>(c: &'a char) -> Option<Self> {
        match c {
            // '"' => Some(Self::Quote),
            // '\'' => Some(Self::SQuote),
            '{' => Some(Self::BlockBracketOpen),
            '}' => Some(Self::BlockBracketClose),
            '(' => Some(Self::TupleBracketOpen),
            ')' => Some(Self::TupleBracketClose),
            // '<' => Some(Self::TemplateBracketOpen),
            // '>' => Some(Self::TemplateBracketClose),
            '[' => Some(Self::IndexBracketOpen),
            ']' => Some(Self::IndexBracketClose),
            // '#' => Some(Self::Comment),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum FANReserved {
    AutomataDeclare,
    StateDeclare,
    LocVarDeclare,
    LinkDeclare,
    NULL,
    If,
    Else,
    For,
    While,
    Upload,
    From
}

impl FANReserved {
    pub fn try_from<'a>(s: &'a str) -> Option<Self> {
        match s {
            "automata" => Some(Self::AutomataDeclare),
            "state" => Some(Self::StateDeclare),
            "let" => Some(Self::LocVarDeclare),
            "link" => Some(Self::LinkDeclare),
            "NULL" | "null" | "Null" => Some(Self::NULL),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "for" => Some(Self::For),
            "while" => Some(Self::While),
            "upload" => Some(Self::Upload),
            "from" => Some(Self::From),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operational(String);

impl Operational {
    const AVALS: [&'static str; 29] = [
        "=", "+=", "-=", "/=", "*=",
        "+", "-", "*", "/", "&", "->",
        "^", ":", "::", ".", ",", "%",
        "~", "==", "!=", ";", "&&", "|", "||",
        "<", ">", ">=", "<=",
        "\\"
    ];

    pub fn try_expand<'a>(&self, s: &'a str) -> Option<Self> {
        let n = self.0.clone() + s;
        if Self::AVALS.iter().any(|&e| e.eq(&n)) {
            Some(Self(n))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum FANGrammarToken {
    Name(String),
    Digital(String),
    NumericalLexem(String),
    CharLiteral(char),
    StringLiteral(String),
    Reserved(FANReserved),
    BlockSymbol(BlockSymbol),
    Operational(Operational),
    TokBrk,
}

#[derive(Debug, Clone)]
pub struct Lexer {}

#[derive(Debug, PartialEq)]
pub struct LexStackItem {
    kind: LexStackItemType,
    data: String
}

#[derive(Debug, PartialEq)]
pub enum LexStackItemType {
    StringLiteral(StringLiteralType),
    Comment
}
impl LexStackItemType {
    pub fn try_from<'a>(c: &'a char) -> Option<Self> {
        match c {
            '"' =>  Some(Self::StringLiteral(StringLiteralType::String)),
            '\'' => Some(Self::StringLiteral(StringLiteralType::Char)),
            '#' =>  Some(Self::Comment),
            _ => None
        }
    }
}

impl Lexer {
    // pub fn new() -> Self {
    //     Self{}
    // }

    pub fn split_line<'a>(line: &'a str, stack: &mut Vec<LexStackItem>) -> Result<TokenLine, LexError> {
        let (tokens, stack) = line.chars().try_fold(
            (TokenLine::new(), stack), 
            |(mut tokenline, stack), char| {
                if let Some(lastst) = stack.last_mut() {
                    // stack is not empty
                    let this=LexStackItemType::try_from(&char);
                    match &mut lastst.kind {
                        LexStackItemType::StringLiteral(stritt) => {
                            match stritt {
                                StringLiteralType::Char => {
                                    if let Some(LexStackItemType::StringLiteral(StringLiteralType::Char)) = this {
                                        // close
                                        if lastst.data.chars().last().unwrap_or(' ').eq(&'\\') {
                                            // ignore
                                            lastst.data.push(char);
                                        } else {
                                            stack.pop();
                                        }
                                    } else {
                                        // check if able to set char value
                                        if lastst.data.len() == 0 || lastst.data.len() == 1 && lastst.data.chars().last().unwrap_or(' ').eq(&'\\') {
                                            lastst.data = char.to_string();
                                        } else {
                                            return Err(LexError::InvalidCharLiteralValue);
                                        }
                                    }
                                },
                                StringLiteralType::String => {
                                    if let Some(LexStackItemType::StringLiteral(StringLiteralType::String)) = this {
                                        // close
                                        if lastst.data.chars().last().unwrap_or(' ').eq(&'\\') {
                                            // ignore
                                            lastst.data.push(char);
                                        } else {
                                            stack.pop();
                                        }
                                    } else {
                                        // regular string
                                        lastst.data.push(char);
                                    }
                                },
                            }
                        },
                        LexStackItemType::Comment => { /* PASS */ },
                    }
                } else {
                    // stack is empty
                    // println!(">>> {}", char);
                    if char.is_ascii_whitespace() {
                        // split
                        if let Some(x) = tokenline.last_mut() {
                            if let TokenFlag::TokBrk = x.0 { /* PASS */}
                            else {
                                tokenline.push((TokenFlag::TokBrk, " ".to_string()));    
                            }
                        } else {
                            tokenline.push((TokenFlag::TokBrk, " ".to_string()));    
                        }
                    } else {
                        match LexStackItemType::try_from(&char) {
                            Some(lst) => {
                                // println!("StackType found: {:?}", lst);
                                stack.push(
                                    LexStackItem {
                                        kind: lst,
                                        data: "".to_string(),
                                    }
                                );
                            },
                            None => {
                                match tokenline.last_mut() {
                                    Some((TokenFlag::Literal, last_tok_str)) => {
                                        // last token exist
                                        if TokenFlag::is_literal(char) {
                                            last_tok_str.push(char);
                                        } else {
                                            tokenline.push(
                                                (
                                                    TokenFlag::Operational,
                                                    char.to_string()
                                                )
                                            );
                                        }
                                    },
                                    _ => {
                                        if TokenFlag::is_literal(char) {
                                            tokenline.push(
                                                (
                                                    TokenFlag::Literal,
                                                    char.to_string()
                                                )
                                            );
                                        } else {
                                            tokenline.push(
                                                (
                                                    TokenFlag::Operational,
                                                    char.to_string()
                                                )
                                            );
                                        }
                                    }
                                }
                            },
                        }
                        // append 
                    }
                }
                Ok((tokenline, stack))
            }
        )?;
        if let Some(LexStackItem{data:_, kind: LexStackItemType::Comment}) = stack.last() {
            stack.pop();
        }
        if stack.iter().filter(|x| {x.kind == LexStackItemType::Comment}).count() != 0 {
            Err(LexError::UnknownError)
        } else {
            Ok(tokens)
        }
    }

    pub fn lex_line<'a>(line: &'a str, stack: &mut Vec<LexStackItem>) -> Result<Vec<FANGrammarToken>, LexError> {
        let tokens = Self::split_line(line, stack)?;
        let lexems = tokens.iter().try_fold(
            vec![],
            |mut lexems, (flag, token)| {
                if token.is_empty() { return Ok(lexems); }
                match flag {
                    TokenFlag::Literal => {
                        // check is reserved
                        if let Some(rsrv) = FANReserved::try_from(&token) {
                            lexems.push(
                                FANGrammarToken::Reserved(rsrv)
                            );
                        } else if token.chars().nth(0).unwrap().is_numeric() {
                            if token.chars().all(|c| c.is_numeric()) {
                                lexems.push(
                                    FANGrammarToken::Digital(token.clone())
                                );
                            } else {
                                lexems.push(
                                    FANGrammarToken::NumericalLexem(token.clone())
                                );
                            }
                        } else {
                            lexems.push(
                                FANGrammarToken::Name(token.clone())
                            );
                        }
                    },
                    TokenFlag::Operational => {
                        // check operator expansion
                        if let Some(FANGrammarToken::Operational(operational)) = lexems.last_mut() {
                            if let Some(y) = operational.try_expand(&token) {
                                operational.0 = y.0;
                                return Ok(lexems);
                            }
                        }
                        // check brackets etc
                        if token.len() > 1 {
                            println!(">> {:?}", token);
                            return Err(LexError::InvalidOperatorToken);
                        } else {
                            if let Some(blocksymb) = BlockSymbol::try_from(&token.chars().nth(0).unwrap()) {
                                lexems.push(FANGrammarToken::BlockSymbol(blocksymb)); 
                            } else {
                                if let Some(op) = Operational("".to_string()).try_expand(&token) {
                                    lexems.push(FANGrammarToken::Operational(op));
                                } else {
                                    println!(">> 2");
                                    return Err(LexError::InvalidOperatorToken);
                                }
                            }
                        }
                    },
                    TokenFlag::StringLiteral(string_literal_type) => {
                        lexems.push(match string_literal_type {
                            StringLiteralType::Char => { FANGrammarToken::CharLiteral(token.chars().nth(0).unwrap()) },
                            StringLiteralType::String => { FANGrammarToken::StringLiteral(token.clone()) },
                        })
                    },
                    TokenFlag::TokBrk => {
                        // Break last token lexing
                        lexems.push(FANGrammarToken::TokBrk);
                    },
                };
                Ok(lexems)
            }
        )?;
        Ok(lexems.into_iter().to_owned().filter(
            |x|
                if let FANGrammarToken::TokBrk = x {false} else {true}
            ).collect()
        )
    }

    pub fn lex_buf<'a, T>(mut data: T) -> Result<Vec<Vec<FANGrammarToken>>, LexError>
        where T: Iterator<Item=&'a str>
    {
        let (r, s) = data.try_fold(
            (vec![], vec![]),
            |(mut r, mut stack), l| {
                let s = Lexer::lex_line(l, &mut stack)?;
                if s.len() > 0 {r.push(s);}
                Ok((r, stack))
            }
        )?;
        if s.len() > 0 {
            Err(LexError::LiteralEndNotFound)
        } else {
            Ok(r)
        }
    }
}

#[test]
fn tokenize_test() {
    let data = "automata Mathematica: Mealy<signal>{# define signal name
         state AddAssign<signal: ()> { # no need to use context
    
            link self -> NULL;
        }

        state Add<signal: char> { # signal is reserved!
            let sub = 8u64 + 1.f32;
            if signal == '=' {
                link self -> AddAssign;
            } else {
                link self -> Null;
            }
        }
    }";
    println!(
        "{:?}", Lexer::lex_buf(data.split("\n"))
    ); 
}