import UIKit
import OSLog

#if SPINON_ENABLE_S03_DOM_GC_FIXTURE_1
let spinonS03DomGcFixtureEnabled = true
#else
let spinonS03DomGcFixtureEnabled = false
#endif

#if SPINON_ENABLE_C14_1_APP_FIXTURE_TEST_1
let spinonC141AppFixtureTestEnabled = true
#else
let spinonC141AppFixtureTestEnabled = false
#endif

#if SPINON_ENABLE_C14_2_APP_FIXTURE_TEST_1
let spinonC142AppFixtureTestEnabled = true
#else
let spinonC142AppFixtureTestEnabled = false
#endif

#if SPINON_ENABLE_C14_3_APP_FIXTURE_TEST_1
let spinonC143AppFixtureTestEnabled = true
#else
let spinonC143AppFixtureTestEnabled = false
#endif

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "runtime")
    private let rootDocumentSessionLock = NSLock()
    private let rootDocumentEnvironmentQueue = DispatchQueue(
        label: "dev.spinon.root-html-environment", qos: .userInitiated)
    private var rootDocumentSession: UInt64 = 0
    private var rootDocumentEnvironmentRequestGeneration: UInt64 = 0
    private weak var rootDocumentHostViewController: RootDocumentHostViewController?
    private var applicationIsTerminating = false

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        let arguments = ProcessInfo.processInfo.arguments
        let runPriorityProbe = arguments.contains("--spinon-priority-probe")
        let runShutdownProbe = arguments.contains("--spinon-shutdown-probe")
        let runUaCascadeProbe = arguments.contains("--spinon-c048-ua-cascade")
        let runC0412RootHtmlFixture = arguments.contains("--spinon-c0412-root-html")
        let runC0412RootHtmlAssetFixture = arguments.contains("--spinon-c0412-root-html-asset")
        let runC124SessionStackCostProbe = arguments.contains("--spinon-c124-session-stack-cost")
        let runC124RuntimeMatrixProbe = arguments.contains("--spinon-c124-runtime-matrix")
        let c124RuntimeMatrixRunID = arguments.first {
            $0.hasPrefix("--spinon-c124-run-id=")
        }?.components(separatedBy: "=").last
        let runRuntimeGpuBase = arguments.contains("--spinon-c0410-runtime-gpu")
            || arguments.contains("--spinon-c131-compact-surface-matrix")
            || arguments.contains("--spinon-c131-compact-hit-identity-grid")
            || arguments.contains("--spinon-c131-compact-metal-pixel-centers")
            || arguments.contains("--spinon-c131-full-physical-metal-pixels")
            || arguments.contains("--spinon-c131-stacking-descendant-clip")
            || arguments.contains("--spinon-c131-overflow-clip")
            || arguments.contains("--spinon-c131-overflow-clip-border")
            || arguments.contains("--spinon-c132-scroll")
            || arguments.contains("--spinon-c133-scroll-gesture")
            || arguments.contains("--spinon-c133-tap-probe")
            || arguments.contains("--spinon-c1121-grid-placement")
            || arguments.contains(where: { $0.hasPrefix("--spinon-c1121-grid-scene=") })
            || arguments.contains("--spinon-c1122-grid-auto-placement")
            || arguments.contains(where: { $0.hasPrefix("--spinon-c1122-grid-scene=") })
            || arguments.contains("--spinon-c1123-grid-auto-placement")
            || arguments.contains(where: { $0.hasPrefix("--spinon-c1123-grid-scene=") })
            || arguments.contains("--spinon-c1132-grid-named-lines")
            || arguments.contains("--spinon-c1133-grid-template-areas")
            || arguments.contains("--spinon-c11-grid")
            || arguments.contains(where: { $0.hasPrefix("--spinon-c11-grid-case=") })
            || arguments.contains("--spinon-c0411-runtime-author-stylesheets")
            || arguments.contains("--spinon-c052-registered-properties")
            || arguments.contains("--spinon-c053-runtime-result-cache")
            || arguments.contains("--spinon-c054-incremental-restyle")
            || arguments.contains("--spinon-c061-percentage-dimensions")
            || arguments.contains("--spinon-c062-spacing-percentages")
            || arguments.contains("--spinon-c063-absolute-lengths")
            || arguments.contains("--spinon-c064-font-relative-units")
            || arguments.contains("--spinon-c065-typed-css-math")
            || arguments.contains("--spinon-c066-viewport-units")
            || arguments.contains("--spinon-c071-min-max-sizing")
            || arguments.contains("--spinon-c072-border-width")
            || arguments.contains("--spinon-c073-aspect-ratio")
            || arguments.contains("--spinon-c08-block-paint")
            || arguments.contains("--spinon-c0412-root-html-block-paint")
            || arguments.contains("--spinon-c0412-root-html-text-paint")
            || arguments.contains("--spinon-c091-block-formatting")
            || arguments.contains("--spinon-c092-margin-collapse")
            || arguments.contains("--spinon-c093-flow-root")
            || arguments.contains("--spinon-c101-flex-wrap")
            || arguments.contains("--spinon-c102-flex-distribution")
            || arguments.contains("--spinon-c1031-flex-reverse")
            || arguments.contains("--spinon-c1032-flex-order")
            || arguments.contains("--spinon-c1033-flex-alignment")
            || arguments.contains("--spinon-c1034-flex-baseline")
            || arguments.contains("--spinon-c1035-positioned-flex")
            || arguments.contains("--spinon-c121-static-relative")
            || arguments.contains("--spinon-c122-absolute-block")
            || arguments.contains("--spinon-c123-fixed-position")
            || arguments.contains("--spinon-c124-stacking-context")
            || arguments.contains("--spinon-c124-depth-safety")
            || arguments.contains(where: { $0.hasPrefix("--spinon-c124-metal-surface-readback=") })
            || arguments.contains("--spinon-c153-glyph-paint")
            || arguments.contains("--spinon-c153-hostdocument-text-paint")
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
        let runC184ImageRoute = arguments.contains("--spinon-c184-async-image")
