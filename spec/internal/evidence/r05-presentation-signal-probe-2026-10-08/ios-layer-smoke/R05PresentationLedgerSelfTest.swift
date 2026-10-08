import Dispatch
import Foundation

private final class EventCollector {
    private let lock = NSLock()
    private let semaphore = DispatchSemaphore(value: 0)
    private var stored: [R05PresentationEvent] = []

    func append(_ event: R05PresentationEvent) {
        lock.lock()
        stored.append(event)
        lock.unlock()
        semaphore.signal()
    }

    func waitForEvents(_ count: Int, timeout: TimeInterval = 2) -> [R05PresentationEvent]? {
        for _ in 0..<count {
            guard semaphore.wait(timeout: .now() + timeout) == .success else { return nil }
        }
        lock.lock()
        defer { lock.unlock() }
        return stored
    }
}

private final class LedgerReference {
    private let lock = NSLock()
    private weak var stored: R05PresentationLedger?

    func set(_ ledger: R05PresentationLedger) {
        lock.lock()
        stored = ledger
        lock.unlock()
    }

    func readPendingCount() -> Int? {
        lock.lock()
        let ledger = stored
        lock.unlock()
        return ledger?.pendingCount
    }
}

@main
private enum R05PresentationLedgerSelfTest {
    private static let token = "layer-test"

    static func main() {
        testTerminalClassifications()
        testDeadlineBoundaries()
        testIdentityCapacityAndGeneration()
        testOverflowAndTombstones()
        testConcurrentCallbackAndClose()
        testEventQueueAndTimer()
        print("R05 callback ledger 자체 시험 통과: 6개 그룹")
    }

    private static func testTerminalClassifications() {
        let ledger = makeLedger()
        check(ledger.setSurfaceGeneration(1, at: 100).isEmpty, "initial generation")

        let positiveKey = key(sequence: 1)
        check(ledger.register(key: positiveKey, ticket: ticket(sequence: 1), at: 100).outcome == .registered, "positive register")
        let positive = ledger.receiveCallback(key: positiveKey, presentedTime: 0.25, at: 199)
        check(positive.outcome == .presented && positive.isLatencyEligible, "positive finite presentation")
        check(positive.inputSequence == 1 && positive.revision == 7, "ticket metadata preserved")
        check(ledger.receiveCallback(key: positiveKey, presentedTime: 0.3, at: 200).outcome == .duplicateCallback, "duplicate after present")

        let zeroKey = key(sequence: 2)
        _ = ledger.register(key: zeroKey, ticket: ticket(sequence: 2), at: 200)
        let zero = ledger.receiveCallback(key: zeroKey, presentedTime: 0, at: 299)
        check(zero.outcome == .zeroTime && !zero.isLatencyEligible, "zero presentation time excluded")

        let invalidTimes: [TimeInterval] = [-1, .nan, .infinity, -.infinity]
        for (index, presentedTime) in invalidTimes.enumerated() {
            let sequence = UInt64(index + 3)
            let invalidKey = key(sequence: sequence)
            _ = ledger.register(key: invalidKey, ticket: ticket(sequence: sequence), at: 300)
            let invalid = ledger.receiveCallback(key: invalidKey, presentedTime: presentedTime, at: 301)
            check(invalid.outcome == .invalidPresentedTime && !invalid.isLatencyEligible, "invalid timestamp \(index)")
        }
        _ = ledger.close(at: 400)
        print("통과: callback 결과 분류·표본 귀속·중복")
    }

    private static func testDeadlineBoundaries() {
        let ledger = makeLedger(timeout: 100)
        _ = ledger.setSurfaceGeneration(1, at: 0)

        let beforeKey = key(sequence: 1)
        _ = ledger.register(key: beforeKey, ticket: ticket(sequence: 1), at: 10)
        check(ledger.receiveCallback(key: beforeKey, presentedTime: 0.1, at: 109).outcome == .presented, "one nanosecond before deadline")

        let exactKey = key(sequence: 2)
        _ = ledger.register(key: exactKey, ticket: ticket(sequence: 2), at: 10)
        let exact = ledger.receiveCallback(key: exactKey, presentedTime: 0.2, at: 110)
        check(exact.outcome == .lateCallback && exact.presentedTime == 0.2, "callback exactly at deadline")
        check(!exact.isLatencyEligible && ledger.pendingCount == 0, "late callback excluded from latency sample")

        let afterKey = key(sequence: 3)
        _ = ledger.register(key: afterKey, ticket: ticket(sequence: 3), at: 10)
        check(ledger.expire(at: 110).contains { $0.key == afterKey && $0.outcome == .callbackTimeout }, "timeout at deadline")
        check(ledger.receiveCallback(key: afterKey, presentedTime: 0.3, at: 111).outcome == .lateCallback, "callback after timeout")
        _ = ledger.close(at: 120)
        print("통과: deadline 직전·동일 시각·이후·timeout 이후 callback")
    }

