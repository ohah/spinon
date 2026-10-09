use std::ffi::c_void;
#[cfg(feature = "test-hooks")]
use std::sync::atomic::{AtomicBool, Ordering};

use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, RawDisplayHandle, RawWindowHandle,
    UiKitDisplayHandle, UiKitWindowHandle,
};
use spinon_render::RuntimeRenderSnapshot;

use crate::geometry::{Vertex, build_vertices};

mod backend;
use backend::{android_auto_backend_order, backend_from_abi, choose_sdr_format, color_encoding};

const SHADER: &str = include_str!("runtime.wgsl");

#[derive(Debug)]
pub enum RuntimeRendererError {
    InvalidArgument(&'static str),
    Initialization(String),
    Surface(String),
    Geometry(String),
}

impl std::fmt::Display for RuntimeRendererError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(message) => formatter.write_str(message),
            Self::Initialization(message) | Self::Surface(message) | Self::Geometry(message) => {
                formatter.write_str(message)
            }
        }
    }
}

impl std::error::Error for RuntimeRendererError {}

pub struct RuntimeRenderer {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: u64,
    vertex_count: u32,
    last_surface_texture_size: (u32, u32),
    surface_configured: bool,
    #[cfg(feature = "test-hooks")]
    draw_failure_injector: DrawFailureInjector,
}

#[cfg(feature = "test-hooks")]
#[derive(Default)]
struct DrawFailureInjector(AtomicBool);

#[cfg(feature = "test-hooks")]
impl DrawFailureInjector {
    fn arm(&self) {
        self.0.store(true, Ordering::Release);
    }

    fn take(&self) -> bool {
        self.0.swap(false, Ordering::AcqRel)
    }
}

/// UIKit view에서 메인 스레드로 생성한 surface와 그 인스턴스입니다.
#[doc(hidden)]
pub struct PendingRuntimeSurface {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
}

impl RuntimeRenderer {
    /// `native_window`는 renderer가 파괴될 때까지 플랫폼에서 살아 있어야 합니다.
    ///
    /// # Safety
    /// 포인터는 유효한 `ANativeWindow`여야 하며 renderer가 파괴될 때까지 수명이 유지되어야 합니다.
    pub unsafe fn new_android(
        native_window: *mut c_void,
        width: u32,
        height: u32,
        backend: u32,
    ) -> Result<(Self, String), RuntimeRendererError> {
        if width == 0 || height == 0 {
            return Err(RuntimeRendererError::InvalidArgument(
                "WGPU 표면 크기는 0보다 커야 합니다",
            ));
        }
        if !matches!(backend, 0..=2) {
            return Err(RuntimeRendererError::InvalidArgument(
                "Android GPU backend 값이 올바르지 않습니다",
            ));
        }
        let native_window = std::ptr::NonNull::new(native_window).ok_or(
            RuntimeRendererError::InvalidArgument("ANativeWindow가 null입니다"),
        )?;
        let display = RawDisplayHandle::Android(AndroidDisplayHandle::new());
        let window = RawWindowHandle::AndroidNdk(AndroidNdkWindowHandle::new(native_window));
        if backend != 0 {
            let pending = unsafe { Self::prepare_surface(display, window, backend) }?;
            return Self::initialize_surface(pending, width, height);
        }

        let mut failures = Vec::new();
        for candidate in android_auto_backend_order() {
            let result = unsafe { Self::prepare_surface(display, window, candidate) }
                .and_then(|pending| Self::initialize_surface(pending, width, height));
            match result {
                Ok((renderer, report)) => {
                    return Ok((renderer, format!("{report} selection=auto")));
                }
                Err(error) => failures.push(format!("backend={candidate}: {error}")),
            }
        }
        Err(RuntimeRendererError::Initialization(format!(
            "Android에서 Vulkan·GL 초기화가 모두 실패했습니다: {}",
            failures.join("; ")
        )))
    }