#else
        let runC184ImageRoute = false
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        let runRuntimeGpu = runRuntimeGpuBase
            || arguments.contains("--spinon-c181-image-paint")
            || arguments.contains("--spinon-c182-alpha-image")
            || arguments.contains("--spinon-c183-object-fit-position")
            || runC184ImageRoute
#else
        let runRuntimeGpu = runRuntimeGpuBase
#endif
        let runR05AttributionProbe = arguments.contains("--spinon-r05-attribution")
        let runLifecycleProbe = arguments.contains("--spinon-dom-gc-auto")
        let runC142ImageRatioFixture = arguments.contains("--spinon-c142-image-ratio")
        if runC0412RootHtmlFixture || runC0412RootHtmlAssetFixture {
            let window = UIWindow(frame: UIScreen.main.bounds)
#if SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST_1
            let controller = C0412RootHtmlFixtureViewController()
            controller.loadsBundledRootHtmlAsset = runC0412RootHtmlAssetFixture
            window.rootViewController = controller
#else
            let controller = UIViewController()
            controller.view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
            let label = UILabel()
            label.translatesAutoresizingMaskIntoConstraints = false
            label.text = "C04.12 root HTML fixture-test 옵션을 켜서 빌드해야 합니다."
            label.textColor = .white
            label.textAlignment = .center
            label.numberOfLines = 0
            controller.view.addSubview(label)
            NSLayoutConstraint.activate([
                label.leadingAnchor.constraint(equalTo: controller.view.leadingAnchor, constant: 24),
                label.trailingAnchor.constraint(equalTo: controller.view.trailingAnchor, constant: -24),
                label.centerYAnchor.constraint(equalTo: controller.view.centerYAnchor),
            ])
            window.rootViewController = controller
#endif
            window.makeKeyAndVisible()
            self.window = window
            return true
        }
        if runC142ImageRatioFixture {
            let conflictingFixtureArguments = arguments.filter {
                $0.hasPrefix("--spinon-") && $0 != "--spinon-c142-image-ratio"
            }
            let window = UIWindow(frame: UIScreen.main.bounds)
            let controller = C142ImageRatioFixtureViewController()
            controller.fixtureEnabled = spinonC142AppFixtureTestEnabled
            controller.conflictingArguments = conflictingFixtureArguments
            window.rootViewController = controller
            window.makeKeyAndVisible()
            self.window = window
            return true
        }
        let runC143IntrinsicContributionsFixture = arguments.contains("--spinon-c143-intrinsic-contributions")
        if runC143IntrinsicContributionsFixture {
            let conflictingFixtureArguments = arguments.filter {
                $0.hasPrefix("--spinon-") && $0 != "--spinon-c143-intrinsic-contributions"
            }
            let window = UIWindow(frame: UIScreen.main.bounds)
            let controller = C143IntrinsicContributionsFixtureViewController()
            controller.fixtureEnabled = spinonC143AppFixtureTestEnabled
            controller.conflictingArguments = conflictingFixtureArguments
            window.rootViewController = controller
            window.makeKeyAndVisible()
            self.window = window
            return true
        }
        let runC141IntrinsicSizeFixture = arguments.contains("--spinon-c141-intrinsic-size")
        if runC141IntrinsicSizeFixture {
            // This route returns before all other experiments; reject their launch flags here.
            let conflictingFixtureArguments = arguments.filter {
                $0.hasPrefix("--spinon-") && $0 != "--spinon-c141-intrinsic-size"
            }
            let window = UIWindow(frame: UIScreen.main.bounds)
            let controller = C141IntrinsicSizeFixtureViewController()
            controller.fixtureEnabled = spinonC141AppFixtureTestEnabled
            controller.hasConflictingRoute = !conflictingFixtureArguments.isEmpty
            controller.conflictingArguments = conflictingFixtureArguments
            window.rootViewController = controller
            window.makeKeyAndVisible()
            self.window = window
            return true
        }
        if runC124RuntimeMatrixProbe {
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.rootViewController = UIViewController()
            window.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
            window.makeKeyAndVisible()
            self.window = window
            DispatchQueue.global(qos: .userInitiated).async {
                C124RuntimeMatrixProbe.run(runID: c124RuntimeMatrixRunID ?? "")
            }
            return true
        }
        if runC124SessionStackCostProbe {
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.rootViewController = UIViewController()
            window.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
            window.makeKeyAndVisible()
            self.window = window
            DispatchQueue.global(qos: .utility).async { [logger] in
                let targets = [0, 1, 2, 4, 8]
                var hosts: [UInt64] = []
                var failed = false
                for target in targets {
                    while hosts.count < target {
                        let handle = SpinonRunner.createRuntimeGpuHostWithC12_4StackingFixture()
                        guard handle != 0 else {
                            logger.error("SPINON_C124_SESSION_STACK status=create_failed target=\(target, privacy: .public) created=\(hosts.count, privacy: .public)")
                            failed = true
                            break
                        }
                        hosts.append(handle)
                    }
                    if failed { break }
                    let reservedBytes = hosts.count * 8 * 1024 * 1024
                    logger.notice("SPINON_C124_SESSION_STACK status=ready session_count=\(hosts.count, privacy: .public) css_stack_reserved_bytes=\(reservedBytes, privacy: .public) hold_seconds=8")
                    #if SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST_1
                    for sample in 1...3 {
                        for (sessionIndex, handle) in hosts.enumerated() {
                            let report = SpinonRunner.runtimeGpuC12_4WorkerStackResidencySample(handle)
                                ?? "{\"status\":\"error\",\"detail\":\"empty native report\"}"
                            logger.notice("SPINON_C124_WORKER_STACK_RESIDENCY stage=\(hosts.count, privacy: .public) sample=\(sample, privacy: .public) session_index=\(sessionIndex, privacy: .public) report=\(report, privacy: .public)")
                        }
                        Thread.sleep(forTimeInterval: 0.1)
                    }
                    #endif
                    Thread.sleep(forTimeInterval: 8)
                }
                for handle in hosts.reversed() {
                    SpinonRunner.freeRuntimeGpuHost(handle)
                }
                logger.notice("SPINON_C124_SESSION_STACK status=freed session_count=0 released_handles=\(hosts.count, privacy: .public) hold_seconds=10")
                Thread.sleep(forTimeInterval: 10)
                logger.notice("SPINON_C124_SESSION_STACK status=finished failed=\(failed ? 1 : 0, privacy: .public)")
            }
            return true
        }
        if runLifecycleProbe && !spinonS03DomGcFixtureEnabled {
            logger.error("SPINON_DOM_GC_FIXTURE_DISABLED · 검증 전용 빌드로 다시 빌드하세요")
        }
        if runRuntimeGpu {
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.rootViewController = C0410RuntimeGpuDemoViewController()
            window.makeKeyAndVisible()
            self.window = window
            return true
        }

        if arguments.contains("--spinon-runtime-threads") || runPriorityProbe || runShutdownProbe || runUaCascadeProbe || runR05AttributionProbe || runLifecycleProbe {
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
            window.rootViewController = RuntimeThreadExperimentViewController(
                automaticallyRun: arguments.contains("--spinon-r06-auto"),
                runPriorityProbe: runPriorityProbe,
                runShutdownProbe: runShutdownProbe,
                runUaCascadeProbe: runUaCascadeProbe,
                runR05AttributionProbe: runR05AttributionProbe,
                automaticallyRunLifecycleProbe: runLifecycleProbe && spinonS03DomGcFixtureEnabled
            )
            window.makeKeyAndVisible()
            self.window = window
            return true
        }

        if arguments.contains("--spinon-s04-ios") {
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.rootViewController = S04GpuDemoViewController()
            window.makeKeyAndVisible()
            self.window = window
            return true
        }

        let isR13 = arguments.contains("--spinon-r13")
        let runR05PresentationProbe = arguments.contains("--spinon-r05-presentation")
        let isR08 = arguments.contains("--spinon-r08")
            || arguments.contains("--spinon-r08-wgpu")
            || arguments.contains("--spinon-r08-native")
            || runR05PresentationProbe
        if isR08 || isR13 {
            let useWgpu = isR13 || !arguments.contains("--spinon-r08-native")
            let failureArgument = arguments.first { $0.hasPrefix("--spinon-r13-failure=") }
            let failureName = failureArgument?.components(separatedBy: "=").last
            let failureInjection: Int32 = switch failureName {
                case "surface": 1
                case "device": 2
                case "outdated": 3
                case "temporary": 4
                default: 0
            }
            let recoveryFailureArgument = arguments.first {
                $0.hasPrefix("--spinon-r13-recovery-failure=")
            }
            let recoveryFailureName = recoveryFailureArgument?.components(separatedBy: "=").last
            let recoveryFailureInjection: Int32 = switch recoveryFailureName {
                case "surface": 1
                case "device": 2
                case "outdated": 3
                case "temporary": 4
                default: 0
            }
            let runWindowCycle = arguments.contains("--spinon-r13-window-cycle")
            let window = UIWindow(frame: UIScreen.main.bounds)
            window.rootViewController = R08GpuDemoViewController(
                useWgpu: useWgpu, r13Enabled: isR13,
                r13FailureInjection: isR13 ? failureInjection : 0,
                r13RecoveryFailureInjection: isR13 ? recoveryFailureInjection : 0,
                r13WindowCycle: isR13 && runWindowCycle,
                r05PresentationProbe: runR05PresentationProbe)
            window.makeKeyAndVisible()
            self.window = window
            return true
        }


        let window = UIWindow(frame: UIScreen.main.bounds)
        let rootHost = RootDocumentHostViewController()
        rootHost.onEnvironmentChange = { [weak self] environment in
            self?.updateRootDocumentEnvironment(environment, source: "surface-change")
        }
        window.rootViewController = rootHost
        window.backgroundColor = .white
        window.makeKeyAndVisible()
        self.window = window
        self.rootDocumentHostViewController = rootHost
        runBootstrapInBackground()

        if ProcessInfo.processInfo.arguments.contains("--spinon-r10") {
            let screen = UIScreen.main
            let report = SpinonRunner.runTaffyR10(
                withWidth: Float(window.bounds.width),
                height: Float(window.bounds.height),
                scale: Float(screen.scale)
            ) ?? "empty R10 report"
            logger.notice("SPINON_TAFFY_R10_RESULT=\(report, privacy: .public)")

            let reportView = UITextView()
            reportView.translatesAutoresizingMaskIntoConstraints = false
            reportView.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
            reportView.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
            reportView.font = .monospacedSystemFont(ofSize: 12, weight: .regular)
            reportView.textContainerInset = UIEdgeInsets(top: 24, left: 18, bottom: 24, right: 18)
            reportView.isEditable = false
            reportView.text = "SPINON · R10 TAFFY 실험\n\niOS 시뮬레이터 · 개발 전용\n\n\(formatR10Report(report))"
            if let rootView = window.rootViewController?.view {
                rootView.addSubview(reportView)
                NSLayoutConstraint.activate([
                    reportView.topAnchor.constraint(equalTo: rootView.safeAreaLayoutGuide.topAnchor),
                    reportView.leadingAnchor.constraint(equalTo: rootView.leadingAnchor),
                    reportView.trailingAnchor.constraint(equalTo: rootView.trailingAnchor),
                    reportView.bottomAnchor.constraint(equalTo: rootView.bottomAnchor)
                ])
            }
        }
        return true
    }

    func applicationWillTerminate(_ application: UIApplication) {
        rootDocumentSessionLock.lock()
        applicationIsTerminating = true
        let handle = rootDocumentSession
        rootDocumentSession = 0
        rootDocumentSessionLock.unlock()
        rootDocumentEnvironmentQueue.sync {
            SpinonRunner.freeRuntimeSession(handle)
        }
    }

    private func runBootstrapInBackground() {
        DispatchQueue.global(qos: .userInitiated).async { [weak self, logger] in
            guard let self else { return }
            logger.notice("SPINON_BOOTSTRAP_EXECUTION is_main_thread=\(Thread.isMainThread)")
            if let sourceURL = Bundle.main.url(forResource: "app", withExtension: "js") {
                do {
                    let source = try String(contentsOf: sourceURL, encoding: .utf8)
                    let result = SpinonRunner.runSource(source) ?? "empty bootstrap result"
                    logger.notice("SPINON_BOOTSTRAP_RESULT=\(result, privacy: .public)")
                } catch {
                    logger.error("SPINON_BOOTSTRAP_ASSET_ERROR=\(String(describing: error), privacy: .public)")
                }
            } else {
                logger.error("SPINON_BOOTSTRAP_ASSET_ERROR=app.js missing")
            }
            self.loadRootDocumentSession()
        }
    }

    private func loadRootDocumentSession() {
        guard let htmlURL = Bundle.main.url(forResource: "index", withExtension: "html"),
              let appURL = Bundle.main.url(forResource: "root-app", withExtension: "js") else {
            logger.error("SPINON_ROOT_HTML_ASSET_ERROR=index.html or root-app.js missing")
            return
        }
        do {
            let html = try String(contentsOf: htmlURL, encoding: .utf8)
            let appSource = try String(contentsOf: appURL, encoding: .utf8)
            let handle = SpinonRunner.createRuntimeSession(withRootHTML: html)
            guard handle != 0 else {
                logger.error("SPINON_ROOT_HTML_BOOT_ERROR reason=session-create-failed")
                return
            }
            guard let environment = currentRootDocumentEnvironmentSnapshot() else {
                SpinonRunner.freeRuntimeSession(handle)
                logger.error("SPINON_ROOT_HTML_ENVIRONMENT_ERROR reason=surface-unavailable")
                return
            }
            let environmentReport = SpinonRunner.resolveRuntimeSessionRootHtmlCanvas(
                handle,
                surfaceWidthPx: environment.surfaceWidthPx,
                surfaceHeightPx: environment.surfaceHeightPx,
                deviceScaleFactor: environment.deviceScaleFactor,
                prefersDark: environment.prefersDark
            ) ?? "native runtime returned no environment report"
            guard environmentReport.hasPrefix("status=0") else {
                SpinonRunner.freeRuntimeSession(handle)
                logger.error("SPINON_ROOT_HTML_ENVIRONMENT_ERROR report=\(environmentReport, privacy: .public)")
                return
            }
            logger.notice("SPINON_ROOT_HTML_ENVIRONMENT source=activity-start \(environmentReport, privacy: .public)")
            let report = SpinonRunner.evalRuntimeSession(handle, source: appSource)
                ?? "native runtime returned no report"
            guard report.hasPrefix("status=0") else {
                SpinonRunner.freeRuntimeSession(handle)
                logger.error("SPINON_ROOT_HTML_BOOT_ERROR report=\(report, privacy: .public)")
                return
            }
            rootDocumentSessionLock.lock()
            let shouldKeepSession = !applicationIsTerminating
            if shouldKeepSession {
                rootDocumentSession = handle
            }
            rootDocumentSessionLock.unlock()
            if shouldKeepSession {
                logger.notice("SPINON_ROOT_HTML_BOOT status=pass source=index.html report=\(report, privacy: .public)")
                DispatchQueue.main.async { [weak self] in
                    self?.rootDocumentHostViewController?.publishCurrentEnvironment(force: true)
                }
            } else {
                SpinonRunner.freeRuntimeSession(handle)
            }
        } catch {
            logger.error("SPINON_ROOT_HTML_ASSET_ERROR=\(String(describing: error), privacy: .public)")
        }
    }

    private func currentRootDocumentEnvironmentSnapshot() -> RootDocumentSurfaceEnvironment? {
        if Thread.isMainThread {
            return rootDocumentHostViewController?.currentEnvironmentSnapshot()
        }
        return DispatchQueue.main.sync {
            rootDocumentHostViewController?.currentEnvironmentSnapshot()
        }
    }

    private func updateRootDocumentEnvironment(
        _ environment: RootDocumentSurfaceEnvironment,
        source: String
    ) {
        rootDocumentSessionLock.lock()
        let handle = applicationIsTerminating ? 0 : rootDocumentSession
        rootDocumentEnvironmentRequestGeneration &+= 1
        let requestGeneration = rootDocumentEnvironmentRequestGeneration
        rootDocumentSessionLock.unlock()
        guard handle != 0 else { return }
        rootDocumentEnvironmentQueue.async { [weak self] in
            guard let self else { return }
            let report = SpinonRunner.resolveRuntimeSessionRootHtmlCanvas(
                handle,
                surfaceWidthPx: environment.surfaceWidthPx,
                surfaceHeightPx: environment.surfaceHeightPx,
                deviceScaleFactor: environment.deviceScaleFactor,
                prefersDark: environment.prefersDark
            ) ?? "native runtime returned no Canvas report"
            guard report.hasPrefix("status=0") else {
                self.logger.error("SPINON_ROOT_HTML_ENVIRONMENT_ERROR source=\(source, privacy: .public) report=\(report, privacy: .public)")
                return
            }
            guard let canvasColor = self.rootCanvasColor(from: report) else {
                self.logger.error("SPINON_ROOT_HTML_CANVAS_ERROR reason=missing-canvas-color report=\(report, privacy: .public)")
                return
            }
            self.logger.notice("SPINON_ROOT_HTML_ENVIRONMENT source=\(source, privacy: .public) \(report, privacy: .public)")
            DispatchQueue.main.async {
                self.rootDocumentSessionLock.lock()
                let isCurrent = !self.applicationIsTerminating
                    && self.rootDocumentSession == handle
                    && self.rootDocumentEnvironmentRequestGeneration == requestGeneration
                self.rootDocumentSessionLock.unlock()
                guard isCurrent else { return }
                self.rootDocumentHostViewController?.applyCanvasColor(canvasColor)
                self.window?.backgroundColor = canvasColor
            }
        }
    }

    private func rootCanvasColor(from report: String) -> UIColor? {
        guard let marker = report.range(of: "canvas_color=") else { return nil }
        let value = report[marker.upperBound...].prefix(7)
        guard value.count == 7, value.first == "#",
              let red = UInt8(value.dropFirst().prefix(2), radix: 16),
              let green = UInt8(value.dropFirst(3).prefix(2), radix: 16),
              let blue = UInt8(value.dropFirst(5).prefix(2), radix: 16) else {
            return nil
        }
        return UIColor(
            red: CGFloat(red) / 255,
            green: CGFloat(green) / 255,
            blue: CGFloat(blue) / 255,
            alpha: 1
        )
    }

    private func formatR10Report(_ report: String) -> String {
        report
            .replacingOccurrences(of: " nodes=", with: "\nnodes=")
            .replacingOccurrences(of: " text-id=", with: "\ntext-id=")
            .replacingOccurrences(of: " measured=", with: "\nmeasured=")
            .replacingOccurrences(of: " rtl=", with: "\nrtl=")
            .replacingOccurrences(of: " ltr-button-offset=", with: "\nltr-button-offset=")
            .replacingOccurrences(of: " rtl-text-offset=", with: "\nrtl-text-offset=")
            .replacingOccurrences(of: " update=equivalent", with: "\nupdate=equivalent")
            .replacingOccurrences(of: " rounding=[", with: "\nrounding:\n  ")
            .replacingOccurrences(of: ",physical-pixel=", with: "\n  physical-pixel=")
            .replacingOccurrences(of: ",float=", with: "\n  float=")
            .replacingOccurrences(of: "] update-us-p50=", with: "\nupdate-us: p50=")
            .replacingOccurrences(of: " update-us-p95=", with: " p95=")
            .replacingOccurrences(of: " rebuild-us-p50=", with: "\nrebuild-us: p50=")
            .replacingOccurrences(of: " rebuild-us-p95=", with: " p95=")
            .replacingOccurrences(of: " iterations=", with: "\niterations=")
    }
}

