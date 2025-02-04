use crate::error::CoreResult;
use tonic::transport::Channel;

pub struct VisitReplyRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub allow: bool,
}

pub async fn visit_reply(
    mut client: signaling_proto::service::signaling_client::SignalingClient<Channel>,
    req: VisitReplyRequest,
) -> CoreResult<()> {
    let _ = client
        .visit_reply(signaling_proto::message::VisitReplyRequest {
            active_device_id: req.active_device_id,
            passive_device_id: req.passive_device_id,
            allow: req.allow,
        })
        .await?
        .into_inner();

    Ok(())
}
