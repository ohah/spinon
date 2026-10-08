import Dispatch
import Foundation

struct R05DrawableTicket {
    let drawSequence: UInt64
    let inputSequence: Int?
    let revision: UInt32
    let surfaceGeneration: Int

    var logFields: String {
        let inputField = inputSequence.map { String($0) } ?? "none"
        return "draw_seq=\(drawSequence) input_seq=\(inputField) revision=\(revision) generation=\(surfaceGeneration)"
    }
}

struct R05PresentationKey: Hashable {
    let layerToken: String
    let surfaceGeneration: Int
    let drawSequence: UInt64
    let drawableID: UInt64
}

enum R05PresentationOutcome: String {
    case registered
    case presented
    case zeroTime = "zero_time"
    case callbackTimeout = "missing_callback_timeout"
    case lateCallback = "late_callback"
    case duplicateKey = "duplicate_key"
    case duplicateCallback = "duplicate_callback"
    case capacityRejected = "capacity_rejected"
    case staleGeneration = "stale_generation"
    case staleGenerationCallback = "stale_generation_callback"
    case unmatchedCallback = "unmatched_callback"
    case closedPending = "closed_pending"
    case callbackAfterClose = "callback_after_close"
    case ledgerClosed = "ledger_closed"
    case invalidPresentedTime = "invalid_presented_time"
    case diagnosticEventsDropped = "diagnostic_events_dropped"
    case deadlineOverflow = "deadline_overflow"
    case generationRegression = "generation_regression"
    case invalidIdentity = "invalid_identity"
}

struct R05PresentationEvent {
    let outcome: R05PresentationOutcome
    let key: R05PresentationKey?
    let inputSequence: Int?
    let revision: UInt32?
    let presentedTime: TimeInterval?
    let observedAtUptimeNanoseconds: UInt64
    let detail: String?

    var isLatencyEligible: Bool { outcome == .presented }

    var logMessage: String {
        let keyFields: String
        if let key {
            keyFields = "layer_token=\(key.layerToken) drawable_id=\(key.drawableID) draw_seq=\(key.drawSequence) generation=\(key.surfaceGeneration)"
        } else {
            keyFields = "layer_token=none drawable_id=none draw_seq=none generation=none"
        }
        let inputField = inputSequence.map { String($0) } ?? "none"
        let revisionField = revision.map { String($0) } ?? "none"
        let presentedField = presentedTime.map { String($0) } ?? "none"
        let detailField = detail ?? "none"
        return "SPINON_R05_CALLBACK outcome=\(outcome.rawValue) \(keyFields) input_seq=\(inputField) revision=\(revisionField) presented_time_s=\(presentedField) observed_uptime_ns=\(observedAtUptimeNanoseconds) detail=\(detailField)"
    }
}

private final class R05DiagnosticDropCounter {
    private let lock = NSLock()
    private var count: UInt64 = 0

    var droppedEventCount: UInt64 {
        lock.lock()
        defer { lock.unlock() }
        return count
    }

    func recordDrop() {
        lock.lock()
        if count < UInt64.max { count += 1 }
        lock.unlock()
    }

    func enqueueSummaryIfPossible(
        on queue: DispatchQueue,
        using slots: DispatchSemaphore,
        eventSink: @escaping (R05PresentationEvent) -> Void
    ) {
        lock.lock()
        guard count > 0, slots.wait(timeout: .now()) == .success else {
            lock.unlock()
            return
        }
        let dropped = count
        count = 0
        let event = R05PresentationEvent(
            outcome: .diagnosticEventsDropped,
            key: nil,
            inputSequence: nil,
            revision: nil,
            presentedTime: nil,
            observedAtUptimeNanoseconds: DispatchTime.now().uptimeNanoseconds,
            detail: "dropped_count=\(dropped); invalidate_measurement=true")
        queue.async { [self, queue, slots, eventSink] in
            eventSink(event)
            slots.signal()
            enqueueSummaryIfPossible(on: queue, using: slots, eventSink: eventSink)
        }
        lock.unlock()
    }
}

