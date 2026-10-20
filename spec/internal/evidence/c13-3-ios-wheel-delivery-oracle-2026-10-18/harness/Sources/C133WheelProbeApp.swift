import SwiftUI
import UIKit

@MainActor
final class C133ApplicationEventAudit {
    static let didChangeNotification = Notification.Name("C133ApplicationEventAuditDidChange")
    private(set) static var enabled = false
    private(set) static var applicationClassName = "unverified"
    private(set) static var counts = zeroedCounts()

    private static func zeroedCounts() -> [String: Int] {
        [
            "scroll": 0,
            "touches": 0,
            "hover": 0,
            "transform": 0,
            "presses": 0,
            "motion": 0,
            "remoteControl": 0,
            "other": 0,
        ]
    }

    static func configure(application: UIApplication, arguments: [String]) {
        enabled = arguments.contains("--app-event-boundary-oracle")
            || arguments.contains("--recognizer-event-pairing-oracle")
        applicationClassName = NSStringFromClass(type(of: application))
        counts = zeroedCounts()
        NSLog(
            "SPINON_C133_APP_READY application=%@ audit_enabled=%@",
            applicationClassName,
            enabled ? "true" : "false"
        )
        NotificationCenter.default.post(name: didChangeNotification, object: nil)
    }

    static func record(_ event: UIEvent, emitLog: Bool = true) {
        let name: String
        switch event.type {
        case .scroll:
            name = "scroll"
        case .touches:
            name = "touches"
        case .hover:
            name = "hover"
        case .transform:
            name = "transform"
        case .presses:
            name = "presses"
        case .motion:
            name = "motion"
        case .remoteControl:
            name = "remoteControl"
        @unknown default:
            name = "other"
        }

        counts[name, default: 0] += 1
        if emitLog {
            NSLog(
                "SPINON_C133_APP_EVENT type=%@ raw_type=%ld subtype=%ld count=%ld",
                name,
                event.type.rawValue,
                event.subtype.rawValue,
                counts[name, default: 0]
            )
        }
    }

    static func resetCountsForCapture() {
        counts = zeroedCounts()
    }

    static func publishRecordedEvent() {
        NotificationCenter.default.post(name: didChangeNotification, object: nil)
    }

    static var summary: String {
        let eventCounts = counts.keys.sorted().map { "\($0)=\(counts[$0, default: 0])" }
        return (["application=\(applicationClassName)"] + eventCounts).joined(separator: " ")
    }

    static var screenSummary: String {
        "application=\(applicationClassName) scroll=\(counts["scroll", default: 0]) touches=\(counts["touches", default: 0])\n"
            + "hover=\(counts["hover", default: 0]) transform=\(counts["transform", default: 0]) presses=\(counts["presses", default: 0]) motion=\(counts["motion", default: 0]) remote=\(counts["remoteControl", default: 0]) other=\(counts["other", default: 0])"
    }
}

@MainActor
private struct C133RecognizerEventProbe {
    let name: String
    let stateRawValue: () -> Int
    let callbackCount: () -> Int

    func sample() -> C133RecognizerEventLedger.Sample {
        C133RecognizerEventLedger.Sample(
            state: stateRawValue(),
            callbacks: callbackCount()
        )
    }
}

@MainActor
private final class C133RecognizerEventLedger {
    static let changedNotification = Notification.Name("C133RecognizerEventLedgerDidChange")
    static let capacity = 4_096

    struct Sample: Codable {
        let state: Int
        let callbacks: Int
    }

    struct EventRow: Codable {
        let sequence: Int
        let eventType: Int
        let receiveMonotonicNs: UInt64
        let mainThread: Bool
        let pre: [String: Sample]
        var superDispatchElapsedNs: UInt64?
        var post: [String: Sample]?
    }

    private struct Export: Codable {
        let schema: String
        let mode: String
        let runID: String
        let launchID: String
        let applicationClass: String
        let expectedRecognizerNames: [String]
        let ready: Bool
        let overflow: Bool
        let mainThreadViolation: Bool
        let incompleteRows: Int
        let eventRows: [EventRow]
    }

    private(set) static var enabled = false
    private(set) static var ready = false
    private(set) static var mode = ""
    private(set) static var runID = ""
    private(set) static var launchID = ""
    private(set) static var overflow = false
    private(set) static var mainThreadViolation = false
    private static var sequence = 0
    private static var rows: [EventRow] = []
    private static var probes: [C133RecognizerEventProbe] = []

