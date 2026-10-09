import Foundation

/// 최신 canonical 상태를 읽는 직렬 작업의 대기 drain을 하나로 제한합니다.
final class C0410LatestTaskLane {
    enum Admission {
        case scheduled
        case coalesced
        case closed
    }

    struct Snapshot {
        let requests: UInt64
        let coalesced: UInt64
        let completed: UInt64
        let failures: UInt64
        let pendingDrains: Int
        let scheduled: Bool
        let dirty: Bool
        let closed: Bool
    }

    private let queue: DispatchQueue
    private let lock = NSLock()
    private let work: () throws -> Void
    private let onFailure: (String) -> Void
    private var scheduled = false
    private var dirty = false
    private var closed = false
    private var requests: UInt64 = 0
    private var coalesced: UInt64 = 0
    private var completed: UInt64 = 0
    private var failures: UInt64 = 0
    private var pendingDrains = 0

    init(
        queue: DispatchQueue,
        work: @escaping () throws -> Void,
        onFailure: @escaping (String) -> Void
    ) {
        self.queue = queue
        self.work = work
        self.onFailure = onFailure
    }

    @discardableResult
    func request() -> Admission {
        lock.lock()
        guard !closed else {
            lock.unlock()
            return .closed
        }
        requests &+= 1
        if scheduled {
            dirty = true
            coalesced &+= 1
            lock.unlock()
            return .coalesced
        }
        scheduled = true
        pendingDrains = 1
        lock.unlock()
        queue.async { [weak self] in self?.runDrain() }
        return .scheduled
    }

    func close() {
        lock.lock()
        closed = true
        dirty = false
        lock.unlock()
    }

    func snapshot() -> Snapshot {
        lock.lock()
        defer { lock.unlock() }
        return Snapshot(
            requests: requests,
            coalesced: coalesced,
            completed: completed,
            failures: failures,
            pendingDrains: pendingDrains,
            scheduled: scheduled,
            dirty: dirty,
            closed: closed
        )
    }

    private func runDrain() {
        lock.lock()
        pendingDrains = 0
        guard !closed else {
            scheduled = false
            dirty = false
            lock.unlock()
            return
        }
        dirty = false
        lock.unlock()

        let failure: String?
        do {
            try work()
            failure = nil
        } catch {
            failure = String(describing: error)
        }

        lock.lock()
        completed &+= 1
        if failure != nil { failures &+= 1 }
        let dispatchAgain = !closed && dirty
        if dispatchAgain {
            dirty = false
            pendingDrains = 1
        } else {
            scheduled = false
            dirty = false
            pendingDrains = 0
        }
        lock.unlock()
        if dispatchAgain {
            queue.async { [weak self] in self?.runDrain() }
        }
        if let failure { onFailure(failure) }
    }
}