final class R05PresentationLedger {
    static let defaultPendingLimit = 64
    static let defaultTimeoutNanoseconds: UInt64 = 2_000_000_000
    static let defaultTombstoneLimit = 64
    static let defaultTombstoneRetentionNanoseconds: UInt64 = 10_000_000_000
    static let defaultSweepNanoseconds: UInt64 = 100_000_000
    static let defaultEventQueueLimit = 256

    let layerToken: String

    private struct PendingEntry {
        let ticket: R05DrawableTicket
        let deadlineNanoseconds: UInt64
    }

    private struct Tombstone {
        let outcome: R05PresentationOutcome
        let recordedAtNanoseconds: UInt64
        let retainUntilNanoseconds: UInt64
        var order: UInt64
    }

    private let lock = NSLock()
    private let pendingLimit: Int
    private let timeoutNanoseconds: UInt64
    private let tombstoneLimit: Int
    private let tombstoneRetentionNanoseconds: UInt64
    private let sweepNanoseconds: UInt64
    private let eventQueueSlots: DispatchSemaphore
    private let now: () -> UInt64
    private let eventSink: (R05PresentationEvent) -> Void
    private let eventQueue: DispatchQueue
    private let droppedEvents = R05DiagnosticDropCounter()
    private var timer: DispatchSourceTimer?
    private var pending: [R05PresentationKey: PendingEntry] = [:]
    private var tombstones: [R05PresentationKey: Tombstone] = [:]
    private var terminalOrder: UInt64 = 0
    private var currentGeneration: Int?
    private var isClosed = false

    init(
        layerToken: String = UUID().uuidString,
        pendingLimit: Int = R05PresentationLedger.defaultPendingLimit,
        timeoutNanoseconds: UInt64 = R05PresentationLedger.defaultTimeoutNanoseconds,
        tombstoneLimit: Int = R05PresentationLedger.defaultTombstoneLimit,
        tombstoneRetentionNanoseconds: UInt64 = R05PresentationLedger.defaultTombstoneRetentionNanoseconds,
        sweepNanoseconds: UInt64 = R05PresentationLedger.defaultSweepNanoseconds,
        eventQueueLimit: Int = R05PresentationLedger.defaultEventQueueLimit,
        startsTimer: Bool = true,
        now: @escaping () -> UInt64 = { DispatchTime.now().uptimeNanoseconds },
        eventSink: @escaping (R05PresentationEvent) -> Void = { _ in }
    ) {
        self.layerToken = layerToken
        self.pendingLimit = max(0, pendingLimit)
        self.timeoutNanoseconds = timeoutNanoseconds
        self.tombstoneLimit = max(0, tombstoneLimit)
        self.tombstoneRetentionNanoseconds = tombstoneRetentionNanoseconds
        self.sweepNanoseconds = max(1, sweepNanoseconds)
        self.eventQueueSlots = DispatchSemaphore(value: max(1, eventQueueLimit))
        self.now = now
        self.eventSink = eventSink
        self.eventQueue = DispatchQueue(label: "dev.spinon.r05.presentation-events")

        guard startsTimer else { return }
        let timer = DispatchSource.makeTimerSource(queue: DispatchQueue(label: "dev.spinon.r05.presentation-timer"))
        let interval = DispatchTimeInterval.nanoseconds(Int(min(self.sweepNanoseconds, UInt64(Int.max))))
        timer.schedule(deadline: .now() + interval, repeating: interval, leeway: .milliseconds(10))
        timer.setEventHandler { [weak self] in
            _ = self?.expire()
        }
        self.timer = timer
        timer.resume()
    }