    static func configure(arguments: [String]) {
        enabled = arguments.contains("--recognizer-event-pairing-oracle")
        mode = argumentValue("--event-ledger-mode", in: arguments) ?? ""
        runID = argumentValue("--event-ledger-run-id", in: arguments) ?? ""
        launchID = argumentValue("--event-ledger-launch-id", in: arguments) ?? ""
        ready = false
        overflow = false
        mainThreadViolation = false
        sequence = 0
        rows = []
        probes = []
    }

    static func register(_ recognizerProbes: [C133RecognizerEventProbe]) {
        guard enabled, !ready else { return }
        probes = recognizerProbes
    }

    static func beginCapture() -> Bool {
        guard enabled, !ready else { return false }
        let names = probes.map(\.name)
        let validMode = (mode == "scroll" && Set(names) == Set(["continuous", "discrete", "union"]))
            || (mode == "direct-touch" && Set(names) == Set(["direct-touch"]))
        let validIdentifiers = !runID.isEmpty && !launchID.isEmpty
            && runID.allSatisfy { $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") }
            && launchID.allSatisfy { $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") }
        guard validMode, Set(names).count == names.count, validIdentifiers, Thread.isMainThread else {
            mainThreadViolation = !Thread.isMainThread
            return false
        }
        C133ApplicationEventAudit.resetCountsForCapture()
        ready = true
        return true
    }

    static func reserve(event: UIEvent, mainThread: Bool) -> Int? {
        guard enabled, ready else { return nil }
        sequence += 1
        guard rows.count < capacity else {
            overflow = true
            return nil
        }
        if !mainThread { mainThreadViolation = true }
        let slot = rows.count
        rows.append(EventRow(
            sequence: sequence,
            eventType: event.type.rawValue,
            receiveMonotonicNs: DispatchTime.now().uptimeNanoseconds,
            mainThread: mainThread,
            pre: mainThread ? snapshot() : [:],
            superDispatchElapsedNs: nil,
            post: nil
        ))
        return slot
    }

    static func complete(slot: Int?, elapsedNs: UInt64) {
        guard enabled, let slot else { return }
        guard rows.indices.contains(slot), rows[slot].post == nil else {
            mainThreadViolation = true
            return
        }
        rows[slot].superDispatchElapsedNs = elapsedNs
        rows[slot].post = Thread.isMainThread ? snapshot() : [:]
        if !Thread.isMainThread { mainThreadViolation = true }
    }

    static func scheduleExport(status: @escaping (String) -> Void) {
        guard enabled, ready else {
            status("status=invalid reason=not_ready")
            return
        }
        DispatchQueue.main.async {
            let incomplete = rows.filter { $0.post == nil || $0.superDispatchElapsedNs == nil }.count
            let export = Export(
                schema: "spinon-c133-recognizer-event-pairing-v1",
                mode: mode,
                runID: runID,
                launchID: launchID,
                applicationClass: C133ApplicationEventAudit.applicationClassName,
                expectedRecognizerNames: probes.map(\.name).sorted(),
                ready: ready,
                overflow: overflow,
                mainThreadViolation: mainThreadViolation,
                incompleteRows: incomplete,
                eventRows: rows.sorted { $0.sequence < $1.sequence }
            )
            do {
                let documents = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
                let fileName = "c133-recognizer-ledger-\(mode)-\(runID)-\(launchID).json"
                let destination = documents.appendingPathComponent(fileName)
                let encoder = JSONEncoder()
                encoder.outputFormatting = [.sortedKeys]
                try encoder.encode(export).write(to: destination, options: .atomic)
                status("status=complete file=\(fileName) rows=\(rows.count) overflow=\(overflow) incomplete=\(incomplete)")
            } catch {
                status("status=error reason=write_failed")
            }
            NotificationCenter.default.post(name: changedNotification, object: nil)
        }
    }

    static var summary: String {
        "ready=\(ready) sequence=\(sequence) rows=\(rows.count) overflow=\(overflow) main_thread_violation=\(mainThreadViolation)"
    }

    private static func snapshot() -> [String: Sample] {
        Dictionary(uniqueKeysWithValues: probes.map { ($0.name, $0.sample()) })
    }

    private static func argumentValue(_ prefix: String, in arguments: [String]) -> String? {
        arguments.first(where: { $0.hasPrefix(prefix + "=") })?.dropFirst(prefix.count + 1).description
    }
}

@objc(C133ProbeApplication)
final class C133ProbeApplication: UIApplication {
    override func sendEvent(_ event: UIEvent) {
        let auditEnabled = C133ApplicationEventAudit.enabled
        let pairingEnabled = C133RecognizerEventLedger.enabled
        let slot = C133RecognizerEventLedger.reserve(event: event, mainThread: Thread.isMainThread)
        if auditEnabled {
            C133ApplicationEventAudit.record(event, emitLog: !pairingEnabled)
        }
        let superStart = DispatchTime.now().uptimeNanoseconds
        super.sendEvent(event)
        let superElapsed = DispatchTime.now().uptimeNanoseconds - superStart
        C133RecognizerEventLedger.complete(slot: slot, elapsedNs: superElapsed)
        if auditEnabled {
            if pairingEnabled {
                DispatchQueue.main.async {
                    C133ApplicationEventAudit.publishRecordedEvent()
                }
            } else {
                C133ApplicationEventAudit.publishRecordedEvent()
            }
        }
    }
}

@objc(C133ProbeAppDelegate)
final class C133ProbeAppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        C133ApplicationEventAudit.configure(
            application: application,
            arguments: ProcessInfo.processInfo.arguments
        )
        C133RecognizerEventLedger.configure(arguments: ProcessInfo.processInfo.arguments)
        let probeWindow = UIWindow(frame: UIScreen.main.bounds)
        probeWindow.rootViewController = UIHostingController(rootView: C133WheelOracleView())
        window = probeWindow
        probeWindow.makeKeyAndVisible()
        return true
    }
}

