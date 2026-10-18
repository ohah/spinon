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

    func testScrollTypeMaskDeliveryClassification() {
        var observations: [String] = []
        var categoryCounts: [String: Int] = [:]
        var validLaunches = 0
        var zeroBaselines = 0
        var validRows = 0
        var unionInvariantViolations = 0

        for launch in 1...3 {
            let observationStart = observations.count
            let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
            app.launchArguments = ["--plain-scroll-type-mask-oracle"]
            app.launch()

            let target = app.descendants(matching: .any)
                .matching(identifier: "plain-oracle-target")
                .firstMatch
            let callbackLabel = app.descendants(matching: .any)
                .matching(identifier: "wheel-callbacks")
                .firstMatch
            let targetExists = target.waitForExistence(timeout: 10)
            let labelExists = callbackLabel.waitForExistence(timeout: 10)
            let targetIsHittable = targetExists && target.isHittable
            guard targetExists && labelExists && targetIsHittable else {
                observations.append(
                    "launch=\(launch) setup_failed target_exists=\(targetExists) label_exists=\(labelExists) target_hittable=\(targetIsHittable)"
                )
                app.terminate()
                continue
            }
            validLaunches += 1

            let initialCounts = scrollTypeMaskCounts(callbackLabel)
            let hasZeroBaseline = ["continuous", "discrete", "union"].allSatisfy {
                initialCounts[$0] == 0
            }
            zeroBaselines += hasZeroBaseline ? 1 : 0
            observations.append(
                "launch=\(launch) baseline=\(formatScrollTypeMaskCounts(initialCounts)) zero=\(hasZeroBaseline)"
            )

            for repetition in 1...5 {
                for direction in ["positive", "negative"] {
                    let before = scrollTypeMaskCounts(callbackLabel)
                    let deltaX: CGFloat = direction == "positive" ? 38 : -38
                    target.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                        .scroll(byDeltaX: deltaX, deltaY: 0)

                    let deadline = Date().addingTimeInterval(2.0)
                    var current = scrollTypeMaskCounts(callbackLabel)
                    while Date() < deadline, current["union", default: -1] <= before["union", default: -1] {
                        current = scrollTypeMaskCounts(callbackLabel)
                        if current["union", default: -1] > before["union", default: -1] {
                            break
                        }
                        Thread.sleep(forTimeInterval: 0.05)
                    }
                    Thread.sleep(forTimeInterval: 0.15)
                    let after = scrollTypeMaskCounts(callbackLabel)
                    validRows += 1

                    let continuousDelta = after["continuous", default: -1] - before["continuous", default: -1]
                    let discreteDelta = after["discrete", default: -1] - before["discrete", default: -1]
                    let unionDelta = after["union", default: -1] - before["union", default: -1]
                    let continuousObserved = continuousDelta > 0
                    let discreteObserved = discreteDelta > 0
                    let unionObserved = unionDelta > 0
                    let category: String
                    if (continuousObserved || discreteObserved) && !unionObserved {
                        category = "invariant-violation"
                        unionInvariantViolations += 1
                    } else if continuousObserved && discreteObserved {
                        category = "both"
                    } else if continuousObserved {
                        category = "continuous-only"
                    } else if discreteObserved {
                        category = "discrete-only"
                    } else if unionObserved {
                        category = "union-only"
                    } else {
                        category = "none"
                    }
                    categoryCounts[category, default: 0] += 1
                    observations.append(
                        "launch=\(launch) repetition=\(repetition) direction=\(direction) requested_delta_x=\(deltaX) before=\(formatScrollTypeMaskCounts(before)) after=\(formatScrollTypeMaskCounts(after)) deltas=continuous:\(continuousDelta),discrete:\(discreteDelta),union:\(unionDelta) category=\(category)"
                    )
                }
            }

            let launchAttachment = XCTAttachment(
                string: observations[observationStart...].joined(separator: "\n")
            )
            launchAttachment.name = "C13.3 scroll type mask launch \(launch) observations"
            launchAttachment.lifetime = .keepAlways
            add(launchAttachment)
            if launch == 1 {
                XCTContext.runActivity(named: "C13.3 scroll type mask final screen") { activity in
                    let screenshot = XCTAttachment(screenshot: app.screenshot())
                    screenshot.name = "C13.3 scroll type mask final screen"
                    screenshot.lifetime = .keepAlways
                    activity.add(screenshot)
                }
            }
            app.terminate()
        }

        let aggregate = XCTAttachment(string: observations.joined(separator: "\n"))
        aggregate.name = "C13.3 scroll type mask all action observations"
        aggregate.lifetime = .keepAlways
        add(aggregate)
        let summary = XCTAttachment(
            string: "valid_fresh_launches=\(validLaunches)/3 zero_baselines=\(zeroBaselines)/3 action_rows=\(validRows)/30 union_invariant_violations=\(unionInvariantViolations) category_counts=\(categoryCounts)"
        )
        summary.name = "C13.3 scroll type mask classification aggregate"
        summary.lifetime = .keepAlways
        add(summary)

        XCTAssertEqual(validLaunches, 3, "scroll type 분류 fresh launch 수")
        XCTAssertEqual(zeroBaselines, 3, "scroll type 분류 zero baseline 수")
        XCTAssertEqual(validRows, 30, "scroll type 분류 action row 수")
        XCTAssertEqual(unionInvariantViolations, 0, "typed recognizer callback은 union recognizer에서도 보여야 함")
    }

    func testDirectTouchRecognizerDeliveryControl() {
        var observations: [String] = []
        var callbacksObserved = 0
        var signedOffsetsObserved = 0
        var validLaunches = 0
        var validBaselines = 0
        for launch in 1...3 {
            let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
            app.launchArguments = ["--plain-direct-touch-oracle"]
            app.launch()

            let target = app.descendants(matching: .any)
                .matching(identifier: "plain-oracle-target")
                .firstMatch
            let offsetLabel = app.descendants(matching: .any)
                .matching(identifier: "scroll-offset")
                .firstMatch
            let callbackLabel = app.descendants(matching: .any)
                .matching(identifier: "wheel-callbacks")
                .firstMatch
            let targetExists = target.waitForExistence(timeout: 10)
            let labelsExist = offsetLabel.waitForExistence(timeout: 10)
                && callbackLabel.waitForExistence(timeout: 10)
            let targetIsHittable = targetExists && target.isHittable
            guard targetExists && labelsExist && targetIsHittable else {
                observations.append(
                    "launch=\(launch) setup_failed target_exists=\(targetExists) labels_exist=\(labelsExist) target_hittable=\(targetIsHittable)"
                )
                app.terminate()
                continue
            }
            validLaunches += 1

            let initialCallbackCount = callbackCount(callbackLabel)
            let initialOffset = offsetX(offsetLabel)
            validBaselines += initialCallbackCount == 0 && initialOffset == 0 ? 1 : 0
            observations.append(
                "launch=\(launch) initial_callback_count=\(initialCallbackCount) initial_offset_x=\(initialOffset)"
            )
            for repetition in 1...5 {
                for direction in ["left", "right"] {
                    let originX: CGFloat = direction == "left" ? 0.8 : 0.2
                    let destinationX: CGFloat = direction == "left" ? 0.2 : 0.8
                    let callbackBefore = callbackCount(callbackLabel)
                    let offsetBefore = offsetX(offsetLabel)
                    let origin = target.coordinate(
                        withNormalizedOffset: CGVector(dx: originX, dy: 0.5)
                    )
                    let destination = target.coordinate(
                        withNormalizedOffset: CGVector(dx: destinationX, dy: 0.5)
                    )
                    origin.press(forDuration: 0.05, thenDragTo: destination)

                    let expectedSign: CGFloat = direction == "left" ? -1 : 1
                    let deadline = Date().addingTimeInterval(2.0)
                    var callbackAfter = callbackCount(callbackLabel)
                    var offsetAfter = offsetX(offsetLabel)
                    while Date() < deadline {
                        callbackAfter = callbackCount(callbackLabel)
                        offsetAfter = offsetX(offsetLabel)
                        let offsetDelta = offsetAfter - offsetBefore
                        if callbackAfter > callbackBefore && offsetDelta * expectedSign > 0 {
                            break
                        }
                        Thread.sleep(forTimeInterval: 0.05)
                    }

                    let offsetDelta = offsetAfter - offsetBefore
                    let callbackObserved = callbackAfter > callbackBefore
                    let signedOffsetObserved = offsetDelta * expectedSign > 0
                    callbacksObserved += callbackObserved ? 1 : 0
                    signedOffsetsObserved += signedOffsetObserved ? 1 : 0
                    observations.append(
                        "launch=\(launch) repetition=\(repetition) direction=\(direction) callback_before=\(callbackBefore) callback_after=\(callbackAfter) offset_before=\(offsetBefore) offset_after=\(offsetAfter) offset_delta=\(offsetDelta) callback_observed=\(callbackObserved) signed_offset_observed=\(signedOffsetObserved)"
                    )
                }
            }

            let attachment = XCTAttachment(string: observations.suffix(11).joined(separator: "\n"))
            attachment.name = "C13.3 direct touch launch \(launch) observations"
            attachment.lifetime = .keepAlways
            add(attachment)
            if launch == 1 {
                XCTContext.runActivity(named: "C13.3 direct touch final screen") { activity in
                    let screenshot = XCTAttachment(screenshot: app.screenshot())
                    screenshot.name = "C13.3 direct touch final screen"
                    screenshot.lifetime = .keepAlways
                    activity.add(screenshot)
                }
            }
            app.terminate()
        }

        let allObservations = XCTAttachment(string: observations.joined(separator: "\n"))
        allObservations.name = "C13.3 direct touch all launch observations"
        allObservations.lifetime = .keepAlways
        add(allObservations)
        let summary = XCTAttachment(
            string: "valid_fresh_launches=\(validLaunches)/3 valid_zero_baselines=\(validBaselines)/3 callbacks_observed=\(callbacksObserved)/30 signed_offsets_observed=\(signedOffsetsObserved)/30"
        )
        summary.name = "C13.3 direct touch aggregate"
        summary.lifetime = .keepAlways
        add(summary)

        XCTAssertEqual(validLaunches, 3, "direct touch 앱 fresh launch 수")
        XCTAssertEqual(validBaselines, 3, "direct touch zero baseline 수")
        XCTAssertEqual(callbacksObserved, 30, "직접 touch gesture의 UIKit callback 관찰 수")
        XCTAssertEqual(signedOffsetsObserved, 30, "직접 touch gesture의 부호 일치 offset 관찰 수")
    }

    private func value(_ element: XCUIElement) -> String {
        (element.value as? String) ?? element.label
    }

    private func callbackCount(_ element: XCUIElement) -> Int {
        let text = value(element)
        guard let token = text.split(separator: " ").first(where: { $0.hasPrefix("callbacks=") }) else {
            return -1
        }
        return Int(token.split(separator: "=").last ?? "") ?? -1
    }

    private func scrollTypeMaskCounts(_ element: XCUIElement) -> [String: Int] {
        var counts = ["continuous": -1, "discrete": -1, "union": -1]
        for token in value(element).split(separator: " ") {
            let parts = token.split(separator: "=", maxSplits: 1)
            guard parts.count == 2 else { continue }
            let name = String(parts[0])
            guard counts[name] != nil, let count = Int(parts[1]) else { continue }
            counts[name] = count
        }
        return counts
    }

    private func formatScrollTypeMaskCounts(_ counts: [String: Int]) -> String {
        "continuous=\(counts["continuous", default: -1]) discrete=\(counts["discrete", default: -1]) union=\(counts["union", default: -1])"
    }

    private func offsetX(_ element: XCUIElement) -> CGFloat {
        let text = value(element)
        guard let first = text.split(separator: ",").first else { return .nan }
        return CGFloat(Double(first.replacingOccurrences(of: "offset=", with: "")) ?? .nan)
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
