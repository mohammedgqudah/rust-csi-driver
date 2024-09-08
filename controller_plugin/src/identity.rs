use std::collections::HashMap;
use tonic::{Request, Response, Status};

use controller_plugin::identity_server::Identity;
pub use controller_plugin::identity_server::IdentityServer;
use controller_plugin::{
    plugin_capability, GetPluginCapabilitiesRequest,
    GetPluginCapabilitiesResponse, GetPluginInfoRequest, GetPluginInfoResponse, PluginCapability,
    ProbeRequest, ProbeResponse,
};

mod controller_plugin {
    tonic::include_proto!("csi.v1");
}

#[derive(Debug, Default)]
pub struct ControllerIdentity {}

#[tonic::async_trait]
impl Identity for ControllerIdentity {
    async fn get_plugin_info(
        &self,
        _request: Request<GetPluginInfoRequest>,
    ) -> Result<Response<GetPluginInfoResponse>, Status> {
        Ok(Response::new(GetPluginInfoResponse {
            name: "rust-csi-driver".to_string(),
            vendor_version: "0.0.1".to_string(),
            manifest: HashMap::new(),
        }))
    }

    async fn get_plugin_capabilities(
        &self,
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
        let reply = GetPluginCapabilitiesResponse {
            capabilities: vec![PluginCapability {
                r#type: Some(plugin_capability::Type::Service(
                    plugin_capability::Service {
                        r#type: plugin_capability::service::Type::ControllerService as i32,
                    },
                )),
            }],
        };

        Ok(Response::new(reply))
    }

    // TODO: check the underlying filesystem + storage
    async fn probe(
        &self,
        _request: Request<ProbeRequest>,
    ) -> Result<Response<ProbeResponse>, Status> {
        let reply = ProbeResponse { ready: Some(true) };

        Ok(Response::new(reply))
    }
}
