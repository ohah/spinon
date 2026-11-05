#!/usr/bin/env python3
"""Build and verify the Root HTML aspect-ratio min/max fixture on both simulators."""

from __future__ import annotations

import hashlib
import re
import subprocess
import time
import zipfile
from pathlib import Path


EVIDENCE = Path(__file__).resolve().parent
ROOT = EVIDENCE.parents[3]
ANDROID_SERIAL = "emulator-5554"
IOS_UDID = "9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
V8_DIR = ROOT / "build/c11-v8-rebuild/v8-source/v8"
APK = ROOT / "platforms/android/app/build/outputs/apk/debug/app-debug.apk"
IOS_APP = ROOT / "build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
INDEX = ROOT / "examples/bootstrap/index.html"
ROOT_APP = ROOT / "examples/bootstrap/root-app.js"
TEMP_ROOT_APP = EVIDENCE / "temporary-root-app.js"
ORIGINAL_INDEX_SHA256 = "e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9"
ORIGINAL_ROOT_APP_SHA256 = "b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32"
FIXTURES = (
    (
        "aspect-ratio-min-max",
        ROOT
        / "tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-min-max-precomparison.html",
    ),
)
RUN_ID = "c11rmx-20261105-r3"


def digest(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def verify_root_app_bundle(content: bytes, description: str) -> None:
    required_markers = (
        b"document.documentElement",
        b"expected document tree",
        b"data-runtime",
        b"ready",
    )
    missing = [marker.decode() for marker in required_markers if marker not in content]
    if missing:
        raise RuntimeError(f"{description} is missing Root HTML verifier markers: {missing}")


def invoke(command: list[str], log_path: Path, timeout: int = 1800) -> str:
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        timeout=timeout,
    )
    log_path.parent.mkdir(parents=True, exist_ok=True)
    log_path.write_text(result.stdout, encoding="utf-8")
    if result.returncode != 0:
        raise RuntimeError(
            f"command failed ({result.returncode}): {' '.join(command)}; see {log_path}"
        )
    return result.stdout


def adb(*args: str) -> list[str]:
    return ["adb", "-s", ANDROID_SERIAL, *args]


def wait_for_android_draw(pid: str) -> str:
    command = adb("logcat", "-d", "-v", "time", f"--pid={pid}")
    for _ in range(60):
        result = subprocess.run(
            command,
            cwd=ROOT,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            timeout=30,
        )
        if result.returncode != 0:
            raise RuntimeError(f"Android logcat failed: {result.stdout[-1000:]}")
        if (
            "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass" in result.stdout
            and re.search(r"SPINON_C0410_DRAW.*status=0 presented", result.stdout)
            and f"SPINON_C111_ROOT_HTML_MINMAX_PIXELCOPY_SUMMARY status=" in result.stdout
        ):
            return result.stdout
        time.sleep(0.5)
    raise RuntimeError("Android app did not reach Root HTML boot and draw markers")


def wait_for_ios_draw(pid: int) -> str:
    command = [
        "xcrun",
        "simctl",
        "spawn",
        IOS_UDID,
        "log",
        "show",
        "--last",
        "2m",
        "--predicate",
        f'processIdentifier == {pid}',
        "--style",
        "compact",
    ]
    for _ in range(60):
        result = subprocess.run(
            command,
            cwd=ROOT,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            timeout=30,
        )
        if result.returncode != 0:
            raise RuntimeError(f"iOS log show failed: {result.stdout[-1000:]}")
        if (
            "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass" in result.stdout
            and re.search(r"SPINON_C0410_DRAW.*status=0 presented", result.stdout)
            and f"SPINON_C111RMX_METAL_PIXEL_SUMMARY status=" in result.stdout
        ):
            return result.stdout
        time.sleep(0.5)
    raise RuntimeError("iOS app did not reach Root HTML boot and draw markers")


