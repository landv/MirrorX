use super::VisitState;
use mirrorx_core::{
    api::endpoint::{message::InputEvent, EndPointClient},
    error::CoreError,
    DesktopDecodeFrame,
};
use strum_macros::AsRefStr;
use tokio::sync::mpsc::Receiver;

#[derive(AsRefStr)]
pub enum Event {
    ConnectEndPoint {
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
        addr: String,
    },

    UpdateEndPointClient {
        client: EndPointClient,
    },

    UpdateVisitState {
        new_state: VisitState,
    },

    UpdateUseOriginalResolution {
        use_original_resolution: bool,
    },

    UpdateError {
        err: CoreError,
    },

    SetRenderFrameReceiver {
        render_rx: Receiver<DesktopDecodeFrame>,
    },

    EmitNegotiateDesktopParams,
    EmitNegotiateFinish {
        expected_frame_rate: u8,
    },
}
