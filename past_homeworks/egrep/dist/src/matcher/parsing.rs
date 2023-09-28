use super::MacherConfig;

#[derive(Debug, PartialEq, Clone)]
pub(super) enum PatternNode<Base> {
    Literal(Base),
    Class { negated: bool, class: Vec<Base> },
    Concat(Box<PatternNode<Base>>, Box<PatternNode<Base>>),
    // NOTE: If both would match, we prefer the left
    Or(Box<PatternNode<Base>>, Box<PatternNode<Base>>),
    Repeat(Box<PatternNode<Base>>),
    NonGreedyRepeat(Box<PatternNode<Base>>),
    Save(Box<PatternNode<Base>>),
    Wild,
    Empty,
}

const METACHARS: [u8; 11] = [
    b'?', b'+', b'*', b'|', b'(', b')', b'.', b'^', b'$', b'[', b']',
];

pub(super) fn pattern_parser(
) -> impl chumsky::Parser<u8, (PatternNode<u8>, MacherConfig), Error = chumsky::error::Simple<u8>> {
    use chumsky::prelude::*;

    (just(b'^').or_not().map(|x| match x {
        None => Some(PatternNode::NonGreedyRepeat(Box::new(PatternNode::Wild))),
        Some(_) => None,
    }))
    .then(recursive(|pat| {
        let char_class = (just(b'^').or_not().map(|x| x.is_some()))
            .then(just(b']').or_not())
            .then(none_of(b']').repeated())
            .map(|((negated, extra), mut class)| {
                if let Some(c) = extra {
                    class.push(c);
                }

                PatternNode::Class { negated, class }
            });

        let atom = (just(b'\\').ignore_then(one_of(&METACHARS).map(PatternNode::Literal)))
            .or(just(b'.').to(PatternNode::Wild))
            // Normal when not used at the end of the string (i.e., as an anchor)
            .or(just(b'$')
                .then_ignore(any().rewind())
                .map(PatternNode::Literal))
            // Normal when non in a character class or start of string
            .or(just(b'^').map(PatternNode::Literal))
            .or(char_class.delimited_by(just(b'['), just(b']')))
            .or(none_of(&METACHARS).map(PatternNode::Literal))
            .or(pat.delimited_by(just(b'('), just(b')')));

        let repetition = atom
            .then(
                choice((
                    just(b'*').to((|x| PatternNode::Repeat(Box::new(x))) as fn(_) -> _),
                    just(b'+').to((|x: PatternNode<u8>| {
                        PatternNode::Concat(
                            Box::new(x.clone()),
                            Box::new(PatternNode::Repeat(Box::new(x))),
                        )
                    }) as _),
                    just(b'?')
                        .to((|x| PatternNode::Or(Box::new(x), Box::new(PatternNode::Empty))) as _),
                ))
                .repeated(),
            )
            .foldl(|pat, k| k(pat));

        let concatentation = repetition
            .clone()
            .then(repetition.repeated())
            .foldl(|a, b| PatternNode::Concat(Box::new(a), Box::new(b)));

        let alternation = concatentation
            .clone()
            .then(just(b'|').ignore_then(concatentation).repeated())
            .foldl(|x, y| PatternNode::Or(Box::new(x), Box::new(y)));

        alternation
    }))
    .then(just(b'$').or_not().map(|x| x.is_none()))
    .then_ignore(end())
    .map(|((start_anchor, inner_pat), prefix_okay)| {
        let saved_pat = PatternNode::Save(Box::new(inner_pat));
        let pat = if let Some(anchor) = start_anchor {
            PatternNode::Concat(Box::new(anchor), Box::new(saved_pat))
        } else {
            saved_pat
        };

        (pat, MacherConfig { prefix_okay })
    })
}

#[cfg(test)]
mod tests {
    use super::{PatternNode::*, *};
    use chumsky::Parser;

    fn concat<T>(x: PatternNode<T>, y: PatternNode<T>) -> PatternNode<T> {
        Concat(Box::new(x), Box::new(y))
    }

