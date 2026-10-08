import MetalKit
import OSLog
import QuartzCore
import UIKit

private struct R05InputSample {
    let sequence: Int
    let eventTimestamp: TimeInterval
    let handlerUptime: TimeInterval
}

private func captureR05ClockAnchor(_ logger: Logger, label: String) {
    let uptimeBefore = ProcessInfo.processInfo.systemUptime
    let hostTime = CACurrentMediaTime()
    let uptimeAfter = ProcessInfo.processInfo.systemUptime
    logger.notice("SPINON_R05_CLOCK_ANCHOR label=\(label, privacy: .public) uptime_before_s=\(uptimeBefore, privacy: .public) host_s=\(hostTime, privacy: .public) uptime_after_s=\(uptimeAfter, privacy: .public)")
}

private struct R05DrawableTicket {
    let drawSequence: UInt64
    let inputSequence: Int?
    let revision: UInt32
    let surfaceGeneration: Int

    var logFields: String {
        let inputField = inputSequence.map { String($0) } ?? "none"
        return "draw_seq=\(drawSequence) input_seq=\(inputField) revision=\(revision) generation=\(surfaceGeneration)"
    }
}

private enum R05TouchDecision {
    case accepted(UITouch)
    case excluded(String)
}

private struct R05TouchGesture {
    private static let maximumMovement: CGFloat = 8
    private var touchID: ObjectIdentifier?
    private var startPoint: CGPoint?
    private var rejectionReason: String?

    mutating func began(_ touches: Set<UITouch>, event: UIEvent?, in view: UIView) {
        guard touchID == nil else {
            reject("multiple_touches")
            return
        }
        reset()
        guard touches.count == 1,
              (event?.allTouches?.count ?? touches.count) == 1,
              let touch = touches.first else {
            reject("multiple_touches")
            return
        }
        touchID = ObjectIdentifier(touch)
        let point = touch.location(in: view)
        startPoint = point
        if !isTarget(point, in: view) { reject("outside_target") }
    }

    mutating func moved(_ touches: Set<UITouch>, event: UIEvent?, in view: UIView) {
        guard touchID != nil else { return }
        guard touches.count == 1,
              (event?.allTouches?.count ?? touches.count) == 1,
              let touch = touches.first,
              ObjectIdentifier(touch) == touchID else {
            reject("multiple_touches")
            return
        }
        rejectIfMoved(touch.location(in: view), in: view)
    }

    mutating func ended(_ touches: Set<UITouch>, event: UIEvent?, in view: UIView) -> R05TouchDecision {
        defer { reset() }
        guard let touchID else {
            return .excluded(rejectionReason ?? "missing_touch_start")
        }
        guard touches.count == 1,
              (event?.allTouches?.count ?? touches.count) == 1,
              let touch = touches.first,
              ObjectIdentifier(touch) == touchID else {
            return .excluded(rejectionReason ?? "multiple_touches")
        }
        let point = touch.location(in: view)
        rejectIfMoved(point, in: view)
        if !isTarget(point, in: view) { reject("outside_target") }
        if let rejectionReason { return .excluded(rejectionReason) }
        return .accepted(touch)
    }

    mutating func cancelled() -> String {
        let reason = rejectionReason ?? "cancelled"
        reset()
        return reason
    }

    mutating func cancelForSurfaceDetach() -> String? {
        guard touchID != nil else { return nil }
        let reason = rejectionReason ?? "surface_detached"
        reset()
        return reason
    }

    private mutating func rejectIfMoved(_ point: CGPoint, in view: UIView) {
        guard let startPoint else { return }
        let deltaX = point.x - startPoint.x
        let deltaY = point.y - startPoint.y
        if deltaX * deltaX + deltaY * deltaY > Self.maximumMovement * Self.maximumMovement
            || !isTarget(point, in: view) {
            reject("gesture_moved")
        }
    }

    private func isTarget(_ point: CGPoint, in view: UIView) -> Bool {
        return point.x >= view.bounds.width * 0.11
            && point.x <= view.bounds.width * 0.89
            && point.y >= view.bounds.height * 0.40
            && point.y <= view.bounds.height * 0.60
    }

    private mutating func reject(_ reason: String) {
        if rejectionReason == nil { rejectionReason = reason }
    }

    private mutating func reset() {
        touchID = nil
        startPoint = nil
        rejectionReason = nil
    }
}

