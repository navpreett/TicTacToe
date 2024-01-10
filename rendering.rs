use eframe::wgpu::{self, include_wgsl, util::DeviceExt};
use encase::{ShaderSize, UniformBuffer};

use crate::{Camera, PerObjectData, Vertex};

pub struct RenderState {
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    per_object_vertex_buffer: wgpu::Buffer,
    per_object_vertex_buffer_count: usize,
    per_object_vertex_buffer_max_size: usize,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: usize,
    render_pipeline: wgpu::RenderPipeline,
}

impl RenderState {
    pub fn new(wgpu_render_state: &eframe::egui_wgpu::RenderState) -> Self {
        let per_object_vertex_buffer =
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Per Object Vertex Buffer"),
                    contents: &[],
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        let per_object_vertex_buffer_count = 0;
        let per_object_vertex_buffer_max_size = 0;

        let shader = wgpu_render_state
            .device
            .create_shader_module(include_wgsl!("./shader.wgsl"));

        let vertex_data = [
            Vertex {
                position: (-0.5, 0.5).into(),
                tex_coord: (0.0, 1.0).into(),
            },
            Vertex {
                position: (0.5, 0.5).into(),
                tex_coord: (1.0, 1.0).into(),
            },
            Vertex {
                position: (0.5, -0.5).into(),
                tex_coord: (1.0, 0.0).into(),
            },
            Vertex {
                position: (-0.5, -0.5).into(),
                tex_coord: (0.0, 0.0).into(),
            },
        ];

        let vertices =
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertex_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_data = [0, 1, 2, 0, 2, 3];

        let indices =
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Index Buffer"),
                    contents: bytemuck::cast_slice(&index_data),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let index_count = index_data.len();

        let camera_bind_group_layout =
            wgpu_render_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Circle Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let render_pipeline_layout =
            wgpu_render_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            wgpu_render_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[PerObjectData::layout(), Vertex::layout()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu_render_state.target_format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        let camera_uniform_buffer = {
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Uniform Buffer"),
                    contents: &[0; <Camera as ShaderSize>::SHADER_SIZE.get() as _],
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                })
        };

        let camera_bind_group =
            wgpu_render_state
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Camera Bind Group"),
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_uniform_buffer.as_entire_binding(),
                    }],
                });

        Self {
            camera_uniform_buffer,
            camera_bind_group,
            per_object_vertex_buffer,
            per_object_vertex_buffer_count,
            per_object_vertex_buffer_max_size,
            vertices,
            indices,
            index_count,
            render_pipeline,
        }
    }

    pub fn prepare(
        &mut self,
        camera: Camera,
        data: &[PerObjectData],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
    ) {
        if data.len() * std::mem::size_of::<PerObjectData>()
            > self.per_object_vertex_buffer_max_size
        {
            self.per_object_vertex_buffer_max_size =
                data.len() * std::mem::size_of::<PerObjectData>();
            self.per_object_vertex_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Per Object Vertex Buffer"),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        } else {
            queue.write_buffer(
                &self.per_object_vertex_buffer,
                0,
                bytemuck::cast_slice(data),
            );
        }
        self.per_object_vertex_buffer_count = data.len();

        let mut buffer = UniformBuffer::new([0; <Camera as ShaderSize>::SHADER_SIZE.get() as _]);
        buffer.write(&camera).unwrap();
        let buffer = buffer.into_inner();
        queue.write_buffer(&self.camera_uniform_buffer, 0, &buffer);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.per_object_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(
            0..self.index_count as u32,
            0,
            0..self.per_object_vertex_buffer_count as u32,
        );
    }
}
