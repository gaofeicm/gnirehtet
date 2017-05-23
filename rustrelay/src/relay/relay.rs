use mio::*;

use super::selector::{EventHandler,Selector};
use super::tunnelserver::TunnelServer;

static TAG: &'static str = "Relay";

pub struct Relay {
    port: u16,
}

impl Relay {
    pub fn new(port: u16) -> Relay {
        Relay {
            port: port,
        }
    }

    pub fn start(&self) {
        info!(target: TAG, "Starting server...");
        let mut selector = Selector::new().unwrap();
        let _tunnel_server = TunnelServer::new(self.port, &mut selector);
        self.poll_loop(&mut selector);
    }

    fn poll_loop(&self, selector: &mut Selector) {
        loop {
            let mut events = Events::with_capacity(1024);
            selector.poll.poll(&mut events, None).expect("Cannot poll");

            for event in &events {
                println!("event={:?}", event);
                // the handler is stored in the selector, so we need to clone
                // the Rc to pass a &mut Selector to on_ready()
                let mut handler = selector.handlers.get_mut(event.token()).unwrap().clone();
                handler.on_ready(selector, event.readiness());
            }
        }
    }
}
