use eframe::wgpu;
use memoffset::offset_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PerObjectData {
    pub object_position: cgmath::Vector2<f32>,
    pub rotation: f32,
    pub scale: cgmath::Vector2<f32>,
    pub color: cgmath::Vector3<f32>,
    pub is_circle: u32,
    pub circle_width: f32,
}

// cgmath::Vector2 doesnt implement these traits, but i know its valid
unsafe impl bytemuck::Zeroable for PerObjectData {}
unsafe impl bytemuck::Pod for PerObjectData {}

impl PerObjectData {
    pub const ATTRIBUTES: &[wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: offset_of!(Self, object_position) as wgpu::BufferAddress,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x2,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Self, rotation) as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Self, scale) as wgpu::BufferAddress,
            shader_location: 2,
            format: wgpu::VertexFormat::Float32x2,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Self, color) as wgpu::BufferAddress,
            shader_location: 3,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Self, is_circle) as wgpu::BufferAddress,
            shader_location: 4,
            format: wgpu::VertexFormat::Uint32,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Self, circle_width) as wgpu::BufferAddress,
            shader_location: 5,
            format: wgpu::VertexFormat::Float32,
        },
    ];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}
