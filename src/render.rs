use messages::{
    actor::Actor,
    prelude::{Address, Context, Notifiable},
};

use crate::net::NetworkManager;

pub struct Renderer {
    network: Address<NetworkManager>,
}

impl Renderer {
    pub fn new(network: Address<NetworkManager>) -> Self {
        Renderer { network }
    }
}

impl Actor for Renderer {}

pub struct RenderMsg;

#[async_trait::async_trait]
impl Notifiable<RenderMsg> for Renderer {
    /// Notify the renderer to refresh the user interface
    async fn notify(&mut self, _input: RenderMsg, _ctx: &Context<Self>) {}
}
