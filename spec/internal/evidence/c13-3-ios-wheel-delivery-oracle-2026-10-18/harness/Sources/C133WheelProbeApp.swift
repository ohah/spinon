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
        applicationClassName = NSStringFromClass(type(of: application))
        counts = zeroedCounts()
        NSLog(
            "SPINON_C133_APP_READY application=%@ audit_enabled=%@",
            applicationClassName,
            enabled ? "true" : "false"
        )
        NotificationCenter.default.post(name: didChangeNotification, object: nil)
    }

    static func record(_ event: UIEvent) {
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
        NSLog(
            "SPINON_C133_APP_EVENT type=%@ raw_type=%ld subtype=%ld count=%ld",
            name,
            event.type.rawValue,
            event.subtype.rawValue,
            counts[name, default: 0]
        )
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

@objc(C133ProbeApplication)
final class C133ProbeApplication: UIApplication {
    override func sendEvent(_ event: UIEvent) {
        let auditEnabled = C133ApplicationEventAudit.enabled
        if auditEnabled {
            C133ApplicationEventAudit.record(event)
        }
        super.sendEvent(event)
        if auditEnabled {
            C133ApplicationEventAudit.publishRecordedEvent()
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

    override func viewDidLoad() {
        super.viewDidLoad()
        plainPanOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-pan-oracle")
        plainDirectTouchOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-direct-touch-oracle")
        scrollTypeMaskOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-scroll-type-mask-oracle")
        appEventBoundaryOracleRequested = ProcessInfo.processInfo.arguments.contains("--app-event-boundary-oracle")
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
            }
            updateScrollTypeMaskLabel()
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
        let targetTopAnchor = appEventBoundaryOracleRequested
            ? appEventCounterLabel.bottomAnchor
            : wheelCallbackLabel.bottomAnchor
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
        if recognizer.state == .ended || recognizer.state == .cancelled || recognizer.state == .failed {
            scrollTypeMaskPreviousTranslations.removeValue(forKey: identity)
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
