import SwiftUI
import UIKit

final class C133WheelOracleViewController: UIViewController, UIScrollViewDelegate {
    private let scrollView = UIScrollView()
    private let offsetLabel = UILabel()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground
        offsetLabel.translatesAutoresizingMaskIntoConstraints = false
        offsetLabel.accessibilityIdentifier = "scroll-offset"
        offsetLabel.font = .monospacedSystemFont(ofSize: 16, weight: .medium)
        view.addSubview(offsetLabel)

        scrollView.translatesAutoresizingMaskIntoConstraints = false
        scrollView.accessibilityIdentifier = "scroll-oracle"
        scrollView.delegate = self
        scrollView.bounces = false
        scrollView.isDirectionalLockEnabled = false
        scrollView.contentSize = CGSize(width: 1600, height: 1200)
        let content = UIView(frame: CGRect(x: 0, y: 0, width: 1600, height: 1200))
        content.backgroundColor = .systemTeal
        scrollView.addSubview(content)
        view.addSubview(scrollView)

        NSLayoutConstraint.activate([
            offsetLabel.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 8),
            offsetLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 16),
            offsetLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
            offsetLabel.heightAnchor.constraint(equalToConstant: 30),
            scrollView.topAnchor.constraint(equalTo: offsetLabel.bottomAnchor, constant: 8),
            scrollView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])
        view.layoutIfNeeded()
        scrollView.contentOffset = CGPoint(x: 300, y: 100)
        updateOffsetLabel()
    }

    func scrollViewDidScroll(_ scrollView: UIScrollView) {
        updateOffsetLabel()
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
