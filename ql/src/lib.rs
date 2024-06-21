pub mod lexer {
    use chumsky::{prelude::*, span::SimpleSpan, Parser};

    #[derive(Debug, PartialEq)]
    pub enum Token<'a> {
        Ident(&'a str),
        String(&'a str),
    }

    pub fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<(SimpleSpan<usize>, Token<'a>)>> {
        let ident = any()
            .filter(|c: &char| c.is_alphanumeric())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(Token::Ident);

        let string = just('"')
            .then(any().filter(|c: &char| *c != '"').repeated())
            .then(just('"'))
            .to_slice()
            .map(Token::String);

        ident
            .or(string)
            .map_with(|token, e| (e.span(), token))
            .padded()
            .repeated()
            .collect()
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        fn poc() {
            assert_eq!(
                lexer()
                    .parse(r#"hello "world" these are "test" woo"#)
                    .into_result(),
                Ok(vec![
                    ((0..5).into(), Token::Ident("hello")),
                    ((6..13).into(), Token::String("\"world\"")),
                    ((14..19).into(), Token::Ident("these")),
                    ((20..23).into(), Token::Ident("are")),
                    ((24..30).into(), Token::String("\"test\"")),
                    ((31..37).into(), Token::Ident("woo")),
                ]),
            );
        }
    }
}
