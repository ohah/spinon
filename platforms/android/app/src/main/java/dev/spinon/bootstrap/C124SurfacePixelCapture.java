package dev.spinon.bootstrap;

import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.Handler;
import android.view.PixelCopy;
import android.view.SurfaceView;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** C12.4 전용 Android SurfaceView PixelCopy 및 고정 표본 캡처기입니다. */
final class C124SurfacePixelCapture {
    private static final int MAX_ATTEMPTS = 3;

    static final class Sample {
        final String id;
        final double xCssPx;
        final double yCssPx;

        Sample(String id, double xCssPx, double yCssPx) {
            this.id = id;
            this.xCssPx = xCssPx;
            this.yCssPx = yCssPx;
        }
    }

    static final class Pixel {
        final String id;
        final int x;
        final int y;
        final int red;
        final int green;
        final int blue;
        final int alpha;

        Pixel(String id, int x, int y, int color) {
            this.id = id;
            this.x = x;
            this.y = y;
            red = android.graphics.Color.red(color);
            green = android.graphics.Color.green(color);
            blue = android.graphics.Color.blue(color);
            alpha = android.graphics.Color.alpha(color);
        }
    }

    static final class Result {
        final String error;
        final String sha256;
        final int width;
        final int height;
        final int attempts;
        final List<Pixel> pixels;

        private Result(
                String error,
                String sha256,
                int width,
                int height,
                int attempts,
                List<Pixel> pixels
        ) {
            this.error = error;
            this.sha256 = sha256;
            this.width = width;
            this.height = height;
            this.attempts = attempts;
            this.pixels = pixels;
        }

        static Result failure(String error, int attempts) {
            return new Result(error, "", 0, 0, attempts, List.of());
        }
    }

    private C124SurfacePixelCapture() {}

    static void request(
            SurfaceView surfaceView,
            int width,
            int height,
            float density,
            int originXCssPx,
            int originYCssPx,
            String caseId,
            String runId,
            File outputFile,
            List<Sample> samples,
            Handler callbackHandler,
            Executor worker,
            BooleanSupplier sourceIsCurrent,
            Consumer<Result> completion
    ) {
        if (width <= 0 || height <= 0 || !Float.isFinite(density) || density <= 0.0f) {
            completion.accept(Result.failure("잘못된 surface 크기 또는 density", 0));
            return;
        }
        if (!caseId.matches("[a-z0-9-]{1,80}")
                || !runId.matches("[A-Za-z0-9_-]{1,80}")
                || samples.isEmpty()) {
            completion.accept(Result.failure("case/run ID 또는 표본 목록이 올바르지 않음", 0));
            return;
        }
        requestAttempt(surfaceView, width, height, density, originXCssPx, originYCssPx,
                caseId, runId, outputFile, samples, callbackHandler, worker,
                sourceIsCurrent, completion, 1);
    }