final class C133WheelOracleViewController: UIViewController, UIScrollViewDelegate, UIGestureRecognizerDelegate {
    private let scrollView = UIScrollView()
    private let offsetLabel = UILabel()
    private let wheelCallbackLabel = UILabel()
    private let appEventCounterLabel = UILabel()
    private var wheelCallbackCount = 0
    private var previousWheelTranslation = CGPoint.zero
    private var plainPanOracleRequested = false
    private var plainDirectTouchOracleRequested = false
    private var scrollTypeMaskOracleRequested = false
    private var appEventBoundaryOracleRequested = false
    private var appEventAuditObserverRegistered = false
    private var plainPanOracleOffsetX: CGFloat = 0
    private var scrollTypeMaskNameByRecognizer: [ObjectIdentifier: String] = [:]
    private var scrollTypeMaskCallbackCounts: [String: Int] = [:]
    private var scrollTypeMaskPreviousTranslations: [ObjectIdentifier: CGPoint] = [:]
    private var recognizerEventPairingRequested = false
    private var directTouchPairingCallbackCount = 0
    private var directTouchPairingPreviousTranslation = CGPoint.zero
    private var directTouchPairingPayload = "direct-touch=0"
    private var pairingRefreshScheduled = false
    private var eventLedgerControls: UIStackView?
    private let eventLedgerReadyLabel = UILabel()
    private let eventLedgerStatusLabel = UILabel()
    private let eventLedgerExportButton = UIButton(type: .system)

