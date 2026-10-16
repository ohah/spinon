package dev.spinon.bootstrap;

public final class AlphaImageFixtureNativeAdapter {
    private AlphaImageFixtureNativeAdapter() {}

    public static native long createHost();

    public static native byte[] evalFixture(long host);
}