final class R08GpuDemoViewController: UIViewController, UITextFieldDelegate {
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "r08")
    private let canvas: UIView
    private let useWgpu: Bool
    private let r13Enabled: Bool
    private let r13WindowCycle: Bool
    private let r05PresentationProbe: Bool
    private let titleLabel = UILabel()
    private let statusLabel = UILabel()
    private let inputField = UITextField()

    init(useWgpu: Bool = false, r13Enabled: Bool = false,
         r13FailureInjection: Int32 = 0,
         r13RecoveryFailureInjection: Int32 = 0,
         r13WindowCycle: Bool = false,
         r05PresentationProbe: Bool = false) {
        self.useWgpu = useWgpu
        self.r13Enabled = r13Enabled
        self.r13WindowCycle = r13WindowCycle
        self.r05PresentationProbe = r05PresentationProbe
        if useWgpu {
            if r05PresentationProbe {
                self.canvas = R05WgpuCanvasView(
                    frame: .zero, r13Enabled: r13Enabled,
                    r13FailureInjection: r13FailureInjection,
                    r13RecoveryFailureInjection: r13RecoveryFailureInjection,
                    r05PresentationProbe: true)
            } else {
                self.canvas = R08WgpuCanvasView(
                    frame: .zero, r13Enabled: r13Enabled,
                    r13FailureInjection: r13FailureInjection,
                    r13RecoveryFailureInjection: r13RecoveryFailureInjection)
            }
        } else {
            self.canvas = R08MetalCanvasView(
                frame: .zero, device: MTLCreateSystemDefaultDevice(),
                r05PresentationProbe: r05PresentationProbe)
        }
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        self.useWgpu = false
        self.r13Enabled = false
        self.r13WindowCycle = false
        self.r05PresentationProbe = false
        self.canvas = R08MetalCanvasView(frame: .zero, device: MTLCreateSystemDefaultDevice())
        super.init(coder: coder)
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)

        canvas.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(canvas)

        titleLabel.translatesAutoresizingMaskIntoConstraints = false
        titleLabel.text = r05PresentationProbe
            ? "SPINON · R05 표시 신호 probe\niOS · \(useWgpu ? "wgpu / Metal" : "native Metal control")"
            : r13Enabled
            ? "SPINON · R13 GPU 복구\niOS · wgpu / Metal"
            : useWgpu
                ? "SPINON · R08 GPU 표면\niOS · wgpu / Metal"
                : "SPINON · R08 GPU 표면\niOS · Metal"
        titleLabel.textColor = UIColor(red: 0.92, green: 0.95, blue: 0.99, alpha: 1)
        titleLabel.font = .systemFont(ofSize: 22, weight: .bold)
        titleLabel.numberOfLines = 0
        titleLabel.accessibilityElementsHidden = true
        view.addSubview(titleLabel)

        statusLabel.translatesAutoresizingMaskIntoConstraints = false
        statusLabel.text = "GPU 표면 대기 중 · 텍스트 입력은 네이티브 IME 실험"
        statusLabel.textColor = UIColor(red: 0.78, green: 0.83, blue: 0.90, alpha: 1)
        statusLabel.font = .systemFont(ofSize: 13)
        statusLabel.numberOfLines = 0
        statusLabel.accessibilityElementsHidden = true
        view.addSubview(statusLabel)

        inputField.translatesAutoresizingMaskIntoConstraints = false
        inputField.borderStyle = .roundedRect
        inputField.backgroundColor = .white
        inputField.textColor = .black
        inputField.font = .systemFont(ofSize: 16)
        inputField.attributedPlaceholder = NSAttributedString(
            string: "텍스트 입력 · IME 경계 실험",
            attributes: [.foregroundColor: UIColor.darkGray])
        inputField.accessibilityLabel = r13Enabled ? "R13 텍스트 입력 실험" : "R08 텍스트 입력 실험"
        inputField.returnKeyType = .done
        inputField.autocorrectionType = .no
        inputField.delegate = self
        inputField.addTarget(self, action: #selector(textDidChange(_:)), for: .editingChanged)
        view.addSubview(inputField)

        let onActivate: (Int) -> Void = { [weak self] count in
            self?.statusLabel.text = "GPU 도형 활성화 \(count)회 · 텍스트 입력은 네이티브 오버레이"
        }
        (canvas as? R08MetalCanvasView)?.onActivate = onActivate
        (canvas as? R08WgpuCanvasView)?.onActivate = onActivate

        constrainCanvasToRootView()
        NSLayoutConstraint.activate([
            titleLabel.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 20),
            titleLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 22),
            titleLabel.trailingAnchor.constraint(lessThanOrEqualTo: view.trailingAnchor, constant: -22),
            statusLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 22),
            statusLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -22),
            statusLabel.bottomAnchor.constraint(equalTo: inputField.topAnchor, constant: -10),
            inputField.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 22),
            inputField.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -22),
            inputField.heightAnchor.constraint(equalToConstant: 54),
            inputField.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -14)
        ])

        if r13WindowCycle { runWindowCycleDiagnostic() }

        logger.notice("SPINON_R08_UI=ready text-input=UITextField accessibility=button+UITextField")
        if r05PresentationProbe {
            logger.notice("SPINON_R05_PROBE=ready renderer=\(self.useWgpu ? "wgpu" : "native-metal-control") input_source=unknown")
            if !useWgpu {
                logger.notice("SPINON_R05_SIGNAL_CAPABILITY renderer=native-metal-control signal=drawable_present_callback available=false reason=installed_sdk_headers_do_not_expose_api")
            }
            captureR05ClockAnchor(logger, label: "launch")
        }
    }

    private func constrainCanvasToRootView() {
        NSLayoutConstraint.activate([
            canvas.topAnchor.constraint(equalTo: view.topAnchor),
            canvas.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            canvas.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            canvas.bottomAnchor.constraint(equalTo: view.bottomAnchor)
        ])
    }

    private func runWindowCycleDiagnostic() {
        DispatchQueue.main.asyncAfter(deadline: .now() + .milliseconds(500)) { [weak self] in
            guard let self, self.canvas.window != nil else { return }
            self.canvas.removeFromSuperview()
            self.logger.notice("SPINON_R13_WINDOW_CYCLE=detached")
            DispatchQueue.main.asyncAfter(deadline: .now() + .milliseconds(250)) { [weak self] in
                guard let self else { return }
                self.view.insertSubview(self.canvas, at: 0)
                self.constrainCanvasToRootView()
                self.view.layoutIfNeeded()
                self.logger.notice("SPINON_R13_WINDOW_CYCLE=reattached")
            }
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        (canvas as? R08MetalCanvasView)?.draw()
        (canvas as? R08WgpuCanvasView)?.draw()
    }

    @objc private func textDidChange(_ textField: UITextField) {
        let length = textField.text?.count ?? 0
        let composing = textField.markedTextRange != nil
        statusLabel.text = "IME 입력 길이 \(length) · 조합 중 \(composing ? "예" : "아니요")"
        logger.notice("SPINON_R08_TEXT_INPUT length=\(length) composing=\(composing)")
    }

    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        logger.notice("SPINON_R08_IME_ACTION=done")
        textField.resignFirstResponder()
        return true
    }
}

