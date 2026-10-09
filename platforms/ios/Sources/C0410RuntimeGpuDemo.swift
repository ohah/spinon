import MetalKit
import UIKit

final class C0410RuntimeGpuDemoViewController: UIViewController {
    private let runtimeQueue = DispatchQueue(label: "spinon.c0410.runtime")
    private let renderQueue = DispatchQueue(label: "spinon.c0410.render")
    private let stateLock = NSLock()
    private let presentationUpdateLock = NSLock()
    private let hostLifetime = C0410RuntimeGpuHostLifetime()
    private let canvasView = C0410RuntimeGpuCanvasView()
    private let resizeButton = UIButton(type: .system)
    private let customPropertiesButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private var canvasWidthConstraint: NSLayoutConstraint?
    private var canvasHeightConstraint: NSLayoutConstraint?
    private var hostHandle: UInt64 = 0
    private var rendererReady = false
    private var rendererCreationPending = false
    private var surfaceConfigurationPending = false
    private var surfaceGeneration: UInt64 = 0
    private var rendererSurfaceGeneration: UInt64?
    private var lastDrawableSize = CGSize.zero
    private var viewportWidthCssPx: Float = 301
    private var viewportHeightCssPx: Float = 100
    private var presentationSequence: UInt64 = 0
    private var expandedSurface = false
    private var automaticResizeScheduled = false
    private let failureProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-failure-probe")
    private let shutdownProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-shutdown-probe")
    private let runtimeResultCacheProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c053-runtime-result-cache")
    private let incrementalRestyleProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c054-incremental-restyle")
    private let registeredPropertiesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c052-registered-properties")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c053-runtime-result-cache")
    private var authorStylesheetsProbeRequested: Bool {
        !registeredPropertiesProbeRequested && ProcessInfo.processInfo.arguments
            .contains("--spinon-c0411-runtime-author-stylesheets")
    }
    private var customPropertiesProbeRequested: Bool {
        !registeredPropertiesProbeRequested && !authorStylesheetsProbeRequested
            && ProcessInfo.processInfo.arguments.contains("--spinon-c051-custom-properties")
    }
    private var failureProbeStarted = false
    private var shutdownProbeStarted = false
    private var closing = false
    private var displayScale: Float = 1
    private var darkMode = false
    private var runtimePresentationLane: C0410LatestTaskLane?
    private var renderDrawLane: C0410LatestTaskLane?

    override func viewDidLoad() {
        super.viewDidLoad()
        displayScale = Float(UIScreen.main.scale)
        darkMode = traitCollection.userInterfaceStyle == .dark
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)

        let title = UILabel()
        title.text = incrementalRestyleProbeRequested
            ? "SPINON · C05.4 incremental restyle"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested ? "SPINON · C05.3 runtime cache"
                : "SPINON · C05.2 @property"
            : authorStylesheetsProbeRequested
                ? "SPINON · C04.11 CSS → WGPU" : "SPINON · C04.10 CSS → WGPU"
        title.textColor = UIColor(red: 0.92, green: 0.95, blue: 0.99, alpha: 1)
        title.font = .systemFont(ofSize: 22, weight: .bold)
        title.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(title)

        let description = UILabel()
        description.text = incrementalRestyleProbeRequested
            ? "V8 inline style → dirty subtree → Stylo → Taffy → WGPU"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "V8 detached DOM → worker cache → WGPU"
                : "V8 DOM <style> → Stylo → Taffy → WGPU"
            : authorStylesheetsProbeRequested
                ? "실제 V8 DOM <style> → Stylo → Taffy → wgpu surface"
            : "실제 V8 DOM → Stylo → Taffy → Rust 장면 → wgpu surface"
        description.textColor = UIColor(red: 0.78, green: 0.83, blue: 0.90, alpha: 1)
        description.font = .systemFont(ofSize: 14)
        description.numberOfLines = 2
        description.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(description)

        canvasView.translatesAutoresizingMaskIntoConstraints = false
        canvasView.isAccessibilityElement = true
        canvasView.accessibilityLabel = incrementalRestyleProbeRequested
            ? "C05.4 inline style 하위 트리 재계산 검증 WGPU 장면"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "C05.3 detached-node 결과 재사용 검증 WGPU 장면"
                : "C05.2 등록 사용자 지정 속성 Chromium fixture의 WGPU 장면"
            : authorStylesheetsProbeRequested
                ? "C04.11 Chromium stylesheet fixture의 WGPU 장면"
            : "C04.10 Chromium fixture의 WGPU 장면"
        view.addSubview(canvasView)

        resizeButton.setTitle("표면 크기 전환 · 301×100 CSS px", for: .normal)
        resizeButton.addTarget(self, action: #selector(toggleSurfaceSize), for: .touchUpInside)
        resizeButton.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(resizeButton)

        customPropertiesButton.setTitle(
            incrementalRestyleProbeRequested
                ? "C05.4 · 왼쪽 branch style 전환"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? "C05.3 · detached / 연결 변경 실행"
                    : "C05.2 · 등록 사용자 지정 속성 다시 적용"
                : "C05 · 사용자 지정 속성 다시 적용",
            for: .normal
        )
        customPropertiesButton.addTarget(
            self,
            action: incrementalRestyleProbeRequested
                ? #selector(evaluateIncrementalRestyleFixture)
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? #selector(evaluateRuntimeResultCacheFixture)
                    : #selector(evaluateRegisteredPropertiesFixture)
                : #selector(evaluateCustomPropertiesFixture),
            for: .touchUpInside
        )
        customPropertiesButton.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(customPropertiesButton)
        if authorStylesheetsProbeRequested { customPropertiesButton.isHidden = true }

        statusLabel.text = "V8·CSS runtime 준비 중…"
        statusLabel.textColor = UIColor(red: 0.38, green: 0.73, blue: 1, alpha: 1)
        statusLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        statusLabel.numberOfLines = 0
        statusLabel.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(statusLabel)

        let canvasWidth = canvasView.widthAnchor.constraint(equalToConstant: 301)
        let canvasHeight = canvasView.heightAnchor.constraint(equalToConstant: 100)
        canvasWidthConstraint = canvasWidth
        canvasHeightConstraint = canvasHeight
        NSLayoutConstraint.activate([
            title.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor, constant: 22),
            title.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -22),
            title.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 24),
            description.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            description.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            description.topAnchor.constraint(equalTo: title.bottomAnchor, constant: 8),
            canvasView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            canvasView.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            canvasWidth,
            canvasHeight,
            resizeButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            resizeButton.topAnchor.constraint(equalTo: canvasView.bottomAnchor, constant: 12),
            customPropertiesButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            customPropertiesButton.topAnchor.constraint(equalTo: resizeButton.bottomAnchor, constant: 4),
            statusLabel.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            statusLabel.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            statusLabel.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -18),
        ])
        if failureProbeRequested && shutdownProbeRequested {
            statusLabel.text = "draw 복구와 종료 검증은 나눠 실행해야 합니다."
            log("SPINON_C0410_PROBE_ARGUMENT_ERROR · failure와 shutdown 검증을 동시에 요청했습니다")
            return
        }
        runtimePresentationLane = C0410LatestTaskLane(
            queue: runtimeQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.applyLatestEnvironment()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("runtime", message) }
        )
        renderDrawLane = C0410LatestTaskLane(
            queue: renderQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.drawLatestIfReady()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("render", message) }
        )
        enqueueRuntime { [weak self] in self?.initializeRuntime() }
    }

    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        beginShutdown()
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !automaticResizeScheduled,
              ProcessInfo.processInfo.arguments.contains("--spinon-c0410-auto-resize") else {
            return
        }
        automaticResizeScheduled = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) { [weak self] in
            self?.runAutomaticResizeProbe(step: 0)
        }
    }

    private func runAutomaticResizeProbe(step: Int) {
        guard step < 3, !isClosing else { return }
        toggleSurfaceSize()
        guard step + 1 < 3 else { return }
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.75) { [weak self] in
            self?.runAutomaticResizeProbe(step: step + 1)
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        let scale = view.window?.screen.scale ?? UIScreen.main.scale
        let viewportSize = canvasView.bounds.size
        guard viewportSize.width > 0, viewportSize.height > 0 else { return }
        stateLock.lock()
        let scaleChanged = displayScale != Float(scale)
        let viewportChanged = viewportWidthCssPx != Float(viewportSize.width)
            || viewportHeightCssPx != Float(viewportSize.height)
        displayScale = Float(scale)
        viewportWidthCssPx = Float(viewportSize.width)
        viewportHeightCssPx = Float(viewportSize.height)
        stateLock.unlock()
        let size = CGSize(
            width: (canvasView.bounds.width * scale).rounded(),
            height: (canvasView.bounds.height * scale).rounded()
        )
        let sizeChanged = size != lastDrawableSize
        guard size.width > 0, size.height > 0 else { return }
        if viewportChanged || sizeChanged {
            log("SPINON_C0410_VIEWPORT_CHANGED css=\(viewportSize.width)x\(viewportSize.height) scale=\(scale) drawable=\(Int(size.width))x\(Int(size.height))")
        }
        if scaleChanged || sizeChanged {
            if sizeChanged {
                stateLock.lock()
                surfaceGeneration &+= 1
                let generation = surfaceGeneration
                stateLock.unlock()
                log("SPINON_C0410_SURFACE_CHANGED generation=\(generation) size=\(Int(size.width))x\(Int(size.height))")
            }
            _ = beginPresentationUpdate()
        }
        guard sizeChanged else {
            if scaleChanged || viewportChanged { refreshEnvironment() }
            return
        }
        lastDrawableSize = size
        (canvasView.layer as? CAMetalLayer)?.drawableSize = size
        if rendererIsReady {
            resizeRenderer(to: size)
        } else {
            ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)
        guard previousTraitCollection?.userInterfaceStyle != traitCollection.userInterfaceStyle else {
            return
        }
        stateLock.lock()
        darkMode = traitCollection.userInterfaceStyle == .dark
        stateLock.unlock()
        refreshEnvironment()
    }

    private var rendererIsReady: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return rendererReady
    }

    private var isClosing: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return closing
    }

    private func currentEnvironment() -> (width: Float, height: Float, scale: Float, dark: Bool) {
        stateLock.lock()
        defer { stateLock.unlock() }
        return (viewportWidthCssPx, viewportHeightCssPx, displayScale, darkMode)
    }

    private func runtimeStatusSummary(_ report: String) -> String {
        let status = String(report.split(separator: " ", maxSplits: 1).first ?? "")
        guard let layoutRange = report.range(of: " layout=") else {
            return String(report.prefix(180))
        }
        return "\(status) layout=\(report[layoutRange.upperBound...])"
    }

    @objc private func toggleSurfaceSize() {
        guard !isClosing else { return }
        expandedSurface.toggle()
        let width: CGFloat = expandedSurface ? 341 : 301
        let height: CGFloat = expandedSurface ? 128 : 100
        canvasWidthConstraint?.constant = width
        canvasHeightConstraint?.constant = height
        resizeButton.setTitle(
            "표면 크기 전환 · \(Int(width))×\(Int(height)) CSS px",
            for: .normal
        )
        log("SPINON_C0410_RESIZE_REQUEST css=\(Int(width))x\(Int(height))")
        view.setNeedsLayout()
        view.layoutIfNeeded()
    }

    private func initializeRuntime() {
        guard !isClosing else { return }
        let handle = SpinonRunner.createRuntimeGpuHost(
            withRegisteredPropertiesFixture: registeredPropertiesProbeRequested
        )
        guard handle != 0 else {
            postStatus("실패 · V8 runtime host를 만들지 못했습니다")
            return
        }
        log("SPINON_C0410_HOST_RETURNED handle=\(handle)")
        stateLock.lock()
        hostHandle = handle
        hostLifetime.store(handle)
        let shouldContinue = !closing
        stateLock.unlock()
        log("SPINON_C0410_HOST_STORED continue=\(shouldContinue)")
        guard shouldContinue else { return }

        let environmentInputs = currentEnvironment()
        log("SPINON_C0410_ENVIRONMENT_REQUEST width=\(environmentInputs.width) height=\(environmentInputs.height)")
        let environment = SpinonRunner.setRuntimeGpuEnvironment(
            handle, width: environmentInputs.width, height: environmentInputs.height,
            scale: environmentInputs.scale, dark: environmentInputs.dark
        )
        guard environment?.hasPrefix("status=0 ") == true else {
            postStatus("실패 · \(environment ?? "환경 보고 없음")")
            return
        }
        log("SPINON_C0410_ENVIRONMENT \(environment ?? "")")

        var result = incrementalRestyleProbeRequested
            ? SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
            : registeredPropertiesProbeRequested
            ? SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
            : authorStylesheetsProbeRequested
                ? SpinonRunner.evalRuntimeGpuAuthorStylesheetsFixture(handle)
                : SpinonRunner.evalRuntimeGpuFixture(handle)
        let sceneWasSupersededAfterCommit = result?.hasPrefix("status=-12 ") == true
            && result?.contains("op=eval status=0 ") == true
        guard result?.hasPrefix("status=0 ") == true || sceneWasSupersededAfterCommit else {
            postStatus("실패 · \(result ?? "JavaScript 보고 없음")")
            return
        }
        if sceneWasSupersededAfterCommit {
            let scope = incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL_SCENE_SUPERSEDED \(result ?? "")")
            postStatus("JavaScript 적용 완료 · 최신 CSS 장면 다시 계산 중")
        } else {
            let scope = incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL \(result ?? "")")
            postStatus(runtimeStatusSummary(result ?? "runtime scene 준비 완료"))
        }
        if customPropertiesProbeRequested && !authorStylesheetsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            guard result?.hasPrefix("status=0 ") == true else {
                postStatus("실패 · C05 사용자 지정 속성 · \(result ?? "보고 없음")")
                return
            }
            log("SPINON_C051_EVAL \(result ?? "")")
            postStatus(runtimeStatusSummary(result ?? "C05 장면 준비 완료"))
        }
        DispatchQueue.main.async { [weak self] in
            guard let self else { return }
            let size = self.lastDrawableSize
            if size.width > 0 && size.height > 0 {
                self.ensureRenderer(width: Int(size.width), height: Int(size.height))
            }
            self.refreshEnvironment()
        }
    }

    @objc private func evaluateCustomPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            log("SPINON_C051_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRegisteredPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
            log("SPINON_C052_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "등록 사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRuntimeResultCacheFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRuntimeResultCacheFixture(handle)
            self.log("SPINON_C053_EVAL \(report ?? "runtime cache fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "runtime cache fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateIncrementalRestyleFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
            self.log("SPINON_C054_EVAL \(report ?? "incremental restyle fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "incremental restyle 결과가 없습니다"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    private func ensureRenderer(width: Int, height: Int) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard width > 0, height > 0 else { return }
        stateLock.lock()
        guard hostHandle != 0, !rendererReady, !rendererCreationPending, !closing else {
            stateLock.unlock()
            return
        }
        rendererCreationPending = true
        let handle = hostHandle
        let generation = surfaceGeneration
        let viewPointer = Unmanaged.passUnretained(canvasView).toOpaque()
        stateLock.unlock()

        let surfaceReport = SpinonRunner.prepareRuntimeGpuWgpuSurface(
            handle, view: viewPointer
        )
        guard surfaceReport?.hasPrefix("status=0 ") == true else {
            stateLock.lock()
            rendererCreationPending = false
            stateLock.unlock()
            postStatus(surfaceReport ?? "UIKit surface 준비 보고 없음")
            return
        }

        enqueueRender { [weak self] in
            guard let self else { return }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            let report = SpinonRunner.createRuntimeGpuWgpu(
                handle, width: UInt32(width), height: UInt32(height)
            )
            guard report?.hasPrefix("status=0 ") == true else {
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.postStatus(report ?? "WGPU renderer 보고 없음")
                return
            }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            DispatchQueue.main.async { [weak self] in
                self?.configureRendererOnMain(
                    handle: handle, generation: generation, rendererReport: report ?? ""
                )
            }
        }
    }

    private func configureRendererOnMain(
        handle: UInt64, generation: UInt64, rendererReport: String
    ) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard isCurrentSurfaceGeneration(generation) else {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }

        let configureReport = SpinonRunner.configureRuntimeGpuWgpuSurface(handle)
        let configureThread = Thread.isMainThread ? "main" : "other"
        log("SPINON_C0410_SURFACE_CONFIGURE thread=\(configureThread) \(configureReport ?? "")")
        let ready = configureReport?.hasPrefix("status=0 ") == true
        stateLock.lock()
        let stale = closing || surfaceGeneration != generation
        rendererReady = ready && !stale
        rendererSurfaceGeneration = ready && !stale ? generation : nil
        if ready && !stale { rendererCreationPending = false }
        stateLock.unlock()
        if ready && stale {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }
        guard ready else {
            postStatus(configureReport ?? "UIKit surface 구성 보고 없음")
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
            }
            return
        }
        postStatus("\(rendererReport) · \(configureReport ?? "surface configured")")
        refreshEnvironment()
    }

    private func resizeRenderer(to size: CGSize) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard size.width > 0, size.height > 0 else { return }
        stateLock.lock()
        guard rendererReady, !surfaceConfigurationPending, !closing else {
            stateLock.unlock()
            return
        }
        surfaceConfigurationPending = true
        rendererSurfaceGeneration = nil
        let generation = surfaceGeneration
        stateLock.unlock()
        enqueueRender { [weak self] in
            guard let self else { return }
            DispatchQueue.main.async { [weak self] in
                self?.configureResizeOnMain(size: size, generation: generation)
            }
        }
    }

    private func configureResizeOnMain(size: CGSize, generation: UInt64) {
        dispatchPrecondition(condition: .onQueue(.main))
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        let handle = hostHandle
        stateLock.unlock()
        guard current, handle != 0 else {
            finishSurfaceResize(generation: generation, success: false)
            return
        }

        let result = SpinonRunner.resizeRuntimeGpuWgpu(
            handle, width: UInt32(size.width), height: UInt32(size.height)
        )
        log("SPINON_C0410_RESIZE status=\(result) thread=main")
        guard result == 0 else {
            finishSurfaceResize(generation: generation, success: false)
            postStatus("실패 · WGPU 표면 크기 변경 status=\(result)")
            return
        }
        finishSurfaceResize(generation: generation, success: true)
    }

    private func finishSurfaceResize(generation: UInt64, success: Bool) {
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        surfaceConfigurationPending = false
        if success && current { rendererSurfaceGeneration = generation }
        let retrySize = lastDrawableSize
        stateLock.unlock()
        guard !isClosing else { return }
        guard current else {
            if rendererIsReady, retrySize.width > 0, retrySize.height > 0 {
                resizeRenderer(to: retrySize)
            }
            return
        }
        guard success else { return }
        scheduleLatestPresentation()
    }

    private func refreshEnvironment() {
        guard beginPresentationUpdate() != 0 else { return }
        scheduleLatestPresentation()
    }

    private func scheduleLatestPresentation() {
        guard let runtimePresentationLane else { return }
        if case .closed = runtimePresentationLane.request() {
            postStatus("실패 · 닫힌 runtime presentation lane에 요청했습니다")
        }
    }

    private func applyLatestEnvironment() throws {
        let generation: UInt64
        let sequence: UInt64
        let handle: UInt64
        let environmentInputs: (width: Float, height: Float, scale: Float, dark: Bool)
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return
        }
        generation = surfaceGeneration
        sequence = presentationSequence
        handle = hostHandle
        environmentInputs = (viewportWidthCssPx, viewportHeightCssPx, displayScale, darkMode)
        stateLock.unlock()
        guard isCurrentPresentation(generation: generation, sequence: sequence) else { return }
        let report = SpinonRunner.setRuntimeGpuEnvironment(
            handle, width: environmentInputs.width, height: environmentInputs.height,
            scale: environmentInputs.scale, dark: environmentInputs.dark
        )
        log("SPINON_C0410_ENVIRONMENT viewport=\(environmentInputs.width)x\(environmentInputs.height) scale=\(environmentInputs.scale) dark=\(environmentInputs.dark) sequence=\(sequence) \(report ?? "")")
        postStatus(report ?? "환경 보고 없음")
        guard report?.hasPrefix("status=0 ") == true else {
            throw C0410LatestTaskLaneError.operationFailed(report ?? "환경 갱신 보고 없음")
        }
        if report?.hasPrefix("status=0 ") == true
            && isCurrentPresentation(generation: generation, sequence: sequence) {
            requestDraw()
        }
    }

    private func requestDraw() {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        guard let renderDrawLane else { return }
        if case .closed = renderDrawLane.request() {
            postStatus("실패 · 닫힌 render draw lane에 요청했습니다")
        }
    }

    private func drawLatestIfReady() throws {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration,
              hostHandle != 0, let generation = rendererSurfaceGeneration else {
            stateLock.unlock()
            return
        }
        let handle = hostHandle
        stateLock.unlock()
        if shutdownProbeRequested && !shutdownProbeStarted {
            shutdownProbeStarted = true
            runShutdownProbeGate()
        }
        guard canUseSurfaceGeneration(generation) else { return }
        let result = SpinonRunner.drawRuntimeGpuWgpu(handle)
        log("SPINON_C0410_DRAW generation=\(generation) \(result ?? "")")
        if result?.hasPrefix("status=0 ") != true {
            postStatus("실패 · \(result ?? "WGPU draw 보고 없음")")
            throw C0410LatestTaskLaneError.operationFailed(result ?? "WGPU draw 실패 보고 없음")
        }
        if failureProbeRequested && !failureProbeStarted {
            failureProbeStarted = true
            runFailureRecoveryProbe(handle: handle, baseline: result ?? "")
        }
    }

    private func runShutdownProbeGate() {
        let release = DispatchSemaphore(value: 0)
        for _ in 0..<10_000 { _ = renderDrawLane?.request() }
        let snapshot = renderDrawLane?.snapshot()
        log("SPINON_C0410_SHUTDOWN_PROBE_PENDING requests=10000 "
            + "scheduled=\(snapshot?.scheduled ?? false) "
            + "dirty=\(snapshot?.dirty ?? false) pending=\(snapshot?.pendingDrains ?? -1)")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) { [weak self] in
            self?.beginShutdown()
        }
        DispatchQueue.global(qos: .utility).asyncAfter(deadline: .now() + 0.35) {
            release.signal()
        }
        release.wait()
        log("SPINON_C0410_SHUTDOWN_PROBE_GATE_RELEASED")
    }

    private func runFailureRecoveryProbe(handle: UInt64, baseline: String) {
#if SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE_1
        let armed = SpinonRunner.injectNextRuntimeGpuDrawFailure(handle)
        log("SPINON_C0410_FAILURE_PROBE_ARM \(armed ?? "")")
        guard armed?.hasPrefix("status=0 ") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=arm")
            postStatus("실패 · draw 오류 주입을 설정하지 못했습니다")
            return
        }

        let expectedFailure = SpinonRunner.drawRuntimeGpuWgpu(handle)
        log("SPINON_C0410_FAILURE_PROBE_EXPECTED_ERROR \(expectedFailure ?? "")")
        guard expectedFailure?.hasPrefix("status=0 ") != true,
              expectedFailure?.contains("시험용으로 다음 draw를 실패") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=expected-error")
            postStatus("실패 · 시험용 draw 오류가 예상대로 전달되지 않았습니다")
            return
        }

        let recovered = SpinonRunner.drawRuntimeGpuWgpu(handle)
        let baselineRevision = reportField(baseline, "environment_revision=")
        let recoveredRevision = reportField(recovered ?? "", "environment_revision=")
        let sameScene = !baselineRevision.isEmpty && baselineRevision == recoveredRevision
        let passed = recovered?.hasPrefix("status=0 ") == true && sameScene
        log("SPINON_C0410_FAILURE_PROBE_RECOVERY passed=\(passed) "
            + "baseline_revision=\(baselineRevision) recovered_revision=\(recoveredRevision) "
            + "\(recovered ?? "")")
        if passed {
            postStatus("draw 오류가 장면 revision을 바꾸지 않았고 다음 draw가 회복됐습니다")
        } else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=recovery")
            postStatus("실패 · draw 오류 뒤 정상 장면 복구가 확인되지 않았습니다")
        }
