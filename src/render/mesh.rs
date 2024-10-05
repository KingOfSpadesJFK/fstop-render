use std::mem::size_of;
use cgmath::{vec2, vec3, vec4};
use vk::Buffer;
use vulkanalia::prelude::v1_3::*;
use anyhow::{anyhow, Ok, Result};

use super::{engine_data::EngineData, memory::AllocatedBuffer};

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Vec4 = cgmath::Vector4<f32>;

const TEST_TRIS: [Vertex; 3] = [
    Vertex {pos: vec2( 0.0, -0.5), color: vec3(1.0, 0.0, 0.0)},
    Vertex {pos: vec2( 0.5,  0.5), color: vec3(0.0, 1.0, 0.0)},
    Vertex {pos: vec2(-0.5,  0.5), color: vec3(0.0, 0.0, 1.0)},
];

const TEST_INDS: [u32; 3] = [ 0, 1, 2 ];

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec2,
    // norm: Vec3,
    color: Vec3,
    // uv: Vec2,
}

// const TEST_MESH: Mesh = Mesh::create(Box::new(TEST_TRIS), Box::new(TEST_INDS));
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertex_buffer: AllocatedBuffer,
    verts: Box<[Vertex]>,
    inds: Box<[u32]>,
}

impl Mesh {
    pub unsafe fn destroy(&mut self, device: &Device) {
        self.vertex_buffer.destroy(device);
    }

    pub fn from_vectors(verts: Vec<Vertex>, inds: Vec<u32>,
        instance: &Instance, device: &Device, data: &EngineData,) -> Result<Self> 
    {
        Ok(Mesh::create(verts.into_boxed_slice(), inds.into_boxed_slice(),
            instance, device, data)?)
    }

    pub fn create(verts: Box<[Vertex]>, inds: Box<[u32]>,
        instance: &Instance, device: &Device, data: &EngineData) -> Result<Self> 
    {
        // Create the vertex buffer
        let vertex_buffer = unsafe { AllocatedBuffer::create(
            verts.as_ptr(), 
            size_of::<Vertex>() * verts.len(), 
            instance, device, data) }?;

        Ok(Self { verts, inds, vertex_buffer })
    }

    pub fn binding_description(&self) -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()

    }

    pub fn attribute_descriptions(&self) -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vec2>() as u32)
            .build();
        
        return [pos, color]
    }

    pub fn get_vertex_count(&self) -> usize { return self.verts.len(); }
    pub fn get_index_count(&self) -> usize { return self.inds.len(); }
}

pub fn create_test_mesh(instance: &Instance, device: &Device, data: &EngineData) -> Result<Mesh> {
    let m = Mesh::create(Box::new(TEST_TRIS), Box::new(TEST_INDS),
        instance, device, data)?;
    Ok(m)
}