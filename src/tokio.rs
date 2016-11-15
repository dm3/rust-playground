//use tokio_core::io::Io;
//use tokio_proto::easy::{EasyFramed, EasyBuf, Parse, Serialize};
//use serde_json as json;
//use futures::{Async, Poll};
//use std::{io, str};
//
//// tokio_core::easy doesn't support streaming protocols
//
//#[derive(Debug, PartialEq, Serialize, Deserialize)]
//struct Message {
//    value: f32,
//}
//
//pub struct Parser;
//
//impl Parse for Parser {
//    type Out = Message;
//
//    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
//    }
//}
//
//pub struct Serializer;
//
//pub type FramedMessageTransport<T> = EasyFramed<T, Parser, Serializer>;
//
//pub fn new_json_transport<T>(inner: T) -> FramedMessageTransport<T> where T: Io {
//    EasyFramed::new(inner, Parser, Serializer)
//}
