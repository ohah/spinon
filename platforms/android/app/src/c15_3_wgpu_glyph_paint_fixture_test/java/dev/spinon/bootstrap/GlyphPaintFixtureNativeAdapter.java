package dev.spinon.bootstrap;

public final class GlyphPaintFixtureNativeAdapter {
    private GlyphPaintFixtureNativeAdapter() {}

    public static native long createHost();

    public static native long createHostDocumentTextPaintHost();

    public static native byte[] evalFixture(long host);
}
