import Dispatch
import Foundation

@main
struct C0410LatestTaskLaneTest {
    static func main() {
        testBurstUsesLatestStateAndOnePendingDrain()
        testConcurrentProducersMaintainBoundedQueue()
        testCloseDropsPendingWork()
        testWorkFailureDoesNotWedgeLane()
        print("C0410LatestTaskLane Swift checks: 4 passed")
    }

    private static func testWorkFailureDoesNotWedgeLane() {
        let queue = DispatchQueue(label: "spinon.c0410.test.failure")
        let callsLock = NSLock()
        let firstFailure = DispatchSemaphore(value: 0)
        let secondSuccess = DispatchSemaphore(value: 0)
        var calls = 0
        var failures: [String] = []
        let failureLock = NSLock()
        let lane = C0410LatestTaskLane(queue: queue, work: {
            callsLock.lock()
            calls += 1
            let call = calls
            callsLock.unlock()
            if call == 1 { throw TestLaneError.injected }
            secondSuccess.signal()
        }, onFailure: { message in
            failureLock.lock()
            failures.append(message)
            failureLock.unlock()
            firstFailure.signal()
        })
        lane.request()
        require(firstFailure.wait(timeout: .now() + 5) == .success, "작업 오류가 보고되지 않았습니다")
        require(lane.snapshot().failures == 1, "작업 오류 계수가 증가하지 않았습니다")
        require(lane.request() == .scheduled, "작업 오류 뒤 lane이 다시 예약되지 않았습니다")
        require(secondSuccess.wait(timeout: .now() + 5) == .success, "작업 오류 뒤 재시도가 실행되지 않았습니다")
        waitUntilIdle(lane)
        callsLock.lock()
        let observedCalls = calls
        callsLock.unlock()
        failureLock.lock()
        let observedFailures = failures.count
        failureLock.unlock()
        require(observedCalls == 2 && observedFailures == 1, "작업 실패 회복 계수가 다릅니다")
        lane.close()
    }

    private static func testConcurrentProducersMaintainBoundedQueue() {
        let producerCount = 8
        let requestsPerProducer = 10_000
        let finalValue = UInt64(producerCount * requestsPerProducer)
        let queue = DispatchQueue(label: "spinon.c0410.test.concurrent")
        let stateLock = NSLock()
        let firstEntered = DispatchSemaphore(value: 0)
        let releaseFirst = DispatchSemaphore(value: 0)
        let latestObserved = DispatchSemaphore(value: 0)
        var currentValue: UInt64 = 0
        var callCount = 0
        var observedValue: UInt64 = 0
        let lane = C0410LatestTaskLane(queue: queue, work: {
            stateLock.lock()
            callCount += 1
            let call = callCount
            stateLock.unlock()
            if call == 1 {
                firstEntered.signal()
                require(releaseFirst.wait(timeout: .now() + 5) == .success,
                        "동시 생산자 latch timeout")
            }
            stateLock.lock()
            observedValue = currentValue
            let latest = observedValue == finalValue
            stateLock.unlock()
            if latest { latestObserved.signal() }
        }, onFailure: { message in
            fatalError("unexpected concurrent lane failure: \(message)")
        })

        require(lane.request() == .scheduled, "동시 생산자 첫 요청이 예약되지 않았습니다")
        require(firstEntered.wait(timeout: .now() + 5) == .success,
                "동시 생산자 시험 작업이 시작되지 않았습니다")
        DispatchQueue.concurrentPerform(iterations: producerCount) { _ in
            for _ in 0..<requestsPerProducer {
                stateLock.lock()
                currentValue += 1
                stateLock.unlock()
                _ = lane.request()
            }
        }
        let queued = lane.snapshot()
        require(queued.pendingDrains <= 1, "동시 생산자가 대기 drain을 여러 개 만들었습니다")
        require(queued.coalesced == finalValue, "동시 요청의 coalescing 계수가 다릅니다")
        releaseFirst.signal()
        require(latestObserved.wait(timeout: .now() + 5) == .success,
                "동시 요청의 최신 상태를 읽지 못했습니다")
        waitUntilIdle(lane)
        stateLock.lock()
        let observedCalls = callCount
        let latestValue = observedValue
        stateLock.unlock()
        require(observedCalls == 2, "동시 burst가 후속 drain 하나로 합쳐지지 않았습니다")
        require(latestValue == finalValue, "동시 burst 뒤 최신 상태가 보존되지 않았습니다")
        lane.close()
    }