#else
        log("SPINON_C0410_FAILURE_PROBE_DISABLED · 내부 실패 fixture 빌드가 아닙니다")
        postStatus("실패 · draw 실패 검증용 빌드 옵션이 꺼져 있습니다")
#endif
    }

    private func reportField(_ report: String, _ field: String) -> String {
        guard let range = report.range(of: field) else { return "" }
        let value = report[range.upperBound...]
        return String(value.prefix { !$0.isWhitespace })
    }

    private func isCurrentSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation
    }

    private func canUseSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && rendererReady && surfaceGeneration == generation
            && rendererSurfaceGeneration == generation
    }

    private func isCurrentPresentation(generation: UInt64, sequence: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation && presentationSequence == sequence
    }

    private func retryRendererCreation() {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            let size = self.lastDrawableSize
            guard size.width > 0, size.height > 0 else { return }
            self.ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    private func postStatus(_ value: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            self.statusLabel.text = value
        }
    }

    private func log(_ value: String) {
        NSLog("%@", value)
    }

    private func reportLaneFailure(_ lane: String, _ message: String) {
        log("SPINON_C0410_QUEUE_FAILURE lane=\(lane) message=\(message)")
        postStatus("실패 · \(lane) queue: \(message)")
    }

    private func beginPresentationUpdate() -> UInt64 {
        presentationUpdateLock.lock()
        defer { presentationUpdateLock.unlock() }
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return 0
        }
        let handle = hostHandle
        stateLock.unlock()

        let sequence = SpinonRunner.beginRuntimeGpuPresentationUpdate(handle)
        stateLock.lock()
        if !closing && hostHandle == handle && sequence != 0 {
            presentationSequence = sequence
        }
        stateLock.unlock()
        return sequence
    }

    private func enqueueRuntime(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        runtimeQueue.async(execute: work)
        stateLock.unlock()
    }

    private func enqueueRender(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        renderQueue.async(execute: work)
        stateLock.unlock()
    }

    private func beginShutdown() {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        _ = beginPresentationUpdate()
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        surfaceGeneration &+= 1
        let destroyedGeneration = surfaceGeneration
        closing = true
        let runtimeQueue = self.runtimeQueue
        let renderQueue = self.renderQueue
        let hostLifetime = self.hostLifetime
        let retainedSurfaceView = canvasView
        stateLock.unlock()
        runtimePresentationLane?.close()
        renderDrawLane?.close()
        log("SPINON_C0410_SURFACE_DESTROYED generation=\(destroyedGeneration)")

        runtimeQueue.async {
            renderQueue.async {
                let handle = hostLifetime.take()
                if handle != 0 {
                    SpinonRunner.destroyRuntimeGpuRenderer(handle)
                    SpinonRunner.freeRuntimeGpuHost(handle)
                }
                _ = retainedSurfaceView
                NSLog("SPINON_C0410_DRAINED generation=%llu", destroyedGeneration)
            }
        }
    }

    deinit {
        beginShutdown()
    }
}

private enum C0410LatestTaskLaneError: Error, CustomStringConvertible {
    case operationFailed(String)

    var description: String {
        switch self {
        case let .operationFailed(message): message
        }
    }
}

private final class C0410RuntimeGpuHostLifetime {
    private let lock = NSLock()
    private var handle: UInt64 = 0

    func store(_ value: UInt64) {
        lock.lock()
        handle = value
        lock.unlock()
    }

    func load() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        return handle
    }

    func take() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        let value = handle
        handle = 0
        return value
    }
}

private final class C0410RuntimeGpuCanvasView: UIView {
    override class var layerClass: AnyClass { CAMetalLayer.self }
}
