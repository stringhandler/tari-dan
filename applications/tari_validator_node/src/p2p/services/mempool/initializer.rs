//  Copyright 2022. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::sync::Arc;

use async_trait::async_trait;
use tari_comms_dht::Dht;
use tari_dan_core::services::mempool::service::MempoolServiceHandle;
use tari_p2p::comms_connector::SubscriptionFactory;
use tari_service_framework::{ServiceInitializationError, ServiceInitializer, ServiceInitializerContext};

use super::{inbound::TariCommsMempoolInboundHandle, outbound::TariCommsMempoolOutboundService};

pub struct MempoolInitializer {
    mempool: MempoolServiceHandle,
    inbound_message_subscription_factory: Arc<SubscriptionFactory>,
}

impl MempoolInitializer {
    pub fn new(mempool: MempoolServiceHandle, inbound_message_subscription_factory: Arc<SubscriptionFactory>) -> Self {
        Self {
            mempool,
            inbound_message_subscription_factory,
        }
    }
}

#[async_trait]
impl ServiceInitializer for MempoolInitializer {
    async fn initialize(&mut self, context: ServiceInitializerContext) -> Result<(), ServiceInitializationError> {
        let mut mempool_service = self.mempool.clone();
        let mut mempool_inbound = TariCommsMempoolInboundHandle::new(
            self.inbound_message_subscription_factory.clone(),
            mempool_service.clone(),
        );
        context.register_handle(mempool_inbound.clone());

        context.spawn_until_shutdown(move |handles| async move {
            let dht = handles.expect_handle::<Dht>();
            let outbound_requester = dht.outbound_requester();
            let mempool_outbound = TariCommsMempoolOutboundService::new(outbound_requester);
            mempool_service.set_outbound_service(Box::new(mempool_outbound)).await;

            mempool_inbound.run().await;
        });

        Ok(())
    }
}