    @discardableResult
    func setSurfaceGeneration(_ generation: Int, at nowNanoseconds: UInt64? = nil) -> [R05PresentationEvent] {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        var events = expirePendingLocked(at: timestamp)
        pruneTombstonesLocked(at: timestamp)
        if isClosed {
            events.append(makeEvent(.ledgerClosed, key: nil, ticket: nil, at: timestamp, detail: "set_generation"))
        } else if generation <= 0 || (currentGeneration.map { generation < $0 } ?? false) {
            let current = currentGeneration.map { String($0) } ?? "none"
            events.append(makeEvent(.generationRegression, key: nil, ticket: nil, at: timestamp, detail: "requested=\(generation) current=\(current)"))
        } else if currentGeneration != generation {
            let staleKeys = orderedKeys(pending.keys.filter { $0.surfaceGeneration != generation })
            for key in staleKeys {
                guard let entry = pending.removeValue(forKey: key) else { continue }
                rememberTerminalLocked(key, outcome: .staleGeneration, at: timestamp)
                events.append(makeEvent(.staleGeneration, key: key, ticket: entry.ticket, at: timestamp, detail: "surface_replaced"))
            }
            currentGeneration = generation
        }
        enqueueEventsLocked(events)
        lock.unlock()
        return events
    }

    @discardableResult
    func retireSurfaceGeneration(_ generation: Int, at nowNanoseconds: UInt64? = nil) -> [R05PresentationEvent] {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        var events = expirePendingLocked(at: timestamp)
        pruneTombstonesLocked(at: timestamp)
        guard !isClosed, currentGeneration == generation else {
            enqueueEventsLocked(events)
            lock.unlock()
            return events
        }
        let staleKeys = orderedKeys(pending.keys.filter { $0.surfaceGeneration == generation })
        for key in staleKeys {
            guard let entry = pending.removeValue(forKey: key) else { continue }
            rememberTerminalLocked(key, outcome: .staleGeneration, at: timestamp)
            events.append(makeEvent(.staleGeneration, key: key, ticket: entry.ticket, at: timestamp, detail: "surface_detached"))
        }
        currentGeneration = nil
        enqueueEventsLocked(events)
        lock.unlock()
        return events
    }

    @discardableResult
    func register(
        key: R05PresentationKey,
        ticket: R05DrawableTicket,
        at nowNanoseconds: UInt64? = nil
    ) -> R05PresentationEvent {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        var events = expirePendingLocked(at: timestamp)
        pruneTombstonesLocked(at: timestamp)
        let event: R05PresentationEvent
        if isClosed {
            event = makeEvent(.ledgerClosed, key: key, ticket: ticket, at: timestamp, detail: "register")
        } else if key.layerToken != layerToken
            || key.drawSequence == 0
            || key.drawSequence != ticket.drawSequence
            || key.surfaceGeneration != ticket.surfaceGeneration
            || ticket.inputSequence == nil {
            event = makeEvent(.invalidIdentity, key: key, ticket: ticket, at: timestamp, detail: "key_ticket_mismatch_or_no_input")
        } else if currentGeneration != key.surfaceGeneration {
            event = makeEvent(.staleGeneration, key: key, ticket: ticket, at: timestamp, detail: "not_current_surface")
        } else if pending[key] != nil || tombstones[key] != nil {
            event = makeEvent(.duplicateKey, key: key, ticket: ticket, at: timestamp, detail: "already_seen")
        } else if pending.count >= pendingLimit {
            event = makeEvent(.capacityRejected, key: key, ticket: ticket, at: timestamp, detail: "pending_limit=\(pendingLimit)")
        } else {
            let (deadline, overflow) = timestamp.addingReportingOverflow(timeoutNanoseconds)
            if overflow {
                event = makeEvent(.deadlineOverflow, key: key, ticket: ticket, at: timestamp, detail: "monotonic_deadline_overflow")
            } else {
                pending[key] = PendingEntry(ticket: ticket, deadlineNanoseconds: deadline)
                event = makeEvent(.registered, key: key, ticket: ticket, at: timestamp, detail: "deadline_ns=\(deadline)")
            }
        }
        events.append(event)
        enqueueEventsLocked(events)
        lock.unlock()
        return event
    }