private struct RootDocumentSurfaceEnvironment: Equatable {
    let surfaceWidthPx: Float
    let surfaceHeightPx: Float
    let deviceScaleFactor: Float
    let prefersDark: Bool
}

private final class RootDocumentHostViewController: UIViewController {
    var onEnvironmentChange: ((RootDocumentSurfaceEnvironment) -> Void)?
    private var lastPublishedEnvironment: RootDocumentSurfaceEnvironment?

    override func loadView() {
        let hostView = UIView()
        hostView.backgroundColor = .white
        view = hostView
    }

    func applyCanvasColor(_ color: UIColor) {
        view.backgroundColor = color
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        registerForTraitChanges([UITraitUserInterfaceStyle.self]) {
            (self: RootDocumentHostViewController, _: UITraitCollection) in
            self.publishCurrentEnvironment()
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        publishCurrentEnvironment()
    }

    func currentEnvironmentSnapshot() -> RootDocumentSurfaceEnvironment? {
        guard isViewLoaded else { return nil }
        view.layoutIfNeeded()
        let scale = view.window?.screen.scale ?? view.contentScaleFactor
        let size = view.bounds.size
        guard size.width.isFinite, size.height.isFinite,
              scale.isFinite, size.width > 0, size.height > 0, scale > 0 else {
            return nil
        }
        return RootDocumentSurfaceEnvironment(
            surfaceWidthPx: Float(size.width * scale),
            surfaceHeightPx: Float(size.height * scale),
            deviceScaleFactor: Float(scale),
            prefersDark: traitCollection.userInterfaceStyle == .dark
        )
    }

    func publishCurrentEnvironment(force: Bool = false) {
        guard let environment = currentEnvironmentSnapshot() else { return }
        guard force || environment != lastPublishedEnvironment else { return }
        lastPublishedEnvironment = environment
        onEnvironmentChange?(environment)
    }
}

#if SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST_1
private final class C0412RootHtmlFixtureViewController: UIViewController {
    var loadsBundledRootHtmlAsset = false
    private let reportView = UITextView()
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "c04-12-fixture")

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        reportView.translatesAutoresizingMaskIntoConstraints = false
        reportView.backgroundColor = .clear
        reportView.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
        reportView.font = .monospacedSystemFont(ofSize: 13, weight: .regular)
        reportView.textContainerInset = UIEdgeInsets(top: 32, left: 20, bottom: 24, right: 20)
        reportView.isEditable = false
        view.addSubview(reportView)
        NSLayoutConstraint.activate([
            reportView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            reportView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            reportView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            reportView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])
        reportView.text = "C04.12 · 실제 V8 root HTML document 확인 중…"

