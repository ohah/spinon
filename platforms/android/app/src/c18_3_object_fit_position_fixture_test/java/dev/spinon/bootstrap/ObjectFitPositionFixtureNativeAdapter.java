package dev.spinon.bootstrap;

public final class ObjectFitPositionFixtureNativeAdapter {
    private ObjectFitPositionFixtureNativeAdapter() {}

    public static native long createHost();

    public static native byte[] evalFixture(long host);
}
