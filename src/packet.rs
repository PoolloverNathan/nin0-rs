use std::borrow::{ Cow, ToOwned };
use std::time::Instant;

pub use crate::roles::Roles;

pub use secrecy::SecretBox;

trait Alloc {
    type Static: 'static;
    fn alloc(self) -> Self::Static;
}

impl<'a, T: ?Sized + ToOwned + 'static> Alloc for Cow<'a, T> {
    type Static = Cow<'static, T>;
    fn alloc(self) -> Cow<'static, T> {
        Cow::Owned(self.into_owned())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Device {
    Web,
    Mobile,
    Bot,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Message<'a> {
    todo: Cow<'a, !>,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Member<'a> {
    todo: Cow<'a, !>,
}

#[derive(Clone, Debug)]
pub struct BridgeMetadata<'a> {
    from: Cow<'a, str>,
    color: rgb::RGB8,
}

#[derive(Clone, Debug)]
pub enum Credentials<'a> {
    Anon(Cow<'a, str>),
    Token(Cow<'a, SecretBox<str>>),
}

#[derive(Clone, Debug)]
pub enum C2SPacket<'a> {
    Message(Cow<'a, str>, Option<BridgeMetadata<'a>>),
    Login(Credentials<'a>, Device),
    Heartbeat,
    Ping,
}

/// Errors will be returned as a read failure, rather than a packet.
#[derive(Clone, Debug)]
pub enum S2CPacket<'a> {
    Message(Message<'a>),
    Login(Cow<'a, str>, Roles),
    Heartbeat,
    History(Vec<Message<'a>>),
    Ping(Instant),
    Users(Vec<Member<'a>>),
}