private final class R05ProbeMetalLayer: CAMetalLayer {
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "r05-metal-layer")
    private let ticketLock = NSLock()
    private var activeDrawCount: UInt64 = 0
    private var activeTicket: R05DrawableTicket?
    private var ticketConsumed = false
    private var overlappingDraws = false

    fileprivate func beginWgpuDraw(ticket: R05DrawableTicket?) -> (tracked: Bool, isolated: Bool) {
        ticketLock.lock()
        let (nextCount, overflow) = activeDrawCount.addingReportingOverflow(1)
        guard !overflow else {
            overlappingDraws = true
            activeTicket = nil
            ticketConsumed = true
            ticketLock.unlock()
            return (false, false)
        }
        activeDrawCount = nextCount
        if nextCount == 1 {
            activeTicket = ticket
            ticketConsumed = false
            overlappingDraws = false
        } else {
            activeTicket = nil
            ticketConsumed = true
            overlappingDraws = true
        }
        ticketLock.unlock()
        return (true, nextCount == 1)
    }

    fileprivate func endWgpuDraw() {
        ticketLock.lock()
        if activeDrawCount > 0 {
            activeDrawCount -= 1
        }
        if activeDrawCount == 0 {
            activeTicket = nil
            ticketConsumed = false
            overlappingDraws = false
        }
        ticketLock.unlock()
    }

    private func takeActiveTicket() -> R05DrawableTicket? {
        ticketLock.lock()
        defer { ticketLock.unlock() }
        guard activeDrawCount == 1, !overlappingDraws, !ticketConsumed else { return nil }
        ticketConsumed = true
        let ticket = activeTicket
        return ticket
    }

    override func nextDrawable() -> (any CAMetalDrawable)? {
        let ticket = takeActiveTicket()
        let drawable = super.nextDrawable()
        let state = drawable == nil ? "unavailable" : "available"
        let ticketFields = ticket?.logFields ?? "attribution=unattributed"
        let thread = Thread.isMainThread ? "main" : "background"
        #if targetEnvironment(simulator)
        let drawableID = "not_recorded"
        #else
        let drawableID = drawable.map { String($0.drawableID) } ?? "none"
        #endif
        logger.notice("SPINON_R05_DRAWABLE_ACQUIRE renderer=wgpu state=\(state, privacy: .public) thread=\(thread, privacy: .public) drawable_id=\(drawableID, privacy: .public) \(ticketFields, privacy: .public)")
        #if !targetEnvironment(simulator)
        if let drawable {
            let callbackLogger = logger
            drawable.addPresentedHandler { presentedDrawable in
                let presentedTime = presentedDrawable.presentedTime
                let outcome = presentedTime > 0 ? "presented" : "zero_time"
                callbackLogger.notice("SPINON_R05_DRAWABLE_PRESENTED renderer=wgpu outcome=\(outcome, privacy: .public) drawable_id=\(presentedDrawable.drawableID, privacy: .public) presented_time_s=\(presentedTime, privacy: .public) \(ticketFields, privacy: .public)")
            }
        }
        #endif
        return drawable
    }
}

