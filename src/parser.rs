use crate::lexer::{FANGrammarToken, LexError, Operational};

pub trait Expandable {
    fn expand(&mut self, t: &FANGrammarToken) -> bool;
}

#[derive(Debug)]
pub enum Name {
    SigleName,
    NamespaceName,
}

#[derive(Debug)]
pub struct Tuple(Vec<Statement>);

#[derive(Debug)]
pub enum Literal {
    /// Char
    Char(char),
    /// String
    String(String),
    /// 100% castable to digital
    Digital(String),
    /// any numerical lexem \\
    /// with type or float - need to identify type
    NumericalLexem(String),
}

#[derive(Debug)]
pub struct Block {
    block: Vec::<Expression>,
}

#[derive(Debug)]
pub enum Statement {
    /// Char String Digital or other Numerical lexem
    Literal(Literal),
    /// block of { code }
    Block(Box<Block>),
    /// variable or path to name
    Name(Name),
    /// any expression in ( round brackets )
    Tuple(Tuple),
}

#[derive(Debug)]
pub struct BinaryOperator {
    arg1:     Expression,
    arg2:     Expression,
    operator: Operational,
}

#[derive(Debug)]
pub enum ReturnableExp {
    Statement(Statement),
    FunctionCall(Statement, Tuple),
    AutomataCall,
    BinaryOperator(Box<BinaryOperator>),
    If,
    Match,
}

#[derive(Debug)]
pub enum ProceduralExp {
    For,
    While,
    Link,
}

#[derive(Debug)]
pub enum DefinitionExp {
    Function,
    Automata,
    AutomataState,
}

#[derive(Debug)]
pub struct Imports {

}

#[derive(Debug, Default)]
pub enum ExpressionType {
    Import(Imports),
    Procedural(ProceduralExp),
    Returnable(ReturnableExp),
    Definition(DefinitionExp),
    #[default]
    Unpredicted
}


#[derive(Debug, Default)]
pub struct Expression {
    tokens: Vec<FANGrammarToken>, 
    predict: ExpressionType,
    closed: bool
}

impl Expandable for Expression {
    fn expand(&mut self, t: &FANGrammarToken) -> bool {
        false
    }
}

/* ============================================================================= */

pub enum ParseError {

}

pub struct Parser {

}

impl Parser {
    pub fn parse(tokens: Vec<FANGrammarToken>) -> Result<(), ParseError> {
        let mut expressions: Vec<Expression> = vec![];
        tokens.iter().for_each(
            |tok: &FANGrammarToken| {
                let expanded = if let Some(lexp) = expressions.last_mut() {
                    lexp.expand(&tok)
                } else { false };
                if !expanded {
                    let mut s = Expression::default();
                    if s.expand(&tok) {
                        expressions.push(s);   
                    } else {
                        panic!("Invalid empty expression implementation");
                    }
                }
            }
        );

        Ok(())
    }
}