    private static func testIdentityCapacityAndGeneration() {
        let ledger = makeLedger(pendingLimit: 1)
        _ = ledger.setSurfaceGeneration(1, at: 0)
        let firstKey = key(sequence: 1)
        let firstTicket = ticket(sequence: 1)
        check(ledger.register(key: firstKey, ticket: firstTicket, at: 1).outcome == .registered, "first capacity slot")
        check(ledger.register(key: firstKey, ticket: firstTicket, at: 2).outcome == .duplicateKey, "duplicate active key")
        check(ledger.register(key: key(sequence: 2), ticket: ticket(sequence: 2), at: 2).outcome == .capacityRejected, "pending capacity bound")
        check(ledger.pendingCount == 1, "rejected registration does not grow pending")

        let wrongLayer = R05PresentationKey(layerToken: "other-layer", surfaceGeneration: 1, drawSequence: 3, drawableID: 103)
        check(ledger.register(key: wrongLayer, ticket: ticket(sequence: 3), at: 2).outcome == .invalidIdentity, "foreign layer registration")
        let wrongSequence = R05PresentationKey(layerToken: token, surfaceGeneration: 1, drawSequence: 4, drawableID: 104)
        check(ledger.register(key: wrongSequence, ticket: ticket(sequence: 5), at: 2).outcome == .invalidIdentity, "key ticket sequence mismatch")
        check(ledger.register(key: key(sequence: 6), ticket: ticket(sequence: 6, inputSequence: nil), at: 2).outcome == .invalidIdentity, "missing input ticket")
        check(ledger.receiveCallback(key: R05PresentationKey(layerToken: "other-layer", surfaceGeneration: 1, drawSequence: 1, drawableID: 101), presentedTime: 1, at: 3).outcome == .unmatchedCallback, "foreign callback")

        let replacement = ledger.setSurfaceGeneration(2, at: 4)
        check(replacement.contains { $0.key == firstKey && $0.outcome == .staleGeneration }, "pending invalidated on replacement")
        check(ledger.receiveCallback(key: firstKey, presentedTime: 1, at: 5).outcome == .staleGenerationCallback, "old surface callback")
        check(ledger.setSurfaceGeneration(1, at: 6).contains { $0.outcome == .generationRegression }, "generation regression")
        check(ledger.setSurfaceGeneration(0, at: 7).contains { $0.outcome == .generationRegression }, "zero generation rejected")
        _ = ledger.setSurfaceGeneration(2, at: 8)

        let lifecycleLedger = makeLedger(pendingLimit: 3)
        _ = lifecycleLedger.setSurfaceGeneration(2, at: 8)
        for sequence in [9, 7, 8] {
            let pendingKey = key(sequence: UInt64(sequence), generation: 2)
            _ = lifecycleLedger.register(key: pendingKey, ticket: ticket(sequence: UInt64(sequence), generation: 2), at: 9)
        }
        let detached = lifecycleLedger.retireSurfaceGeneration(2, at: 10)
        check(detached.map { $0.key?.drawSequence } == [7, 8, 9], "detach events have deterministic key order")
        check(detached.allSatisfy { $0.outcome == .staleGeneration }, "pending invalidated on detach")
        check(lifecycleLedger.receiveCallback(key: key(sequence: 7, generation: 2), presentedTime: 1, at: 11).outcome == .staleGenerationCallback, "detached surface callback")
        check(lifecycleLedger.pendingCount == 0, "surface transitions leave no pending entries")
        _ = lifecycleLedger.close(at: 20)
        _ = ledger.close(at: 20)
        print("통과: identity·capacity·generation 교체·회귀·detach")
    }