    fn or<T>(x: PatternNode<T>, y: PatternNode<T>) -> PatternNode<T> {
        Or(Box::new(x), Box::new(y))
    }

    fn repeat<T>(x: PatternNode<T>) -> PatternNode<T> {
        Repeat(Box::new(x))
    }

    fn non_greedy_repeat<T>(x: PatternNode<T>) -> PatternNode<T> {
        NonGreedyRepeat(Box::new(x))
    }

    fn save<T>(x: PatternNode<T>) -> PatternNode<T> {
        Save(Box::new(x))
    }

    fn class<T: Clone>(negated: bool, x: &[T]) -> PatternNode<T> {
        Class {
            negated,
            class: x.to_vec(),
        }
    }

    fn parse(s: &'static str) -> Result<(PatternNode<u8>, bool), Vec<chumsky::error::Simple<u8>>> {
        pattern_parser()
            .parse(s.as_bytes())
            .map(|(node, conf)| (node, conf.prefix_okay))
    }

    #[test]
    fn simple_concat() {
        assert_eq!(
            parse("^abc$"),
            Ok((
                save(concat(concat(Literal(b'a'), Literal(b'b')), Literal(b'c'))),
                false
            )),
        )
    }

    #[test]
    fn simple_parens() {
        assert_eq!(
            parse("^a(bc)$"),
            Ok((
                save(concat(Literal(b'a'), concat(Literal(b'b'), Literal(b'c')))),
                false
            )),
        )
    }

    #[test]
    fn simple_alter() {
        assert_eq!(
            parse("^a|b$"),
            Ok((save(or(Literal(b'a'), Literal(b'b'))), false))
        )
    }

    #[test]
    fn alter_precedence() {
        assert_eq!(
            parse("^a|b$"),
            Ok((save(or(Literal(b'a'), Literal(b'b'))), false))
        )
    }

    #[test]
    fn alter_parens() {
        assert_eq!(
            parse("^a|(bc)$"),
            Ok((
                save(or(Literal(b'a'), concat(Literal(b'b'), Literal(b'c')))),
                false
            ))
        )
    }

    #[test]
    fn nested_parens() {
        assert_eq!(
            parse("^a|(b(cd)|e)$"),
            Ok((
                save(or(
                    Literal(b'a'),
                    or(
                        concat(Literal(b'b'), concat(Literal(b'c'), Literal(b'd'))),
                        Literal(b'e')
                    )
                )),
                false
            ))
        );

        assert_eq!(
            parse("^a|(b(cd|e))$"),
            Ok((
                save(or(
                    Literal(b'a'),
                    concat(
                        Literal(b'b'),
                        or(concat(Literal(b'c'), Literal(b'd')), Literal(b'e'))
                    )
                )),
                false
            ))
        )
    }

    #[test]
    fn unanchored() {
        assert_eq!(
            parse("a"),
            Ok((concat(non_greedy_repeat(Wild), save(Literal(b'a'))), true))
        )
    }

    #[test]
    fn unanchored_front() {
        assert_eq!(
            parse("a$"),
            Ok((concat(non_greedy_repeat(Wild), save(Literal(b'a'))), false))
        )
    }

    #[test]
    fn unanchored_back() {
        assert_eq!(parse("^a"), Ok((save(Literal(b'a')), true)))
    }

    #[test]
    fn char_class() {
        assert_eq!(parse("^[abcd]$"), Ok((save(class(false, b"abcd")), false)))
    }

    #[test]
    fn char_class_bracket() {
        assert_eq!(
            parse("^[]abcd]$"),
            Ok((save(class(false, b"abcd]")), false))
        )
    }

    #[test]
    fn negated_class() {
        assert_eq!(parse("^[^abcd]$"), Ok((save(class(true, b"abcd")), false)))
    }

    #[test]
    fn negated_class_bracket() {
        assert_eq!(parse("^[^]123]$"), Ok((save(class(true, b"123]")), false)))
    }
}
