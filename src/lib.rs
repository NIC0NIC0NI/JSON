#[macro_use]
mod json_object;
mod from_json_str;
mod convert;
mod type_adapt;

pub use convert::{TryFromIterator, FromJSONStr, FromPremitive};
pub use from_json_str::ParseError;
pub use json_object::{DefaultJSON, PreservingJSON};
pub use type_adapt::{MakeJSON, JSONObject, JSONArray};
use from_json_str::{TokenizeState, ParseState, from_char_stream};

/// Boxing makes `size_of::<TokenConsumer>()` much smaller, faster in parameter passing
type TokenConsumer<JSON> = Box<ParseState<JSON>>;
/// Boxing this makes more overhead than benefit
type Tokenizer<JSON> = TokenizeState<TokenConsumer<JSON>>;




impl <JSON> TryFromIterator<char> for JSON 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    type Err = ParseError;
    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Err>
         where I: IntoIterator<Item=char>{
        from_char_stream::<JSON,Tokenizer<JSON>,_>(iter.into_iter())
    }
}

impl <JSON> FromJSONStr for JSON 
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    type Err = ParseError;
    fn from_json_str(s: &str) -> Result<Self, Self::Err> {
        from_char_stream::<JSON,Tokenizer<JSON>,_>(s.chars())
    }
}