use messages::{
    actor::Actor,
    address::Address,
    prelude::{Context, Notifiable},
};

use crate::render::Renderer;

pub struct NetworkManager {
    own_address: Address<NetworkManager>,
    renderer: Address<Renderer>,
}

impl NetworkManager {
    pub fn new(own_address: Address<NetworkManager>, renderer: Address<Renderer>) -> Self {
        NetworkManager {
            own_address,
            renderer,
        }
    }
}

#[async_trait::async_trait]
impl Actor for NetworkManager {
    async fn started(&mut self) {}
}

#[async_trait::async_trait]
impl Notifiable<ReactToNotify> for NetworkManager {
    async fn notify(&mut self, _input: ReactToNotify, _context: &Context<Self>) {}
}

#[derive(Debug)]
struct ReactToNotify {
    group: String,
    id: String,
}
