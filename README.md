
# Finite Automata Notation

`fan
    upload ContextType1, ContextType2 from contexts.fan

    automata ContextAutomata {
        state Base<context1: ContextType1, context2: ContextType2> {
            ...
        }
        ...
        state Verdict1<arg1: int64, ...>{ ... }
        state Verdict2<arg1, ...>{ ... }
    }

    automata MyAutomata { # no input, default is Moore
        state Base < context: (int64, ContextType1, ContextType2) > { # tuples!
            # default link is NULL (finish automata)
            if self.previous.is_me() {
                context[0] += 1; # standard arithmetic and binary operations
            }
            
            if context[0] == 10 {
                link self -> StateA<context[1], context[2]>;
            } else {
                link self -> Base<context>;
            }

        }

        state StateA<context: ContextType1, context2: ContextType2> { # or just args
            let con_autom_result = run ContextAutomata<context1, context2>; # local variable
            if con_con_autom_result is ContextAutomata::Verdict1(verd1) {
                link self -> Base<(verd1.arg1, ContextType1(), ContextType2())>
            } else {
                link self -> NULL;
            }
        }
    } # automata MyAutomata end
`

# Lingua automata

`fan
    automata Mathematica: Mealy<signal> { # define signal name
        ...
        state AddAssign<signal: ()> { # no need to use context
            link self -> NULL;
        }

        state Add<signal: char> { # signal is reserved!
            if signal == '=' {
                link self -> AddAssign;
            } else {
                link self -> Null;
            }
        }
    }
    ...
`