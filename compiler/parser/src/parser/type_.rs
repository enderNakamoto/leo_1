// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use leo_errors::Result;

pub(crate) const TYPE_TOKENS: &[Token] = &[
    Token::I8,
    Token::I16,
    Token::I32,
    Token::I64,
    Token::I128,
    Token::U8,
    Token::U16,
    Token::U32,
    Token::U64,
    Token::U128,
    Token::Field,
    Token::Group,
    Token::Address,
    Token::Bool,
    Token::Char,
];

impl ParserContext<'_> {
    ///
    /// Returns a [`IntegerType`] AST node if the given token is a supported integer type, or [`None`].
    ///
    pub fn token_to_int_type(token: &Token) -> Option<IntegerType> {
        Some(match token {
            Token::I8 => IntegerType::I8,
            Token::I16 => IntegerType::I16,
            Token::I32 => IntegerType::I32,
            Token::I64 => IntegerType::I64,
            Token::I128 => IntegerType::I128,
            Token::U8 => IntegerType::U8,
            Token::U16 => IntegerType::U16,
            Token::U32 => IntegerType::U32,
            Token::U64 => IntegerType::U64,
            Token::U128 => IntegerType::U128,
            _ => return None,
        })
    }

    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type.
    /// Also returns the span of the parsed token.
    pub fn parse_non_ident_types(&mut self) -> Result<(Type, Span)> {
        let span = self.expect_any(TYPE_TOKENS)?;
        Ok((
            match &self.prev_token.token {
                Token::Field => Type::Field,
                Token::Group => Type::Group,
                Token::Address => Type::Address,
                Token::Bool => Type::Boolean,
                Token::Char => Type::Char,
                x => Type::IntegerType(Self::token_to_int_type(x).expect("invalid int type")),
            },
            span,
        ))
    }

    /// Returns a [`(Type, Span)`] tuple of AST nodes if the next token represents a type.
    /// Also returns the span of the parsed token.
    pub fn parse_all_types(&mut self) -> Result<(Type, Span)> {
        Ok(if let Some(ident) = self.eat_identifier() {
            let span = ident.span.clone();
            (Type::Identifier(ident), span)
        } else {
            self.parse_non_ident_types()?
        })
    }
}