use ash::vk;

pub struct GraphicsPipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl GraphicsPipeline {
    pub fn new(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        swapchain_extent: vk::Extent2D,
    ) -> Self {
        let vert_shader_code = include_bytes!(concat!(env!("OUT_DIR"), "/node.vert.spv"));
        let frag_shader_code = include_bytes!(concat!(env!("OUT_DIR"), "/node.frag.spv"));

        let vert_shader_module = unsafe { create_shader_module(device, vert_shader_code) };
        let frag_shader_module = unsafe { create_shader_module(device, frag_shader_code) };

        let vert_stage = vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        };

        let frag_stage = vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        };

        let binding_desc = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }];

        let attrib_descs = [
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: 8, // 2 floats * 4 bytes
            },
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: binding_desc.len() as u32,
            p_vertex_binding_descriptions: binding_desc.as_ptr(),
            vertex_attribute_description_count: attrib_descs.len() as u32,
            p_vertex_attribute_descriptions: attrib_descs.as_ptr(),
            ..Default::default()
        };

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        }];

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: viewports.len() as u32,
            p_viewports: viewports.as_ptr(),
            scissor_count: scissors.len() as u32,
            p_scissors: scissors.as_ptr(),
            ..Default::default()
        };

        let rasterizer = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            ..Default::default()
        };

        let multisampling = vk::PipelineMultisampleStateCreateInfo {
            sample_shading_enable: vk::FALSE,
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };

        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
            blend_enable: vk::FALSE,
            ..Default::default()
        }];

        let color_blending = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            attachment_count: color_blend_attachment.len() as u32,
            p_attachments: color_blend_attachment.as_ptr(),
            ..Default::default()
        };

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(
                    &vk::PipelineLayoutCreateInfo::default(),
                    None,
                )
                .unwrap()
        };

        let stages = [vert_stage, frag_stage];
        let pipeline_info = vk::GraphicsPipelineCreateInfo {
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vertex_input,
            p_input_assembly_state: &input_assembly,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterizer,
            p_multisample_state: &multisampling,
            p_color_blend_state: &color_blending,
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            ..Default::default()
        };

        let pipeline = unsafe {
            device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[pipeline_info],
                    None,
                )
                .unwrap()[0]
        };

        unsafe {
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        }

        Self {
            pipeline,
            pipeline_layout,
        }
    }
}

pub unsafe fn create_shader_module(device: &ash::Device, code: &[u8]) -> vk::ShaderModule {
    let code_u32 = std::slice::from_raw_parts(code.as_ptr() as *const u32, code.len() / 4);
    let create_info = vk::ShaderModuleCreateInfo {
        code_size: code_u32.len() * 4,
        p_code: code_u32.as_ptr(),
        ..Default::default()
    };
    device.create_shader_module(&create_info, None).unwrap()
}

// Estructura de vértice que coincide con los descriptores
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}
