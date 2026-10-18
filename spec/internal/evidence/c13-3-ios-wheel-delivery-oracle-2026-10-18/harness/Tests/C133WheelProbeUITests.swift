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

    func testApplicationEventBoundaryForScrollTypeMasks() {
        var observations: [String] = []
        var recognizerCategories: [String: Int] = [:]
        var eventCategories: [String: Int] = [:]
        var validLaunches = 0
        var validApplicationClasses = 0
        var zeroBaselines = 0
        var validRows = 0
        var validCounterWindows = 0
        var invalidCounterWindows = 0
        var unionInvariantViolations = 0
        var recognizersWithoutScrollEvents = 0

        for launch in 1...3 {
            let observationStart = observations.count
            let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
            app.launchArguments = ["--plain-scroll-type-mask-oracle", "--app-event-boundary-oracle"]
            app.launch()

            let target = app.descendants(matching: .any)
                .matching(identifier: "plain-oracle-target")
                .firstMatch
            let callbackLabel = app.descendants(matching: .any)
                .matching(identifier: "wheel-callbacks")
                .firstMatch
            let eventLabel = app.descendants(matching: .any)
                .matching(identifier: "app-event-counters")
                .firstMatch
            let targetExists = target.waitForExistence(timeout: 10)
            let callbackExists = callbackLabel.waitForExistence(timeout: 10)
            let eventLabelExists = eventLabel.waitForExistence(timeout: 10)
            let targetIsHittable = targetExists && target.isHittable
            guard targetExists && callbackExists && eventLabelExists && targetIsHittable else {
                observations.append(
                    "launch=\(launch) setup_failed target_exists=\(targetExists) callback_exists=\(callbackExists) event_label_exists=\(eventLabelExists) target_hittable=\(targetIsHittable)"
                )
                app.terminate()
                continue
            }
            validLaunches += 1

            let initialRecognizerCounts = scrollTypeMaskCounts(callbackLabel)
            let initialEventSnapshot = applicationEventSnapshot(eventLabel)
            let hasExpectedApplication = initialEventSnapshot.application == "C133ProbeApplication"
            let hasZeroBaseline = ["continuous", "discrete", "union"].allSatisfy {
                initialRecognizerCounts[$0] == 0
            } && initialEventSnapshot.counts.values.allSatisfy { $0 == 0 }
            validApplicationClasses += hasExpectedApplication ? 1 : 0
            zeroBaselines += hasZeroBaseline ? 1 : 0
            observations.append(
                "launch=\(launch) app_class=\(initialEventSnapshot.application) app_events=\(formatApplicationEventSnapshot(initialEventSnapshot)) recognizers=\(formatScrollTypeMaskCounts(initialRecognizerCounts)) zero=\(hasZeroBaseline)"
            )

            for repetition in 1...5 {
                for direction in ["positive", "negative"] {
                    let beforeRecognizerCounts = scrollTypeMaskCounts(callbackLabel)
                    let beforeEventSnapshot = applicationEventSnapshot(eventLabel)
                    let deltaX: CGFloat = direction == "positive" ? 38 : -38
                    target.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
                        .scroll(byDeltaX: deltaX, deltaY: 0)

                    let deadline = Date().addingTimeInterval(2.0)
                    while Date() < deadline {
                        let currentRecognizers = scrollTypeMaskCounts(callbackLabel)
                        let currentEvents = applicationEventSnapshot(eventLabel)
                        let recognizerChanged = ["continuous", "discrete", "union"].contains {
                            currentRecognizers[$0, default: -1] > beforeRecognizerCounts[$0, default: -1]
                        }
                        let eventChanged = currentEvents.counts.contains { key, value in
                            value > beforeEventSnapshot.counts[key, default: -1]
                        }
                        if recognizerChanged || eventChanged {
                            break
                        }
                        Thread.sleep(forTimeInterval: 0.05)
                    }
                    Thread.sleep(forTimeInterval: 0.15)

                    let afterRecognizerCounts = scrollTypeMaskCounts(callbackLabel)
                    let afterEventSnapshot = applicationEventSnapshot(eventLabel)
                    let validEventWindow = applicationEventWindowIsValid(
                        before: beforeEventSnapshot,
                        after: afterEventSnapshot
                    )
                    let validRecognizerWindow = recognizerCountWindowIsValid(
                        before: beforeRecognizerCounts,
                        after: afterRecognizerCounts
                    )
                    let validCounterWindow = validEventWindow && validRecognizerWindow
                    validCounterWindows += validCounterWindow ? 1 : 0
                    invalidCounterWindows += validCounterWindow ? 0 : 1

                    let continuousDelta = validRecognizerWindow
                        ? afterRecognizerCounts["continuous", default: -1]
                            - beforeRecognizerCounts["continuous", default: -1]
                        : -1
                    let discreteDelta = validRecognizerWindow
                        ? afterRecognizerCounts["discrete", default: -1]
                            - beforeRecognizerCounts["discrete", default: -1]
                        : -1
                    let unionDelta = validRecognizerWindow
                        ? afterRecognizerCounts["union", default: -1]
                            - beforeRecognizerCounts["union", default: -1]
                        : -1
                    let scrollEventDelta = validEventWindow
                        ? afterEventSnapshot.counts["scroll", default: -1]
                            - beforeEventSnapshot.counts["scroll", default: -1]
                        : -1
                    let nonScrollEventDelta = validEventWindow
                        ? afterEventSnapshot.counts.reduce(into: 0) {
                            if $1.key != "scroll" {
                                $0 += $1.value - beforeEventSnapshot.counts[$1.key, default: -1]
                            }
                        }
                        : -1
                    let continuousObserved = continuousDelta > 0
                    let discreteObserved = discreteDelta > 0
                    let unionObserved = unionDelta > 0
                    let recognizerObserved = continuousObserved || discreteObserved || unionObserved
                    let recognizerCategory: String
                    if (continuousObserved || discreteObserved) && !unionObserved {
                        recognizerCategory = "invariant-violation"
                        unionInvariantViolations += 1
                    } else if continuousObserved && discreteObserved {
                        recognizerCategory = "continuous+discrete+union"
                    } else if continuousObserved {
                        recognizerCategory = "continuous+union"
                    } else if discreteObserved {
                        recognizerCategory = "discrete+union"
                    } else if unionObserved {
                        recognizerCategory = "union-only"
                    } else if !validRecognizerWindow {
                        recognizerCategory = "invalid-counter-window"
                    } else {
                        recognizerCategory = "none"
                    }

                    let eventCategory: String
                    if !validEventWindow {
                        eventCategory = "invalid-counter-window"
                    } else if scrollEventDelta > 0 {
                        eventCategory = recognizerObserved
                            ? "app-scroll-and-recognizer"
                            : "app-scroll-no-recognizer"
                    } else if nonScrollEventDelta > 0 {
                        eventCategory = "app-nonscroll-event"
                    } else {
                        eventCategory = "no-app-event"
                    }
                    if validEventWindow && validRecognizerWindow
                        && recognizerObserved && scrollEventDelta <= 0
                    {
                        recognizersWithoutScrollEvents += 1
                    }
                    recognizerCategories[recognizerCategory, default: 0] += 1
                    eventCategories[eventCategory, default: 0] += 1
                    validRows += 1
                    observations.append(
                        "launch=\(launch) repetition=\(repetition) direction=\(direction) requested_delta_x=\(deltaX) app_before=\(formatApplicationEventSnapshot(beforeEventSnapshot)) app_after=\(formatApplicationEventSnapshot(afterEventSnapshot)) app_scroll_delta=\(scrollEventDelta) app_non_scroll_delta=\(nonScrollEventDelta) recognizers_before=\(formatScrollTypeMaskCounts(beforeRecognizerCounts)) recognizers_after=\(formatScrollTypeMaskCounts(afterRecognizerCounts)) recognizer_deltas=continuous:\(continuousDelta),discrete:\(discreteDelta),union:\(unionDelta) valid_counter_window=\(validCounterWindow) recognizer_category=\(recognizerCategory) event_category=\(eventCategory)"
                    )
                }
            }

            let launchAttachment = XCTAttachment(
                string: observations[observationStart...].joined(separator: "\n")
            )
            launchAttachment.name = "C13.3 application event boundary launch \(launch) observations"
            launchAttachment.lifetime = .keepAlways
            add(launchAttachment)
            if launch == 1 {
                XCTContext.runActivity(named: "C13.3 application event boundary final screen") { activity in
                    let screenshot = XCTAttachment(screenshot: app.screenshot())
                    screenshot.name = "C13.3 application event boundary final screen"
                    screenshot.lifetime = .keepAlways
                    activity.add(screenshot)
                }
            }
            app.terminate()
        }

        let aggregate = XCTAttachment(string: observations.joined(separator: "\n"))
        aggregate.name = "C13.3 application event boundary all action observations"
        aggregate.lifetime = .keepAlways
        add(aggregate)
        let summary = XCTAttachment(
            string: "valid_fresh_launches=\(validLaunches)/3 expected_application_class=\(validApplicationClasses)/3 zero_baselines=\(zeroBaselines)/3 action_rows=\(validRows)/30 valid_counter_windows=\(validCounterWindows)/30 invalid_counter_windows=\(invalidCounterWindows) union_invariant_violations=\(unionInvariantViolations) recognizer_without_scroll_event=\(recognizersWithoutScrollEvents) recognizer_categories=\(recognizerCategories) app_event_categories=\(eventCategories)"
        )
        summary.name = "C13.3 application event boundary aggregate"
        summary.lifetime = .keepAlways
        add(summary)

        XCTAssertEqual(validLaunches, 3, "app event 경계 fresh launch 수")
        XCTAssertEqual(validApplicationClasses, 3, "custom UIApplication principal class 확인 수")
        XCTAssertEqual(zeroBaselines, 3, "app/recognizer zero baseline 수")
        XCTAssertEqual(validRows, 30, "app event 경계 action row 수")
        XCTAssertEqual(validCounterWindows, 30, "모든 action의 event/recognizer counter가 완전하고 단조 증가해야 함")
        XCTAssertEqual(invalidCounterWindows, 0, "파싱 실패 또는 counter 역행 row가 없어야 함")
        XCTAssertEqual(unionInvariantViolations, 0, "typed recognizer callback은 union recognizer에서도 보여야 함")
        XCTAssertEqual(recognizersWithoutScrollEvents, 0, "recognizer callback은 app의 scroll event 진입 뒤에 관찰돼야 함")
    }

    func testDirectTouchRecognizerDeliveryControl() {
        var observations: [String] = []
        var callbacksObserved = 0
        var signedOffsetsObserved = 0
        var directTouchEventRows = 0
        var directTouchScrollEventRows = 0
        var validCounterRows = 0
        var invalidCounterRows = 0
        var validLaunches = 0
        var validBaselines = 0
        for launch in 1...3 {
            let app = XCUIApplication(bundleIdentifier: "dev.spinon.probe.oracle.host")
            app.launchArguments = ["--plain-direct-touch-oracle", "--app-event-boundary-oracle"]
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
            let eventLabel = app.descendants(matching: .any)
                .matching(identifier: "app-event-counters")
                .firstMatch
            let targetExists = target.waitForExistence(timeout: 10)
            let labelsExist = offsetLabel.waitForExistence(timeout: 10)
                && callbackLabel.waitForExistence(timeout: 10)
                && eventLabel.waitForExistence(timeout: 10)
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
            let initialEventSnapshot = applicationEventSnapshot(eventLabel)
            let hasZeroBaseline = initialCallbackCount == 0 && initialOffset == 0
                && initialEventSnapshot.application == "C133ProbeApplication"
                && initialEventSnapshot.counts.values.allSatisfy { $0 == 0 }
            validBaselines += hasZeroBaseline ? 1 : 0
            observations.append(
                "launch=\(launch) initial_callback_count=\(initialCallbackCount) initial_offset_x=\(initialOffset) app_events=\(formatApplicationEventSnapshot(initialEventSnapshot)) zero=\(hasZeroBaseline)"
            )
            for repetition in 1...5 {
                for direction in ["left", "right"] {
                    let originX: CGFloat = direction == "left" ? 0.8 : 0.2
                    let destinationX: CGFloat = direction == "left" ? 0.2 : 0.8
                    let callbackBefore = callbackCount(callbackLabel)
                    let offsetBefore = offsetX(offsetLabel)
                    let eventsBefore = applicationEventSnapshot(eventLabel)
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
                    Thread.sleep(forTimeInterval: 0.15)

                    callbackAfter = callbackCount(callbackLabel)
                    offsetAfter = offsetX(offsetLabel)
                    let offsetDelta = offsetAfter - offsetBefore
                    let eventsAfter = applicationEventSnapshot(eventLabel)
                    let validEventWindow = applicationEventWindowIsValid(
                        before: eventsBefore,
                        after: eventsAfter
                    )
                    let validCounterWindow = validEventWindow
                        && callbackBefore >= 0
                        && callbackAfter >= callbackBefore
                        && offsetBefore.isFinite
                        && offsetAfter.isFinite
                    let touchesDelta = validEventWindow
                        ? eventsAfter.counts["touches", default: -1]
                            - eventsBefore.counts["touches", default: -1]
                        : -1
                    let scrollEventsDelta = validEventWindow
                        ? eventsAfter.counts["scroll", default: -1]
                            - eventsBefore.counts["scroll", default: -1]
                        : -1
                    let callbackObserved = callbackAfter > callbackBefore
                    let signedOffsetObserved = offsetDelta * expectedSign > 0
                    callbacksObserved += callbackObserved ? 1 : 0
                    signedOffsetsObserved += signedOffsetObserved ? 1 : 0
                    validCounterRows += validCounterWindow ? 1 : 0
                    observations.append(
                        "launch=\(launch) repetition=\(repetition) direction=\(direction) app_before=\(formatApplicationEventSnapshot(eventsBefore)) app_after=\(formatApplicationEventSnapshot(eventsAfter)) touches_delta=\(touchesDelta) scroll_delta=\(scrollEventsDelta) callback_before=\(callbackBefore) callback_after=\(callbackAfter) offset_before=\(offsetBefore) offset_after=\(offsetAfter) offset_delta=\(offsetDelta) valid_counter_window=\(validCounterWindow) callback_observed=\(callbackObserved) signed_offset_observed=\(signedOffsetObserved)"
                    )
                    if !validCounterWindow {
                        observations[observations.count - 1] += " invalid_counter_window=true"
                    }
                    directTouchEventRows += validEventWindow && touchesDelta > 0 && scrollEventsDelta == 0 ? 1 : 0
                    directTouchScrollEventRows += validEventWindow && scrollEventsDelta > 0 ? 1 : 0
                    invalidCounterRows += validCounterWindow ? 0 : 1
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
            string: "valid_fresh_launches=\(validLaunches)/3 valid_zero_baselines=\(validBaselines)/3 callbacks_observed=\(callbacksObserved)/30 signed_offsets_observed=\(signedOffsetsObserved)/30 valid_counter_windows=\(validCounterRows)/30 invalid_counter_rows=\(invalidCounterRows) direct_touch_UIEventTypeTouches_rows=\(directTouchEventRows)/30 unexpected_scroll_event_rows=\(directTouchScrollEventRows)"
        )
        summary.name = "C13.3 direct touch aggregate"
        summary.lifetime = .keepAlways
        add(summary)

        XCTAssertEqual(validLaunches, 3, "direct touch 앱 fresh launch 수")
        XCTAssertEqual(validBaselines, 3, "direct touch zero baseline 수")
        XCTAssertEqual(callbacksObserved, 30, "직접 touch gesture의 UIKit callback 관찰 수")
        XCTAssertEqual(signedOffsetsObserved, 30, "직접 touch gesture의 부호 일치 offset 관찰 수")
        XCTAssertEqual(invalidCounterRows, 0, "direct-touch counter parse/monotonicity 위반 수")
        XCTAssertEqual(directTouchEventRows, 30, "직접 touch gesture의 UIApplication touches event 수")
        XCTAssertEqual(directTouchScrollEventRows, 0, "direct touch 입력에서 scroll event가 없어야 함")
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

    private func applicationEventSnapshot(_ element: XCUIElement) -> (application: String, counts: [String: Int]) {
        var application = "missing"
        var counts = [
            "scroll": -1,
            "touches": -1,
            "hover": -1,
            "transform": -1,
            "presses": -1,
            "motion": -1,
            "remoteControl": -1,
            "other": -1,
        ]
        for token in value(element).split(separator: " ") {
            let parts = token.split(separator: "=", maxSplits: 1)
            guard parts.count == 2 else { continue }
            let key = String(parts[0])
            let payload = String(parts[1])
            if key == "application" {
                application = payload
            } else if counts[key] != nil, let count = Int(payload) {
                counts[key] = count
            }
        }
        return (application, counts)
    }

    private func formatApplicationEventSnapshot(
        _ snapshot: (application: String, counts: [String: Int])
    ) -> String {
        let eventCounts = snapshot.counts.keys.sorted().map {
            "\($0)=\(snapshot.counts[$0, default: -1])"
        }
        return (["application=\(snapshot.application)"] + eventCounts).joined(separator: " ")
    }

    private func applicationEventWindowIsValid(
        before: (application: String, counts: [String: Int]),
        after: (application: String, counts: [String: Int])
    ) -> Bool {
        let eventTypes = [
            "scroll",
            "touches",
            "hover",
            "transform",
            "presses",
            "motion",
            "remoteControl",
            "other",
        ]
        guard before.application == "C133ProbeApplication",
              after.application == before.application
        else {
            return false
        }
        return eventTypes.allSatisfy { eventType in
            guard let beforeCount = before.counts[eventType],
                  let afterCount = after.counts[eventType]
            else {
                return false
            }
            return beforeCount >= 0 && afterCount >= beforeCount
        }
    }

    private func recognizerCountWindowIsValid(before: [String: Int], after: [String: Int]) -> Bool {
        ["continuous", "discrete", "union"].allSatisfy { name in
            guard let beforeCount = before[name], let afterCount = after[name] else {
                return false
            }
            return beforeCount >= 0 && afterCount >= beforeCount
        }
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