    @discardableResult
    func receiveCallback(
        key: R05PresentationKey,
        presentedTime: TimeInterval,
        at nowNanoseconds: UInt64? = nil
    ) -> R05PresentationEvent {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        var events = expirePendingLocked(at: timestamp)
        pruneTombstonesLocked(at: timestamp)
        let event: R05PresentationEvent
        if isClosed {
            event = makeEvent(.callbackAfterClose, key: key, ticket: nil, at: timestamp, presentedTime: presentedTime, detail: "ledger_closed")
        } else if key.layerToken != layerToken {
            event = makeEvent(.unmatchedCallback, key: key, ticket: nil, at: timestamp, presentedTime: presentedTime, detail: "foreign_layer")
        } else if let terminal = tombstones[key] {
            let outcome: R05PresentationOutcome
            switch terminal.outcome {
            case .callbackTimeout: outcome = .lateCallback
            case .staleGeneration: outcome = .staleGenerationCallback
            case .closedPending: outcome = .callbackAfterClose
            default: outcome = .duplicateCallback
            }
            event = makeEvent(outcome, key: key, ticket: nil, at: timestamp, presentedTime: presentedTime, detail: "prior=\(terminal.outcome.rawValue)")
        } else if currentGeneration != key.surfaceGeneration {
            event = makeEvent(.staleGenerationCallback, key: key, ticket: nil, at: timestamp, presentedTime: presentedTime, detail: "generation_not_current")
        } else if let entry = pending.removeValue(forKey: key) {
            let outcome: R05PresentationOutcome
            if presentedTime.isFinite, presentedTime > 0 {
                outcome = .presented
            } else if presentedTime == 0 {
                outcome = .zeroTime
            } else {
                outcome = .invalidPresentedTime
            }
            rememberTerminalLocked(key, outcome: outcome, at: timestamp)
            event = makeEvent(outcome, key: key, ticket: entry.ticket, at: timestamp, presentedTime: presentedTime, detail: "callback_received")
        } else {
            event = makeEvent(.unmatchedCallback, key: key, ticket: nil, at: timestamp, presentedTime: presentedTime, detail: "no_pending_key")
        }
        events.append(event)
        enqueueEventsLocked(events)
        lock.unlock()
        return event
    }

    @discardableResult
    func expire(at nowNanoseconds: UInt64? = nil) -> [R05PresentationEvent] {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        pruneTombstonesLocked(at: timestamp)
        let events = expirePendingLocked(at: timestamp)
        enqueueEventsLocked(events)
        lock.unlock()
        return events
    }

    @discardableResult
    func close(at nowNanoseconds: UInt64? = nil) -> [R05PresentationEvent] {
        lock.lock()
        let timestamp = nowNanoseconds ?? now()
        guard !isClosed else {
            lock.unlock()
            return []
        }
        isClosed = true
        currentGeneration = nil
        let timerToCancel = timer
        timer = nil
        var events: [R05PresentationEvent] = []
        for key in orderedKeys(pending.keys) {
            guard let entry = pending[key] else { continue }
            rememberTerminalLocked(key, outcome: .closedPending, at: timestamp)
            events.append(makeEvent(.closedPending, key: key, ticket: entry.ticket, at: timestamp, detail: "layer_closed"))
        }
        pending.removeAll(keepingCapacity: false)
        enqueueEventsLocked(events)
        lock.unlock()
        timerToCancel?.cancel()
        return events
    }

    var pendingCount: Int {
        lock.lock()
        defer { lock.unlock() }
        return pending.count
    }

    var tombstoneCount: Int {
        lock.lock()
        defer { lock.unlock() }
        return tombstones.count
    }

    var droppedEventCount: UInt64 {
        droppedEvents.droppedEventCount
    }