        let loadsBundledRootHtmlAsset = self.loadsBundledRootHtmlAsset
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            let report: String
            if loadsBundledRootHtmlAsset {
                guard let sourceURL = Bundle.main.url(
                    forResource: "c04-12-root-html-asset",
                    withExtension: "html"
                ) else {
                    report = "status=-1 bundled root HTML asset가 없습니다"
                    DispatchQueue.main.async {
                        self?.showC0412Result(report, isAssetFixture: true)
                    }
                    return
                }
                do {
                    let source = try String(contentsOf: sourceURL, encoding: .utf8)
                    report = SpinonRunner.runC0412RootHtmlAssetFixture(source)
                        ?? "status=-1 C ABI 응답이 비었습니다"
                } catch {
                    report = "status=-1 bundled root HTML asset 읽기 실패: \(error)"
                }
            } else {
                report = SpinonRunner.runC0412RootHtmlAccessorsFixture()
                    ?? "status=-1 C ABI 응답이 비었습니다"
            }
            DispatchQueue.main.async {
                guard let self else { return }
                self.showC0412Result(report, isAssetFixture: loadsBundledRootHtmlAsset)
            }
        }
    }

    private func showC0412Result(_ report: String, isAssetFixture: Bool) {
        let passed = report.contains("status=0") && (isAssetFixture
            ? report.contains("c04_12_root_html_asset=PASS") && report.contains("page_scale_wgpu=PASS")
            : report.contains("c04_12_root_html_accessors=PASS"))
        reportView.text = passed
            ? (isAssetFixture
                ? "C04.12 · root HTML + WGPU scene fixture\n\nPASS\n\n"
                    + "bundled HTML → V8 Document\n"
                    + "DPR × page scale · 3 viewport cases\n"
                    + "30 surface samples · image · glyph\n"
                    + "HTML body paint 아님"
                : "C04.12 · 실제 V8 DOM\n\nPASS\n\n"
                    + "documentElement · head · body\n"
                    + "meta viewport · meta color-scheme · style tree\n"
                    + "script inertness · HostDocument text/attribute")
            : (isAssetFixture
                ? "C04.12 · root HTML + WGPU scene fixture 실패\n\n\(report)"
                : "C04.12 · 실제 V8 DOM 실패\n\n\(report)")
        let name = isAssetFixture ? "SPINON_C0412_ROOT_HTML_ASSET" : "SPINON_C0412_ROOT_HTML"
        if passed {
            logger.notice("\(name, privacy: .public) status=pass")
        } else {
            logger.error("\(name, privacy: .public) status=fail report=\(report, privacy: .public)")
        }
    }
}
#endif