    private static func testOverflowAndTombstones() {
        let overflowLedger = makeLedger(timeout: 10)
        _ = overflowLedger.setSurfaceGeneration(1, at: 0)
        let overflowKey = key(sequence: 1)
        check(overflowLedger.register(key: overflowKey, ticket: ticket(sequence: 1), at: UInt64.max - 5).outcome == .deadlineOverflow, "deadline addition overflow")
        check(overflowLedger.pendingCount == 0, "overflow does not create pending entry")
        _ = overflowLedger.close(at: UInt64.max)

        let bounded = makeLedger(timeout: 100, tombstoneLimit: 2, retention: 100)
        _ = bounded.setSurfaceGeneration(1, at: 0)
        for sequence in 1...3 {
            let itemKey = key(sequence: UInt64(sequence))
            let registeredAt = UInt64(sequence * 5)
            _ = bounded.register(key: itemKey, ticket: ticket(sequence: UInt64(sequence)), at: registeredAt)
            check(bounded.receiveCallback(key: itemKey, presentedTime: 1, at: registeredAt + 1).outcome == .presented, "bounded tombstone sample")
        }
        check(bounded.tombstoneCount == 2, "tombstone cap")
        check(bounded.receiveCallback(key: key(sequence: 1), presentedTime: 1, at: 6).outcome == .unmatchedCallback, "oldest tombstone evicted first")
        check(bounded.receiveCallback(key: key(sequence: 2), presentedTime: 1, at: 6).outcome == .duplicateCallback, "newer tombstone retained")
        _ = bounded.close(at: 10)

        let retained = makeLedger(timeout: 100, tombstoneLimit: 4, retention: 10)
        _ = retained.setSurfaceGeneration(1, at: 0)
        let retainedKey = key(sequence: 1)
        _ = retained.register(key: retainedKey, ticket: ticket(sequence: 1), at: 1)
        _ = retained.receiveCallback(key: retainedKey, presentedTime: 1, at: 5)
        check(retained.tombstoneCount == 1, "terminal tombstone retained")
        check(retained.receiveCallback(key: retainedKey, presentedTime: 1, at: 15).outcome == .unmatchedCallback, "retention expires at boundary")
        _ = retained.close(at: 20)
        print("통과: checked deadline overflow·tombstone 상한·FIFO·보존 만료")
    }

