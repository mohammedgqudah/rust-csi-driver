use tonic::{Request, Response, Status};

pub use controller_plugin::controller_server::{Controller, ControllerServer};
use controller_plugin::{ControllerGetCapabilitiesRequest, ControllerGetCapabilitiesResponse};

mod controller_plugin {
    tonic::include_proto!("csi.v1");
}

#[derive(Debug, Default)]
pub struct ControllerPlugin {}

#[tonic::async_trait]
impl Controller for ControllerPlugin {
    async fn create_volume(
        &self,
        _request: tonic::Request<controller_plugin::CreateVolumeRequest>,
    ) -> std::result::Result<tonic::Response<controller_plugin::CreateVolumeResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn delete_volume(
        &self,
        _request: tonic::Request<controller_plugin::DeleteVolumeRequest>,
    ) -> std::result::Result<tonic::Response<controller_plugin::DeleteVolumeResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn controller_publish_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerPublishVolumeRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerPublishVolumeResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn controller_unpublish_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerUnpublishVolumeRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerUnpublishVolumeResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn validate_volume_capabilities(
        &self,
        _request: tonic::Request<controller_plugin::ValidateVolumeCapabilitiesRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ValidateVolumeCapabilitiesResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn list_volumes(
        &self,
        _request: tonic::Request<controller_plugin::ListVolumesRequest>,
    ) -> std::result::Result<tonic::Response<controller_plugin::ListVolumesResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn get_capacity(
        &self,
        _request: tonic::Request<controller_plugin::GetCapacityRequest>,
    ) -> std::result::Result<tonic::Response<controller_plugin::GetCapacityResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn controller_get_capabilities(
        &self,
        _request: tonic::Request<controller_plugin::ControllerGetCapabilitiesRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerGetCapabilitiesResponse>,
        tonic::Status,
    > {
        Ok(Response::new(
            ControllerGetCapabilitiesResponse {
                capabilities: vec![
                    controller_plugin::ControllerServiceCapability {
                        r#type: Some(controller_plugin::controller_service_capability::Type::Rpc(
                            controller_plugin::controller_service_capability::Rpc {
                                r#type: controller_plugin::controller_service_capability::rpc::Type::CreateDeleteVolume as i32
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
    ) -> std::result::Result<
        tonic::Response<controller_plugin::CreateSnapshotResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn delete_snapshot(
        &self,
        _request: tonic::Request<controller_plugin::DeleteSnapshotRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::DeleteSnapshotResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn list_snapshots(
        &self,
        _request: tonic::Request<controller_plugin::ListSnapshotsRequest>,
    ) -> std::result::Result<tonic::Response<controller_plugin::ListSnapshotsResponse>, tonic::Status>
    {
        unimplemented!();
    }

    async fn controller_expand_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerExpandVolumeRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerExpandVolumeResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn controller_get_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerGetVolumeRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerGetVolumeResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }

    async fn controller_modify_volume(
        &self,
        _request: tonic::Request<controller_plugin::ControllerModifyVolumeRequest>,
    ) -> std::result::Result<
        tonic::Response<controller_plugin::ControllerModifyVolumeResponse>,
        tonic::Status,
    > {
        unimplemented!();
    }
}
