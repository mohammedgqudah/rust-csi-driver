use log::{debug, error, info};
use tonic::{Response, Status};

pub use controller_plugin::controller_server::{Controller, ControllerServer};
use controller_plugin::ControllerGetCapabilitiesResponse;

mod controller_plugin {
    tonic::include_proto!("csi.v1");
}

async fn volume_exists(volume_name: &str) -> Result<bool, tonic::Status> {
    let path = std::path::Path::new(volume_name);

    path.try_exists().map_err(|err| {
        error!(
            "Couldn't check if the volume {} exists: {}",
            volume_name, err
        );
        Status::new(tonic::Code::Internal, err.to_string())
    })
}

#[derive(Debug, Default)]
pub struct ControllerPlugin {}

#[tonic::async_trait]
impl Controller for ControllerPlugin {
    async fn create_volume(
        &self,
        request: tonic::Request<controller_plugin::CreateVolumeRequest>,
    ) -> Result<Response<controller_plugin::CreateVolumeResponse>, tonic::Status> {
        let message = request.into_inner();
        let volume_name = format!("volumes/{}", message.name);
        let path = std::path::Path::new(&volume_name);

        debug!("requesting to create volume {}", volume_name);

        // This operation MUST be idempotent...
        // if the specified volume name already exists...
        // the Plugin MUST reply 0 OK
        if !volume_exists(&volume_name).await? {
            debug!("volume {} does not exist, creating..", volume_name);
            std::fs::create_dir_all(&path).map_err(|e| {
                error!("Couldn't create volume {volume_name}: {}", e.to_string());
                Status::new(tonic::Code::Internal, e.to_string())
            })?;
        }

        let reply = controller_plugin::CreateVolumeResponse {
            volume: Some(controller_plugin::Volume {
                volume_id: message.name,
                volume_context: std::collections::HashMap::new(),
                accessible_topology: vec![],
                capacity_bytes: 100,
                content_source: None,
            }),
        };

        Ok(Response::new(reply))
    }

    async fn delete_volume(
        &self,
        request: tonic::Request<controller_plugin::DeleteVolumeRequest>,
    ) -> Result<Response<controller_plugin::DeleteVolumeResponse>, tonic::Status> {
        let message = request.into_inner();
        // in CreateVolume, the name is used as the volume_id
        // this may change later, depending on the storage backend
        let volume_name = format!("volumes/{}", message.volume_id);
        let path = std::path::Path::new(&volume_name);

        debug!("requesting to delete volume {}", volume_name);

        if volume_exists(&volume_name).await? {
            std::fs::remove_dir_all(&path).map_err(|e| {
                error!("Couldn't delete volume {volume_name}: {}", e.to_string());
                Status::new(tonic::Code::Internal, e.to_string())
            })?;
        }

        Ok(Response::new(controller_plugin::DeleteVolumeResponse {}))
    }

    async fn controller_publish_volume(
        &self,
        request: tonic::Request<controller_plugin::ControllerPublishVolumeRequest>,
    ) -> Result<Response<controller_plugin::ControllerPublishVolumeResponse>, tonic::Status> {
        let message = request.into_inner();
        let volume_name = format!("volumes/{}", message.volume_id);
        if !volume_exists(&volume_name).await? {
            return Err(Status::new(tonic::Code::NotFound, "volume does not exist"));
        }

        info!(
            "publishing volume {} for node {}",
            message.volume_id, message.node_id
        );

        Ok(Response::new(
            controller_plugin::ControllerPublishVolumeResponse {
                publish_context: std::collections::HashMap::new(),
            },
        ))
    }

    async fn controller_unpublish_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerUnpublishVolumeRequest>,
    ) -> Result<Response<controller_plugin::ControllerUnpublishVolumeResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn validate_volume_capabilities(
        &self,
        _request: tonic::Request<controller_plugin::ValidateVolumeCapabilitiesRequest>,
    ) -> Result<Response<controller_plugin::ValidateVolumeCapabilitiesResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn list_volumes(
        &self,
        _request: tonic::Request<controller_plugin::ListVolumesRequest>,
    ) -> Result<Response<controller_plugin::ListVolumesResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn get_capacity(
        &self,
        _request: tonic::Request<controller_plugin::GetCapacityRequest>,
    ) -> Result<Response<controller_plugin::GetCapacityResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn controller_get_capabilities(
        &self,
        _request: tonic::Request<controller_plugin::ControllerGetCapabilitiesRequest>,
    ) -> Result<Response<controller_plugin::ControllerGetCapabilitiesResponse>, tonic::Status> {
        Ok(Response::new(
            ControllerGetCapabilitiesResponse {
                capabilities: vec![
                    controller_plugin::ControllerServiceCapability {
                        r#type: Some(controller_plugin::controller_service_capability::Type::Rpc(
                            controller_plugin::controller_service_capability::Rpc {
                                r#type: controller_plugin::controller_service_capability::rpc::Type::CreateDeleteVolume as i32
                            }
                        ))
                    },
                    controller_plugin::ControllerServiceCapability {
                        r#type: Some(controller_plugin::controller_service_capability::Type::Rpc(
                            controller_plugin::controller_service_capability::Rpc {
                                r#type: controller_plugin::controller_service_capability::rpc::Type::PublishUnpublishVolume as i32
                            }
                        ))
                    }
                ]
            }
        ))
    }

    async fn create_snapshot(
        &self,
        _request: tonic::Request<controller_plugin::CreateSnapshotRequest>,
    ) -> Result<Response<controller_plugin::CreateSnapshotResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn delete_snapshot(
        &self,
        _request: tonic::Request<controller_plugin::DeleteSnapshotRequest>,
    ) -> Result<Response<controller_plugin::DeleteSnapshotResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn list_snapshots(
        &self,
        _request: tonic::Request<controller_plugin::ListSnapshotsRequest>,
    ) -> Result<Response<controller_plugin::ListSnapshotsResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn controller_expand_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerExpandVolumeRequest>,
    ) -> Result<Response<controller_plugin::ControllerExpandVolumeResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn controller_get_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerGetVolumeRequest>,
    ) -> Result<Response<controller_plugin::ControllerGetVolumeResponse>, tonic::Status> {
        unimplemented!();
    }

    async fn controller_modify_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerModifyVolumeRequest>,
    ) -> Result<Response<controller_plugin::ControllerModifyVolumeResponse>, tonic::Status> {
        unimplemented!();
    }
}
