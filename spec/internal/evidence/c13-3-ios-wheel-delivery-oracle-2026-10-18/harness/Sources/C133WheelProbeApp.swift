import SwiftUI
import UIKit

final class C133WheelOracleViewController: UIViewController, UIScrollViewDelegate, UIGestureRecognizerDelegate {
    private let scrollView = UIScrollView()
    private let offsetLabel = UILabel()
    private let wheelCallbackLabel = UILabel()
    private var wheelCallbackCount = 0
    private var previousWheelTranslation = CGPoint.zero
    private var plainPanOracleRequested = false
    private var plainDirectTouchOracleRequested = false
    private var plainPanOracleOffsetX: CGFloat = 0

    override func viewDidLoad() {
        super.viewDidLoad()
        plainPanOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-pan-oracle")
        plainDirectTouchOracleRequested = ProcessInfo.processInfo.arguments.contains("--plain-direct-touch-oracle")
        plainPanOracleRequested = plainPanOracleRequested || plainDirectTouchOracleRequested
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
        if plainPanOracleRequested {
            constraints += [
                plainTarget.topAnchor.constraint(equalTo: wheelCallbackLabel.bottomAnchor, constant: 8),
                plainTarget.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
                plainTarget.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
                plainTarget.bottomAnchor.constraint(equalTo: view.bottomAnchor),
            ]
        } else {
            constraints += [
                scrollView.topAnchor.constraint(equalTo: wheelCallbackLabel.bottomAnchor, constant: 8),
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

@main
struct C133WheelProbeApp: App {
    var body: some Scene {
        WindowGroup {
            C133WheelOracleView()
        }
    }
}
