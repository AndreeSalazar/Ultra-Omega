use ash::vk;
use ab_glyph::{Font, FontArc, ScaleFont};
use std::collections::HashMap;

use crate::vulkan::pipeline::create_shader_module;

pub const ATLAS_FONT_SIZE: f32 = 64.0;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub pos: [f32; 2],
    pub tex_coord: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Clone, Debug)]
pub struct GlyphInfo {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub px_width: f32,
    pub px_height: f32,
    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
}

pub struct TextPipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
}

pub struct FontAtlas {
    pub texture: vk::Image,
    pub memory: vk::DeviceMemory,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_set: vk::DescriptorSet,
    pub glyph_cache: HashMap<char, GlyphInfo>,
    pub atlas_w: u32,
    pub atlas_h: u32,
    space_advance: f32,
}

impl FontAtlas {
    pub fn new(
        device: &ash::Device,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        descriptor_set_layout: vk::DescriptorSetLayout,
    ) -> Option<Self> {
        let font_data = Self::load_font_data()?;
        let font = FontArc::try_from_vec(font_data).ok()?;
        let scaled_font = font.as_scaled(ATLAS_FONT_SIZE);

        let mut glyphs: Vec<(char, GlyphInfo, u32, u32)> = Vec::new();
        let mut glyph_cache = HashMap::new();
        let mut max_glyph_height: u32 = 0;

        for code in 32u32..127 {
            let ch = char::from_u32(code)?;
            let glyph_id = font.glyph_id(ch);
            let glyph = glyph_id.with_scale_and_position(ATLAS_FONT_SIZE, ab_glyph::point(0.0, 0.0));

            let (gw, gh, bearing_x, bearing_y) = if let Some(outlined) = font.outline_glyph(glyph) {
                let pb = outlined.px_bounds();
                let w = (pb.width().ceil() as u32).max(1);
                let h = (pb.height().ceil() as u32).max(1);
                (w, h, pb.min.x as f32, -pb.min.y as f32)
            } else {
                (1, 1, 0.0, 0.0)
            };

            let advance = scaled_font.h_advance(glyph_id);

            let info = GlyphInfo {
                u0: 0.0, v0: 0.0, u1: 0.0, v1: 0.0,
                px_width: gw as f32,
                px_height: gh as f32,
                advance,
                bearing_x,
                bearing_y,
            };

            glyphs.push((ch, info.clone(), gw, gh));
            glyph_cache.insert(ch, info);
            max_glyph_height = max_glyph_height.max(gh);
        }

        let space_advance = glyph_cache.get(&' ').map(|g| g.advance).unwrap_or(14.0);

        let row_height = max_glyph_height + 2;
        let mut atlas_w: u32 = 0;
        let mut atlas_h: u32 = row_height;

        let mut rows: Vec<Vec<(usize, u32, u32)>> = Vec::new();
        let mut current_row: Vec<(usize, u32, u32)> = Vec::new();
        let mut current_row_width: u32 = 0;
        let max_atlas_width = 1024;

        for (idx, &(_, _, gw, _gh)) in glyphs.iter().enumerate() {
            if current_row_width + gw + 1 > max_atlas_width {
                rows.push(std::mem::take(&mut current_row));
                atlas_h += row_height;
                current_row_width = 0;
            }
            current_row.push((idx, current_row_width, gw));
            current_row_width += gw + 1;
            atlas_w = atlas_w.max(current_row_width);
        }
        if !current_row.is_empty() {
            rows.push(current_row);
        }

        if atlas_w == 0 || atlas_h == 0 {
            return None;
        }

        let mut pixel_data = vec![0u8; (atlas_w * atlas_h) as usize];

        for (row_idx, row) in rows.iter().enumerate() {
            let y_offset = row_idx as u32 * row_height;
            for &(glyph_idx, x_offset, _gw) in row {
                let (ch, _, gw, gh) = &mut glyphs[glyph_idx];
                let glyph_id = font.glyph_id(*ch);
                let glyph = glyph_id.with_scale_and_position(ATLAS_FONT_SIZE, ab_glyph::point(0.0, 0.0));

                if let Some(outlined) = font.outline_glyph(glyph) {
                    let pb = outlined.px_bounds();
                    outlined.draw(|x, y, coverage| {
                        let dst_x = x_offset + x;
                        let dst_y = y_offset + y;
                        if dst_x < atlas_w && dst_y < atlas_h {
                            let idx = (dst_y * atlas_w + dst_x) as usize;
                            if idx < pixel_data.len() {
                                pixel_data[idx] = (coverage * 255.0) as u8;
                            }
                        }
                    });

                    let gi = glyph_cache.get_mut(ch).unwrap();
                    gi.u0 = x_offset as f32 / atlas_w as f32;
                    gi.v0 = y_offset as f32 / atlas_h as f32;
                    gi.u1 = (x_offset + *gw) as f32 / atlas_w as f32;
                    gi.v1 = (y_offset + *gh) as f32 / atlas_h as f32;
                    gi.bearing_x = pb.min.x as f32;
                    gi.bearing_y = -pb.min.y as f32;
                }
            }
        }

        let (texture, memory) = Self::create_texture(device, instance, physical_device, atlas_w, atlas_h, &pixel_data)?;
        let view = Self::create_texture_view(device, texture)?;
        let sampler = Self::create_sampler(device)?;
        let (descriptor_pool, descriptor_set) = Self::create_descriptors(device, descriptor_set_layout, sampler, view)?;

        Some(Self {
            texture, memory, view, sampler, descriptor_pool, descriptor_set,
            glyph_cache, atlas_w, atlas_h, space_advance,
        })
    }

