#[macro_use]
mod json_token;
mod tokenize;
mod parse;
mod error;

use std::mem::replace;

pub use self::parse::State as ParseState;
pub use self::tokenize::State as TokenizeState;
pub use self::json_token::JSONToken;

use super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::convert::TryInto;

/// Represents parse error
pub struct ParseError {
    description : String
}

/// Used in self::tokenize
pub trait TokenConsumer {
    fn new() -> Self;
    fn consume(self, token: JSONToken) -> Self; 
}
impl <TC:TokenConsumer+Default> TokenConsumer for Box<TC> {
    fn new() -> Self {
        Box::new(TC::new())
    }
    fn consume(mut self, token: JSONToken) -> Self {
        /// Original code:
        /// ```Box::new((*self).parse_token(token))```
        /// optimized for memory reuse
        let placeholder = replace(&mut(*self), TC::default());  // use default to minimize construct overhead
        *self = placeholder.consume(token);
        self
    }
}

pub trait Tokenizer {
    fn new() -> Self;
    fn tokenize(self, c: char) -> Self;
}

pub fn from_char_stream<JSON, T, I>(iter: I) -> Result<JSON, ParseError>
    where T:Tokenizer + TryInto<JSON, Err=ParseError>, 
          I:Iterator<Item=char>, 
          JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    iter.fold(T::new(), T::tokenize).tokenize(' ').try_into()
}
