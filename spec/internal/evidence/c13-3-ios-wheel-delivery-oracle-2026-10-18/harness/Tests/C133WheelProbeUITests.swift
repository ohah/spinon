import XCTest

final class C133WheelProbeUITests: XCTestCase {
    func testPositiveXScroll() { exerciseScroll(name: "positive-x", dx: 38, dy: 0) }
    func testNegativeXScroll() { exerciseScroll(name: "negative-x", dx: -38, dy: 0) }
    func testPositiveYScroll() { exerciseScroll(name: "positive-y", dx: 0, dy: 51) }
    func testNegativeYScroll() { exerciseScroll(name: "negative-y", dx: 0, dy: -51) }
    func testHorizontalRoundTrip() { exerciseRoundTrip(axis: "horizontal", dx: 38, dy: 0) }
    func testVerticalRoundTrip() { exerciseRoundTrip(axis: "vertical", dx: 0, dy: 51) }
    func testControlPositiveXScroll() { exerciseControlWheel(name: "control-positive-x", dx: 38, dy: 0) }
    func testOrdinaryPositiveXScroll() { exerciseScroll(name: "ordinary-positive-x", dx: 38, dy: 0) }
    func testNegativeXLargeScroll() { exerciseScroll(name: "negative-x-large", dx: -152, dy: 0) }
    func testNegativeXAtInnerTarget() { exerciseNegativeXAtInnerTarget() }
    func testHorizontalRoundTripAtInnerTarget() { exerciseHorizontalRoundTripAtInnerTarget() }
    func testVerticalRoundTripAtInnerTarget() { exerciseVerticalRoundTripAtInnerTarget() }

    func testContinuousScrollRecognizerDeliveryOracle() {
        let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
        app.launch()
        defer { app.terminate() }

        let scrollView = app.scrollViews.matching(identifier: "scroll-oracle").firstMatch
        let offsetLabel = app.descendants(matching: .any)
            .matching(identifier: "scroll-offset")
            .firstMatch
        let callbackLabel = app.descendants(matching: .any)
            .matching(identifier: "wheel-callbacks")
            .firstMatch
        XCTAssertTrue(scrollView.waitForExistence(timeout: 10))
        XCTAssertTrue(offsetLabel.waitForExistence(timeout: 10))
        XCTAssertTrue(callbackLabel.waitForExistence(timeout: 10))

        var observations: [String] = []
        for repetition in 1...5 {
            let positiveOffsetBefore = value(offsetLabel)
            let positiveCallbacksBefore = value(callbackLabel)
            scrollView.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                .scroll(byDeltaX: 38, deltaY: 0)
            Thread.sleep(forTimeInterval: 0.25)
            let positiveOffsetAfter = value(offsetLabel)
            let positiveCallbacksAfter = value(callbackLabel)
            observations.append("repetition=\(repetition) direction=positive offset_before=\(positiveOffsetBefore) offset_after=\(positiveOffsetAfter) callbacks_before=\(positiveCallbacksBefore) callbacks_after=\(positiveCallbacksAfter)")

            let negativeOffsetBefore = value(offsetLabel)
            let negativeCallbacksBefore = value(callbackLabel)
            scrollView.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                .scroll(byDeltaX: -38, deltaY: 0)
            Thread.sleep(forTimeInterval: 0.25)
            let negativeOffsetAfter = value(offsetLabel)
            let negativeCallbacksAfter = value(callbackLabel)
            observations.append("repetition=\(repetition) direction=negative offset_before=\(negativeOffsetBefore) offset_after=\(negativeOffsetAfter) callbacks_before=\(negativeCallbacksBefore) callbacks_after=\(negativeCallbacksAfter)")
        }

        let attachment = XCTAttachment(string: observations.joined(separator: "\n"))
        attachment.name = "C13.3 UIKit delivery oracle observations"
        attachment.lifetime = .keepAlways
        add(attachment)
    }

    func testContinuousScrollRecognizerWithoutScrollView() {
        let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
        app.launchArguments = ["--plain-pan-oracle"]
        app.launch()
        defer { app.terminate() }

        let target = app.descendants(matching: .any)
            .matching(identifier: "plain-oracle-target")
            .firstMatch
        let offsetLabel = app.descendants(matching: .any)
            .matching(identifier: "scroll-offset")
            .firstMatch
        let callbackLabel = app.descendants(matching: .any)
            .matching(identifier: "wheel-callbacks")
            .firstMatch
        XCTAssertTrue(target.waitForExistence(timeout: 10))
        XCTAssertTrue(offsetLabel.waitForExistence(timeout: 10))
        XCTAssertTrue(callbackLabel.waitForExistence(timeout: 10))

        var observations: [String] = []
        for repetition in 1...5 {
            let positiveOffsetBefore = value(offsetLabel)
            let positiveCallbacksBefore = value(callbackLabel)
            target.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                .scroll(byDeltaX: 38, deltaY: 0)
            Thread.sleep(forTimeInterval: 0.25)
            let positiveOffsetAfter = value(offsetLabel)
            let positiveCallbacksAfter = value(callbackLabel)
            observations.append("repetition=\(repetition) direction=positive offset_before=\(positiveOffsetBefore) offset_after=\(positiveOffsetAfter) callbacks_before=\(positiveCallbacksBefore) callbacks_after=\(positiveCallbacksAfter)")

            let negativeOffsetBefore = value(offsetLabel)
            let negativeCallbacksBefore = value(callbackLabel)
            target.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                .scroll(byDeltaX: -38, deltaY: 0)
            Thread.sleep(forTimeInterval: 0.25)
            let negativeOffsetAfter = value(offsetLabel)
            let negativeCallbacksAfter = value(callbackLabel)
            observations.append("repetition=\(repetition) direction=negative offset_before=\(negativeOffsetBefore) offset_after=\(negativeOffsetAfter) callbacks_before=\(negativeCallbacksBefore) callbacks_after=\(negativeCallbacksAfter)")
        }

        let attachment = XCTAttachment(string: observations.joined(separator: "\n"))
        attachment.name = "C13.3 plain UIView delivery oracle observations"
        attachment.lifetime = .keepAlways
        add(attachment)
    }

