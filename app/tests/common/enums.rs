use app::UserContext;

use crate::common::services::SessionToken;

pub enum Auth {
    Key(String),
    Token(SessionToken),
    Context(UserContext),
}
