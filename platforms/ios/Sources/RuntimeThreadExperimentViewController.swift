import UIKit
import OSLog

final class RuntimeThreadExperimentViewController: UIViewController {
    private let maximumReportCharacters = 20_000
    private let reportTrimTargetCharacters = 16_000
    private let reportTruncationMarker = "… 이전 로그 생략 · 최근 기록만 표시 …\n\n"
    private let lifecycleLargeRegistryNodeCount = 16_385
    private let lifecycleLargeRegistryScanSamples = 10
    private let automaticallyRun: Bool
    private let runPriorityProbe: Bool
    private let runShutdownProbe: Bool
    private let runUaCascadeProbe: Bool
    private let runR05AttributionProbe: Bool
    private let automaticallyRunLifecycleProbe: Bool
    private let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "r06")
    private let runtimeCalls = DispatchQueue(
        label: "dev.spinon.r06.runtime-calls",
        qos: .userInitiated,
        attributes: .concurrent
    )
    private let runtimeCallGroup = DispatchGroup()
    private let runtimeCallSlots = DispatchSemaphore(value: 64)
    private let runtimeControl = DispatchQueue(label: "dev.spinon.r06.runtime-control", qos: .userInitiated)
    private let bootstrapSource: String
    private let lifecycleProbeSource: String
    private let attributionLogger = Logger(
        subsystem: "dev.spinon.bootstrap", category: "r05-ios-attribution"
    )

    private struct AttributionSample {
        let sequence: Int
        let actionNs: UInt64
        let workerStartNs: UInt64
        let ffiStartNs: UInt64
        let ffiEndNs: UInt64
        let mainPostNs: UInt64
        let mainCallbackNs: UInt64
        let report: String
    }

    private var session: UInt64 = 0
    private var heartbeatTimer: Timer?
    private var heartbeatCount = 0
    private var eventCount = 0
    private var scenarioRunning = false
    private var scenarioHeartbeatStart = 0
    private var scenarioEvalReport: String?
    private var scenarioDispatchReport: String?
    private var scenarioHadDispatch = false
    private var scenarioCancelStatus: Int32?
    private var cancelRequestPending = false
    private var delayedHostResponsePending = false
    private var isClosing = false
    private var reportBuffer = ""
    private var pendingReportEntries = ""
    private var reportFlushScheduled = false
    private var attributionDisplayLink: CADisplayLink?
    private var attributionPendingSample: AttributionSample?
    private var attributionCompletedSamples = 0
    private let attributionTargetSamples = 32

    private let statusLabel = UILabel()
    private let reportView = UITextView()
    private let eventButton = UIButton(type: .system)
    private let longEvalButton = UIButton(type: .system)
    private let cancelButton = UIButton(type: .system)
    private let delayedButton = UIButton(type: .system)
    private let recreateButton = UIButton(type: .system)
    private let lifecycleButton = UIButton(type: .system)

    init(
        automaticallyRun: Bool,
        runPriorityProbe: Bool = false,
        runShutdownProbe: Bool = false,
        runUaCascadeProbe: Bool = false,
        runR05AttributionProbe: Bool = false,
        automaticallyRunLifecycleProbe: Bool = false
    ) {
        self.automaticallyRun = automaticallyRun
        self.runPriorityProbe = runPriorityProbe
        self.runShutdownProbe = runShutdownProbe
        self.runUaCascadeProbe = runUaCascadeProbe
        self.runR05AttributionProbe = runR05AttributionProbe
        self.automaticallyRunLifecycleProbe = automaticallyRunLifecycleProbe
        if let sourceURL = Bundle.main.url(forResource: "app", withExtension: "js"),
           let source = try? String(contentsOf: sourceURL, encoding: .utf8) {
            bootstrapSource = source
        } else {
            bootstrapSource = "spinon.onEvent((nodeId) => spinon.setText(`이벤트:${nodeId}`));"
        }
        if let sourceURL = Bundle.main.url(forResource: "s03-lifecycle-probe", withExtension: "js"),
           let source = try? String(contentsOf: sourceURL, encoding: .utf8) {
            lifecycleProbeSource = source
        } else {
            lifecycleProbeSource = ""
        }
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("코더 초기화는 사용하지 않습니다")
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        configureView()
        if runUaCascadeProbe {
            setButtons(enabled: false)
            setStatus("실제 V8 UA cascade 검증 중…")
            DispatchQueue.global(qos: .userInitiated).async { [weak self] in
                let report = SpinonRunner.runUaCascadeProbe() ?? "UA cascade 검증 응답 없음"
                DispatchQueue.main.async {
                    guard let self else { return }
                    self.appendReport(report)
                    self.setStatus(report.contains("status=0 ua_cascade_probe=PASS")
                        ? "실제 V8 UA cascade 검증 통과"
                        : "실제 V8 UA cascade 검증 실패")
                }
            }
            return
        }
        if runShutdownProbe {
            setButtons(enabled: false)
            setStatus("실제 V8 세션 종료 검증 중…")
            DispatchQueue.global(qos: .userInitiated).async { [weak self] in
                let report = SpinonRunner.runRuntimeShutdownProbe() ?? "세션 종료 검증 응답 없음"
                DispatchQueue.main.async {
                    guard let self else { return }
                    self.appendReport(report)
                    self.setStatus(report.contains("status=0 shutdown_probe=PASS")
                        ? "V8 세션 종료 검증 통과"
                        : "V8 세션 종료 검증 실패")
                }
            }
            return
        }
        if runPriorityProbe {
            setButtons(enabled: false)
            setStatus("실제 V8 우선순위·큐 포화 검증 중…")
            DispatchQueue.global(qos: .userInitiated).async { [weak self] in
                let priority = SpinonRunner.runRuntimePriorityProbe() ?? "우선순위 검증 응답 없음"
                let fairness = SpinonRunner.runRuntimePriorityFairnessProbe() ?? "유입 검증 응답 없음"
                let report = priority + "\n" + fairness
                DispatchQueue.main.async {
                    guard let self else { return }
                    self.appendReport(report)
                    let passed = report.contains("status=0 priority_probe=PASS")
                        && report.contains("status=0 priority_stream_probe=PASS")
                        && report.contains("queue_saturation_probe=PASS")
                    self.setStatus(passed ? "실제 V8 우선순위·큐 포화 복구 검증 통과" : "실제 V8 우선순위·큐 포화 복구 검증 실패")
                }
            }
            return
        }
        heartbeatTimer = Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { [weak self] _ in
            guard let self else { return }
            heartbeatCount += 1
        }
        createInitialSession()
    }

    deinit {
        heartbeatTimer?.invalidate()
        attributionDisplayLink?.invalidate()
    }

    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        guard isMovingFromParent || isBeingDismissed || navigationController?.isBeingDismissed == true else {
            return
        }
        closeRuntimeSession()
    }

    private func configureView() {
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)

        let title = UILabel()
        if runUaCascadeProbe {
            title.text = "SPINON · C04.8 Runtime UA cascade"
        } else if automaticallyRunLifecycleProbe {
            title.text = "SPINON · iOS DOM wrapper 수명 검증"
        } else if runShutdownProbe {
            title.text = "SPINON · iOS V8 세션 종료 검증"
        } else {
            title.text = runPriorityProbe
                ? "SPINON · V8 우선순위·큐 포화 검증"
                : "SPINON · V8 실행 스레드 실험"
        }
        title.font = .systemFont(ofSize: 20, weight: .bold)
        title.textColor = UIColor(red: 0.90, green: 0.93, blue: 0.98, alpha: 1)
        title.numberOfLines = 0

        let description = UILabel()
        if runUaCascadeProbe {
            description.text = "개발 전용 · 실제 V8 DOM 변경, Stylo worker, revision JSON을 확인합니다"
        } else if automaticallyRunLifecycleProbe {
            description.text = "개발 전용 · V8 weak Global 회수 후 Rust HostDocument root와 node count를 확인합니다"
        } else if runShutdownProbe {
            description.text = "개발 전용 · 활성 평가 취소, 대기 명령 거부, 종료 후 호출 거부를 확인합니다"
        } else {
            description.text = runPriorityProbe
                ? "개발 전용 · 실제 V8 우선순위 FIFO, 큐 포화·거부·복구를 확인합니다"
                : "개발 전용 · 긴 JavaScript 실행 중에도 화면 입력과 JS 이벤트 대기·취소를 확인합니다"
        }
        description.font = .systemFont(ofSize: 13)
        description.textColor = UIColor(red: 0.66, green: 0.72, blue: 0.81, alpha: 1)
        description.numberOfLines = 0

        statusLabel.text = "V8 세션 초기화 중…"
        statusLabel.font = .systemFont(ofSize: 14, weight: .medium)
        statusLabel.textColor = UIColor(red: 0.38, green: 0.73, blue: 1, alpha: 1)
        statusLabel.numberOfLines = 0

        configureButton(eventButton, title: "터치 이벤트 보내기", action: #selector(sendEvent))
        configureButton(longEvalButton, title: "긴 JavaScript 실행 시작", action: #selector(startLongEvaluation))
        configureButton(cancelButton, title: "실행 취소", action: #selector(cancelExecution))
        configureButton(delayedButton, title: "지연 호스트 응답 모의 (0.5초)", action: #selector(simulateDelayedHostResponse))
        configureButton(recreateButton, title: "세션 종료 후 재생성", action: #selector(recreateSession))
        configureButton(lifecycleButton, title: "V8 약한 wrapper 회수 검증", action: #selector(runLifecycleCollectionProbe))

        reportView.backgroundColor = UIColor(red: 0.035, green: 0.047, blue: 0.075, alpha: 1)
        reportView.textColor = UIColor(red: 0.86, green: 0.89, blue: 0.94, alpha: 1)
        reportView.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        reportView.isEditable = false
        reportView.accessibilityIdentifier = "r06-report"

        var arrangedSubviews: [UIView] = [
            title, description, statusLabel, eventButton, longEvalButton, cancelButton,
            delayedButton
        ]
        if spinonS03DomGcFixtureEnabled {
            arrangedSubviews.append(lifecycleButton)
        }
        arrangedSubviews.append(reportView)
        let stack = UIStackView(arrangedSubviews: arrangedSubviews)
        stack.axis = .vertical
        stack.spacing = 8
        stack.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(stack)
        NSLayoutConstraint.activate([
            stack.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 12),
            stack.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
            stack.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
            stack.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -12),
            reportView.heightAnchor.constraint(greaterThanOrEqualToConstant: 120)
        ])
        reportView.setContentHuggingPriority(.defaultLow, for: .vertical)
        reportView.setContentCompressionResistancePriority(.defaultLow, for: .vertical)
        setButtons(enabled: false)
    }

    private func configureButton(_ button: UIButton, title: String, action: Selector) {
        var configuration = UIButton.Configuration.filled()
        configuration.title = title
        configuration.baseBackgroundColor = UIColor(red: 0.10, green: 0.31, blue: 0.58, alpha: 1)
        configuration.baseForegroundColor = .white
        configuration.cornerStyle = .medium
        configuration.contentInsets = NSDirectionalEdgeInsets(
            top: 0, leading: 12, bottom: 0, trailing: 12
        )
        button.configuration = configuration
        button.translatesAutoresizingMaskIntoConstraints = false
        button.contentHorizontalAlignment = .leading
        button.accessibilityIdentifier = title
        button.heightAnchor.constraint(equalToConstant: 48).isActive = true
        button.addTarget(self, action: action, for: .touchUpInside)
    }

    private func createInitialSession() {
        enqueueRuntimeCall(label: "초기 세션 생성") { [weak self] in
            guard let self else { return }
            let handle = SpinonRunner.createRuntimeSession()
            guard handle != 0 else {
                DispatchQueue.main.async {
                    guard !self.isClosing else { return }
                    self.appendReport("세션 생성 실패 · 시뮬레이터 로그의 생성 오류를 확인하세요")
                    self.setStatus("V8 세션 생성 실패")
                }
                return
            }
            let report = SpinonRunner.evalRuntimeSession(handle, source: bootstrapSource) ?? "응답 없음"
            DispatchQueue.main.async {
                guard !self.isClosing else {
                    self.enqueueSessionFree(handle)
                    return
                }
                self.session = handle
                self.appendReport("초기 JavaScript · \(report)")
                self.setStatus("준비됨 · V8 세션 대기 중")
                self.setButtons(enabled: true)
                if self.runR05AttributionProbe {
                    self.startR05AttributionProbe()
                    return
                }
                if self.automaticallyRunLifecycleProbe {
                    self.lifecycleButton.sendActions(for: .touchUpInside)
                }
                if self.automaticallyRun {
                    self.appendReport("자동 검증 시작 · UI 타깃 액션을 보낸 뒤 JS 취소")
                    self.startLongEvaluation()
                }
            }
        }
    }

    @objc private func runLifecycleCollectionProbe() {
        guard session != 0, !scenarioRunning, !isClosing else { return }
        lifecycleButton.isEnabled = false
        setStatus("V8 callback root와 반복 DOM 수명 회수 검증 중…")
        let handle = session
        let accepted = enqueueRuntimeCall(label: "DOM weak wrapper 회수") { [weak self] in
            guard let self else { return }
            guard !self.lifecycleProbeSource.isEmpty else {
                DispatchQueue.main.async {
                    self.appendReport("실패 · DOM 수명 fixture asset 없음")
                    self.lifecycleButton.isEnabled = true
                    self.setStatus("검증 전용 fixture resource가 없습니다")
                }
                return
            }

            let loaded = SpinonRunner.evalRuntimeSession(
                handle, source: self.lifecycleProbeSource) ?? "응답 없음"
            guard loaded.hasPrefix("status=0 ") else {
                DispatchQueue.main.async {
                    self.appendReport("실패 · DOM 수명 fixture 로드 · \(loaded)")
                    self.lifecycleButton.isEnabled = true
                    self.setStatus("DOM 수명 fixture 로드 실패")
                }
                return
            }

            let reset = self.callLifecycleFixture(handle, "reset")
            let baselineFirst = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
            let baseline = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
            let baselineNodes = self.integerField("document_nodes", in: baseline)
            let baselineStringUnits = self.integerField("document_string_units", in: baseline)
            let baselineWrapperHandles = self.integerField(
                "document_collection_scanned_handles", in: baseline)
            let baselineReady = reset.hasPrefix("status=0 ")
                && self.collectionIsHealthy(reset)
                && self.collectionIsHealthy(baselineFirst)
                && self.collectionIsHealthy(baseline)
                && baselineNodes >= 0 && baselineStringUnits >= 0
                && self.integerField("document_collection_empty_handles", in: baseline) == 0
                && self.integerField("document_collection_live_handles", in: baseline)
                    == baselineWrapperHandles
            guard baselineReady else {
                DispatchQueue.main.async {
                    self.appendReport("실패 · 기준선 안정화 · nodes=\(baselineNodes) strings=\(baselineStringUnits) wrappers=\(baselineWrapperHandles)")
                    self.appendReport("기준선 실행 · \(baseline)")
                    self.lifecycleButton.isEnabled = true
                    self.setStatus("DOM 자원 기준선을 만들지 못했습니다")
                }
                return
            }

            let rootSetup = self.callLifecycleFixture(handle, "setupRootCases")
            let rootVerify = self.callLifecycleFixture(handle, "verifyRootCases")
            let rootSetupNodes = self.integerField("document_nodes", in: rootSetup)
            let rootCasesPassed = rootSetup.hasPrefix("status=0 ")
                && rootVerify.hasPrefix("status=0 ")
                && rootSetupNodes == baselineNodes + 4
                && self.collectionIsHealthy(rootSetup)
                && self.collectionIsHealthy(rootVerify)
                && self.scanCountsAreConsistent(rootVerify)
            let rootCleanup = self.callLifecycleFixture(handle, "cleanupRootCases")
            let rootCleanupStable = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
            let initialBaselineReturned = rootCleanup.hasPrefix("status=0 ")
                && self.collectionIsHealthy(rootCleanup)
                && self.matchesResourceBaseline(rootCleanupStable, nodes: baselineNodes,
                    stringUnits: baselineStringUnits, wrapperHandles: baselineWrapperHandles)

            let callbackSetup = self.callLifecycleFixture(handle, "setupCallbackClosureRoot")
            let callbackNodes = self.integerField("document_nodes", in: callbackSetup)
            let callbackStringUnits = self.integerField("document_string_units", in: callbackSetup)
            let expectedCallbackStringUnits = baselineStringUnits
                + "http://www.w3.org/1999/xhtml".utf16.count
                + "aside".utf16.count + "callback-root".utf16.count
            var callbackClosureRootPassed = callbackSetup.hasPrefix("status=0 ")
                && callbackNodes == baselineNodes + 2
                && callbackStringUnits == expectedCallbackStringUnits
                && self.collectionIsHealthy(callbackSetup)
            let callbackDispatch = SpinonRunner.dispatchRuntimeSession(handle, nodeID: 1) ?? "응답 없음"
            let callbackRootVerify = self.callLifecycleFixture(handle, "verifyCallbackClosureRoot")
            callbackClosureRootPassed = callbackClosureRootPassed
                && callbackDispatch.hasPrefix("status=0 ")
                && callbackRootVerify.hasPrefix("status=0 ")
                && self.collectionIsHealthy(callbackDispatch)
                && self.collectionIsHealthy(callbackRootVerify)

            let callbackRelease = self.callLifecycleFixture(handle, "replaceCallbackClosureRoot")
            let callbackReleaseVerify = self.callLifecycleFixture(
                handle, "verifyCallbackClosureReleased")
            let callbackReleaseStable = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
            let callbackClosureReleased = callbackRelease.hasPrefix("status=0 ")
                && callbackReleaseVerify.hasPrefix("status=0 ")
                && self.collectionIsHealthy(callbackRelease)
                && self.collectionIsHealthy(callbackReleaseVerify)
                && self.matchesResourceBaseline(callbackReleaseStable, nodes: baselineNodes,
                    stringUnits: baselineStringUnits, wrapperHandles: baselineWrapperHandles)

            let largeSetup = self.callLifecycleFixture(handle, "setupLargeRegistry")
            let largeVerify = self.callLifecycleFixture(handle, "verifyLargeRegistry")
            let expectedLargeNodes = baselineNodes + self.lifecycleLargeRegistryNodeCount
            let expectedLargeWrappers = baselineWrapperHandles
                + self.lifecycleLargeRegistryNodeCount
            var largeRegistryScanSamplesUs = [Int]()
            var largeRegistryRepeatedScansPassed = true
            for _ in 0..<self.lifecycleLargeRegistryScanSamples {
                let scanReport = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
                let scanUs = self.integerField("document_collection_last_scan_us", in: scanReport)
                let samplePassed = scanReport.hasPrefix("status=0 ")
                    && self.collectionIsHealthy(scanReport)
                    && self.integerField("document_nodes", in: scanReport) == expectedLargeNodes
                    && self.integerField("document_collection_scanned_handles", in: scanReport)
                        == expectedLargeWrappers
                    && self.integerField("document_collection_live_handles", in: scanReport)
                        == expectedLargeWrappers
                    && self.integerField("document_collection_empty_handles", in: scanReport) == 0
                    && scanUs >= 0
                guard samplePassed else {
                    largeRegistryRepeatedScansPassed = false
                    break
                }
                largeRegistryScanSamplesUs.append(scanUs)
            }
            let largeSetupScanUs = self.integerField(
                "document_collection_last_scan_us", in: largeSetup)
            let largeSetupRootBufferBytes = self.integerField(
                "document_collection_wrapper_root_buffer_bytes", in: largeSetup)
            let largeRegistryRetained = largeSetup.hasPrefix("status=0 ")
                && largeVerify.hasPrefix("status=0 ")
                && self.collectionIsHealthy(largeSetup)
                && self.collectionIsHealthy(largeVerify)
                && self.integerField("document_nodes", in: largeSetup) == expectedLargeNodes
                && self.integerField("document_collection_scanned_handles", in: largeSetup)
                    == expectedLargeWrappers
                && self.integerField("document_collection_live_handles", in: largeSetup)
                    == expectedLargeWrappers
                && self.integerField("document_collection_empty_handles", in: largeSetup) == 0
                && largeSetupRootBufferBytes >= expectedLargeWrappers * MemoryLayout<Int32>.size
                && largeSetupScanUs >= 0
                && largeRegistryRepeatedScansPassed
                && largeRegistryScanSamplesUs.count == self.lifecycleLargeRegistryScanSamples
            let largeRelease = self.callLifecycleFixture(handle, "releaseLargeRegistry")
            let largeEmptyWrappers = self.integerField(
                "document_collection_empty_handles", in: largeRelease)
            let largeReclaimedNodeBufferBytes = self.integerField(
                "document_collection_reclaimed_node_buffer_bytes", in: largeRelease)
            let largeReleaseVerify = self.callLifecycleFixture(
                handle, "verifyLargeRegistryReleased")
            let largeReleaseStable = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
            let largeRegistryReleased = largeRelease.hasPrefix("status=0 ")
                && largeReleaseVerify.hasPrefix("status=0 ")
                && self.collectionIsHealthy(largeRelease)
                && self.collectionIsHealthy(largeReleaseVerify)
                && largeEmptyWrappers == self.lifecycleLargeRegistryNodeCount
                && self.integerField("document_collection_scanned_handles", in: largeRelease)
                    == expectedLargeWrappers
                && self.integerField("document_collection_live_handles", in: largeRelease)
                    == baselineWrapperHandles
                && largeReclaimedNodeBufferBytes
                    >= self.lifecycleLargeRegistryNodeCount * MemoryLayout<Int32>.size
                && self.matchesResourceBaseline(largeReleaseStable, nodes: baselineNodes,
                    stringUnits: baselineStringUnits, wrapperHandles: baselineWrapperHandles)

            var baselineReturnRounds = 0
            var maximumEmptyWrappers = 0
            for _ in 0..<6 {
                let stress = self.callLifecycleFixture(handle, "stressRound")
                let stable = SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음"
                let emptyWrappers = self.integerField(
                    "document_collection_empty_handles", in: stress)
                maximumEmptyWrappers = max(maximumEmptyWrappers, emptyWrappers)
                let roundReturned = stress.hasPrefix("status=0 ")
                    && self.collectionIsHealthy(stress)
                    && self.scanCountsAreConsistent(stress)
                    && self.integerField("document_nodes", in: stress) == baselineNodes
                    && self.integerField("document_string_units", in: stress) == baselineStringUnits
                    && emptyWrappers >= 64
                    && self.matchesResourceBaseline(stable, nodes: baselineNodes,
                        stringUnits: baselineStringUnits, wrapperHandles: baselineWrapperHandles)
                guard roundReturned else { break }
                baselineReturnRounds += 1
            }

            let finalReport = baselineReturnRounds > 0
                ? (SpinonRunner.evalRuntimeSession(handle, source: "") ?? "응답 없음")
                : callbackReleaseStable
            let finalNodes = self.integerField("document_nodes", in: finalReport)
            let finalStringUnits = self.integerField("document_string_units", in: finalReport)
            let finalBaselineReturned = self.matchesResourceBaseline(finalReport,
                nodes: baselineNodes, stringUnits: baselineStringUnits,
                wrapperHandles: baselineWrapperHandles)
            let repeatedBaseline = baselineReturnRounds == 6 && finalBaselineReturned
            let scanStatsPassed = [reset, baselineFirst, baseline, rootSetup, rootVerify,
                rootCleanup, rootCleanupStable, callbackSetup, callbackDispatch,
                callbackRootVerify, callbackRelease, callbackReleaseVerify,
                callbackReleaseStable, largeSetup, largeVerify, largeRelease,
                largeReleaseVerify, largeReleaseStable, finalReport]
                .allSatisfy(self.scanCountsAreConsistent)
                && baselineReturnRounds == 6
            let passed = rootCasesPassed && initialBaselineReturned
                && callbackClosureRootPassed && callbackClosureReleased
                && largeRegistryRetained && largeRegistryReleased && repeatedBaseline
                && largeRegistryRepeatedScansPassed
                && scanStatsPassed
            DispatchQueue.main.async {
                guard !self.isClosing else { return }
                self.appendReport("DOM 자원 기준선 · nodes=\(baselineNodes) strings=\(baselineStringUnits) wrappers=\(baselineWrapperHandles)")
                self.appendReport("live root 보존·orphan 회수 · \(rootCasesPassed ? "통과" : "실패") · setup nodes=\(rootSetupNodes)")
                self.appendReport("callback closure root·호출 · \(callbackClosureRootPassed ? "통과" : "실패") · nodes=\(callbackNodes) strings=\(callbackStringUnits)")
                self.appendReport("callback 교체 후 해제·기준선 복귀 · \(callbackClosureReleased ? "통과" : "실패")")
                self.appendReport("16,385개 wrapper 유지 · \(largeRegistryRetained ? "통과" : "실패") · nodes=\(self.integerField("document_nodes", in: largeSetup)) wrappers=\(self.integerField("document_collection_live_handles", in: largeSetup)) scan=\(largeSetupScanUs)µs root-buffer=\(largeSetupRootBufferBytes)B")
                self.appendReport("16,385개 wrapper 재scan · \(largeRegistryRepeatedScansPassed ? "통과" : "실패") · 표본 \(largeRegistryScanSamplesUs.count)회 · scan-us=\(largeRegistryScanSamplesUs.map(String.init).joined(separator: ","))")
                self.appendReport("16,385개 wrapper 해제 · \(largeRegistryReleased ? "통과" : "실패") · empty=\(largeEmptyWrappers) reclaimed-buffer=\(largeReclaimedNodeBufferBytes)B")
                self.appendReport("반복 수명 회수 · \(repeatedBaseline ? "통과" : "실패") · \(baselineReturnRounds)/6회 · 회차당 32쌍 · 최대 빈 wrapper \(maximumEmptyWrappers)")
                self.appendReport("DOM 자원 최종 기준선 · nodes=\(finalNodes) strings=\(finalStringUnits) wrappers=\(self.integerField("document_collection_scanned_handles", in: finalReport))")
                self.appendReport("회수 scan 계수 일관성 · \(scanStatsPassed ? "통과" : "실패") · scanned=live+empty")
                self.appendReport("\(passed ? "통과" : "실패") · Rust 노드·UTF-16 문자열·weak wrapper 기준선 대조")
                self.lifecycleButton.isEnabled = true
                self.setStatus(passed ? "V8 반복 수명 회수 검증 통과" : "V8 반복 수명 회수 검증 실패")
            }
        }
        if !accepted {
            lifecycleButton.isEnabled = true
            setStatus("DOM GC 검증 호출이 대기열에서 거부되었습니다")
        }
    }

    private func callLifecycleFixture(_ handle: UInt64, _ method: String) -> String {
        let source = "globalThis.__spinonS03LifecycleProbeV1.\(method)();"
        return SpinonRunner.evalRuntimeSession(handle, source: source) ?? "응답 없음"
    }

    private func integerField(_ key: String, in report: String) -> Int {
        Int(field(key, in: report) ?? "") ?? -1
    }

    private func collectionIsHealthy(_ report: String) -> Bool {
        report.hasPrefix("status=0 ")
            && field("document_collection_error", in: report) == "none"
            && field("document_collection_poisoned", in: report) == "0"
            && field("document_collection_deferred", in: report) == "0"
            && scanCountsAreConsistent(report)
    }

    private func scanCountsAreConsistent(_ report: String) -> Bool {
        let scanned = integerField("document_collection_scanned_handles", in: report)
        let live = integerField("document_collection_live_handles", in: report)
        let empty = integerField("document_collection_empty_handles", in: report)
        return scanned >= 0 && live >= 0 && empty >= 0 && scanned == live + empty
    }

    private func matchesResourceBaseline(
        _ report: String, nodes: Int, stringUnits: Int, wrapperHandles: Int
    ) -> Bool {
        collectionIsHealthy(report)
            && integerField("document_nodes", in: report) == nodes
            && integerField("document_string_units", in: report) == stringUnits
            && integerField("document_collection_scanned_handles", in: report) == wrapperHandles
            && integerField("document_collection_live_handles", in: report) == wrapperHandles
            && integerField("document_collection_empty_handles", in: report) == 0
    }

    @objc private func sendEvent() {
        guard session != 0, !isClosing else { return }
        eventCount += 1
        let eventNumber = eventCount
        if runR05AttributionProbe {
            submitR05AttributionEvent(eventNumber)
            return
        }
        setStatus("UI 탭 \(eventNumber)회 · JS 이벤트 제출")
        appendReport("UI 타깃 액션 \(eventNumber) 제출")
        if scenarioRunning {
            scenarioHadDispatch = true
        }
        let handle = session
        let accepted = enqueueRuntimeCall(label: "이벤트 \(eventNumber)") { [weak self] in
            let report = SpinonRunner.dispatchRuntimeSession(handle, nodeID: Int32(eventNumber)) ?? "응답 없음"
            DispatchQueue.main.async {
                guard let self else { return }
                self.appendReport("이벤트 \(eventNumber) · \(report)")
                if self.scenarioRunning && self.scenarioDispatchReport == nil {
                    self.scenarioDispatchReport = report
                    self.finishScenarioIfReady()
                }
            }
        }
        if !accepted && scenarioRunning {
            scenarioDispatchReport = "status=-5 플랫폼 실행 대기열 포화"
            finishScenarioIfReady()
        }
    }

    private func startR05AttributionProbe() {
        let displayLink = CADisplayLink(target: self, selector: #selector(captureR05DisplayLinkTick))
        displayLink.add(to: .main, forMode: .common)
        attributionDisplayLink = displayLink
        attributionLogger.notice(
            "SPINON_R05_IOS_START samples=\(self.attributionTargetSamples)"
        )
        scheduleR05AttributionEvent()
    }

    private func scheduleR05AttributionEvent() {
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.20) { [weak self] in
            guard let self,
                  self.runR05AttributionProbe,
                  !self.isClosing,
                  self.attributionCompletedSamples < self.attributionTargetSamples else { return }
            self.eventButton.sendActions(for: .touchUpInside)
        }
    }

    private func submitR05AttributionEvent(_ sequence: Int) {
        let actionNs = DispatchTime.now().uptimeNanoseconds
        let handle = session
        let accepted = enqueueRuntimeCall(label: "R05 iOS 진단 이벤트 \(sequence)") { [weak self] in
            guard let self else { return }
            let workerStartNs = DispatchTime.now().uptimeNanoseconds
            let ffiStartNs = DispatchTime.now().uptimeNanoseconds
            let report = SpinonRunner.dispatchRuntimeSession(handle, nodeID: Int32(sequence)) ?? "응답 없음"
            let ffiEndNs = DispatchTime.now().uptimeNanoseconds
            let mainPostNs = DispatchTime.now().uptimeNanoseconds
            DispatchQueue.main.async {
                let mainCallbackNs = DispatchTime.now().uptimeNanoseconds
                guard !self.isClosing else { return }
                let sample = AttributionSample(
                    sequence: sequence,
                    actionNs: actionNs,
                    workerStartNs: workerStartNs,
                    ffiStartNs: ffiStartNs,
                    ffiEndNs: ffiEndNs,
                    mainPostNs: mainPostNs,
                    mainCallbackNs: mainCallbackNs,
                    report: report
                )
                guard self.attributionPendingSample == nil else {
                    self.attributionLogger.error(
                        "SPINON_R05_IOS_OVERLAP seq=\(sequence) previous_sample_not_closed"
                    )
                    return
                }
                self.attributionPendingSample = sample
            }
        }
        if !accepted {
            attributionLogger.error("SPINON_R05_IOS_REJECTED seq=\(sequence) reason=runtime_queue")
        }
    }

    @objc private func captureR05DisplayLinkTick() {
        guard let sample = attributionPendingSample else { return }
        attributionPendingSample = nil
        let displayLinkNs = DispatchTime.now().uptimeNanoseconds
        let queueUs = Double(sample.workerStartNs - sample.actionNs) / 1_000
        let ffiUs = Double(sample.ffiEndNs - sample.ffiStartNs) / 1_000
        let mainQueueUs = Double(sample.mainCallbackNs - sample.mainPostNs) / 1_000
        let callbackToTickUs = Double(displayLinkNs - sample.mainCallbackNs) / 1_000
        let totalUs = Double(displayLinkNs - sample.actionNs) / 1_000
        let v8Us = integerField("v8_call_us", in: sample.report)
        let actorUs = integerField("actor_before_reply_us", in: sample.report)
        attributionLogger.notice(
            "SPINON_R05_IOS_SAMPLE seq=\(sample.sequence) queue_us=\(queueUs, format: .fixed(precision: 3)) ffi_us=\(ffiUs, format: .fixed(precision: 3)) v8_us=\(v8Us) actor_us=\(actorUs) main_queue_us=\(mainQueueUs, format: .fixed(precision: 3)) callback_to_display_tick_us=\(callbackToTickUs, format: .fixed(precision: 3)) total_to_display_tick_us=\(totalUs, format: .fixed(precision: 3)) status=\(self.field("status", in: sample.report) ?? "missing")"
        )
        attributionCompletedSamples += 1
        if attributionCompletedSamples == attributionTargetSamples {
            attributionLogger.notice("SPINON_R05_IOS_DONE samples=\(self.attributionCompletedSamples)")
            attributionDisplayLink?.invalidate()
            attributionDisplayLink = nil
            setStatus("iOS 진단 완료 · 결과는 unified log에서 확인")
            return
        }
        scheduleR05AttributionEvent()
    }

    @objc private func simulateDelayedHostResponse() {
        guard session != 0, !isClosing, !delayedHostResponsePending else { return }
        delayedHostResponsePending = true
        delayedButton.isEnabled = false
        appendReport("호스트가 0.5초 뒤 JavaScript 콜백을 큐에 넣도록 예약했습니다.")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            guard let self else { return }
            guard !self.isClosing, self.session != 0 else {
                self.delayedHostResponsePending = false
                return
            }
            let handle = self.session
            let accepted = self.enqueueRuntimeCall(label: "지연 호스트 응답") { [weak self] in
                let report = SpinonRunner.evalRuntimeSession(
                    handle,
                    source: "spinon.setText('delayed-host-response')"
                ) ?? "응답 없음"
                DispatchQueue.main.async {
                    guard let self else { return }
                    self.appendReport("지연 호스트 응답 · \(report)")
                    self.delayedHostResponsePending = false
                    self.updateRuntimeButtons()
                }
            }
            if !accepted {
                self.delayedHostResponsePending = false
                self.setStatus("실행 대기열이 가득 차 호스트 응답을 제출하지 못했습니다")
                self.updateRuntimeButtons()
            }
        }
    }

    @objc private func startLongEvaluation() {
        guard session != 0, !scenarioRunning, !isClosing else { return }
        scenarioRunning = true
        scenarioHeartbeatStart = heartbeatCount
        scenarioEvalReport = nil
        scenarioDispatchReport = nil
        scenarioHadDispatch = false
        scenarioCancelStatus = nil
        cancelRequestPending = false
        updateRuntimeButtons()
        recreateButton.isEnabled = false
        setStatus("긴 JavaScript 실행 중 · 화면 입력 가능 · JS 이벤트 대기 중")
        appendReport("무한 JavaScript 평가 제출 · UI 메인 스레드는 대기하지 않음")

        let handle = session
        let accepted = enqueueRuntimeCall(label: "긴 JavaScript 평가") { [weak self] in
            let report = SpinonRunner.evalRuntimeSession(
                handle,
                source: "while (true) { /* iOS 시뮬레이터 취소 검증 */ }"
            ) ?? "응답 없음"
            DispatchQueue.main.async {
                guard let self else { return }
                self.scenarioEvalReport = report
                self.appendReport("긴 평가 반환 · \(report)")
                self.finishScenarioIfReady()
            }
        }
        if !accepted {
            scenarioRunning = false
            cancelRequestPending = false
            updateRuntimeButtons()
            recreateButton.isEnabled = true
            setStatus("실행 대기열이 가득 차 긴 JavaScript를 시작하지 못했습니다")
        }

        if automaticallyRun {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.30) { [weak self] in
                guard let self, self.scenarioRunning else { return }
                self.eventButton.sendActions(for: .touchUpInside)
            }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.75) { [weak self] in
                guard let self, self.scenarioRunning else { return }
                let handle = self.session
                self.cancelRequestPending = true
                self.updateRuntimeButtons()
                self.setStatus("JavaScript 취소 요청 중…")
                self.runtimeControl.async {
                    let status = SpinonRunner.cancelRuntimeSession(handle)
                    DispatchQueue.main.async {
                        self.scenarioCancelStatus = status
                        if status != 0 {
                            self.cancelRequestPending = false
                        }
                        self.updateRuntimeButtons()
                        self.appendReport("취소 요청 · status=\(status)")
                        self.setStatus(status == 0
                            ? "취소 요청 접수 · JavaScript 종료 대기 중"
                            : "실행 중인 JavaScript가 없습니다 · 시작 중이면 다시 취소하세요")
                        self.finishScenarioIfReady()
                    }
                }
            }
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + 12) { [weak self] in
            guard let self, self.scenarioRunning else { return }
            self.appendReport("시간 초과 · 안전을 위해 V8 취소를 요청합니다")
            let handle = self.session
            self.cancelRequestPending = true
            self.updateRuntimeButtons()
            self.setStatus("JavaScript 취소 요청 중…")
            self.runtimeControl.async {
                let status = SpinonRunner.cancelRuntimeSession(handle)
                DispatchQueue.main.async {
                    guard self.scenarioRunning else { return }
                    if self.scenarioCancelStatus == nil || status == 0 {
                        self.scenarioCancelStatus = status
                    }
                    if status != 0 {
                        self.cancelRequestPending = false
                    }
                    self.updateRuntimeButtons()
                    self.appendReport("시간 초과 안전 취소 · status=\(status)")
                    self.setStatus(status == 0
                        ? "취소 요청 접수 · JavaScript 종료 대기 중"
                        : "R06 검증 시간 초과 · V8 응답을 기다립니다")
                    self.finishScenarioIfReady()
                }
            }
        }
    }

    @objc private func cancelExecution() {
        guard session != 0, scenarioRunning, !cancelRequestPending, !isClosing else { return }
        let handle = session
        cancelRequestPending = true
        updateRuntimeButtons()
        setStatus("JavaScript 취소 요청 중…")
        runtimeControl.async { [weak self] in
            let status = SpinonRunner.cancelRuntimeSession(handle)
            DispatchQueue.main.async {
                guard let self else { return }
                self.appendReport("취소 요청 · status=\(status)")
                if self.scenarioRunning {
                    if self.scenarioCancelStatus == nil || status == 0 {
                        self.scenarioCancelStatus = status
                    }
                    if status == 0 {
                        self.setStatus("취소 요청 접수 · JavaScript 종료 대기 중")
                    } else if status == 1 {
                        self.cancelRequestPending = false
                        self.setStatus("실행 중인 JavaScript가 없습니다 · 시작 중이면 다시 취소하세요")
                    } else {
                        self.cancelRequestPending = false
                        self.setStatus("취소 요청 실패 · status=\(status)")
                    }
                    self.updateRuntimeButtons()
                    self.finishScenarioIfReady()
                }
            }
        }
    }

    private func finishScenarioIfReady() {
        guard scenarioRunning,
              let evalReport = scenarioEvalReport,
              let cancelStatus = scenarioCancelStatus,
              !scenarioHadDispatch || scenarioDispatchReport != nil else { return }

        let heartbeatDelta = heartbeatCount - scenarioHeartbeatStart
        let evalOwner = field("owner_tid", in: evalReport)
        var checks = [
            ("취소된 평가", evalReport.contains("status=-8")),
            ("실행 중 취소 요청", cancelStatus == 0)
        ]
        var dispatchSucceeded = false
        if let dispatchReport = scenarioDispatchReport {
            let dispatchOwner = field("owner_tid", in: dispatchReport)
            let callbackOwner = field("callback_tid", in: dispatchReport)
            dispatchSucceeded = dispatchReport.contains("status=0")
            checks.append(("메인 UI heartbeat", heartbeatDelta >= 5))
            checks.append(("대기 이벤트 처리", dispatchSucceeded))
            checks.append(("Isolate 소유 스레드", evalOwner != nil && evalOwner == dispatchOwner))
            checks.append(("JS 콜백 소유 스레드", dispatchOwner != nil && dispatchOwner == callbackOwner))
        } else {
            appendReport("생략 · UI heartbeat·대기 이벤트 검증 · 이벤트 탭 없음")
        }
        let passed = checks.allSatisfy(\.1)
        for (name, result) in checks {
            appendReport("\(result ? "통과" : "실패") · \(name)")
        }
        appendReport("heartbeat 증가량=\(heartbeatDelta) · owner_tid=\(evalOwner ?? "없음")")
        scenarioRunning = false
        cancelRequestPending = false
        updateRuntimeButtons()
        recreateButton.isEnabled = true
        setStatus(passed
            ? (dispatchSucceeded
                ? "취소 완료 · 대기 중이던 JS 이벤트 처리 완료"
                : "취소 완료 · JavaScript가 종료되었습니다")
            : "취소 검증 실패 · 상세 결과는 아래 기록을 확인하세요")

        if automaticallyRun && passed {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.4) { [weak self] in
                self?.recreateSession()
            }
        }
    }

    @objc private func recreateSession() {
        guard session != 0, !scenarioRunning, !isClosing else { return }
        let oldHandle = session
        session = 0
        setButtons(enabled: false)
        setStatus("기존 세션 종료 후 새 세션 생성 중…")
        let pendingCalls = runtimeCallGroup
        let lifecycleQueue = runtimeControl
        let source = bootstrapSource
        pendingCalls.notify(queue: lifecycleQueue) { [weak self] in
            SpinonRunner.freeRuntimeSession(oldHandle)
            let newHandle = SpinonRunner.createRuntimeSession()
            guard newHandle != 0 else {
                DispatchQueue.main.async {
                    self?.appendReport("재생성 실패")
                    self?.setStatus("새 V8 세션 생성 실패")
                }
                return
            }
            let bootstrap = SpinonRunner.evalRuntimeSession(newHandle, source: source) ?? "응답 없음"
            let dispatch = SpinonRunner.dispatchRuntimeSession(newHandle, nodeID: 99) ?? "응답 없음"
            DispatchQueue.main.async {
                guard let self, !self.isClosing else {
                    lifecycleQueue.async {
                        SpinonRunner.freeRuntimeSession(newHandle)
                    }
                    return
                }
                self.session = newHandle
                let passed = bootstrap.contains("status=0") && dispatch.contains("status=0")
                self.appendReport("기존 세션 해제 완료")
                self.appendReport("새 세션 초기화 · \(bootstrap)")
                self.appendReport("새 세션 이벤트 · \(dispatch)")
                self.appendReport("\(passed ? "통과" : "실패") · 세션 종료·재생성 후 호출")
                self.setStatus(passed ? "준비됨 · 재생성 검증 완료" : "세션 재생성 검증 실패")
                self.setButtons(enabled: true)
                if self.automaticallyRun {
                    self.appendReport("자동 검증 전체 완료")
                }
            }
        }
    }

    private func setButtons(enabled: Bool) {
        updateRuntimeButtons(ready: enabled)
        recreateButton.isEnabled = enabled && !scenarioRunning
        lifecycleButton.isEnabled = enabled && !scenarioRunning
    }

    private func updateRuntimeButtons(ready: Bool? = nil) {
        let isReady = ready ?? (session != 0 && !isClosing)
        eventButton.isEnabled = isReady
        longEvalButton.isEnabled = isReady && !scenarioRunning
        cancelButton.isEnabled = isReady && scenarioRunning && !cancelRequestPending
        delayedButton.isEnabled = isReady && !delayedHostResponsePending
    }

    @discardableResult
    private func enqueueRuntimeCall(label: String, _ operation: @escaping () -> Void) -> Bool {
        guard !isClosing else { return false }
        guard runtimeCallSlots.wait(timeout: .now()) == .success else {
            appendReport("거부 · 플랫폼 실행 대기열 포화 · \(label)")
            return false
        }
        let slots = runtimeCallSlots
        runtimeCallGroup.enter()
        runtimeCalls.async {
            defer { slots.signal() }
            defer { self.runtimeCallGroup.leave() }
            operation()
        }
        return true
    }

    private func enqueueSessionFree(_ handle: UInt64) {
        guard handle != 0 else { return }
        runtimeCalls.async {
            SpinonRunner.freeRuntimeSession(handle)
        }
    }

    private func closeRuntimeSession() {
        guard !isClosing else { return }
        isClosing = true
        scenarioRunning = false
        heartbeatTimer?.invalidate()
        heartbeatTimer = nil
        let handle = session
        session = 0
        setButtons(enabled: false)
        guard handle != 0 else { return }
        let pendingCalls = runtimeCallGroup
        let lifecycleQueue = runtimeControl
        lifecycleQueue.async {
            _ = SpinonRunner.cancelRuntimeSession(handle)
            pendingCalls.notify(queue: lifecycleQueue) {
                SpinonRunner.freeRuntimeSession(handle)
            }
        }
    }

    private func setStatus(_ text: String) {
        statusLabel.text = text
        statusLabel.accessibilityValue = text
    }

    private func appendReport(_ text: String) {
        logger.notice("SPINON_R06_IOS \(text, privacy: .public)")
        pendingReportEntries.append("\(text)\n\n")
        guard !reportFlushScheduled else { return }
        reportFlushScheduled = true
        DispatchQueue.main.async { [weak self] in
            self?.flushReport()
        }
    }

    private func flushReport() {
        let entries = pendingReportEntries
        pendingReportEntries = ""
        reportFlushScheduled = false
        guard !entries.isEmpty, !isClosing else { return }
        reportBuffer.append(entries)
        if reportBuffer.count > maximumReportCharacters {
            let trimCount = reportBuffer.count - reportTrimTargetCharacters
            let trimEnd = reportBuffer.index(reportBuffer.startIndex, offsetBy: trimCount)
            reportBuffer.removeSubrange(..<trimEnd)
            if let nextLine = reportBuffer.firstIndex(of: "\n") {
                reportBuffer.removeSubrange(..<reportBuffer.index(after: nextLine))
            }
            reportView.text = reportTruncationMarker + reportBuffer
        } else {
            reportView.text.append(entries)
        }
        let end = NSRange(location: max(reportView.text.utf16.count - 1, 0), length: 1)
        reportView.scrollRangeToVisible(end)
    }

    private func field(_ key: String, in report: String) -> String? {
        report.split(separator: " ").first(where: { $0.hasPrefix("\(key)=") })
            .map { String($0.dropFirst(key.count + 1)) }
    }
}
