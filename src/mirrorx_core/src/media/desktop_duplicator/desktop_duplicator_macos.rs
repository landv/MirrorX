use super::macos::{
    av_capture_screen_input::AVCaptureScreenInput,
    av_capture_session::{AVCaptureSession, AVCaptureSessionPreset},
    av_capture_video_data_output::AVCaptureVideoDataOutput,
};
use crate::media::{bindings::macos::*, video_encoder::VideoEncoder};
use anyhow::bail;

pub struct DesktopDuplicator {
    capture_session: AVCaptureSession,
}

unsafe impl Send for DesktopDuplicator {}

impl DesktopDuplicator {
    pub fn new(fps: i32, encoder: VideoEncoder) -> anyhow::Result<Self> {
        let mut capture_session = AVCaptureSession::new();
        capture_session.begin_configuration();
        capture_session.set_session_preset(AVCaptureSessionPreset::AVCaptureSessionPresetHigh);

        let capture_screen_input = AVCaptureScreenInput::new(0);
        capture_screen_input.set_captures_cursor(true);
        capture_screen_input.set_captures_mouse_clicks(true);
        capture_screen_input.set_min_frame_duration(unsafe { CMTimeMake(1, fps) });
        // CMTimeMakeWithSeconds(DISPLAY_DELAY, 1000)
        if capture_session.can_add_input(&capture_screen_input) {
            capture_session.add_input(capture_screen_input);
        } else {
            bail!("can't add input");
        }

        let capture_video_data_output =
            AVCaptureVideoDataOutput::new(encoder, |encoder, cm_sample_buffer| unsafe {
                if !CMSampleBufferIsValid(cm_sample_buffer) {
                    tracing::error!("invalid sample buffer");
                    return;
                }

                let mut timing_info: CMSampleTimingInfo = std::mem::zeroed();
                let ret = CMSampleBufferGetSampleTimingInfo(cm_sample_buffer, 0, &mut timing_info);
                if ret != 0 {
                    tracing::error!(ret = ret, "CMSampleBufferGetSampleTimingInfo failed");
                    return;
                }

                let image_buffer = CMSampleBufferGetImageBuffer(cm_sample_buffer);
                if image_buffer.is_null() {
                    tracing::error!("CMSampleBufferGetImageBuffer failed");
                    return;
                }

                // let pix_fmt = CVPixelBufferGetPixelFormatType(image_buffer);

                let lock_result = CVPixelBufferLockBaseAddress(image_buffer, 1);
                if lock_result != 0 {
                    tracing::error!("CVPixelBufferLockBaseAddress failed");
                    return;
                }

                let width = CVPixelBufferGetWidth(image_buffer);
                let height = CVPixelBufferGetHeight(image_buffer);
                let y_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
                let y_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
                // let y_plane_height = CVPixelBufferGetHeightOfPlane(image_buffer, 0);

                let uv_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
                let uv_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

                encoder.encode(
                    y_plane_bytes_address as *mut u8,
                    y_plane_stride as i32,
                    uv_plane_bytes_address as *mut u8,
                    uv_plane_stride as i32,
                    timing_info.decode_timestamp.value,
                    timing_info.decode_timestamp.time_scale,
                    timing_info.presentation_timestamp.value,
                    timing_info.presentation_timestamp.time_scale,
                );

                CVPixelBufferUnlockBaseAddress(image_buffer, 1);
            });

        if capture_session.can_add_output(&capture_video_data_output) {
            capture_session.add_output(capture_video_data_output);
        } else {
            bail!("can't add output");
        }

        capture_session.commit_configuration();

        Ok(DesktopDuplicator { capture_session })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.capture_session.start_running();
        Ok(())
    }

    pub fn stop(&mut self) {
        self.capture_session.stop_running();
    }
}
