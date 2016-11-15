use ws;
use std::fmt;
use std::net;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// This module explores inter-thread communication via a queue and piping the contents of the queue
// into the connected WS clients.

type PipeIn<T> = mpsc::SyncSender<T>;
type PipeOut<T> = mpsc::Receiver<T>;
type SubscriptionId = ws::util::Token;

#[derive(Debug)]
enum DataMessage {
    Value(i32),
    Done,
}

#[derive(Debug)]
enum Filter {
    Gt(i32),
    Lt(i32),
}

struct Subscription {
    filter: Filter,
    pipe: PipeIn<DataMessage>,
}

struct Production {
    sender: ws::Sender,
    out: PipeOut<DataMessage>,
}

struct WsHandler {
    sender: ws::Sender,
    in_chans: Arc<Mutex<HashMap<SubscriptionId, Subscription>>>,
    out_chans: Arc<Mutex<HashMap<SubscriptionId, Production>>>
}

impl ws::Handler for WsHandler {
    fn on_open(&mut self, hs: ws::Handshake) -> Result<(), ws::Error> {
        info!("Client connected: {:?}", hs);
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        info!("Received message: {:?}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        match code {
            ws::CloseCode::Normal => info!("Client closed the connection"),
            ws::CloseCode::Away => info!("Client timed out"),
            ws::CloseCode::Abnormal => warn!("Closing handshake failed!"),
            _ => warn!("Client encountered an error: {}!", reason),
        }

        self.in_chans.lock().unwrap().remove(&self.sender.token());
        self.out_chans.lock().unwrap().remove(&self.sender.token());
    }

    fn on_error(&mut self, err: ws::Error) {
        error!("Error communicating with client!");
    }
}

pub fn start<A>(listen_address: A, buffer_size: usize)
    where A: net::ToSocketAddrs + fmt::Debug + Send + Copy + 'static
{
    let in_ref = Arc::new(Mutex::new(HashMap::new()));
    let out_ref = Arc::new(Mutex::new(HashMap::new()));

    {
        let in_s_chans = in_ref.clone();
        let out_s_chans = out_ref.clone();
        let server = thread::Builder::new().name("server".to_owned()).spawn(move || {
            println!("> started server...");
            ws::listen(listen_address, |sender| {
                println!("> got connection...");
                let in_chans = in_s_chans.clone();
                let out_chans = out_s_chans.clone();

                let (msg_in, msg_out) = mpsc::sync_channel(buffer_size);
                let sub = Subscription { filter: Filter::Gt(5), pipe: msg_in };
                let prd = Production { sender: sender.clone(), out: msg_out };

                let token = sender.token().clone();
                in_chans.lock().unwrap().insert(token, sub);
                out_chans.lock().unwrap().insert(token, prd);

                WsHandler {
                    sender: sender,
                    in_chans: in_chans,
                    out_chans: out_chans,
                }
            }).expect(&*format!("Could not start a server at {:?}!", listen_address))
        }).expect(&*format!("Could not start the server thread at {:?}!", listen_address));
    }

    {
        let p_chans = in_ref.clone();
        let producer = thread::Builder::new().name("producer".to_owned()).spawn(move || {
            println!("> started producer...");
            for i in 0..100 {
                for (_, sub) in &*p_chans.lock().unwrap() {
                    sub.pipe.send(DataMessage::Value(i)).unwrap();
                }
                thread::sleep_ms(1000);
            }
            for (_, sub) in &*p_chans.lock().unwrap() {
                sub.pipe.send(DataMessage::Done).unwrap();
            }
            info!("Producer is done!");
        }).expect("Could not start the producer!");
    }

    {
        let o_chans = out_ref.clone();
        let pipe = thread::Builder::new().name("pipe".to_owned()).spawn(move || {
            println!("> started pipe...");
            loop {
                for (_, prod) in &*o_chans.lock().unwrap() {
                    if let Ok(msg) = prod.out.try_recv() {
                        debug!("Sending {:?} to {:?}...", msg, prod.sender);
                        prod.sender.send(format!("Got: {:?}", msg)).unwrap();
                    }
                }
            }
        }).expect("Could not start the pipe!");
    }
}
