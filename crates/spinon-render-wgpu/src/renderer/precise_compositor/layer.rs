use super::FLOAT_FORMAT;

pub(super) struct AccumulationLayer {
    textures: [wgpu::Texture; 2],
    views: [wgpu::TextureView; 2],
    pub(super) current: usize,
    pub(super) size: (u32, u32),
}

impl AccumulationLayer {
    pub(super) fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let textures = std::array::from_fn(|_| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("spinon-runtime-precise-accumulation-layer"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: FLOAT_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        });
        let views = std::array::from_fn(|index| {
            textures[index].create_view(&wgpu::TextureViewDescriptor::default())
        });
        Self {
            textures,
            views,
            current: 0,
            size: (width, height),
        }
    }

    pub(super) fn current_texture(&self) -> &wgpu::Texture {
        &self.textures[self.current]
    }

    pub(super) fn current_view(&self) -> &wgpu::TextureView {
        &self.views[self.current]
    }

    pub(super) fn next_texture(&self) -> &wgpu::Texture {
        &self.textures[1 - self.current]
    }

    pub(super) fn next_view(&self) -> &wgpu::TextureView {
        &self.views[1 - self.current]
    }

    pub(super) fn advance(&mut self) {
        self.current = 1 - self.current;
    }
}

pub(super) fn copy_to_next(encoder: &mut wgpu::CommandEncoder, layer: &AccumulationLayer) {
    encoder.copy_texture_to_texture(
        wgpu::TexelCopyTextureInfo {
            texture: layer.current_texture(),
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyTextureInfo {
            texture: layer.next_texture(),
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::Extent3d {
            width: layer.size.0,
            height: layer.size.1,
            depth_or_array_layers: 1,
        },
    );
}

pub(super) fn clear_layer(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    color: wgpu::Color,
) {
    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("spinon-runtime-precise-clear-pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
}
