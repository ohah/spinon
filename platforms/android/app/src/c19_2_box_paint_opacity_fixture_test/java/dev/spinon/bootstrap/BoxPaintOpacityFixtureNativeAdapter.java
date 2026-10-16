package dev.spinon.bootstrap;

public final class BoxPaintOpacityFixtureNativeAdapter {
    private BoxPaintOpacityFixtureNativeAdapter() {}

    public static native long createHost();

    public static native byte[] evalFixture(long host);
}