private class R08WgpuCanvasView: UIView {
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "r08")
    private let r13Enabled: Bool
    private let r05PresentationProbe: Bool
    private var renderer: UnsafeMutableRawPointer?
    private var activationCount: UInt32 = 0
    private var firstFrameLogged = false
    private var configuredSize = CGSize.zero
    private var rendererGeneration = 0
    private var pendingFailureInjection: Int32
    private var pendingRecoveryFailureInjection: Int32
    private var recoveryFailureForNextRenderer: Int32 = 0
    private var hostActive: Bool
    private var hasReachedActiveState = false
    private var resumeRedrawPending = false
    private var r05InputSequence = 0
    private var r05DrawSequence: UInt64 = 0
    private var pendingR05Input: R05InputSample?
    private var r05TouchGesture = R05TouchGesture()
    private var r05SurfaceGeneration = 0
    var onActivate: ((Int) -> Void)?

    override class var layerClass: AnyClass { CAMetalLayer.self }

    init(frame: CGRect, r13Enabled: Bool = false,
         r13FailureInjection: Int32 = 0,
         r13RecoveryFailureInjection: Int32 = 0,
         r05PresentationProbe: Bool = false) {
        self.r13Enabled = r13Enabled
        self.r05PresentationProbe = r05PresentationProbe
        self.pendingFailureInjection = r13FailureInjection
        self.pendingRecoveryFailureInjection = r13RecoveryFailureInjection
        self.hostActive = r13Enabled
            ? UIApplication.shared.applicationState == .active
            : true
        super.init(frame: frame)
        configure()
    }

    required init?(coder: NSCoder) {
        self.r13Enabled = false
        self.r05PresentationProbe = false
        self.pendingFailureInjection = 0
        self.pendingRecoveryFailureInjection = 0
        self.hostActive = true
        super.init(coder: coder)
        configure()
    }

    private func configure() {
        isOpaque = true
        backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        isAccessibilityElement = true
        accessibilityLabel = r13Enabled ? "R13 GPU 도형" : "R08 GPU 도형"
        accessibilityValue = "활성화 0회"
        accessibilityHint = "중앙 도형을 두 번 탭하면 색이 바뀝니다."
        accessibilityTraits = .button
        isMultipleTouchEnabled = r05PresentationProbe
        if r13Enabled {
            NotificationCenter.default.addObserver(
                self, selector: #selector(hostWillResignActive),
                name: UIApplication.willResignActiveNotification, object: nil)
            NotificationCenter.default.addObserver(
                self, selector: #selector(hostDidBecomeActive),
                name: UIApplication.didBecomeActiveNotification, object: nil)
        }
    }

    override func didMoveToWindow() {
        super.didMoveToWindow()
        if r05PresentationProbe {
            if window == nil {
                excludePendingR05Input(renderer: "wgpu", reason: "surface_detached")
                if let reason = r05TouchGesture.cancelForSurfaceDetach() {
                    logger.notice("SPINON_R05_INPUT=excluded renderer=wgpu reason=\(reason, privacy: .public)")
                }
            } else {
                r05SurfaceGeneration += 1
                logger.notice("SPINON_R05_SURFACE renderer=wgpu generation=\(self.r05SurfaceGeneration, privacy: .public)")
            }
        }
        if r13Enabled && window == nil {
            destroyRenderer()
            configuredSize = .zero
            logger.notice("SPINON_R13_SURFACE=detached")
        } else {
            setNeedsLayout()
            if window != nil { layoutIfNeeded() }
        }
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        guard window != nil, bounds.width > 0, bounds.height > 0 else { return }
        let scale = window?.screen.scale ?? UIScreen.main.scale
        let width = UInt32(max(1, Int((bounds.width * scale).rounded())))
        let height = UInt32(max(1, Int((bounds.height * scale).rounded())))
        layer.contentsScale = scale
        (layer as? CAMetalLayer)?.drawableSize = CGSize(width: CGFloat(width), height: CGFloat(height))

        guard ensureRenderer(width: width, height: height, reason: "layout") else { return }
        configuredSize = CGSize(width: CGFloat(width), height: CGFloat(height))
        if hostActive { draw() }
    }

    func draw() {
        guard hostActive else {
            excludePendingR05Input(renderer: "wgpu", reason: "host_inactive")
            return
        }
        if renderer == nil {
            setNeedsLayout()
            layoutIfNeeded()
            if renderer == nil {
                excludePendingR05Input(renderer: "wgpu", reason: "renderer_unavailable")
            }
            return
        }
        guard let renderer else {
            excludePendingR05Input(renderer: "wgpu", reason: "renderer_unavailable")
            return
        }
        let probeLayer = r05PresentationProbe ? layer as? R05ProbeMetalLayer : nil
        var probeTicket: R05DrawableTicket?
        var probeDrawTracked = false
        if let probeLayer {
            let (nextDrawSequence, overflow) = r05DrawSequence.addingReportingOverflow(1)
            if overflow {
                logger.error("SPINON_R05_DRAW_TICKET=unattributed reason=draw_sequence_overflow")
            } else {
                r05DrawSequence = nextDrawSequence
                probeTicket = R05DrawableTicket(
                    drawSequence: nextDrawSequence,
                    inputSequence: pendingR05Input?.sequence,
                    revision: activationCount,
                    surfaceGeneration: r05SurfaceGeneration)
            }
            let admission = probeLayer.beginWgpuDraw(ticket: probeTicket)
            probeDrawTracked = admission.tracked
            if !admission.isolated {
                logger.error("SPINON_R05_DRAW_TICKET=unattributed reason=overlapping_wgpu_draw")
            }
        }
        defer {
            if probeDrawTracked { probeLayer?.endWgpuDraw() }
        }
        let result = SpinonRunner.drawR08Wgpu(renderer, activationCount: activationCount)
        if let sample = pendingR05Input {
            let handlerReturn = ProcessInfo.processInfo.systemUptime
            logger.notice("SPINON_R05_SUBMIT renderer=wgpu input_seq=\(sample.sequence, privacy: .public) revision=\(self.activationCount, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public) event_time_s=\(sample.eventTimestamp, privacy: .public) handler_uptime_s=\(sample.handlerUptime, privacy: .public) call_return_uptime_s=\(handlerReturn, privacy: .public) result=\(result, privacy: .public) present_signal=unavailable")
            pendingR05Input = nil
        }
        if result == 0, !firstFrameLogged {
            firstFrameLogged = true
            logger.notice("SPINON_R08_WGPU_FRAME=first_draw_submitted")
            if r13Enabled { logger.notice("SPINON_R13_FRAME=submitted generation=\(self.rendererGeneration)") }
        }
        if result == 0 {
            logResumeRedrawSuccess()
        } else if r13Enabled && isRecoverable(result) {
            recoverRenderer(failureCode: result)
        } else if r13Enabled {
            logger.error("SPINON_R13_DRAW=failed code=\(result)")
        }
    }

    @objc private func hostWillResignActive() {
        hostActive = false
        if r13Enabled { logger.notice("SPINON_R13_HOST=inactive") }
    }

    @objc private func hostDidBecomeActive() {
        resumeRedrawPending = hasReachedActiveState && !hostActive
        hostActive = true
        hasReachedActiveState = true
        if r13Enabled { logger.notice("SPINON_R13_HOST=active") }
        setNeedsLayout()
        layoutIfNeeded()
    }

    private func ensureRenderer(width: UInt32, height: UInt32, reason: String) -> Bool {
        if renderer == nil {
            renderer = SpinonRunner.createR08Wgpu(
                withUIKitView: Unmanaged.passUnretained(self).toOpaque(),
                width: width,
                height: height)
            guard renderer != nil else {
                if r13Enabled { logger.error("SPINON_R13_RECOVERY=failed stage=create reason=\(reason)") }
                return false
            }
            rendererGeneration += 1
            if r13Enabled {
                logger.notice("SPINON_R13_RENDERER=created generation=\(self.rendererGeneration) reason=\(reason)")
            }
            var failure = pendingFailureInjection
            var injectionStage = "initial"
            if failure != 0 {
                pendingFailureInjection = 0
            } else if recoveryFailureForNextRenderer != 0 {
                failure = recoveryFailureForNextRenderer
                recoveryFailureForNextRenderer = 0
                injectionStage = "recovery_redraw"
            }
            if failure != 0, let renderer {
                let result = SpinonRunner.injectR13Failure(renderer, kind: UInt32(failure))
                if result == 0 {
                    logger.notice("SPINON_R13_FAULT=injected stage=\(injectionStage) kind=\(failure)")
                } else {
                    logger.error("SPINON_R13_FAULT=injection_failed code=\(result)")
                }
            }
            return true
        }
        if configuredSize != CGSize(width: CGFloat(width), height: CGFloat(height)) {
            let result = SpinonRunner.resizeR08Wgpu(renderer, width: width, height: height)
            guard result == 0 else {
                logger.error("SPINON_R08_WGPU_RESIZE_ERROR code=\(result)")
                if r13Enabled {
                    destroyRenderer()
                    return ensureRenderer(width: width, height: height, reason: "resize_recreate")
                }
                return false
            }
            if r13Enabled { logger.notice("SPINON_R13_SURFACE=resized \(width)x\(height)") }
        }
        return true
    }

    private func isRecoverable(_ result: Int32) -> Bool {
        return result == -3 || result == -4 || result == -5
    }

    private func failureName(_ result: Int32) -> String {
        if result == -3 { return "surface_lost" }
        if result == -4 { return "surface_outdated" }
        return "device_lost"
    }

    private func recoverRenderer(failureCode: Int32) {
        let reason = failureName(failureCode)
        logger.notice("SPINON_R13_RECOVERY=started reason=\(reason) generation=\(self.rendererGeneration)")
        recoveryFailureForNextRenderer = pendingRecoveryFailureInjection
        pendingRecoveryFailureInjection = 0
        destroyRenderer()
        guard let window else {
            logger.error("SPINON_R13_RECOVERY=failed reason=\(reason) stage=detached")
            return
        }
        let scale = window.screen.scale
        let width = UInt32(max(1, Int((bounds.width * scale).rounded())))
        let height = UInt32(max(1, Int((bounds.height * scale).rounded())))
        guard ensureRenderer(width: width, height: height, reason: "recover_\(reason)"),
              let renderer else {
            recoveryFailureForNextRenderer = 0
            logger.error("SPINON_R13_RECOVERY=failed reason=\(reason) stage=create")
            return
        }
        configuredSize = CGSize(width: CGFloat(width), height: CGFloat(height))
        let retry = SpinonRunner.drawR08Wgpu(renderer, activationCount: activationCount)
        if retry == 0 {
            logger.notice("SPINON_R13_RECOVERY=complete reason=\(reason) generation=\(self.rendererGeneration) redraw=success")
            logResumeRedrawSuccess()
        } else {
            logger.error("SPINON_R13_RECOVERY=failed reason=\(reason) stage=redraw code=\(retry)")
        }
    }

    private func logResumeRedrawSuccess() {
        if r13Enabled && resumeRedrawPending {
            resumeRedrawPending = false
            logger.notice("SPINON_R13_RESUME=redraw_success")
        }
    }

    private func destroyRenderer() {
        if let renderer {
            self.renderer = nil
            SpinonRunner.destroyR08Wgpu(renderer)
        }
    }

    private func excludePendingR05Input(renderer: String, reason: String) {
        guard let sample = pendingR05Input else { return }
        logger.notice("SPINON_R05_INPUT=excluded renderer=\(renderer, privacy: .public) reason=\(reason, privacy: .public) input_seq=\(sample.sequence, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public)")
        pendingR05Input = nil
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe { r05TouchGesture.began(touches, event: event, in: self) }
        super.touchesBegan(touches, with: event)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe { r05TouchGesture.moved(touches, event: event, in: self) }
        super.touchesMoved(touches, with: event)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe {
            let reason = r05TouchGesture.cancelled()
            logger.notice("SPINON_R05_INPUT=excluded renderer=wgpu reason=\(reason, privacy: .public)")
        }
        super.touchesCancelled(touches, with: event)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        let touch: UITouch
        if r05PresentationProbe {
            switch r05TouchGesture.ended(touches, event: event, in: self) {
            case .accepted(let acceptedTouch): touch = acceptedTouch
            case .excluded(let reason):
                logger.notice("SPINON_R05_INPUT=excluded renderer=wgpu reason=\(reason, privacy: .public)")
                super.touchesEnded(touches, with: event)
                return
            }
        } else {
            guard let endedTouch = touches.first else {
                super.touchesEnded(touches, with: event)
                return
            }
            let point = endedTouch.location(in: self)
            guard point.x >= bounds.width * 0.11,
                  point.x <= bounds.width * 0.89,
                  point.y >= bounds.height * 0.40,
                  point.y <= bounds.height * 0.60 else {
                super.touchesEnded(touches, with: event)
                return
            }
            touch = endedTouch
        }
        activate(from: touch)
    }

    override func accessibilityActivate() -> Bool {
        activate(from: nil)
        return true
    }

    override var accessibilityFrame: CGRect {
        get {
            guard let window else { return .zero }
            let visibleBounds = CGRect(
                x: bounds.width * 0.11,
                y: bounds.height * 0.40,
                width: bounds.width * 0.78,
                height: bounds.height * 0.20)
            let windowBounds = convert(visibleBounds, to: window)
            return window.screen.coordinateSpace.convert(windowBounds, from: window)
        }
        set {}
    }

    private func activate(from touch: UITouch?) {
        activationCount += 1
        accessibilityValue = "활성화 \(activationCount)회"
        onActivate?(Int(activationCount))
        let tag = r13Enabled ? "R13" : "R08"
        logger.notice("SPINON_\(tag)_TOUCH count=\(self.activationCount)")
        if r05PresentationProbe {
            guard let touch else {
                logger.notice("SPINON_R05_INPUT=excluded renderer=wgpu reason=no_touch_event revision=\(self.activationCount, privacy: .public)")
                logger.notice("SPINON_R05_REVISION_UNMATCHED renderer=wgpu revision=\(self.activationCount, privacy: .public) reason=non_touch_activation")
                draw()
                return
            }
            excludePendingR05Input(renderer: "wgpu", reason: "overlap_before_submission")
            r05InputSequence += 1
            let sample = R05InputSample(
                sequence: r05InputSequence,
                eventTimestamp: touch.timestamp,
                handlerUptime: ProcessInfo.processInfo.systemUptime)
            pendingR05Input = sample
            captureR05ClockAnchor(logger, label: "wgpu_input_\(sample.sequence)")
            logger.notice("SPINON_R05_INPUT renderer=wgpu input_seq=\(sample.sequence, privacy: .public) phase=ended input_source=unknown event_time_s=\(sample.eventTimestamp, privacy: .public) handler_uptime_s=\(sample.handlerUptime, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public)")
        }
        draw()
    }

    deinit {
        destroyRenderer()
    }
}

