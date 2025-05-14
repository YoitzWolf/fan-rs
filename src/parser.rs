use std::default;

use crate::lexer::{FANGrammarToken, LexError, Operational};

pub trait Expandable {
    fn expand(&mut self, t: &FANGrammarToken) ->  Result<bool, ParseError>;
}

#[derive(Debug)]
pub struct TypeTemplate {

}

#[derive(Debug)]
pub struct SingleName(String, Option<TypeTemplate>);

#[derive(Debug)]
pub enum Name {
    SingleName(SingleName),
    NamespaceName(Vec<SingleName>),
}

#[derive(Debug, Default)]
pub struct Tuple{
    tuple: Vec<Statement>,
    closed: bool
}


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
    /// ist NULL :3
    NULL,
}

#[derive(Debug, Default)]
pub struct Block {
    block: Vec::<Expression>,
    closed: bool
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
pub struct UnaryOperator {
    arg:     Expression,
    operator: Operational,
}

#[derive(Debug)]
pub enum ReturnableExp {
    Statement(Statement),
    FunctionCall(Statement, Tuple),
    AutomataCall,
    BinaryOperator(Box<BinaryOperator>),
    UnaryyOperator(Box<UnaryOperator>),
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
    Define
}

#[derive(Debug, Default)]
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
    // tokens: Vec<FANGrammarToken>,  
    predict: ExpressionType,
    closed: bool
}

impl Expandable for Expression {
    fn expand(&mut self, t: &FANGrammarToken) -> Result<bool, ParseError> {
        match &self.predict {
            ExpressionType::Import(imports) => todo!(),
            ExpressionType::Procedural(procedural_exp) => todo!(),
            ExpressionType::Returnable(returnable_exp) => todo!(),
            ExpressionType::Definition(definition_exp) => todo!(),
            ExpressionType::Unpredicted => {
                // If new Expression starts then we have to make prediction what is it
                // It can be changed in futute, when more tokens will be accessable
                match t {
                    FANGrammarToken::Name(n) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::Statement(
                                Statement::Name(Name::SingleName(
                                    SingleName(n.clone(), None)
                                ))
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::Digital(d) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::Statement(
                                Statement::Literal(
                                    Literal::Digital(d.clone())
                                )
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::NumericalLexem(nl) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::Statement(
                                Statement::Literal(
                                    Literal::NumericalLexem(nl.clone())
                                )
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::CharLiteral(c) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::Statement(
                                Statement::Literal(
                                    Literal::Char(c.clone())
                                )
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::StringLiteral(c) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::Statement(
                                Statement::Literal(
                                    Literal::String(c.clone())
                                )
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::Reserved(fanreserved) => {
                        match fanreserved {
                            crate::lexer::FANReserved::NULL => {
                                self.predict = ExpressionType::Returnable(
                                    ReturnableExp::Statement(
                                        Statement::Literal(
                                            Literal::NULL
                                        )
                                    )
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::AutomataDeclare => {
                                self.predict = ExpressionType::Definition(
                                    DefinitionExp::Automata
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::StateDeclare => {
                                self.predict = ExpressionType::Definition(
                                    DefinitionExp::AutomataState
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::LocVarDeclare => {
                                self.predict = ExpressionType::Definition(
                                    DefinitionExp::Define
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::LinkDeclare => {
                                self.predict = ExpressionType::Procedural(
                                    ProceduralExp::Link
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::If => {
                                self.predict = ExpressionType::Returnable(
                                    ReturnableExp::If
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::For => {
                                self.predict = ExpressionType::Procedural(
                                    ProceduralExp::For
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::While => {
                                {
                                self.predict = ExpressionType::Procedural(
                                    ProceduralExp::While
                                );
                                Ok(true)
                            }
                            },
                            crate::lexer::FANReserved::Upload => {
                                self.predict = ExpressionType::Import(
                                    Imports::default()
                                );
                                Ok(true)
                            },
                            crate::lexer::FANReserved::From | crate::lexer::FANReserved::Else => {
                                Err(ParseError::Unexpected(t.clone()))
                            },
                        }
                    },
                    FANGrammarToken::BlockSymbol(block_symbol) => {
                        match block_symbol {
                            crate::lexer::BlockSymbol::BlockBracketOpen => {
                                self.predict = ExpressionType::Returnable(
                                    ReturnableExp::Statement(
                                        Statement::Block(
                                            Box::new(
                                                Block::default()
                                            )
                                        )
                                    )
                                );
                                Ok(true)
                            },
                            crate::lexer::BlockSymbol::TupleBracketOpen => {
                                self.predict = ExpressionType::Returnable(
                                    ReturnableExp::Statement(
                                        Statement::Tuple(
                                            Tuple::default()
                                        )
                                    )
                                );
                                Ok(true)
                            },
                            crate::lexer::BlockSymbol::IndexBracketOpen => {
                                Err(ParseError::Unexpected(t.clone()))
                            },
                            crate::lexer::BlockSymbol::BlockBracketClose | crate::lexer::BlockSymbol::TupleBracketClose | crate::lexer::BlockSymbol::IndexBracketClose => {
                                Err(ParseError::Unexpected(t.clone()))
                            },
                        }
                    },
                    FANGrammarToken::Operational(operational) => {
                        self.predict = ExpressionType::Returnable(
                            ReturnableExp::UnaryyOperator(
                                Box::new(UnaryOperator {
                                    arg: Expression::default(),
                                    operator: operational.clone()
                                })
                            )
                        );
                        Ok(true)
                    },
                    FANGrammarToken::TokBrk => {
                        Err(ParseError::Unexpected(t.clone()))
                    },
                }
            },
        }
    }
}

/* ============================================================================= */

#[derive(Debug)]
pub enum ParseError {
    Unexpected(FANGrammarToken)
}

pub struct Parser {

}

impl Parser {
    pub fn parse(tokens: Vec<FANGrammarToken>) -> Result<(), ParseError> {
        let mut expressions: Vec<Expression> = vec![];
        for tok in tokens.iter() {
            let expanded = if let Some(lexp) = expressions.last_mut() {
                lexp.expand(&tok)?
            } else { false };
            if !expanded {
                let mut s = Expression::default();
                if s.expand(&tok)? {
                    expressions.push(s);   
                } else {
                    panic!("Invalid empty expression implementation");
                }
            }
        };

        Ok(())
    }
}