    private func value(_ element: XCUIElement) -> String {
        (element.value as? String) ?? element.label
    }

    private func launchApp() -> (XCUIApplication, XCUIElement) {
        let app = XCUIApplication(bundleIdentifier: "dev.spinon.bootstrap")
        app.launchArguments = [
            "--spinon-c133-scroll-gesture",
            "--spinon-c133-native-wheel-adapter",
        ]
        app.launch()
        let canvas = app.otherElements.matching(
            NSPredicate(
                format: "label == %@",
                "C13.3 iOS UIPanGestureRecognizer nested scroll gesture WGPU Metal 장면"
            )
        ).firstMatch
        XCTAssertTrue(canvas.waitForExistence(timeout: 30))
        canvas.hover()
        return (app, canvas)
    }

    private func attach(_ app: XCUIApplication, name: String) {
        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = "C13.3 iOS native adapter \(name)"
        attachment.lifetime = .keepAlways
        add(attachment)
    }

    private func exerciseScroll(name: String, dx: CGFloat, dy: CGFloat) {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
            .scroll(byDeltaX: dx, deltaY: dy)
        Thread.sleep(forTimeInterval: 1.0)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=%@ dx=%.0f dy=%.0f", name, dx, dy)
        attach(app, name: name)
    }

    private func exerciseRoundTrip(axis: String, dx: CGFloat, dy: CGFloat) {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
            .scroll(byDeltaX: dx, deltaY: dy)
        Thread.sleep(forTimeInterval: 1.0)
        NSLog("SPINON_C133_XCTEST_ADAPTER axis=%@ direction=positive", axis)
        attach(app, name: "\(axis) positive")
        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
            .scroll(byDeltaX: -dx, deltaY: -dy)
        Thread.sleep(forTimeInterval: 1.0)
        NSLog("SPINON_C133_XCTEST_ADAPTER axis=%@ direction=negative", axis)
        attach(app, name: "\(axis) negative")
    }

    private func exerciseControlWheel(name: String, dx: CGFloat, dy: CGFloat) {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        XCUIElement.perform(withKeyModifiers: .control) {
            canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                .scroll(byDeltaX: dx, deltaY: dy)
        }
        Thread.sleep(forTimeInterval: 1.0)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=%@ dx=%.0f dy=%.0f modifiers=control", name, dx, dy)
        attach(app, name: name)
    }

    private func exerciseNegativeXAtInnerTarget() {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        let innerTarget = canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.33, dy: 0.467))
        innerTarget.hover()
        innerTarget.scroll(byDeltaX: -38, deltaY: 0)
        Thread.sleep(forTimeInterval: 1.0)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=negative-x-inner-target dx=-38 dy=0")
        attach(app, name: "negative-x inner target")
    }

    private func exerciseHorizontalRoundTripAtInnerTarget() {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        let innerTarget = canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.33, dy: 0.467))
        innerTarget.hover()
        innerTarget.scroll(byDeltaX: 38, deltaY: 0)
        Thread.sleep(forTimeInterval: 0.5)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=inner-target positive-x")
        attach(app, name: "inner target positive-x")
        innerTarget.hover()
        innerTarget.scroll(byDeltaX: -38, deltaY: 0)
        Thread.sleep(forTimeInterval: 0.5)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=inner-target negative-x")
        attach(app, name: "inner target negative-x")
    }

    private func exerciseVerticalRoundTripAtInnerTarget() {
        let (app, canvas) = launchApp()
        defer { app.terminate() }
        let innerTarget = canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.33, dy: 0.467))
        innerTarget.hover()
        innerTarget.scroll(byDeltaX: 0, deltaY: 51)
        Thread.sleep(forTimeInterval: 0.5)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=inner-target positive-y")
        attach(app, name: "inner target positive-y")
        innerTarget.hover()
        innerTarget.scroll(byDeltaX: 0, deltaY: -51)
        Thread.sleep(forTimeInterval: 0.5)
        NSLog("SPINON_C133_XCTEST_ADAPTER case=inner-target negative-y")
        attach(app, name: "inner target negative-y")
    }
}
