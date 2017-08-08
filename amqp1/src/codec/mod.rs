use bytes::BytesMut;

mod decode;
mod encode;

pub trait Encodable {
    fn encode(&self, buf: &mut BytesMut) -> ();
}

pub use self::decode::{
    null,
    ubyte,
    ushort,
    uint,
    ulong,
    byte,
    short,
    int,
    long,
    float,
    double,
    timestamp,
    uuid,
    binary,
    string,
    symbol,
};

pub use self::decode::variant;