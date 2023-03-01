#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TokenKind {
    Punct,
    Num,
}

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) val: Option<i32>,
    pub(crate) loc: &'a str,
}

impl<'a> Token<'a> {
    pub(crate) fn new(kind: TokenKind, slice: &'a str) -> Self {
        Self {
            kind,
            val: None,
            loc: slice,
        }
    }

    pub(crate) fn with_value(value: i32, slice: &'a str) -> Self {
        Self {
            kind: TokenKind::Num,
            val: Some(value),
            loc: slice,
        }
    }

    pub(crate) fn is_plus(&self) -> bool {
        self.kind == TokenKind::Punct && self.loc == "+"
    }

    pub(crate) fn is_minus(&self) -> bool {
        self.kind == TokenKind::Punct && self.loc == "-"
    }
}

#[derive(Debug)]
pub(crate) enum TokenError {
    UnexpectedCharacter,
}

pub(crate) fn tokenize(p: &str) -> Result<Vec<Token>, TokenError> {
    let mut result = vec![];
    let mut index = 0usize;
    let resource_str_len = p.len();

    while index < resource_str_len {
        match p.chars().enumerate().nth(index) {
            Some((_, c)) if c.is_whitespace() => {
                index += 1;
                continue;
            }
            Some((start, c)) if c.is_ascii_digit() => {
                let mut end = start + 1;
                while end < resource_str_len && p.chars().nth(end).unwrap().is_ascii_digit() {
                    end += 1;
                }
                result.push(Token::with_value(
                    p[start..end].parse::<i32>().unwrap(),
                    &p[start..end],
                ));
                index = end;
                continue;
            }
            Some((start, c)) if c == '+' || c == '-' => {
                result.push(Token::new(TokenKind::Punct, &p[start..start + 1]));
                index += 1;
                continue;
            }
            _ => return Err(TokenError::UnexpectedCharacter),
        }
    }

    Ok(result)
}
