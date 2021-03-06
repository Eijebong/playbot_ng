use failure::Error;
use std::sync::Arc;

pub trait Message {
    /// The body of the message without address prefixes.
    /// E.g. `bot: hello` would be returned as `hello`.
    fn body(&self) -> &str;

    /// Wether the message was aimed directetly at the bot,
    /// either via private message or by prefixing a channel message with
    /// the bot's name, followed by ',' or ':'.
    fn is_directly_addressed(&self) -> bool;

    fn reply(&self, message: &str) -> Result<(), Error>;

    fn source_nickname(&self) -> &str;

    fn current_nickname(&self) -> Arc<String>;
}
