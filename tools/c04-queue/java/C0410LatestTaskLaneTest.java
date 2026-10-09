package dev.spinon.bootstrap;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executor;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

/** Android latest-only admission의 독립적인 JVM 경쟁 시험입니다. */
public final class C0410LatestTaskLaneTest {
    private C0410LatestTaskLaneTest() {}

    public static void main(String[] args) throws Exception {
        testBurstUsesLatestStateAndOnePendingDrain();
        testConcurrentProducersMaintainBoundedQueue();
        testCloseDropsPendingWork();
        testConsumerFailureDoesNotWedgeLane();
        testExecutorRejectionCanRecover();
        System.out.println("C0410LatestTaskLane JVM checks: 5 passed");
    }

    private static void testConcurrentProducersMaintainBoundedQueue() throws Exception {
        final int producerCount = 8;
        final int requestsPerProducer = 10_000;
        ThreadPoolExecutor executor = singleThreadExecutor();
        ExecutorService producers = Executors.newFixedThreadPool(producerCount);
        AtomicInteger currentValue = new AtomicInteger();
        AtomicInteger calls = new AtomicInteger();
        CountDownLatch firstEntered = new CountDownLatch(1);
        CountDownLatch releaseFirst = new CountDownLatch(1);
        CountDownLatch latestObserved = new CountDownLatch(1);
        int finalValue = producerCount * requestsPerProducer;
        C0410LatestTaskLane lane = new C0410LatestTaskLane(executor, () -> {
            if (calls.incrementAndGet() == 1) {
                firstEntered.countDown();
                await(releaseFirst);
            }
            if (currentValue.get() == finalValue) latestObserved.countDown();
        }, error -> { throw new AssertionError("unexpected lane error", error); });

        lane.request();
        check(firstEntered.await(5, TimeUnit.SECONDS), "동시 생산자 시험 작업이 시작되지 않았습니다");
        List<java.util.concurrent.Future<?>> submissions = new ArrayList<>();
        for (int producer = 0; producer < producerCount; producer++) {
            submissions.add(producers.submit(() -> {
                for (int request = 0; request < requestsPerProducer; request++) {
                    currentValue.incrementAndGet();
                    lane.request();
                }
            }));
        }
        for (java.util.concurrent.Future<?> submission : submissions) submission.get();
        check(executor.getQueue().size() <= 1, "동시 생산자가 대기 drain을 여러 개 만들었습니다");
        check(lane.snapshot().coalesced() == finalValue,
                "동시 생산자 요청 수가 coalescing 계수와 다릅니다");
        releaseFirst.countDown();
        check(latestObserved.await(5, TimeUnit.SECONDS), "동시 요청의 최신 상태를 읽지 못했습니다");
        awaitIdle(lane);
        check(calls.get() == 2, "동시 burst가 하나의 후속 drain으로 합쳐지지 않았습니다");
        lane.close();
        producers.shutdown();
        executor.shutdown();
        check(producers.awaitTermination(5, TimeUnit.SECONDS), "생산자 executor가 끝나지 않았습니다");
        check(executor.awaitTermination(5, TimeUnit.SECONDS), "lane executor가 끝나지 않았습니다");
    }

    private static void testBurstUsesLatestStateAndOnePendingDrain() throws Exception {
        final int requestCount = 100_000;
        ThreadPoolExecutor executor = singleThreadExecutor();
        AtomicInteger currentValue = new AtomicInteger();
        AtomicInteger calls = new AtomicInteger();
        List<Integer> observed = new ArrayList<>();
        CountDownLatch firstEntered = new CountDownLatch(1);
        CountDownLatch releaseFirst = new CountDownLatch(1);
        CountDownLatch latestObserved = new CountDownLatch(1);
        C0410LatestTaskLane lane = new C0410LatestTaskLane(executor, () -> {
            int call = calls.incrementAndGet();
            if (call == 1) {
                firstEntered.countDown();
                await(releaseFirst);
            }
            int value = currentValue.get();
            synchronized (observed) {
                observed.add(value);
            }
            if (value == requestCount) latestObserved.countDown();
        }, error -> { throw new AssertionError("unexpected lane error", error); });

        check(lane.request() == C0410LatestTaskLane.Admission.SCHEDULED,
                "첫 요청은 drain을 예약해야 합니다");
        check(firstEntered.await(5, TimeUnit.SECONDS), "첫 작업이 시작되지 않았습니다");
        for (int value = 1; value <= requestCount; value++) {
            currentValue.set(value);
            check(lane.request() == C0410LatestTaskLane.Admission.COALESCED,
                    "실행 중 입력은 같은 lane에 합쳐야 합니다");
            check(executor.getQueue().size() <= 1, "대기 drain이 1개를 넘었습니다");
        }
        releaseFirst.countDown();
        check(latestObserved.await(5, TimeUnit.SECONDS), "최신 입력이 실행되지 않았습니다");
        awaitIdle(lane);
        check(calls.get() == 2, "100,000개 입력은 실행 중 1회와 최신 drain 1회로 합쳐야 합니다");
        check(observed.get(observed.size() - 1) == requestCount,
                "마지막 실행은 가장 최신 값을 읽어야 합니다");
        check(lane.snapshot().coalesced() == requestCount,
                "coalescing 계수에 누락이 있습니다");
        lane.close();
        executor.shutdown();
        check(executor.awaitTermination(5, TimeUnit.SECONDS), "시험 executor가 종료되지 않았습니다");
    }

