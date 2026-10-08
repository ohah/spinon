import XCTest

final class R05LayerSmokeUITests: XCTestCase {
    func testTapCorrelatesInputRevisionWithDrawableAcquire() {
        let app = XCUIApplication()
        app.launchArguments = ["--r05-probe"]
        app.launch()

        let canvas = app.buttons["R08 GPU 도형"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 20))
        XCTAssertTrue(canvas.isHittable)
        canvas.tap()

        let firstActivation = NSPredicate(format: "value == %@", "활성화 1회")
        expectation(for: firstActivation, evaluatedWith: canvas)
        waitForExpectations(timeout: 5)

        let screenshot = XCTAttachment(screenshot: app.screenshot())
        screenshot.name = "R05 iOS WGPU 탭 후 화면"
        screenshot.lifetime = .keepAlways
        add(screenshot)
    }
}