private final class C142ImageRatioFixtureViewController: UIViewController {
    var fixtureEnabled = false
    var conflictingArguments: [String] = []
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "c14-2-fixture")
    private let reportView = UITextView()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        reportView.translatesAutoresizingMaskIntoConstraints = false
        reportView.backgroundColor = .clear
        reportView.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
        reportView.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        reportView.textContainerInset = UIEdgeInsets(top: 28, left: 18, bottom: 24, right: 18)
        reportView.isEditable = false
        view.addSubview(reportView)
        NSLayoutConstraint.activate([
            reportView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            reportView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            reportView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            reportView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])

        guard conflictingArguments.isEmpty else {
            reportView.text = "C14.2 · 잘못된 실행 인자\n\n다른 fixture route와 함께 실행할 수 없습니다.\n"
                + conflictingArguments.joined(separator: "\n")
            logger.error("SPINON_C142_ARGUMENT_ERROR conflicting=fixture-route")
            return
        }
        guard fixtureEnabled else {
            reportView.text = "C14.2 · fixture-test 빌드가 꺼져 있습니다."
            logger.error("SPINON_C142_DISABLED fixture-test feature is off")
            return
        }

        reportView.text = "C14.2 · live V8 이미지 비율 계산 중…"
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            let report = SpinonRunner.runC142ImageRatioFixture()
                ?? "status=-1 report=empty native report"
            let passed = self?.reportMatches(report) ?? false
            DispatchQueue.main.async {
                guard let self else { return }
                self.reportView.text = self.formatReport(report, passed: passed)
                if passed {
                    self.logger.notice("SPINON_C142_SIMULATOR_REPORT status=ok")
                } else {
                    self.logger.error("SPINON_C142_SIMULATOR_REPORT status=error")
                }
            }
        }
    }

    private func reportMatches(_ report: String) -> Bool {
        guard report.hasPrefix("status=0 "),
              let result = decodedReport(report),
              result["status"] as? String == "ok",
              result["document_origin"] as? String == "live-v8",
              let cases = result["cases"] as? [[String: Any]], cases.count == 29,
              let documents = result["documents"] as? [[String: Any]], documents.count == 2,
              result["case_count"] as? Int == 29,
              result["matched_count"] as? Int == 29
        else { return false }

        for (index, document) in documents.enumerated() {
            let expectedKind = index == 0 ? "block" : "flex"
            let expectedCount = index == 0 ? 21 : 8
            guard document["kind"] as? String == expectedKind,
                  document["case_count"] as? Int == expectedCount,
                  document["matched_count"] as? Int == expectedCount,
                  document["intrinsic_source_matches"] as? Bool == true,
                  document["intrinsic_revision_matches"] as? Bool == true,
                  document["style_stamp_matches"] as? Bool == true,
                  document["style_diagnostics_empty"] as? Bool == true
            else { return false }
        }

        var identifiers = Set<String>()
        var blockCount = 0
        var flexCount = 0
        for item in cases {
            guard let id = item["id"] as? String,
                  identifiers.insert(id).inserted,
                  item["status"] as? String == "ok",
                  item["natural_size_matches"] as? Bool == true,
                  item["frame_matches"] as? Bool == true,
                  item["source_stamp_matches"] as? Bool == true,
                  item["parent_size_matches"] as? Bool == true,
                  item["align_items_matches"] as? Bool == true,
                  let frame = item["frame_css_px"] as? [NSNumber], frame.count == 4,
                  frame.allSatisfy({ $0.doubleValue.isFinite }),
                  let natural = item["natural_size_css_px"] as? [NSNumber],
                  natural.count == 2, natural[0].doubleValue == 24, natural[1].doubleValue == 16
            else { return false }
            if item["document_kind"] as? String == "block" {
                blockCount += 1
            } else if item["document_kind"] as? String == "flex" {
                flexCount += 1
            } else {
                return false
            }
        }
        return identifiers.count == 29 && blockCount == 21 && flexCount == 8
    }

    private func formatReport(_ report: String, passed: Bool) -> String {
        guard passed, let result = decodedReport(report),
              let cases = result["cases"] as? [[String: Any]],
              let documents = result["documents"] as? [[String: Any]]
        else { return "C14.2 · 앱 fixture 실패\n\n\(report)" }

        var lines = [
            "C14.2 · live V8 / Chrome 155",
            "",
            "PASS · 29/29 CSS px cases",
            "Block 21/21 · Flex 8/8 · PNG 24×16",
            "",
            "선택 경계 case · image frame (CSS px)",
        ]
        let selected = [
            "max-width-only", "content-box-edges", "aspect-ratio-border-box",
            "flex-auto-basis", "fixed-cross-stretch-max-height",
        ]
        for id in selected {
            guard let item = cases.first(where: { $0["id"] as? String == id }),
                  item["status"] as? String == "ok"
            else { return "C14.2 · 앱 fixture 결과가 일치하지 않습니다\n\n\(report)" }
            var line = "\(id) · \(frame(item["frame_css_px"] as? [NSNumber]))"
            if let parent = item["parent_frame_css_px"] as? [NSNumber] {
                line += " · parent \(frame(parent))"
            }
            lines.append(line)
        }
        for document in documents {
            lines.append("")
            lines.append("\(document["kind"] as? String ?? "unknown") revisions · doc "
                + "\(document["document_revision"] as? Int ?? -1) / "
                + "style \(document["style_revision"] as? Int ?? -1) / "
                + "env \(document["environment_revision"] as? Int ?? -1) / "
                + "intrinsic \(document["intrinsic_size_revision"] as? Int ?? -1)")
        }
        return lines.joined(separator: "\n")
    }

    private func decodedReport(_ report: String) -> [String: Any]? {
        let jsonReport = report.hasPrefix("status=0 report=")
            ? String(report.dropFirst("status=0 report=".count))
            : report
        guard let data = jsonReport.data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data)
        else { return nil }
        return value as? [String: Any]
    }

    private func frame(_ values: [NSNumber]?) -> String {
        guard let values, values.count == 4 else { return "frame 없음" }
        return "x=\(number(values[0])) y=\(number(values[1])) · "
            + "\(number(values[2]))×\(number(values[3]))"
    }

    private func number(_ value: NSNumber) -> String {
        let number = value.doubleValue
        return number.rounded() == number ? String(Int64(number)) : String(number)
    }
}

