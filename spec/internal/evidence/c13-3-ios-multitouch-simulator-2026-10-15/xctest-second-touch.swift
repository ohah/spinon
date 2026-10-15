import XCTest

final class R05LayerSmokeUITests: XCTestCase {
    func testSecondTouchWhileSpinonAppIsRunning() {
        let app = XCUIApplication(bundleIdentifier: "dev.spinon.bootstrap")
        guard app.state != .notRunning else {
            XCTFail("Spinon fixture app must already be running; refusing to launch another app instance")
            return
        }
        app.activate()
        XCTAssertTrue(app.wait(for: .runningForeground, timeout: 10))
        NSLog("SPINON_XCUITEST_READY_FOR_SECOND_TOUCH")
        Thread.sleep(forTimeInterval: 8)
        let secondTouch = app.coordinate(withNormalizedOffset: CGVector(dx: 0.37, dy: 0.49))
        secondTouch.tap()
        NSLog("SPINON_XCUITEST_SECOND_TOUCH_SENT")
    }
}
