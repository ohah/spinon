package dev.spinon.bootstrap;

import java.util.concurrent.Executor;
import java.util.concurrent.RejectedExecutionException;
import java.util.function.Consumer;

/** 최신 상태를 읽는 직렬 작업의 대기량을 하나로 제한합니다. */
final class C0410LatestTaskLane {
    enum Admission {
        SCHEDULED,
        COALESCED,
        CLOSED,
        REJECTED,
    }

    static final class Snapshot {
        private final long requests;
        private final long coalesced;
        private final long completed;
        private final long failures;
        private final boolean scheduled;
        private final boolean dirty;
        private final boolean closed;

        Snapshot(
                long requests,
                long coalesced,
                long completed,
                long failures,
                boolean scheduled,
                boolean dirty,
                boolean closed
        ) {
            this.requests = requests;
            this.coalesced = coalesced;
            this.completed = completed;
            this.failures = failures;
            this.scheduled = scheduled;
            this.dirty = dirty;
            this.closed = closed;
        }

        long requests() { return requests; }
        long coalesced() { return coalesced; }
        long completed() { return completed; }
        long failures() { return failures; }
        boolean scheduled() { return scheduled; }
        boolean dirty() { return dirty; }
        boolean closed() { return closed; }
    }

    private final Object lock = new Object();
    private final Executor executor;
    private final Runnable work;
    private final Consumer<RuntimeException> onFailure;
    private boolean scheduled;
    private boolean dirty;
    private boolean closed;
    private long requests;
    private long coalesced;
    private long completed;
    private long failures;

    C0410LatestTaskLane(
            Executor executor,
            Runnable work,
            Consumer<RuntimeException> onFailure
    ) {
        this.executor = executor;
        this.work = work;
        this.onFailure = onFailure;
    }

    Admission request() {
        synchronized (lock) {
            if (closed) return Admission.CLOSED;
            requests++;
            if (scheduled) {
                dirty = true;
                coalesced++;
                return Admission.COALESCED;
            }
            scheduled = true;
        }
        return dispatchDrain() ? Admission.SCHEDULED : Admission.REJECTED;
    }

    void close() {
        synchronized (lock) {
            closed = true;
            dirty = false;
        }
    }

    Snapshot snapshot() {
        synchronized (lock) {
            return new Snapshot(
                    requests, coalesced, completed, failures, scheduled, dirty, closed);
        }
    }

    private boolean dispatchDrain() {
        try {
            executor.execute(this::runDrain);
            return true;
        } catch (RejectedExecutionException error) {
            synchronized (lock) {
                scheduled = false;
                dirty = false;
                failures++;
            }
            onFailure.accept(error);
            return false;
        }
    }

    private void runDrain() {
        synchronized (lock) {
            if (closed) {
                scheduled = false;
                dirty = false;
                return;
            }
            dirty = false;
        }

        RuntimeException failure = null;
        try {
            work.run();
        } catch (RuntimeException error) {
            failure = error;
        }

        boolean dispatchAgain;
        synchronized (lock) {
            completed++;
            if (failure != null) failures++;
            if (closed || !dirty) {
                scheduled = false;
                dirty = false;
                dispatchAgain = false;
            } else {
                // executor queue에는 이 다음 drain 하나만 넣습니다.
                dirty = false;
                dispatchAgain = true;
            }
        }
        if (dispatchAgain) dispatchDrain();
        if (failure != null) onFailure.accept(failure);
    }
}