private final class C143IntrinsicContributionsFixtureViewController: UIViewController {
    var fixtureEnabled = false
    var conflictingArguments: [String] = []
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "c14-3-fixture")
    private let reportView = UITextView()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        reportView.translatesAutoresizingMaskIntoConstraints = false
        reportView.backgroundColor = .clear
        reportView.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
        reportView.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        reportView.textContainerInset = UIEdgeInsets(top: 28, left: 18, bottom: 24, right: 18)
        reportView.isEditable = false
        view.addSubview(reportView)
        NSLayoutConstraint.activate([
            reportView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            reportView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            reportView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            reportView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])

        guard conflictingArguments.isEmpty else {
            reportView.text = "C14.3 · 잘못된 실행 인자\n\n다른 fixture route와 함께 실행할 수 없습니다.\n"
                + conflictingArguments.joined(separator: "\n")
            logger.error("SPINON_C143_ARGUMENT_ERROR conflicting=fixture-route")
            return
        }
        guard fixtureEnabled else {
            reportView.text = "C14.3 · fixture-test 빌드가 꺼져 있습니다."
            logger.error("SPINON_C143_DISABLED fixture-test feature is off")
            return
        }

        reportView.text = "C14.3 · 14개 intrinsic contribution 계산 중…"
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            let report = SpinonRunner.runC143IntrinsicContributionsFixture()
                ?? "status=-1 report=empty native report"
            let passed = report.hasPrefix("status=0 ")
                && report.contains("\"status\":\"ok\"")
                && report.contains("\"matched_count\":14")
            DispatchQueue.main.async {
                guard let self else { return }
                self.reportView.text = self.formatReport(report, passed: passed)
                if passed {
                    self.logger.notice("SPINON_C143_SIMULATOR_REPORT status=ok")
                } else {
                    self.logger.error("SPINON_C143_SIMULATOR_REPORT status=error")
                }
            }
        }
    }

    private func formatReport(_ report: String, passed: Bool) -> String {
        let jsonReport = report.hasPrefix("status=0 report=")
            ? String(report.dropFirst("status=0 report=".count))
            : report
        guard passed,
              let data = jsonReport.data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data),
              let result = value as? [String: Any],
              let cases = result["cases"] as? [[String: Any]], cases.count == 14
        else {
            return "C14.3 · 앱 fixture 실패\n\n\(report)"
        }
        var lines = ["C14.3 · Chrome 155 oracle", "", "PASS · 14/14 CSS px cases", ""]
        for item in cases {
            let basis = pair(item["containing_block_content_size_css_px"] as? [NSNumber])
            let frame = item["image_frame_css_px"] as? [NSNumber]
            let frameText: String
            if let frame, frame.count >= 4 {
                frameText = "x=\(number(frame[0])) y=\(number(frame[1])) · "
                    + "\(number(frame[2]))×\(number(frame[3]))"
            } else {
                frameText = "확인할 수 없음"
            }
            lines.append("\(item["id"] as? String ?? "unknown") · \(basis) · \(frameText)")
        }
        lines.append("")
        lines.append("fit-content right overflow · 7 CSS px")
        return lines.joined(separator: "\n")
    }

    private func pair(_ values: [NSNumber]?) -> String {
        guard let values, values.count >= 2 else { return "확인할 수 없음" }
        return "\(number(values[0]))×\(number(values[1]))"
    }

    private func number(_ value: NSNumber) -> String {
        let number = value.doubleValue
        let rounded = (number * 1000).rounded() / 1000
        return rounded.rounded() == rounded ? String(Int64(rounded)) : String(rounded)
    }
}