    private static void requestAttempt(
            SurfaceView surfaceView,
            int width,
            int height,
            float density,
            int originXCssPx,
            int originYCssPx,
            String caseId,
            String runId,
            File outputFile,
            List<Sample> samples,
            Handler callbackHandler,
            Executor worker,
            BooleanSupplier sourceIsCurrent,
            Consumer<Result> completion,
            int attempt
    ) {
        surfaceView.postOnAnimation(() -> {
            if (!sourceIsCurrent.getAsBoolean()) {
                completion.accept(Result.failure("PixelCopy 요청 전 surface 세대가 바뀜", attempt));
                return;
            }
            Bitmap bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888);
            PixelCopy.request(surfaceView, bitmap, copyResult -> {
                if (copyResult != PixelCopy.SUCCESS) {
                    bitmap.recycle();
                    if (isRetryable(copyResult) && attempt < MAX_ATTEMPTS
                            && sourceIsCurrent.getAsBoolean()) {
                        surfaceView.postDelayed(() -> requestAttempt(surfaceView, width, height,
                                density, originXCssPx, originYCssPx, caseId, runId,
                                outputFile, samples, callbackHandler, worker,
                                sourceIsCurrent, completion, attempt + 1), 50);
                    } else {
                        completion.accept(Result.failure(
                                "PixelCopy 결과=" + copyResult, attempt));
                    }
                    return;
                }
                if (!sourceIsCurrent.getAsBoolean()) {
                    bitmap.recycle();
                    completion.accept(Result.failure("PixelCopy 중 surface 세대가 바뀜", attempt));
                    return;
                }
                try {
                    worker.execute(() -> completion.accept(saveAndReadSamples(bitmap,
                            width, height, density, originXCssPx, originYCssPx,
                            outputFile, samples, attempt)));
                } catch (RuntimeException error) {
                    bitmap.recycle();
                    completion.accept(Result.failure(
                            "PixelCopy 결과 작업 queue 실패: " + error.getClass().getSimpleName(),
                            attempt));
                }
            }, callbackHandler);
        });
    }

    private static boolean isRetryable(int result) {
        return result == PixelCopy.ERROR_SOURCE_NO_DATA || result == PixelCopy.ERROR_TIMEOUT;
    }

    private static Result saveAndReadSamples(
            Bitmap bitmap,
            int expectedWidth,
            int expectedHeight,
            float density,
            int originXCssPx,
            int originYCssPx,
            File outputFile,
            List<Sample> samples,
            int attempts
    ) {
        Bitmap decoded = null;
        try {
            if (bitmap.getWidth() != expectedWidth || bitmap.getHeight() != expectedHeight) {
                return Result.failure("PixelCopy bitmap 크기가 surface와 다름", attempts);
            }
            File parent = outputFile.getParentFile();
            if (parent == null || (!parent.isDirectory() && !parent.mkdirs())) {
                return Result.failure("PixelCopy evidence directory 생성 실패", attempts);
            }
            if (outputFile.exists()) {
                return Result.failure("동일 run ID의 기존 PixelCopy PNG가 있음", attempts);
            }
            try (FileOutputStream output = new FileOutputStream(outputFile)) {
                if (!bitmap.compress(Bitmap.CompressFormat.PNG, 100, output)) {
                    return Result.failure("PixelCopy PNG 인코딩 실패", attempts);
                }
                output.flush();
                output.getFD().sync();
            }
            byte[] pngBytes = java.nio.file.Files.readAllBytes(outputFile.toPath());
            String sha256 = sha256(pngBytes);
            decoded = BitmapFactory.decodeFile(outputFile.getAbsolutePath());
            if (decoded == null || decoded.getWidth() != expectedWidth
                    || decoded.getHeight() != expectedHeight) {
                return Result.failure("저장 PixelCopy PNG 재해석 또는 크기 검증 실패", attempts);
            }
            List<Pixel> pixels = new ArrayList<>(samples.size());
            for (Sample sample : samples) {
                int x = (int) Math.floor((originXCssPx + sample.xCssPx + 0.5) * density);
                int y = (int) Math.floor((originYCssPx + sample.yCssPx + 0.5) * density);
                if (x < 0 || y < 0 || x >= decoded.getWidth() || y >= decoded.getHeight()) {
                    return Result.failure("고정 표본이 PixelCopy bitmap 범위를 벗어남: "
                            + sample.id, attempts);
                }
                pixels.add(new Pixel(sample.id, x, y, decoded.getPixel(x, y)));
            }
            return new Result("", sha256, decoded.getWidth(), decoded.getHeight(), attempts, pixels);
        } catch (IOException | NoSuchAlgorithmException | RuntimeException error) {
            return Result.failure("PixelCopy evidence 처리 실패: "
                    + error.getClass().getSimpleName(), attempts);
        } finally {
            if (decoded != null) decoded.recycle();
            if (!bitmap.isRecycled()) bitmap.recycle();
        }
    }

    private static String sha256(byte[] bytes) throws NoSuchAlgorithmException {
        byte[] digest = MessageDigest.getInstance("SHA-256").digest(bytes);
        StringBuilder hex = new StringBuilder(digest.length * 2);
        for (byte value : digest) hex.append(String.format("%02x", value & 0xff));
        return hex.toString();
    }
}