    private static void testCloseDropsPendingWork() throws Exception {
        ThreadPoolExecutor executor = singleThreadExecutor();
        AtomicInteger calls = new AtomicInteger();
        CountDownLatch entered = new CountDownLatch(1);
        CountDownLatch release = new CountDownLatch(1);
        C0410LatestTaskLane lane = new C0410LatestTaskLane(executor, () -> {
            calls.incrementAndGet();
            entered.countDown();
            await(release);
        }, error -> { throw new AssertionError("unexpected lane error", error); });
        lane.request();
        check(entered.await(5, TimeUnit.SECONDS), "종료 시험 작업이 시작되지 않았습니다");
        for (int index = 0; index < 10; index++) lane.request();
        lane.close();
        release.countDown();
        awaitIdle(lane);
        check(calls.get() == 1, "close 뒤 pending 작업을 실행했습니다");
        check(lane.request() == C0410LatestTaskLane.Admission.CLOSED,
                "close 뒤 새 요청을 거부하지 않았습니다");
        executor.shutdown();
        check(executor.awaitTermination(5, TimeUnit.SECONDS), "종료 시험 executor가 끝나지 않았습니다");
    }

    private static void testConsumerFailureDoesNotWedgeLane() throws Exception {
        ThreadPoolExecutor executor = singleThreadExecutor();
        AtomicInteger calls = new AtomicInteger();
        AtomicInteger failures = new AtomicInteger();
        CountDownLatch firstFailure = new CountDownLatch(1);
        CountDownLatch secondSuccess = new CountDownLatch(1);
        C0410LatestTaskLane lane = new C0410LatestTaskLane(executor, () -> {
            if (calls.incrementAndGet() == 1) throw new IllegalStateException("injected consumer error");
            secondSuccess.countDown();
        }, error -> {
            failures.incrementAndGet();
            firstFailure.countDown();
        });
        lane.request();
        check(firstFailure.await(5, TimeUnit.SECONDS), "consumer 오류가 보고되지 않았습니다");
        check(lane.request() == C0410LatestTaskLane.Admission.SCHEDULED,
                "consumer 오류 뒤 lane을 다시 예약할 수 없습니다");
        check(secondSuccess.await(5, TimeUnit.SECONDS), "consumer 오류 뒤 작업이 회복되지 않았습니다");
        awaitIdle(lane);
        check(failures.get() == 1 && calls.get() == 2, "consumer 오류 회복 기록이 잘못됐습니다");
        lane.close();
        executor.shutdown();
        check(executor.awaitTermination(5, TimeUnit.SECONDS), "오류 시험 executor가 끝나지 않았습니다");
    }

    private static void testExecutorRejectionCanRecover() {
        AtomicBoolean accept = new AtomicBoolean(false);
        AtomicInteger calls = new AtomicInteger();
        AtomicInteger failures = new AtomicInteger();
        Executor executor = work -> {
            if (!accept.get()) throw new RejectedExecutionException("injected rejection");
            work.run();
        };
        C0410LatestTaskLane lane = new C0410LatestTaskLane(
                executor, calls::incrementAndGet, error -> failures.incrementAndGet());
        check(lane.request() == C0410LatestTaskLane.Admission.REJECTED,
                "executor 거부를 admission 결과로 전달하지 않았습니다");
        check(!lane.snapshot().scheduled(), "executor 거부 뒤 lane이 예약 상태에 고착됐습니다");
        accept.set(true);
        check(lane.request() == C0410LatestTaskLane.Admission.SCHEDULED,
                "executor 거부 후 새 요청을 받지 못했습니다");
        check(calls.get() == 1 && failures.get() == 1, "executor 복구 계수가 잘못됐습니다");
        lane.close();
    }

    private static ThreadPoolExecutor singleThreadExecutor() {
        return new ThreadPoolExecutor(
                1, 1, 0, TimeUnit.MILLISECONDS, new ArrayBlockingQueue<>(1));
    }

    private static void await(CountDownLatch latch) {
        try {
            if (!latch.await(5, TimeUnit.SECONDS)) throw new AssertionError("latch timed out");
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            throw new AssertionError("latch interrupted", error);
        }
    }

    private static void awaitIdle(C0410LatestTaskLane lane) throws InterruptedException {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
        while (lane.snapshot().scheduled() && System.nanoTime() < deadline) {
            Thread.sleep(1);
        }
        check(!lane.snapshot().scheduled(), "lane가 idle로 돌아오지 않았습니다");
    }

    private static void check(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }
}