    deinit {
        _ = close()
    }

    private func expirePendingLocked(at timestamp: UInt64) -> [R05PresentationEvent] {
        guard !isClosed else { return [] }
        let expiredKeys = orderedKeys(pending.compactMap { key, entry in
            timestamp >= entry.deadlineNanoseconds ? key : nil
        })
        var events: [R05PresentationEvent] = []
        for key in expiredKeys {
            guard let entry = pending.removeValue(forKey: key) else { continue }
            rememberTerminalLocked(key, outcome: .callbackTimeout, at: timestamp)
            events.append(makeEvent(.callbackTimeout, key: key, ticket: entry.ticket, at: timestamp, detail: "deadline_elapsed"))
        }
        return events
    }

    private func pruneTombstonesLocked(at timestamp: UInt64) {
        let expired = tombstones.filter { timestamp >= $0.value.retainUntilNanoseconds }.map(\.key)
        for key in expired { tombstones.removeValue(forKey: key) }
    }

    private func orderedKeys<S: Sequence>(_ keys: S) -> [R05PresentationKey] where S.Element == R05PresentationKey {
        keys.sorted {
            if $0.drawSequence != $1.drawSequence { return $0.drawSequence < $1.drawSequence }
            if $0.surfaceGeneration != $1.surfaceGeneration { return $0.surfaceGeneration < $1.surfaceGeneration }
            if $0.drawableID != $1.drawableID { return $0.drawableID < $1.drawableID }
            return $0.layerToken < $1.layerToken
        }
    }

    private func rememberTerminalLocked(_ key: R05PresentationKey, outcome: R05PresentationOutcome, at timestamp: UInt64) {
        guard tombstoneLimit > 0 else { return }
        if terminalOrder == UInt64.max {
            let ordered = tombstones.sorted { $0.value.order < $1.value.order }
            for (index, item) in ordered.enumerated() {
                tombstones[item.key]?.order = UInt64(index + 1)
            }
            terminalOrder = UInt64(ordered.count)
        }
        terminalOrder += 1
        let (retainedUntil, overflow) = timestamp.addingReportingOverflow(tombstoneRetentionNanoseconds)
        tombstones[key] = Tombstone(
            outcome: outcome,
            recordedAtNanoseconds: timestamp,
            retainUntilNanoseconds: overflow ? UInt64.max : retainedUntil,
            order: terminalOrder)
        while tombstones.count > tombstoneLimit {
            guard let oldest = tombstones.min(by: { $0.value.order < $1.value.order })?.key else { break }
            tombstones.removeValue(forKey: oldest)
        }
    }

    private func makeEvent(
        _ outcome: R05PresentationOutcome,
        key: R05PresentationKey?,
        ticket: R05DrawableTicket?,
        at timestamp: UInt64,
        presentedTime: TimeInterval? = nil,
        detail: String?
    ) -> R05PresentationEvent {
        R05PresentationEvent(
            outcome: outcome,
            key: key,
            inputSequence: ticket?.inputSequence,
            revision: ticket?.revision,
            presentedTime: presentedTime,
            observedAtUptimeNanoseconds: timestamp,
            detail: detail)
    }

    private func enqueueEventsLocked(_ events: [R05PresentationEvent]) {
        droppedEvents.enqueueSummaryIfPossible(on: eventQueue, using: eventQueueSlots, eventSink: eventSink)
        for event in events {
            guard eventQueueSlots.wait(timeout: .now()) == .success else {
                droppedEvents.recordDrop()
                continue
            }
            let slots = eventQueueSlots
            let queue = eventQueue
            let droppedEvents = self.droppedEvents
            queue.async { [eventSink, slots, droppedEvents, queue] in
                eventSink(event)
                slots.signal()
                droppedEvents.enqueueSummaryIfPossible(on: queue, using: slots, eventSink: eventSink)
            }
        }
    }
}