    fn load_font_data() -> Option<Vec<u8>> {
        let paths = [
            // High-quality coding fonts (preferred order)
            r"C:\Users\andre\AppData\Local\Microsoft\Windows\Fonts\JetBrainsMono-Regular.ttf",
            r"C:\Windows\Fonts\JetBrainsMono-Regular.ttf",
            r"C:\Users\andre\AppData\Local\Microsoft\Windows\Fonts\CascadiaCode-Regular.ttf",
            r"C:\Windows\Fonts\CascadiaCode-Regular.ttf",
            r"C:\Users\andre\AppData\Local\Microsoft\Windows\Fonts\SourceCodePro-Regular.ttf",
            r"C:\Windows\Fonts\SourceCodePro-Regular.ttf",
            r"C:\Users\andre\AppData\Local\Microsoft\Windows\Fonts\FiraCode-Regular.ttf",
            r"C:\Windows\Fonts\FiraCode-Regular.ttf",
            r"C:\Users\andre\AppData\Local\Microsoft\Windows\Fonts\UbuntuMono-R.ttf",
            r"C:\Windows\Fonts\UbuntuMono-R.ttf",
            // System monospace fonts (fallback)
            r"C:\Windows\Fonts\consola.ttf",
            r"C:\Windows\Fonts\cour.ttf",
            r"C:\Windows\Fonts\lucon.ttf",
            // Linux fonts
            "/usr/share/fonts/truetype/jetbrains/JetBrainsMono-Regular.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
        ];
        for path in &paths {
            if let Ok(data) = std::fs::read(path) {
                log::info!("Font loaded: {}", path);
                return Some(data);
            }
        }
        log::warn!("No font found - text will not render");
        None
    }

    pub fn space_advance(&self) -> f32 {
        self.space_advance
    }

    fn create_texture(
        device: &ash::Device,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        w: u32,
        h: u32,
        data: &[u8],
    ) -> Option<(vk::Image, vk::DeviceMemory)> {
        let image_create_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::R8_UNORM,
            extent: vk::Extent3D { width: w, height: h, depth: 1 },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::LINEAR,
            usage: vk::ImageUsageFlags::SAMPLED,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            ..Default::default()
        };

        let image = unsafe { device.create_image(&image_create_info, None).ok()? };
        let mem_req = unsafe { device.get_image_memory_requirements(image) };

        let mem_type = find_memory_type(instance, physical_device, mem_req.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)?;

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_req.size,
            memory_type_index: mem_type,
            ..Default::default()
        };
        let memory = unsafe { device.allocate_memory(&alloc_info, None).ok()? };
        unsafe { device.bind_image_memory(image, memory, 0).ok()? };

        let subresource = vk::ImageSubresource {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            array_layer: 0,
        };
        let layout = unsafe { device.get_image_subresource_layout(image, subresource) };

        unsafe {
            let ptr = device.map_memory(memory, 0, mem_req.size, vk::MemoryMapFlags::empty()).ok()? as *mut u8;
            for y in 0..h {
                let src_start = (y * w) as usize;
                let src_end = src_start + w as usize;
                if src_end > data.len() { break; }
                let src_row = &data[src_start..src_end];
                let dst_row = std::slice::from_raw_parts_mut(ptr.add(y as usize * layout.row_pitch as usize), w as usize);
                dst_row.copy_from_slice(src_row);
            }
            device.unmap_memory(memory);
        }

        Some((image, memory))
    }

    fn create_texture_view(device: &ash::Device, image: vk::Image) -> Option<vk::ImageView> {
        let create_info = vk::ImageViewCreateInfo {
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::R8_UNORM,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe { device.create_image_view(&create_info, None).ok() }
    }

    fn create_sampler(device: &ash::Device) -> Option<vk::Sampler> {
        let create_info = vk::SamplerCreateInfo {
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
            address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
            address_mode_w: vk::SamplerAddressMode::CLAMP_TO_EDGE,
            max_anisotropy: 1.0,
            ..Default::default()
        };
        unsafe { device.create_sampler(&create_info, None).ok() }
    }

    fn create_descriptors(
        device: &ash::Device,
        layout: vk::DescriptorSetLayout,
        sampler: vk::Sampler,
        view: vk::ImageView,
    ) -> Option<(vk::DescriptorPool, vk::DescriptorSet)> {
        let pool_size = vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        };
        let pool_info = vk::DescriptorPoolCreateInfo {
            max_sets: 1,
            pool_size_count: 1,
            p_pool_sizes: &pool_size,
            ..Default::default()
        };
        let pool = unsafe { device.create_descriptor_pool(&pool_info, None).ok()? };

        let alloc_info = vk::DescriptorSetAllocateInfo {
            descriptor_pool: pool,
            descriptor_set_count: 1,
            p_set_layouts: &layout,
            ..Default::default()
        };
        let sets = unsafe { device.allocate_descriptor_sets(&alloc_info).ok()? };
        let set = sets[0];

        let image_info = vk::DescriptorImageInfo {
            sampler,
            image_view: view,
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        };
        let write = vk::WriteDescriptorSet {
            dst_set: set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_image_info: &image_info,
            ..Default::default()
        };
        unsafe { device.update_descriptor_sets(&[write], &[]); }

        Some((pool, set))
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_sampler(self.sampler, None);
            device.destroy_image_view(self.view, None);
            device.destroy_image(self.texture, None);
            device.free_memory(self.memory, None);
            device.destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}

