package dev.spinon.bootstrap;

public final class AsyncImageFixtureNativeAdapter {
    private AsyncImageFixtureNativeAdapter() {}

    public static native long createHost();

    public static native byte[] evalFixture(long host);
}