private final class C141IntrinsicSizeFixtureViewController: UIViewController {
    var fixtureEnabled = false
    var hasConflictingRoute = false
    var conflictingArguments: [String] = []
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "c14-1-fixture")
    private let reportView = UITextView()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)
        reportView.translatesAutoresizingMaskIntoConstraints = false
        reportView.backgroundColor = .clear
        reportView.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
        reportView.font = .monospacedSystemFont(ofSize: 12, weight: .regular)
        reportView.textContainerInset = UIEdgeInsets(top: 28, left: 18, bottom: 24, right: 18)
        reportView.isEditable = false
        view.addSubview(reportView)
        NSLayoutConstraint.activate([
            reportView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            reportView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            reportView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            reportView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])

        if hasConflictingRoute {
            reportView.text = "C14.1 · 잘못된 실행 인자\n\n"
                + "다른 fixture route와 함께 실행할 수 없습니다.\n"
                + conflictingArguments.joined(separator: "\n")
            logger.error("SPINON_C141_ARGUMENT_ERROR conflicting=fixture-route")
            return
        }
        guard fixtureEnabled else {
            reportView.text = "C14.1 · fixture-test 빌드가 꺼져 있습니다."
            logger.error("SPINON_C141_DISABLED fixture-test feature is off")
            return
        }

        reportView.text = "C14.1 · CSS px intrinsic size 계산 중…"
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            let report = SpinonRunner.runC141IntrinsicSizeFixture()
                ?? "status=-1 report=empty native report"
            let passed = report.hasPrefix("status=0 ")
                && report.contains("\"status\":\"ok\"")
            DispatchQueue.main.async {
                guard let self else { return }
                self.reportView.text = self.formatReport(report, passed: passed)
                if passed {
                    self.logger.notice("SPINON_C141_SIMULATOR_REPORT status=ok")
                } else {
                    self.logger.error("SPINON_C141_SIMULATOR_REPORT status=error")
                }
            }
        }
    }

    private func formatReport(_ report: String, passed: Bool) -> String {
        let jsonReport = report.hasPrefix("status=0 report=")
            ? String(report.dropFirst("status=0 report=".count))
            : report
        guard passed,
              let data = jsonReport.data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data),
              let result = value as? [String: Any],
              let eval = result["live_v8_eval"] as? [String: Any],
              let parsed = result["parsed_root_html_v8"] as? [String: Any]
        else {
            return "C14.1 · 앱 fixture 실패\n\n\(report)"
        }
        return "C14.1 · 앱 fixture report\n\n"
            + "PASS · 두 입력 경로가 Chrome 155 oracle과 일치\n\n"
            + "JavaScript 생성 DOM\n\(measurement(eval))\n\n"
            + "파싱된 루트 HTML\n\(measurement(parsed))\n\n"
            + "HTML parser → 초기 V8 document → HostDocument snapshot\n"
            + "외부 fetch / 이미지 paint는 검사하지 않음\n\n"
            + "Viewport\n120 × 80 CSS px"
    }

    private func measurement(_ result: [String: Any]) -> String {
        let natural = pair(result["natural_size_css_px"] as? [NSNumber])
        let frame = result["frame_css_px"] as? [NSNumber]
        let frameDescription: String
        if let frame, frame.count >= 4 {
            frameDescription = "x=\(number(frame[0])) · y=\(number(frame[1])) · "
                + "width=\(number(frame[2])) · height=\(number(frame[3]))"
        } else {
            frameDescription = "확인할 수 없음"
        }
        let revisionsMatch = (result["intrinsic_source_matches"] as? Bool == true)
            && (result["intrinsic_revision_matches"] as? Bool == true)
        let status = result["status"] as? String ?? "missing"
        return "natural=\(natural) CSS px\nframe=\(frameDescription) CSS px\n"
            + "source/revision=\(revisionsMatch ? "일치" : "불일치") · status=\(status)"
    }

    private func pair(_ values: [NSNumber]?) -> String {
        guard let values, values.count >= 2 else { return "확인할 수 없음" }
        return "\(number(values[0])) × \(number(values[1]))"
    }

    private func number(_ value: NSNumber) -> String {
        let doubleValue = value.doubleValue
        return doubleValue.rounded() == doubleValue
            ? String(Int64(doubleValue))
            : String(doubleValue)
    }
}