impl TextPipeline {
    pub fn new(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        swapchain_extent: vk::Extent2D,
    ) -> Self {
        let vert_code = include_bytes!(concat!(env!("OUT_DIR"), "/text.vert.spv"));
        let frag_code = include_bytes!(concat!(env!("OUT_DIR"), "/text.frag.spv"));

        let vert_module = unsafe { create_shader_module(device, vert_code) };
        let frag_module = unsafe { create_shader_module(device, frag_code) };

        let stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vert_module,
                p_name: c"main".as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: frag_module,
                p_name: c"main".as_ptr(),
                ..Default::default()
            },
        ];

        let binding_desc = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<TextVertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }];

        let attrib_descs = [
            vk::VertexInputAttributeDescription {
                binding: 0, location: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                binding: 0, location: 1,
                format: vk::Format::R32G32_SFLOAT,
                offset: 8,
            },
            vk::VertexInputAttributeDescription {
                binding: 0, location: 2,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: 16,
            },
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: binding_desc.len() as u32,
            p_vertex_binding_descriptions: binding_desc.as_ptr(),
            vertex_attribute_description_count: attrib_descs.len() as u32,
            p_vertex_attribute_descriptions: attrib_descs.as_ptr(),
            ..Default::default()
        };

        let descriptor_set_layout = unsafe {
            let binding = vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            };
            let layout_info = vk::DescriptorSetLayoutCreateInfo {
                binding_count: 1,
                p_bindings: &binding,
                ..Default::default()
            };
            device.create_descriptor_set_layout(&layout_info, None).unwrap()
        };

        let pipeline_layout = unsafe {
            let layout_info = vk::PipelineLayoutCreateInfo {
                set_layout_count: 1,
                p_set_layouts: &descriptor_set_layout,
                ..Default::default()
            };
            device.create_pipeline_layout(&layout_info, None).unwrap()
        };

        let viewports = [vk::Viewport {
            x: 0.0, y: 0.0,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.0, max_depth: 1.0,
        }];
        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        }];

        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
        }];

        let pipeline_info = vk::GraphicsPipelineCreateInfo {
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vertex_input,
            p_input_assembly_state: &vk::PipelineInputAssemblyStateCreateInfo {
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: vk::FALSE,
                ..Default::default()
            },
            p_viewport_state: &vk::PipelineViewportStateCreateInfo {
                viewport_count: 1,
                p_viewports: viewports.as_ptr(),
                scissor_count: 1,
                p_scissors: scissors.as_ptr(),
                ..Default::default()
            },
            p_rasterization_state: &vk::PipelineRasterizationStateCreateInfo {
                depth_clamp_enable: vk::FALSE,
                rasterizer_discard_enable: vk::FALSE,
                polygon_mode: vk::PolygonMode::FILL,
                line_width: 1.0,
                cull_mode: vk::CullModeFlags::NONE,
                front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_bias_enable: vk::FALSE,
                ..Default::default()
            },
            p_multisample_state: &vk::PipelineMultisampleStateCreateInfo {
                sample_shading_enable: vk::FALSE,
                rasterization_samples: vk::SampleCountFlags::TYPE_1,
                ..Default::default()
            },
            p_color_blend_state: &vk::PipelineColorBlendStateCreateInfo {
                logic_op_enable: vk::FALSE,
                attachment_count: 1,
                p_attachments: color_blend_attachment.as_ptr(),
                ..Default::default()
            },
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            ..Default::default()
        };

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .unwrap()[0]
        };

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        Self { pipeline, pipeline_layout, descriptor_set_layout }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
    }
}

fn find_memory_type(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mem = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    for i in 0..mem.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && mem.memory_types[i as usize].property_flags & properties == properties
        {
            return Some(i);
        }
    }
    None
}