    /// UIKit view에서 WGPU surface를 생성합니다. UIKit 접근이 있으므로 메인 스레드에서만 호출합니다.
    /// 반환된 surface는 이후 다른 전용 렌더 스레드에서 초기화할 수 있습니다.
    ///
    /// # Safety
    /// 포인터는 유효한 UIKit `UIView`여야 하며 WGPU surface가 파괴될 때까지 view와 layer가 살아 있어야 합니다.
    /// 반드시 iOS 메인 스레드에서 호출해야 합니다.
    pub unsafe fn prepare_uikit_surface(
        view: *mut c_void,
        backend: u32,
    ) -> Result<PendingRuntimeSurface, RuntimeRendererError> {
        if backend != 3 {
            return Err(RuntimeRendererError::InvalidArgument(
                "UIKit GPU backend는 Metal이어야 합니다",
            ));
        }
        let view = std::ptr::NonNull::new(view).ok_or(RuntimeRendererError::InvalidArgument(
            "UIKit view가 null입니다",
        ))?;
        let display = RawDisplayHandle::UiKit(UiKitDisplayHandle::new());
        let window = RawWindowHandle::UiKit(UiKitWindowHandle::new(view));
        unsafe { Self::prepare_surface(display, window, backend) }
    }

    unsafe fn prepare_surface(
        display: RawDisplayHandle,
        window: RawWindowHandle,
        backend: u32,
    ) -> Result<PendingRuntimeSurface, RuntimeRendererError> {
        let backends = backend_from_abi(backend)?;
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        // SAFETY: 호출자가 네이티브 표면을 renderer보다 오래 유지해야 합니다.
        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: Some(display),
                raw_window_handle: window,
            })
        }
        .map_err(|error| RuntimeRendererError::Surface(error.to_string()))?;
        Ok(PendingRuntimeSurface { instance, surface })
    }

    pub fn initialize_surface(
        pending: PendingRuntimeSurface,
        width: u32,
        height: u32,
    ) -> Result<(Self, String), RuntimeRendererError> {
        Self::initialize_surface_with_configuration(pending, width, height, true)
    }

    /// UIKit surface를 준비하되 `CAMetalLayer`의 속성을 바꾸는 configure는 미룹니다.
    /// 장치·pipeline 초기화는 render queue에서, `configure_surface`는 메인 스레드에서 호출합니다.
    #[doc(hidden)]
    pub fn initialize_uikit_surface(
        pending: PendingRuntimeSurface,
        width: u32,
        height: u32,
    ) -> Result<(Self, String), RuntimeRendererError> {
        Self::initialize_surface_with_configuration(pending, width, height, false)
    }

    fn initialize_surface_with_configuration(
        pending: PendingRuntimeSurface,
        width: u32,
        height: u32,
        configure_surface: bool,
    ) -> Result<(Self, String), RuntimeRendererError> {
        if width == 0 || height == 0 {
            return Err(RuntimeRendererError::InvalidArgument(
                "WGPU 표면 크기는 0보다 커야 합니다",
            ));
        }
        let PendingRuntimeSurface { instance, surface } = pending;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .map_err(|error| RuntimeRendererError::Initialization(error.to_string()))?;
        let info = adapter.get_info();
        let mut config = surface
            .get_default_config(&adapter, width, height)
            .ok_or_else(|| {
                RuntimeRendererError::Initialization(
                    "요청한 GPU가 화면 표면에 제출할 수 없습니다".to_owned(),
                )
            })?;
        let capabilities = surface.get_capabilities(&adapter);
        config.format = choose_sdr_format(&capabilities)?;
        config.color_space = wgpu::SurfaceColorSpace::Srgb;
        config.present_mode = wgpu::PresentMode::Fifo;
        let format = config.format;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            required_limits: adapter.limits(),
            ..Default::default()
        }))
        .map_err(|error| RuntimeRendererError::Initialization(error.to_string()))?;
        if configure_surface {
            surface.configure(&device, &config);
        }
        let pipeline = create_pipeline(&device, config.format);
        let vertex_capacity = std::mem::size_of::<Vertex>() as u64;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("spinon-runtime-gpu-vertices"),
            size: vertex_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Ok((
            Self {
                _instance: instance,
                surface,
                device,
                queue,
                config,
                pipeline,
                vertex_buffer,
                vertex_capacity,
                vertex_count: 0,
                last_surface_texture_size: (0, 0),
                surface_configured: configure_surface,
                #[cfg(feature = "test-hooks")]
                draw_failure_injector: DrawFailureInjector::default(),
            },
            format!(
                "backend={:?} device={} format={:?} size={}x{}",
                info.backend, info.name, format, width, height
            ),
        ))
    }

    /// UIKit의 `CAMetalLayer` 속성을 갱신하므로 반드시 메인 스레드에서 호출합니다.
    pub fn configure_surface(&mut self) {
        if self.surface_configured {
            return;
        }
        self.surface.configure(&self.device, &self.config);
        self.surface_configured = true;
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), RuntimeRendererError> {
        if width == 0 || height == 0 {
            return Err(RuntimeRendererError::InvalidArgument(
                "WGPU 표면 크기는 0보다 커야 합니다",
            ));
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.surface_configured = true;
        Ok(())
    }

    pub fn last_surface_texture_size(&self) -> (u32, u32) {
        self.last_surface_texture_size
    }

    pub fn draw(
        &mut self,
        scene: Option<&RuntimeRenderSnapshot>,
    ) -> Result<(), RuntimeRendererError> {
        self.draw_if(scene, || true).map(|_| ())
    }

    #[cfg(feature = "test-hooks")]
    pub fn inject_next_draw_failure_for_test(&self) {
        self.draw_failure_injector.arm();
    }

    /// GPU submit 전과 present 직전에 호출자가 revision 유효성을 확인합니다.
    pub fn draw_if(
        &mut self,
        scene: Option<&RuntimeRenderSnapshot>,
        is_current: impl Fn() -> bool,
    ) -> Result<bool, RuntimeRendererError> {
        if !self.surface_configured {
            return Err(RuntimeRendererError::Surface(
                "WGPU surface가 아직 구성되지 않았습니다".to_owned(),
            ));
        }
        #[cfg(feature = "test-hooks")]
        if self.draw_failure_injector.take() {
            return Err(RuntimeRendererError::Surface(
                "C04.10 시험용으로 다음 draw를 실패시켰습니다".to_owned(),
            ));
        }
        let vertices = match scene {
            Some(scene) => build_vertices(
                scene,
                self.config.width,
                self.config.height,
                color_encoding(self.config.format),
            )
            .map_err(RuntimeRendererError::Geometry)?,
            None => Vec::new(),
        };
        self.upload_vertices(&vertices)?;
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                drop(frame);
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface 구성을 다시 해야 합니다".to_owned(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface가 끊겼습니다".to_owned(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface 구성을 다시 해야 합니다".to_owned(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Timeout => {
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface 획득 시간이 초과됐습니다".to_owned(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface가 가려져 있습니다".to_owned(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RuntimeRendererError::Surface(
                    "WGPU surface 획득 검증에 실패했습니다".to_owned(),
                ));
            }
        };
        self.last_surface_texture_size = (frame.texture.width(), frame.texture.height());
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("spinon-runtime-gpu-command-encoder"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("spinon-runtime-gpu-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if self.vertex_count != 0 {
                pass.set_pipeline(&self.pipeline);
                pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                pass.draw(0..self.vertex_count, 0..1);
            }
        }
        if !is_current() {
            return Ok(false);
        }
        self.queue.submit([encoder.finish()]);
        if !is_current() {
            return Ok(false);
        }
        self.queue.present(frame);
        Ok(true)
    }

    fn upload_vertices(&mut self, vertices: &[Vertex]) -> Result<(), RuntimeRendererError> {
        let count = u32::try_from(vertices.len()).map_err(|_| {
            RuntimeRendererError::Geometry("정점 수가 범위를 넘었습니다".to_owned())
        })?;
        self.vertex_count = count;
        if vertices.is_empty() {
            return Ok(());
        }
        let bytes = bytemuck::cast_slice(vertices);
        let required = u64::try_from(bytes.len()).map_err(|_| {
            RuntimeRendererError::Geometry("정점 버퍼 크기가 범위를 넘었습니다".to_owned())
        })?;
        if required > self.vertex_capacity {
            let capacity = required.checked_next_power_of_two().ok_or_else(|| {
                RuntimeRendererError::Geometry("정점 버퍼 용량 계산이 범위를 넘었습니다".to_owned())
            })?;
            self.vertex_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("spinon-runtime-gpu-vertices"),
                size: capacity,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vertex_capacity = capacity;
        }
        self.queue.write_buffer(&self.vertex_buffer, 0, bytes);
        Ok(())
    }
}

pub(crate) fn create_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("spinon-runtime-gpu-shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("spinon-runtime-gpu-pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &ATTRIBUTES,
            })],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg(test)]
#[path = "renderer/tests.rs"]
mod tests;