    private static func testConcurrentCallbackAndClose() {
        let duplicateLedger = makeLedger(timeout: 1_000)
        _ = duplicateLedger.setSurfaceGeneration(1, at: 0)
        let duplicateKey = key(sequence: 1)
        _ = duplicateLedger.register(key: duplicateKey, ticket: ticket(sequence: 1), at: 1)
        let resultLock = NSLock()
        var duplicateOutcomes: [R05PresentationOutcome] = []
        DispatchQueue.concurrentPerform(iterations: 64) { _ in
            let result = duplicateLedger.receiveCallback(key: duplicateKey, presentedTime: 1, at: 2)
            resultLock.lock()
            duplicateOutcomes.append(result.outcome)
            resultLock.unlock()
        }
        check(duplicateOutcomes.filter { $0 == .presented }.count == 1, "one winner for concurrent duplicate callbacks")
        check(duplicateOutcomes.filter { $0 == .duplicateCallback }.count == 63, "all other callbacks remain duplicates")
        _ = duplicateLedger.close(at: 3)

        for iteration in 1...100 {
            let ledger = makeLedger(timeout: 1_000)
            _ = ledger.setSurfaceGeneration(1, at: 0)
            let itemKey = key(sequence: UInt64(iteration))
            _ = ledger.register(key: itemKey, ticket: ticket(sequence: UInt64(iteration)), at: 1)
            let gate = DispatchSemaphore(value: 0)
            let group = DispatchGroup()
            let resultLock = NSLock()
            var callbackOutcome: R05PresentationOutcome?
            var closeEvents: [R05PresentationEvent] = []
            group.enter()
            DispatchQueue.global().async {
                gate.wait()
                let result = ledger.receiveCallback(key: itemKey, presentedTime: 1, at: 2)
                resultLock.lock()
                callbackOutcome = result.outcome
                resultLock.unlock()
                group.leave()
            }
            group.enter()
            DispatchQueue.global().async {
                gate.wait()
                let result = ledger.close(at: 2)
                resultLock.lock()
                closeEvents = result
                resultLock.unlock()
                group.leave()
            }
            gate.signal()
            gate.signal()
            check(group.wait(timeout: .now() + 2) == .success, "callback close race completes")
            resultLock.lock()
            let observedCallback = callbackOutcome
            let observedClose = closeEvents
            resultLock.unlock()
            if observedCallback == .presented {
                check(observedClose.isEmpty, "callback wins close race")
            } else {
                check(observedCallback == .callbackAfterClose, "close race callback terminal state")
                check(observedClose.contains { $0.key == itemKey && $0.outcome == .closedPending }, "close wins race with one closed terminal")
            }
            check(ledger.pendingCount == 0, "close race leaves no pending key")
        }

        for iteration in 1...100 {
            let ledger = makeLedger(timeout: 100)
            _ = ledger.setSurfaceGeneration(1, at: 0)
            let itemKey = key(sequence: UInt64(iteration))
            _ = ledger.register(key: itemKey, ticket: ticket(sequence: UInt64(iteration)), at: 1)
            let gate = DispatchSemaphore(value: 0)
            let group = DispatchGroup()
            let resultLock = NSLock()
            var expiryEvents: [R05PresentationEvent] = []
            var closeEvents: [R05PresentationEvent] = []
            group.enter()
            DispatchQueue.global().async {
                gate.wait()
                let result = ledger.expire(at: 101)
                resultLock.lock()
                expiryEvents = result
                resultLock.unlock()
                group.leave()
            }
            group.enter()
            DispatchQueue.global().async {
                gate.wait()
                let result = ledger.close(at: 101)
                resultLock.lock()
                closeEvents = result
                resultLock.unlock()
                group.leave()
            }
            gate.signal()
            gate.signal()
            check(group.wait(timeout: .now() + 2) == .success, "expiry close race completes")
            resultLock.lock()
            let observedExpiry = expiryEvents
            let observedClose = closeEvents
            resultLock.unlock()
            let timedOut = observedExpiry.contains { $0.key == itemKey && $0.outcome == .callbackTimeout }
            let closed = observedClose.contains { $0.key == itemKey && $0.outcome == .closedPending }
            check(timedOut != closed, "expiry and close produce exactly one terminal outcome")
            check(ledger.pendingCount == 0, "expiry close race leaves no pending key")
        }
        print("통과: 병렬 중복 callback 64개·close/callback 100회·expire/close 100회")
    }