private final class R05WgpuCanvasView: R08WgpuCanvasView {
    override class var layerClass: AnyClass { R05ProbeMetalLayer.self }
}

private final class R08MetalCanvasView: MTKView, MTKViewDelegate {
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "r08")
    private var r05PresentationProbe = false
    private var pipeline: MTLRenderPipelineState?
    private var commandQueue: MTLCommandQueue?
    private var vertexBuffer: MTLBuffer?
    private var activationCount = 0
    private var firstFrameLogged = false
    private var r05InputSequence = 0
    private var r05SurfaceGeneration = 0
    private var pendingR05Input: R05InputSample?
    private var r05TouchGesture = R05TouchGesture()
    var onActivate: ((Int) -> Void)?

    override init(frame: CGRect, device: MTLDevice?) {
        super.init(frame: frame, device: device ?? MTLCreateSystemDefaultDevice())
        configure()
    }

    convenience init(frame: CGRect, device: MTLDevice?, r05PresentationProbe: Bool) {
        self.init(frame: frame, device: device)
        self.r05PresentationProbe = r05PresentationProbe
        isMultipleTouchEnabled = r05PresentationProbe
    }

    required init(coder: NSCoder) {
        self.r05PresentationProbe = false
        super.init(coder: coder)
        configure()
    }

    private func configure() {
        delegate = self
        colorPixelFormat = .bgra8Unorm
        clearColor = MTLClearColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        framebufferOnly = true
        isPaused = true
        enableSetNeedsDisplay = true
        isAccessibilityElement = true
        accessibilityLabel = "R08 GPU 도형"
        accessibilityValue = "활성화 0회"
        accessibilityHint = "중앙 도형을 두 번 탭하면 색이 바뀝니다."
        accessibilityTraits = .button
        isMultipleTouchEnabled = r05PresentationProbe

        guard let device else {
            logger.error("SPINON_R08_METAL_ERROR=no_device")
            return
        }
        commandQueue = device.makeCommandQueue()
        guard commandQueue != nil else {
            logger.error("SPINON_R08_METAL_ERROR=no_command_queue")
            return
        }

        let points: [SIMD2<Float>] = [
            SIMD2(-0.78, -0.20), SIMD2(0.78, -0.20),
            SIMD2(-0.78, 0.20), SIMD2(0.78, 0.20)
        ]
        vertexBuffer = device.makeBuffer(
            bytes: points,
            length: MemoryLayout<SIMD2<Float>>.stride * points.count,
            options: [])

        do {
            let source = """
            #include <metal_stdlib>
            using namespace metal;
            struct VertexOut { float4 position [[position]]; };
            vertex VertexOut r08_vertex(uint id [[vertex_id]], const device float2 *points [[buffer(0)]]) {
                VertexOut out;
                out.position = float4(points[id], 0.0, 1.0);
                return out;
            }
            fragment float4 r08_fragment(VertexOut in [[stage_in]], constant float4 &color [[buffer(0)]]) {
                return color;
            }
            """
            let library = try device.makeLibrary(source: source, options: nil)
            guard let vertex = library.makeFunction(name: "r08_vertex"),
                  let fragment = library.makeFunction(name: "r08_fragment") else {
                logger.error("SPINON_R08_METAL_ERROR=shader_function_missing")
                return
            }
            let descriptor = MTLRenderPipelineDescriptor()
            descriptor.vertexFunction = vertex
            descriptor.fragmentFunction = fragment
            descriptor.colorAttachments[0].pixelFormat = colorPixelFormat
            pipeline = try device.makeRenderPipelineState(descriptor: descriptor)
            logger.notice("SPINON_R08_METAL=ready device=\(device.name, privacy: .public)")
        } catch {
            logger.error("SPINON_R08_METAL_ERROR=\(String(describing: error), privacy: .public)")
        }
    }

    override func didMoveToWindow() {
        super.didMoveToWindow()
        if window != nil {
            if r05PresentationProbe {
                r05SurfaceGeneration += 1
                logger.notice("SPINON_R05_SURFACE renderer=native-metal-control generation=\(self.r05SurfaceGeneration, privacy: .public)")
            }
            draw()
        } else if r05PresentationProbe,
                  let reason = r05TouchGesture.cancelForSurfaceDetach() {
            logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=\(reason, privacy: .public)")
        }
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        logger.notice("SPINON_R08_SURFACE=size \(Int(size.width))x\(Int(size.height))")
    }

    func draw(in view: MTKView) {
        guard let descriptor = currentRenderPassDescriptor,
              let drawable = currentDrawable,
              let pipeline,
              let vertexBuffer,
              let commandBuffer = commandQueue?.makeCommandBuffer(),
              let encoder = commandBuffer.makeRenderCommandEncoder(descriptor: descriptor) else {
            if r05PresentationProbe, let sample = pendingR05Input {
                logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=drawable_or_pipeline_unavailable input_seq=\(sample.sequence, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public)")
                pendingR05Input = nil
            }
            return
        }

        encoder.setRenderPipelineState(pipeline)
        encoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)
        var color = activationCount.isMultiple(of: 2)
            ? SIMD4<Float>(0.20, 0.49, 0.96, 1.0)
            : SIMD4<Float>(0.98, 0.39, 0.28, 1.0)
        encoder.setFragmentBytes(&color, length: MemoryLayout<SIMD4<Float>>.stride, index: 0)
        encoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)
        encoder.endEncoding()
        if let sample = pendingR05Input {
            let inputSequence = sample.sequence
            let revision = activationCount
            let generation = r05SurfaceGeneration
            let logger = self.logger
            let eventTimestamp = sample.eventTimestamp
            let handlerUptime = sample.handlerUptime
            commandBuffer.addCompletedHandler { completedBuffer in
                let callbackUptime = ProcessInfo.processInfo.systemUptime
                logger.notice("SPINON_R05_GPU_COMPLETE renderer=native-metal-control input_seq=\(inputSequence, privacy: .public) revision=\(revision, privacy: .public) generation=\(generation, privacy: .public) event_time_s=\(eventTimestamp, privacy: .public) handler_uptime_s=\(handlerUptime, privacy: .public) callback_uptime_s=\(callbackUptime, privacy: .public) command_status=\(completedBuffer.status.rawValue, privacy: .public) presentation_signal=unavailable")
            }
            let requestUptime = ProcessInfo.processInfo.systemUptime
            logger.notice("SPINON_R05_SUBMIT renderer=native-metal-control input_seq=\(inputSequence, privacy: .public) revision=\(revision, privacy: .public) generation=\(generation, privacy: .public) event_time_s=\(eventTimestamp, privacy: .public) handler_uptime_s=\(handlerUptime, privacy: .public) present_request_uptime_s=\(requestUptime, privacy: .public) presentation_signal=unavailable")
            pendingR05Input = nil
        }
        commandBuffer.present(drawable)
        commandBuffer.commit()
        if !firstFrameLogged {
            firstFrameLogged = true
            logger.notice("SPINON_R08_FRAME=first_draw_submitted")
        }
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe { r05TouchGesture.began(touches, event: event, in: self) }
        super.touchesBegan(touches, with: event)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe { r05TouchGesture.moved(touches, event: event, in: self) }
        super.touchesMoved(touches, with: event)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        if r05PresentationProbe {
            let reason = r05TouchGesture.cancelled()
            logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=\(reason, privacy: .public)")
        }
        super.touchesCancelled(touches, with: event)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        let touch: UITouch
        if r05PresentationProbe {
            switch r05TouchGesture.ended(touches, event: event, in: self) {
            case .accepted(let acceptedTouch): touch = acceptedTouch
            case .excluded(let reason):
                logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=\(reason, privacy: .public)")
                super.touchesEnded(touches, with: event)
                return
            }
        } else {
            guard let endedTouch = touches.first else {
                super.touchesEnded(touches, with: event)
                return
            }
            let point = endedTouch.location(in: self)
            guard point.x >= bounds.width * 0.11,
                  point.x <= bounds.width * 0.89,
                  point.y >= bounds.height * 0.40,
                  point.y <= bounds.height * 0.60 else {
                super.touchesEnded(touches, with: event)
                return
            }
            touch = endedTouch
        }
        activate(from: touch)
    }

    override func accessibilityActivate() -> Bool {
        activate(from: nil)
        return true
    }

    override var accessibilityFrame: CGRect {
        get {
            guard let window else { return .zero }
            let visibleBounds = CGRect(
                x: bounds.width * 0.11,
                y: bounds.height * 0.40,
                width: bounds.width * 0.78,
                height: bounds.height * 0.20)
            let windowBounds = convert(visibleBounds, to: window)
            return window.screen.coordinateSpace.convert(windowBounds, from: window)
        }
        set {}
    }

    private func activate(from touch: UITouch?) {
        activationCount += 1
        accessibilityValue = "활성화 \(activationCount)회"
        onActivate?(activationCount)
        logger.notice("SPINON_R08_TOUCH count=\(self.activationCount)")
        if r05PresentationProbe {
            guard let touch else {
                logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=no_touch_event revision=\(self.activationCount, privacy: .public)")
                logger.notice("SPINON_R05_REVISION_UNMATCHED renderer=native-metal-control revision=\(self.activationCount, privacy: .public) reason=non_touch_activation")
                draw()
                return
            }
            if let previous = pendingR05Input {
                logger.notice("SPINON_R05_INPUT=excluded renderer=native-metal-control reason=overlap_before_submission input_seq=\(previous.sequence, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public)")
                pendingR05Input = nil
            }
            r05InputSequence += 1
            let sample = R05InputSample(
                sequence: r05InputSequence,
                eventTimestamp: touch.timestamp,
                handlerUptime: ProcessInfo.processInfo.systemUptime)
            pendingR05Input = sample
            captureR05ClockAnchor(logger, label: "native_input_\(sample.sequence)")
            logger.notice("SPINON_R05_INPUT renderer=native-metal-control input_seq=\(sample.sequence, privacy: .public) phase=ended input_source=unknown event_time_s=\(sample.eventTimestamp, privacy: .public) handler_uptime_s=\(sample.handlerUptime, privacy: .public) generation=\(self.r05SurfaceGeneration, privacy: .public)")
        }
        draw()
    }
}
