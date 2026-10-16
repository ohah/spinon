package dev.spinon.bootstrap;

public final class ImagePaintFixtureNativeAdapter {
    private ImagePaintFixtureNativeAdapter() {}

    public static native long createHost();

    public static native byte[] evalFixture(long host);
}