    private static func testBurstUsesLatestStateAndOnePendingDrain() {
        let requestCount: UInt64 = 100_000
        let queue = DispatchQueue(label: "spinon.c0410.test.burst")
        let stateLock = NSLock()
        let firstEntered = DispatchSemaphore(value: 0)
        let releaseFirst = DispatchSemaphore(value: 0)
        let latestObserved = DispatchSemaphore(value: 0)
        var currentValue: UInt64 = 0
        var callCount = 0
        var lastValue: UInt64 = 0
        let lane = C0410LatestTaskLane(queue: queue, work: {
            stateLock.lock()
            callCount += 1
            let call = callCount
            stateLock.unlock()
            if call == 1 {
                firstEntered.signal()
                require(releaseFirst.wait(timeout: .now() + 5) == .success, "첫 작업 latch timeout")
            }
            stateLock.lock()
            lastValue = currentValue
            let latest = lastValue == requestCount
            stateLock.unlock()
            if latest { latestObserved.signal() }
        }, onFailure: { message in
            fatalError("unexpected lane failure: \(message)")
        })

        require(lane.request() == .scheduled, "첫 요청은 drain을 예약해야 합니다")
        require(firstEntered.wait(timeout: .now() + 5) == .success, "첫 작업이 시작되지 않았습니다")
        for value in 1...requestCount {
            stateLock.lock()
            currentValue = value
            stateLock.unlock()
            require(lane.request() == .coalesced, "실행 중 요청이 합쳐지지 않았습니다")
            let snapshot = lane.snapshot()
            require(snapshot.pendingDrains <= 1, "pending drain이 1개를 넘었습니다")
            require(snapshot.dirty, "대량 요청의 최신 상태 표시가 사라졌습니다")
        }
        releaseFirst.signal()
        require(latestObserved.wait(timeout: .now() + 5) == .success, "최신 입력이 실행되지 않았습니다")
        waitUntilIdle(lane)
        stateLock.lock()
        let observedCalls = callCount
        let observedValue = lastValue
        stateLock.unlock()
        require(observedCalls == 2, "100,000개 입력이 실행 2회로 합쳐지지 않았습니다")
        require(observedValue == requestCount, "마지막 실행이 최신 상태를 읽지 않았습니다")
        require(lane.snapshot().coalesced == requestCount, "coalescing 계수가 다릅니다")
        lane.close()
    }

    private static func testCloseDropsPendingWork() {
        let queue = DispatchQueue(label: "spinon.c0410.test.close")
        let firstEntered = DispatchSemaphore(value: 0)
        let releaseFirst = DispatchSemaphore(value: 0)
        let closedWorkRan = DispatchSemaphore(value: 0)
        let callLock = NSLock()
        var calls = 0
        let lane = C0410LatestTaskLane(queue: queue, work: {
            callLock.lock()
            calls += 1
            let call = calls
            callLock.unlock()
            if call == 1 {
                firstEntered.signal()
                require(releaseFirst.wait(timeout: .now() + 5) == .success, "종료 latch timeout")
            } else {
                closedWorkRan.signal()
            }
        }, onFailure: { message in
            fatalError("unexpected close failure: \(message)")
        })
        lane.request()
        require(firstEntered.wait(timeout: .now() + 5) == .success, "종료 시험 작업이 시작되지 않았습니다")
        for _ in 0..<10 { lane.request() }
        lane.close()
        releaseFirst.signal()
        require(queue.sync { true }, "serial queue barrier 실패")
        callLock.lock()
        let observedCalls = calls
        callLock.unlock()
        require(observedCalls == 1, "close 뒤 pending 작업을 실행했습니다")
        require(closedWorkRan.wait(timeout: .now()) == .timedOut, "닫힌 lane이 callback을 실행했습니다")
        require(lane.request() == .closed, "close 뒤 요청을 거부하지 않았습니다")
    }

    private static func waitUntilIdle(_ lane: C0410LatestTaskLane) {
        let deadline = Date().addingTimeInterval(5)
        while lane.snapshot().scheduled && Date() < deadline {
            Thread.sleep(forTimeInterval: 0.001)
        }
        require(!lane.snapshot().scheduled, "lane가 idle로 돌아오지 않았습니다")
    }

    private static func require(_ condition: @autoclosure () -> Bool, _ message: String) {
        if !condition() { fatalError(message) }
    }

    private enum TestLaneError: Error {
        case injected
    }
}