    private static func testEventQueueAndTimer() {
        let collector = EventCollector()
        let reference = LedgerReference()
        let ordered = R05PresentationLedger(
            layerToken: token,
            timeoutNanoseconds: 1_000,
            startsTimer: false,
            now: { 10 },
            eventSink: { event in
                _ = reference.readPendingCount()
                collector.append(event)
            })
        reference.set(ordered)
        _ = ordered.setSurfaceGeneration(1, at: 0)
        let orderedKey = key(sequence: 1)
        _ = ordered.register(key: orderedKey, ticket: ticket(sequence: 1), at: 1)
        _ = ordered.receiveCallback(key: orderedKey, presentedTime: 1, at: 2)
        _ = ordered.receiveCallback(key: orderedKey, presentedTime: 1, at: 3)
        guard let observed = collector.waitForEvents(3) else { fatalError("event sink timed out") }
        check(observed.map(\.outcome) == [.registered, .presented, .duplicateCallback], "events preserve transition order")
        _ = ordered.close(at: 4)

        let limitedCollector = EventCollector()
        let sinkEntered = DispatchSemaphore(value: 0)
        let releaseSink = DispatchSemaphore(value: 0)
        let limited = R05PresentationLedger(
            layerToken: "bounded-event-layer",
            timeoutNanoseconds: 1_000,
            eventQueueLimit: 1,
            startsTimer: false,
            now: { 10 },
            eventSink: { event in
                if event.outcome == .registered {
                    sinkEntered.signal()
                    _ = releaseSink.wait(timeout: .now() + 2)
                }
                limitedCollector.append(event)
            })
        _ = limited.setSurfaceGeneration(1, at: 0)
        let limitedKey = R05PresentationKey(layerToken: "bounded-event-layer", surfaceGeneration: 1, drawSequence: 1, drawableID: 1)
        let limitedTicket = R05DrawableTicket(drawSequence: 1, inputSequence: 1, revision: 1, surfaceGeneration: 1)
        _ = limited.register(key: limitedKey, ticket: limitedTicket, at: 1)
        check(sinkEntered.wait(timeout: .now() + 2) == .success, "diagnostic sink starts")
        check(limited.receiveCallback(key: limitedKey, presentedTime: 1, at: 2).outcome == .presented, "state transition survives diagnostic queue saturation")
        check(limited.droppedEventCount == 1, "diagnostic queue has a hard bound and counts dropped events")
        releaseSink.signal()
        guard let limitedEvents = limitedCollector.waitForEvents(2) else { fatalError("dropped-event summary timed out") }
        check(limitedEvents.map(\.outcome) == [.registered, .diagnosticEventsDropped], "dropped diagnostic is reported in order")
        check(limitedEvents[1].detail?.contains("invalidate_measurement=true") == true, "dropped logs invalidate the measurement")
        check(limited.droppedEventCount == 0, "dropped-event count clears after summary enqueue")
        _ = limited.close(at: 3)

        let timerSignal = DispatchSemaphore(value: 0)
        let timerLedger = R05PresentationLedger(
            layerToken: "timer-layer",
            pendingLimit: 2,
            timeoutNanoseconds: 30_000_000,
            tombstoneLimit: 2,
            tombstoneRetentionNanoseconds: 100_000_000,
            sweepNanoseconds: 1_000_000,
            eventSink: { event in
                if event.outcome == .callbackTimeout { timerSignal.signal() }
            })
        _ = timerLedger.setSurfaceGeneration(1)
        let timerKey = R05PresentationKey(layerToken: "timer-layer", surfaceGeneration: 1, drawSequence: 1, drawableID: 1)
        let timerTicket = R05DrawableTicket(drawSequence: 1, inputSequence: 1, revision: 1, surfaceGeneration: 1)
        _ = timerLedger.register(key: timerKey, ticket: timerTicket)
        check(timerSignal.wait(timeout: .now() + 2) == .success, "serial timer expires pending callback")
        check(timerLedger.pendingCount == 0, "timer removes expired pending entry")
        _ = timerLedger.close()

        let cancelledTimeoutSignal = DispatchSemaphore(value: 0)
        let cancelledTimer = R05PresentationLedger(
            layerToken: "cancelled-timer-layer",
            timeoutNanoseconds: 80_000_000,
            sweepNanoseconds: 2_000_000,
            eventSink: { event in
                if event.outcome == .callbackTimeout { cancelledTimeoutSignal.signal() }
            })
        _ = cancelledTimer.setSurfaceGeneration(1)
        let cancelledKey = R05PresentationKey(layerToken: "cancelled-timer-layer", surfaceGeneration: 1, drawSequence: 1, drawableID: 1)
        let cancelledTicket = R05DrawableTicket(drawSequence: 1, inputSequence: 1, revision: 1, surfaceGeneration: 1)
        _ = cancelledTimer.register(key: cancelledKey, ticket: cancelledTicket)
        _ = cancelledTimer.close()
        check(cancelledTimeoutSignal.wait(timeout: .now() + 0.2) == .timedOut, "close cancels future timer timeout")
        print("통과: bounded event 큐·drop 무효 표식·sink 순서/lock·serial timer timeout")
    }

    private static func makeLedger(
        pendingLimit: Int = 64,
        timeout: UInt64 = 100,
        tombstoneLimit: Int = 64,
        retention: UInt64 = 100
    ) -> R05PresentationLedger {
        R05PresentationLedger(
            layerToken: token,
            pendingLimit: pendingLimit,
            timeoutNanoseconds: timeout,
            tombstoneLimit: tombstoneLimit,
            tombstoneRetentionNanoseconds: retention,
            startsTimer: false,
            now: { 1 })
    }

    private static func key(sequence: UInt64, generation: Int = 1) -> R05PresentationKey {
        R05PresentationKey(layerToken: token, surfaceGeneration: generation, drawSequence: sequence, drawableID: 100 + sequence)
    }

    private static func ticket(sequence: UInt64, generation: Int = 1, inputSequence: Int? = 1) -> R05DrawableTicket {
        R05DrawableTicket(drawSequence: sequence, inputSequence: inputSequence, revision: 7, surfaceGeneration: generation)
    }

    private static func check(_ condition: @autoclosure () -> Bool, _ message: String) {
        guard condition() else {
            FileHandle.standardError.write(Data("실패: \(message)\n".utf8))
            fatalError("실패: \(message)")
        }
    }

}
