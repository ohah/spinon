import UIKit

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        let runProbe = ProcessInfo.processInfo.arguments.contains("--r05-probe")
        let window = UIWindow(frame: UIScreen.main.bounds)
        window.rootViewController = R08GpuDemoViewController(
            useWgpu: true,
            r05PresentationProbe: runProbe)
        window.makeKeyAndVisible()
        self.window = window
        return true
    }
}