def capture_android_screenshot(path: Path) -> None:
    result = subprocess.run(
        adb("exec-out", "screencap", "-p"),
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=30,
    )
    if result.returncode != 0 or not result.stdout.startswith(b"\x89PNG\r\n\x1a\n"):
        raise RuntimeError(f"Android screencap failed: {result.stderr.decode(errors='replace')}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(result.stdout)


def capture_ios_screenshot(path: Path, log_path: Path) -> None:
    invoke(["xcrun", "simctl", "io", IOS_UDID, "screenshot", str(path)], log_path, 60)
    if not path.read_bytes().startswith(b"\x89PNG\r\n\x1a\n"):
        raise RuntimeError(f"iOS screenshot is not a PNG: {path}")


def capture_android_pixelcopy(path: Path) -> None:
    result = subprocess.run(
        adb(
            "exec-out",
            "run-as",
            "dev.spinon.bootstrap",
            "cat",
            f"/data/user/0/dev.spinon.bootstrap/cache/c11-root-html-grid-minmax-pixelcopy/{RUN_ID}.png",
        ),
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=30,
    )
    if result.returncode != 0 or not result.stdout.startswith(b"\x89PNG\r\n\x1a\n"):
        raise RuntimeError(
            f"Android PixelCopy PNG export failed: {result.stderr.decode(errors='replace')}"
        )
    path.write_bytes(result.stdout)


def record_artifact_hashes(directory: Path, fixture: Path, artifact: Path) -> None:
    lines = [
        f"fixture_sha256={digest(fixture.read_bytes())}",
        f"root_app_js_sha256={digest(TEMP_ROOT_APP.read_bytes())}",
        f"artifact_sha256={digest(artifact.read_bytes())}",
    ]
    (directory / "artifact-hashes.txt").write_text("\n".join(lines) + "\n")


def main() -> None:
    original_index = INDEX.read_bytes()
    original_root_app = ROOT_APP.read_bytes()
    if digest(original_index) != ORIGINAL_INDEX_SHA256:
        raise RuntimeError("examples/bootstrap/index.html changed before simulator run")
    if digest(original_root_app) != ORIGINAL_ROOT_APP_SHA256:
        raise RuntimeError("examples/bootstrap/root-app.js changed before simulator run")
    if not V8_DIR.is_dir():
        raise RuntimeError(f"pinned V8 directory is missing: {V8_DIR}")
    if TEMP_ROOT_APP.read_bytes() == original_root_app:
        raise RuntimeError("temporary Root HTML verifier unexpectedly equals the app source")

    android_started = False
    ios_started = False
    try:
        android_state = subprocess.run(
            adb("get-state"), capture_output=True, text=True, check=False, timeout=30
        )
        if android_state.returncode != 0 or android_state.stdout.strip() != "device":
            raise RuntimeError(f"selected Android AVD is not ready: {android_state.stdout}")
        emulator = subprocess.run(
            adb("shell", "getprop", "ro.kernel.qemu"),
            capture_output=True,
            text=True,
            check=False,
            timeout=30,
        )
        if emulator.returncode != 0 or emulator.stdout.strip() != "1":
            raise RuntimeError("selected Android target is not an emulator")
        ios_state = subprocess.run(
            ["xcrun", "simctl", "list", "devices", "booted", "-j"],
            capture_output=True,
            text=True,
            check=True,
            timeout=30,
        )
        if IOS_UDID not in ios_state.stdout:
            raise RuntimeError(f"selected iOS Simulator is not booted: {IOS_UDID}")

        for fixture_name, fixture in FIXTURES:
            fixture_bytes = fixture.read_bytes()
            fixture_hash = digest(fixture_bytes)
            INDEX.write_bytes(fixture_bytes)
            ROOT_APP.write_bytes(TEMP_ROOT_APP.read_bytes())
            for platform in ("android", "ios-sim"):
                label = "ios" if platform == "ios-sim" else "android"
                output_dir = EVIDENCE / label
                if fixture_name != "main-14-scenes":
                    output_dir /= fixture_name
                output_dir.mkdir(parents=True, exist_ok=True)
                build_log = output_dir / "build.log"
                invoke(
                    [
                        "mise",
                        "exec",
                        "--",
                        "env",
                        f"SPINON_V8_DIR={V8_DIR}",
                        "SPINON_ENABLE_C04_RUNTIME_GPU=1",
                        "SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1",
                        "SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1",
                        "bun",
                        "run",
                        f"build:{platform}",
                    ],
                    build_log,
                )

                if platform == "android":
                    with zipfile.ZipFile(APK) as apk:
                        if apk.read("assets/index.html") != fixture_bytes:
                            raise RuntimeError("APK index.html bytes do not match the fixture")
                        verify_root_app_bundle(
                            apk.read("assets/root-app.js"), "APK root-app.js"
                        )
                    record_artifact_hashes(output_dir, fixture, APK)
                    invoke(
                        adb("install", "-r", str(APK)), output_dir / "install.log", 120
                    )
                    android_started = True
                    invoke(
                        adb("logcat", "-c"), output_dir / "logcat-clear.log", 30
                    )
                    invoke(
                        adb(
                            "shell",
                            "am",
                            "start",
                            "-S",
                            "-W",
                            "-n",
                            "dev.spinon.bootstrap/.MainActivity",
                            "--ez",
                            "spinon_root_html_grid_minmax_pixel_capture",
                            "true",
                            "--es",
                            "spinon_root_html_grid_minmax_pixel_capture_run_id",
                            RUN_ID,
                        ),
                        output_dir / "launch.log",
                        60,
                    )
                    pid_result = subprocess.run(
                        adb("shell", "pidof", "dev.spinon.bootstrap"),
                        capture_output=True,
                        text=True,
                        check=True,
                        timeout=30,
                    )
                    pid = pid_result.stdout.strip().split()[0]
                    if not pid.isdecimal():
                        raise RuntimeError(f"invalid Android app pid: {pid!r}")
                    run_log = wait_for_android_draw(pid)
                    (output_dir / "run.log").write_text(run_log, encoding="utf-8")
                    if f"SPINON_C111_ROOT_HTML_MINMAX_PIXELCOPY_SUMMARY status=matched run_id={RUN_ID}" not in run_log:
                        raise RuntimeError(
                            f"Android PixelCopy samples did not exact-match; see {output_dir / 'run.log'}"
                        )
                    capture_android_screenshot(output_dir / "screenshot.png")
                    capture_android_pixelcopy(output_dir / "pixelcopy.png")
                    invoke(
                        adb("shell", "am", "force-stop", "dev.spinon.bootstrap"),
                        output_dir / "stop.log",
                        30,
                    )
                    android_started = False
                else:
                    embedded_index = IOS_APP / "index.html"
                    embedded_root_app = IOS_APP / "root-app.js"
                    if embedded_index.read_bytes() != fixture_bytes:
                        raise RuntimeError("iOS app index.html bytes do not match the fixture")
                    verify_root_app_bundle(
                        embedded_root_app.read_bytes(), "iOS app root-app.js"
                    )
                    record_artifact_hashes(output_dir, fixture, IOS_APP / "SpinonBootstrap")
                    invoke(
                        ["xcrun", "simctl", "install", IOS_UDID, str(IOS_APP)],
                        output_dir / "install.log",
                        120,
                    )
                    ios_started = True
                    launch = invoke(
                        [
                            "xcrun",
                            "simctl",
                            "launch",
                            "--terminate-running-process",
                            IOS_UDID,
                            "dev.spinon.bootstrap",
                            f"--spinon-root-html-grid-minmax-metal-surface-pixels={RUN_ID}",
                        ],
                        output_dir / "launch.log",
                        60,
                    )
                    match = re.search(r":\s*(\d+)\s*$", launch.strip())
                    if match is None:
                        raise RuntimeError(f"could not identify launched iOS PID: {launch!r}")
                    run_log = wait_for_ios_draw(int(match.group(1)))
                    (output_dir / "run.log").write_text(run_log, encoding="utf-8")
                    if f"SPINON_C111RMX_METAL_PIXEL_SUMMARY status=matched run_id={RUN_ID}" not in run_log:
                        raise RuntimeError(
                            f"iOS Metal samples did not exact-match; see {output_dir / 'run.log'}"
                        )
                    capture_ios_screenshot(
                        output_dir / "screenshot.png", output_dir / "screenshot.log"
                    )
                    invoke(
                        ["xcrun", "simctl", "terminate", IOS_UDID, "dev.spinon.bootstrap"],
                        output_dir / "stop.log",
                        30,
                    )
                    ios_started = False
                print(
                    f"{fixture_name}/{label}: PASS, fixture sha256={fixture_hash}, "
                    f"screenshot={output_dir / 'screenshot.png'}"
                )
    finally:
        if android_started:
            subprocess.run(
                adb("shell", "am", "force-stop", "dev.spinon.bootstrap"),
                cwd=ROOT,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                timeout=30,
            )
        if ios_started:
            subprocess.run(
                ["xcrun", "simctl", "terminate", IOS_UDID, "dev.spinon.bootstrap"],
                cwd=ROOT,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                timeout=30,
            )
        INDEX.write_bytes(original_index)
        ROOT_APP.write_bytes(original_root_app)
        if digest(INDEX.read_bytes()) != ORIGINAL_INDEX_SHA256:
            raise RuntimeError("examples/bootstrap/index.html was not restored exactly")
        if digest(ROOT_APP.read_bytes()) != ORIGINAL_ROOT_APP_SHA256:
            raise RuntimeError("examples/bootstrap/root-app.js was not restored exactly")
        print("bootstrap sources restored to their exact original SHA-256 values")


if __name__ == "__main__":
    main()