    override func viewDidLoad() {
        super.viewDidLoad()
        plainPanOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-pan-oracle")
        plainDirectTouchOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-direct-touch-oracle")
        scrollTypeMaskOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-scroll-type-mask-oracle")
        recognizerEventPairingRequested = ProcessInfo.processInfo.arguments.contains("--recognizer-event-pairing-oracle")
        appEventBoundaryOracleRequested = ProcessInfo.processInfo.arguments.contains("--app-event-boundary-oracle")
            || recognizerEventPairingRequested
        if recognizerEventPairingRequested {
            plainDirectTouchOracleRequested = plainDirectTouchOracleRequested
                || C133RecognizerEventLedger.mode == "direct-touch"
            scrollTypeMaskOracleRequested = scrollTypeMaskOracleRequested
                || C133RecognizerEventLedger.mode == "scroll"
        }
        plainPanOracleRequested = plainPanOracleRequested
            || plainDirectTouchOracleRequested
            || scrollTypeMaskOracleRequested
        view.backgroundColor = .systemBackground
        offsetLabel.translatesAutoresizingMaskIntoConstraints = false
        offsetLabel.accessibilityIdentifier = "scroll-offset"
        offsetLabel.font = .monospacedSystemFont(ofSize: 16, weight: .medium)
        view.addSubview(offsetLabel)

        wheelCallbackLabel.translatesAutoresizingMaskIntoConstraints = false
        wheelCallbackLabel.accessibilityIdentifier = "wheel-callbacks"
        wheelCallbackLabel.font = .monospacedSystemFont(ofSize: 12, weight: .regular)
        wheelCallbackLabel.numberOfLines = 1
        wheelCallbackLabel.text = "callbacks=0"
        wheelCallbackLabel.accessibilityValue = "callbacks=0"
        view.addSubview(wheelCallbackLabel)

        if appEventBoundaryOracleRequested {
            appEventCounterLabel.translatesAutoresizingMaskIntoConstraints = false
            appEventCounterLabel.accessibilityIdentifier = "app-event-counters"
            appEventCounterLabel.font = .monospacedSystemFont(ofSize: 10, weight: .regular)
            appEventCounterLabel.numberOfLines = 2
            appEventCounterLabel.lineBreakMode = .byWordWrapping
            view.addSubview(appEventCounterLabel)
            NotificationCenter.default.addObserver(
                self,
                selector: #selector(applicationEventAuditDidChange(_:)),
                name: C133ApplicationEventAudit.didChangeNotification,
                object: nil
            )
            appEventAuditObserverRegistered = true
            refreshApplicationEventCounters()
        }

        if recognizerEventPairingRequested {
            eventLedgerReadyLabel.translatesAutoresizingMaskIntoConstraints = false
            eventLedgerReadyLabel.accessibilityIdentifier = "event-ledger-ready"
            eventLedgerReadyLabel.accessibilityValue = "ready=false"
            eventLedgerReadyLabel.font = .monospacedSystemFont(ofSize: 10, weight: .medium)
            eventLedgerReadyLabel.text = "recognizer ledger 준비 중"
            eventLedgerReadyLabel.isAccessibilityElement = true

            eventLedgerStatusLabel.translatesAutoresizingMaskIntoConstraints = false
            eventLedgerStatusLabel.accessibilityIdentifier = "event-ledger-status"
            eventLedgerStatusLabel.accessibilityValue = "status=pending"
            eventLedgerStatusLabel.font = .monospacedSystemFont(ofSize: 9, weight: .regular)
            eventLedgerStatusLabel.text = "status=pending"
            eventLedgerStatusLabel.isAccessibilityElement = true

            eventLedgerExportButton.translatesAutoresizingMaskIntoConstraints = false
            eventLedgerExportButton.accessibilityIdentifier = "event-ledger-export"
            eventLedgerExportButton.setTitle("이벤트 기록 내보내기", for: .normal)
            eventLedgerExportButton.addTarget(
                self,
                action: #selector(exportRecognizerEventLedger),
                for: .touchUpInside
            )

            let ledgerControls = UIStackView(arrangedSubviews: [
                eventLedgerReadyLabel,
                eventLedgerExportButton,
                eventLedgerStatusLabel,
            ])
            ledgerControls.translatesAutoresizingMaskIntoConstraints = false
            ledgerControls.accessibilityIdentifier = "event-ledger-controls"
            ledgerControls.axis = .vertical
            ledgerControls.alignment = .fill
            ledgerControls.distribution = .fill
            ledgerControls.spacing = 1
            view.addSubview(ledgerControls)
            eventLedgerControls = ledgerControls
        }

        let plainTarget = UIView()
        if plainPanOracleRequested {
            plainTarget.translatesAutoresizingMaskIntoConstraints = false
            plainTarget.accessibilityIdentifier = "plain-oracle-target"
            plainTarget.accessibilityLabel = "plain-pan-oracle"
            plainTarget.isAccessibilityElement = true
            plainTarget.accessibilityTraits = .button
            plainTarget.backgroundColor = .systemTeal
            view.addSubview(plainTarget)
        } else {
            scrollView.translatesAutoresizingMaskIntoConstraints = false
            scrollView.accessibilityIdentifier = "scroll-oracle"
            scrollView.delegate = self
            scrollView.bounces = false
            scrollView.isDirectionalLockEnabled = false
            scrollView.contentSize = CGSize(width: 20000, height: 1200)
            let content = UIView(frame: CGRect(x: 0, y: 0, width: 20000, height: 1200))
            content.backgroundColor = .systemTeal
            scrollView.addSubview(content)
            view.addSubview(scrollView)
        }

        if scrollTypeMaskOracleRequested {
            var eventLedgerProbes: [C133RecognizerEventProbe] = []
            let masks: [(name: String, mask: UIScrollTypeMask)] = [
                ("continuous", .continuous),
                ("discrete", .discrete),
                ("union", [.continuous, .discrete]),
            ]
            for entry in masks {
                let recognizer = UIPanGestureRecognizer(
                    target: self,
                    action: #selector(handleContinuousScroll(_:))
                )
                recognizer.allowedScrollTypesMask = entry.mask
                recognizer.allowedTouchTypes = []
                recognizer.cancelsTouchesInView = false
                recognizer.delaysTouchesBegan = false
                recognizer.delegate = self
                scrollTypeMaskNameByRecognizer[ObjectIdentifier(recognizer)] = entry.name
                scrollTypeMaskCallbackCounts[entry.name] = 0
                plainTarget.addGestureRecognizer(recognizer)
                if recognizerEventPairingRequested {
                    let name = entry.name
                    eventLedgerProbes.append(C133RecognizerEventProbe(
                        name: name,
                        stateRawValue: { [weak recognizer] in recognizer?.state.rawValue ?? -1 },
                        callbackCount: { [weak self] in
                            self?.scrollTypeMaskCallbackCounts[name, default: 0] ?? 0
                        }
                    ))
                }
            }
            updateScrollTypeMaskLabel()
            C133RecognizerEventLedger.register(eventLedgerProbes)
        } else {
            let continuousScrollRecognizer = UIPanGestureRecognizer(
                target: self,
                action: #selector(handleContinuousScroll(_:))
            )
            if plainDirectTouchOracleRequested {
                continuousScrollRecognizer.allowedScrollTypesMask = []
                continuousScrollRecognizer.allowedTouchTypes = [
                    NSNumber(value: UITouch.TouchType.direct.rawValue),
                ]
            } else {
                continuousScrollRecognizer.allowedScrollTypesMask = .continuous
                continuousScrollRecognizer.allowedTouchTypes = []
            }
            continuousScrollRecognizer.cancelsTouchesInView = false
            continuousScrollRecognizer.delaysTouchesBegan = false
            if plainPanOracleRequested {
                plainTarget.addGestureRecognizer(continuousScrollRecognizer)
            } else {
                continuousScrollRecognizer.delegate = self
                view.addGestureRecognizer(continuousScrollRecognizer)
            }
            if recognizerEventPairingRequested, C133RecognizerEventLedger.mode == "direct-touch" {
                C133RecognizerEventLedger.register([C133RecognizerEventProbe(
                    name: "direct-touch",
                    stateRawValue: { [weak continuousScrollRecognizer] in
                        continuousScrollRecognizer?.state.rawValue ?? -1
                    },
                    callbackCount: { [weak self] in self?.directTouchPairingCallbackCount ?? 0 }
                )])
            }
        }

        var constraints = [
            offsetLabel.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 8),
            offsetLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
            offsetLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
            offsetLabel.heightAnchor.constraint(equalToConstant: 30),
            wheelCallbackLabel.topAnchor.constraint(equalTo: offsetLabel.bottomAnchor, constant: 4),
            wheelCallbackLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
            wheelCallbackLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
            wheelCallbackLabel.heightAnchor.constraint(equalToConstant: 22),
        ]
        if appEventBoundaryOracleRequested {
            constraints += [
                appEventCounterLabel.topAnchor.constraint(equalTo: wheelCallbackLabel.bottomAnchor, constant: 2),
                appEventCounterLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
                appEventCounterLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
                appEventCounterLabel.heightAnchor.constraint(equalToConstant: 32),
            ]
        }
        var targetTopAnchor = appEventBoundaryOracleRequested
            ? appEventCounterLabel.bottomAnchor
            : wheelCallbackLabel.bottomAnchor
        if let eventLedgerControls {
            constraints += [
                eventLedgerControls.topAnchor.constraint(equalTo: targetTopAnchor, constant: 4),
                eventLedgerControls.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
                eventLedgerControls.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
            ]
            targetTopAnchor = eventLedgerControls.bottomAnchor
        }
        if plainPanOracleRequested {
            constraints += [
                plainTarget.topAnchor.constraint(equalTo: targetTopAnchor, constant: 8),
                plainTarget.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
                plainTarget.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
                plainTarget.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            ]
        } else {
            constraints += [
                scrollView.topAnchor.constraint(equalTo: targetTopAnchor, constant: 8),
                scrollView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
                scrollView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
                scrollView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            ]
        }
        NSLayoutConstraint.activate(constraints)
        view.layoutIfNeeded()
        if !plainPanOracleRequested {
            scrollView.contentOffset = CGPoint(x: 10000, y: 100)
        }
        updateOffsetLabel()
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard recognizerEventPairingRequested else { return }
        if C133RecognizerEventLedger.ready { return }
        guard C133RecognizerEventLedger.beginCapture() else {
            setEventLedgerReadyStatus("ready=false reason=invalid_setup")
            NSLog("SPINON_C133_EVENT_LEDGER_READY status=invalid mode=%@", C133RecognizerEventLedger.mode)
            return
        }
        setEventLedgerReadyStatus(
            "ready=true mode=\(C133RecognizerEventLedger.mode)"
        )
        refreshApplicationEventCounters()
        NSLog("SPINON_C133_EVENT_LEDGER_READY %@", C133RecognizerEventLedger.summary)
    }

    deinit {
        if appEventAuditObserverRegistered {
            NotificationCenter.default.removeObserver(self)
        }
    }

    @objc private func refreshApplicationEventCounters() {
        guard appEventBoundaryOracleRequested else { return }
        appEventCounterLabel.text = C133ApplicationEventAudit.screenSummary
        appEventCounterLabel.accessibilityValue = C133ApplicationEventAudit.summary
    }

    @objc private func applicationEventAuditDidChange(_ notification: Notification) {
        refreshApplicationEventCounters()
    }

    private func setEventLedgerReadyStatus(_ status: String) {
        eventLedgerReadyLabel.text = status
        eventLedgerReadyLabel.accessibilityValue = status
    }

    @objc private func exportRecognizerEventLedger() {
        C133RecognizerEventLedger.scheduleExport { [weak self] status in
            guard let self else { return }
            let displayStatus: String
            if status.hasPrefix("status=complete") {
                displayStatus = status.split(separator: " ").map(String.init)
                    .filter {
                        $0.hasPrefix("status=") || $0.hasPrefix("rows=")
                            || $0.hasPrefix("overflow=") || $0.hasPrefix("incomplete=")
                    }
                    .joined(separator: " ")
            } else {
                displayStatus = status
            }
            self.eventLedgerStatusLabel.text = displayStatus
            self.eventLedgerStatusLabel.accessibilityValue = status
            self.eventLedgerExportButton.accessibilityValue = status
        }
    }

    private func schedulePairingInterfaceRefresh() {
        guard !pairingRefreshScheduled else { return }
        pairingRefreshScheduled = true
        DispatchQueue.main.async { [weak self] in
            guard let self else { return }
            self.pairingRefreshScheduled = false
            self.refreshApplicationEventCounters()
            if C133RecognizerEventLedger.mode == "scroll" {
                self.updateScrollTypeMaskLabel()
            } else if C133RecognizerEventLedger.mode == "direct-touch" {
                self.wheelCallbackLabel.text = self.directTouchPairingPayload
                self.wheelCallbackLabel.accessibilityValue = self.directTouchPairingPayload
                self.offsetLabel.text = String(format: "offset=%.1f,0.0", self.plainPanOracleOffsetX)
                self.offsetLabel.accessibilityValue = self.offsetLabel.text
            }
        }
    }

    func scrollViewDidScroll(_ scrollView: UIScrollView) {
        updateOffsetLabel()
        NSLog("SPINON_C133_UKIT_SCROLL %@", offsetLabel.text ?? "missing-offset")
    }

    func gestureRecognizer(
        _ gestureRecognizer: UIGestureRecognizer,
        shouldRecognizeSimultaneouslyWith otherGestureRecognizer: UIGestureRecognizer
    ) -> Bool {
        true
    }

    @objc private func handleContinuousScroll(_ recognizer: UIPanGestureRecognizer) {
        if recognizerEventPairingRequested, C133RecognizerEventLedger.mode == "direct-touch" {
            handleDirectTouchPairing(recognizer)
            return
        }
        if scrollTypeMaskOracleRequested {
            handleScrollTypeMaskOracle(recognizer)
            return
        }
        if recognizer.state == .began {
            previousWheelTranslation = .zero
        }
        let translation = recognizer.translation(in: view)
        let delta = CGPoint(
            x: translation.x - previousWheelTranslation.x,
            y: translation.y - previousWheelTranslation.y
        )
        previousWheelTranslation = translation
        if plainPanOracleRequested {
            plainPanOracleOffsetX += delta.x
            offsetLabel.text = String(format: "offset=%.1f,0.0", plainPanOracleOffsetX)
            offsetLabel.accessibilityValue = offsetLabel.text
        }
        wheelCallbackCount += 1
        let payload = "callbacks=\(wheelCallbackCount) state=\(recognizer.state.rawValue) translation_points=\(translation.x),\(translation.y) delta_points=\(delta.x),\(delta.y)"
        wheelCallbackLabel.text = payload
        wheelCallbackLabel.accessibilityValue = payload
        NSLog("SPINON_C133_UKIT_ORACLE %@", payload)
        if recognizer.state == .ended || recognizer.state == .cancelled || recognizer.state == .failed {
            previousWheelTranslation = .zero
        }
    }

    private func handleScrollTypeMaskOracle(_ recognizer: UIPanGestureRecognizer) {
        let identity = ObjectIdentifier(recognizer)
        let name = scrollTypeMaskNameByRecognizer[identity] ?? "unmapped"
        if recognizer.state == .began {
            scrollTypeMaskPreviousTranslations[identity] = .zero
        }
        let translation = recognizer.translation(in: view)
        let previous = scrollTypeMaskPreviousTranslations[identity] ?? .zero
        let delta = CGPoint(x: translation.x - previous.x, y: translation.y - previous.y)
        scrollTypeMaskPreviousTranslations[identity] = translation
        scrollTypeMaskCallbackCounts[name, default: 0] += 1
        if recognizerEventPairingRequested {
            schedulePairingInterfaceRefresh()
        } else {
            updateScrollTypeMaskLabel()
            NSLog(
                "SPINON_C133_SCROLL_TYPE name=%@ state=%ld translation_points=%.3f,%.3f delta_points=%.3f,%.3f count=%ld",
                name,
                recognizer.state.rawValue,
                translation.x,
                translation.y,
                delta.x,
                delta.y,
                scrollTypeMaskCallbackCounts[name, default: 0]
            )
        }
        if recognizer.state == .ended || recognizer.state == .cancelled || recognizer.state == .failed {
            scrollTypeMaskPreviousTranslations.removeValue(forKey: identity)
        }
    }

    private func handleDirectTouchPairing(_ recognizer: UIPanGestureRecognizer) {
        if recognizer.state == .began {
            directTouchPairingPreviousTranslation = .zero
        }
        let translation = recognizer.translation(in: view)
        let delta = CGPoint(
            x: translation.x - directTouchPairingPreviousTranslation.x,
            y: translation.y - directTouchPairingPreviousTranslation.y
        )
        directTouchPairingPreviousTranslation = translation
        directTouchPairingCallbackCount += 1
        plainPanOracleOffsetX += delta.x
        directTouchPairingPayload = "callbacks=\(directTouchPairingCallbackCount) state=\(recognizer.state.rawValue)"
        schedulePairingInterfaceRefresh()
        if recognizer.state == .ended || recognizer.state == .cancelled || recognizer.state == .failed {
            directTouchPairingPreviousTranslation = .zero
        }
    }

    private func updateScrollTypeMaskLabel() {
        let continuous = scrollTypeMaskCallbackCounts["continuous", default: 0]
        let discrete = scrollTypeMaskCallbackCounts["discrete", default: 0]
        let union = scrollTypeMaskCallbackCounts["union", default: 0]
        let payload = "continuous=\(continuous) discrete=\(discrete) union=\(union)"
        wheelCallbackLabel.text = payload
        wheelCallbackLabel.accessibilityValue = payload
    }

    private func updateOffsetLabel() {
        offsetLabel.text = String(format: "offset=%.1f,%.1f", scrollView.contentOffset.x, scrollView.contentOffset.y)
        offsetLabel.accessibilityValue = offsetLabel.text
    }
}

struct C133WheelOracleView: UIViewControllerRepresentable {
    func makeUIViewController(context: Context) -> C133WheelOracleViewController {
        C133WheelOracleViewController()
    }

    func updateUIViewController(_ uiViewController: C133WheelOracleViewController, context: Context) {}
}
