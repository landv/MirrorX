use super::shader;
use anyhow::bail;
use log::{error, info};
use std::mem::zeroed;
use windows::{
    core::PCSTR,
    Win32::Graphics::{Direct3D::*, Direct3D11::*, Dxgi::Common::*},
};

pub struct DX {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    vertex_shader: ID3D11VertexShader,
    pixel_shader_lumina: ID3D11PixelShader,
    pixel_shader_chrominance: ID3D11PixelShader,
    input_layout: ID3D11InputLayout,
}

impl DX {
    pub fn new() -> anyhow::Result<Self> {
        unsafe {
            let driver_types = [
                D3D_DRIVER_TYPE_HARDWARE,
                D3D_DRIVER_TYPE_WARP,
                D3D_DRIVER_TYPE_REFERENCE,
            ];

            let feature_levels = [
                D3D_FEATURE_LEVEL_11_1,
                D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_10_1,
                D3D_FEATURE_LEVEL_10_0,
            ];

            let mut device = None;
            let mut device_context = None;
            let mut feature_level = zeroed();

            for driver_type in driver_types {
                match D3D11CreateDevice(
                    None,
                    driver_type,
                    None,
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
                    &feature_levels,
                    D3D11_SDK_VERSION,
                    &mut device,
                    &mut feature_level,
                    &mut device_context,
                ) {
                    Ok(_) => {
                        info!(
                            r#"create_device: create device successfully {{"driver_type": "{}", "feature_level": "{}"}}"#,
                            match driver_type {
                                D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                                D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                                D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                                D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                                D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                                D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                                _ => "Unknown",
                            },
                            match feature_level {
                                D3D_FEATURE_LEVEL_12_2 => "D3D_FEATURE_LEVEL_12_2",
                                D3D_FEATURE_LEVEL_12_1 => "D3D_FEATURE_LEVEL_12_1",
                                D3D_FEATURE_LEVEL_12_0 => "D3D_FEATURE_LEVEL_12_0",
                                D3D_FEATURE_LEVEL_11_1 => "D3D_FEATURE_LEVEL_11_1",
                                D3D_FEATURE_LEVEL_11_0 => "D3D_FEATURE_LEVEL_11_0",
                                D3D_FEATURE_LEVEL_10_1 => "D3D_FEATURE_LEVEL_10_1",
                                D3D_FEATURE_LEVEL_10_0 => "D3D_FEATURE_LEVEL_10_0",
                                D3D_FEATURE_LEVEL_9_3 => "D3D_FEATURE_LEVEL_9_3",
                                D3D_FEATURE_LEVEL_9_2 => "D3D_FEATURE_LEVEL_9_2",
                                D3D_FEATURE_LEVEL_9_1 => "D3D_FEATURE_LEVEL_9_1",
                                D3D_FEATURE_LEVEL_1_0_CORE => "D3D_FEATURE_LEVEL_1_0_CORE",
                                _ => "Unknown",
                            }
                        );
                        break;
                    }
                    Err(err) => {
                        error!(
                            r#"create_device: failed to create device {{"driver_type": "{}", "error":"{}"}}"#,
                            match driver_type {
                                D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
                                D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
                                D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
                                D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
                                D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
                                D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
                                _ => "Unknown",
                            },
                            err
                        )
                    }
                };
            }

            if let (Some(device), Some(device_context)) = (device, device_context) {
                let vertex_shader = device.CreateVertexShader(shader::VERTEX_SHADER_BYTES, None)?;
                let pixel_shader_lumina =
                    device.CreatePixelShader(shader::PIXEL_SHADER_LUMINA_BYTES, None)?;
                let pixel_shader_chrominance =
                    device.CreatePixelShader(shader::PIXEL_SHADER_CHROMINANCE_BYTES, None)?;

                let input_element_desc_array = [
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: PCSTR(b"POSITION\0".as_ptr()),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32B32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 0,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: PCSTR(b"TEXCOORD\0".as_ptr()),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 12,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                ];

                let input_layout = device
                    .CreateInputLayout(&input_element_desc_array, shader::VERTEX_SHADER_BYTES)?;

                device_context.IASetInputLayout(&input_layout);

                Ok(DX {
                    device,
                    device_context,
                    vertex_shader,
                    pixel_shader_lumina,
                    pixel_shader_chrominance,
                    input_layout,
                })
            } else {
                bail!("create_device: create device failed with all driver types");
            }
        }
    }

    /// Get a reference to the dxresource's device.
    #[must_use]
    pub fn device(&self) -> &ID3D11Device {
        &self.device
    }

    /// Get a reference to the dxresource's device context.
    #[must_use]
    pub fn device_context(&self) -> &ID3D11DeviceContext {
        &self.device_context
    }

    /// Get a reference to the dxresource's vertex shader.
    #[must_use]
    pub fn vertex_shader(&self) -> &ID3D11VertexShader {
        &self.vertex_shader
    }

    /// Get a reference to the dxresource's pixel shader lumina.
    #[must_use]
    pub fn pixel_shader_lumina(&self) -> &ID3D11PixelShader {
        &self.pixel_shader_lumina
    }

    /// Get a reference to the dxresource's pixel shader chrominance.
    #[must_use]
    pub fn pixel_shader_chrominance(&self) -> &ID3D11PixelShader {
        &self.pixel_shader_chrominance
    }
}