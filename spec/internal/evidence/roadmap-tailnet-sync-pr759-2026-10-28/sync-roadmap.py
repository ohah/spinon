from __future__ import annotations

import hashlib
import json
import os
import stat
import subprocess
import time
import urllib.request
import uuid
from pathlib import Path

EVIDENCE = Path(__file__).resolve().parent
PREVIEW_ROOT = Path.home() / "Library" / "Application Support" / "SpinonPreview"
BACKUP_PARENT = Path.home() / "Library" / "Application Support" / "SpinonPreviewBackups"
BACKUP_TASK_ROOT = BACKUP_PARENT / "roadmap-tailnet-sync-pr759-2026-10-28"
BACKUP_ROOT = BACKUP_TASK_ROOT / "pass01"
TARGET = PREVIEW_ROOT / "roadmap.html"
SOURCE_PATH = "spec/public/roadmap.html"
SOURCE = Path(__file__).resolve().parents[4] / SOURCE_PATH
LOCAL_URL = "http://127.0.0.1:8768/spinon/roadmap.html"
TAILNET_URL = "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html"
EXPECTED_MAIN = "3210face8b77dd921c403188fe73f2c6993c53c2"
EXPECTED_SOURCE_SHA256 = "d24e7dfc498cd33ecdaeedee897b902f826103b9d9b1688ed50ccd1cfa4f070c"
EXPECTED_PREVIOUS_SHA256 = "faff974c222573d5c7ae8683a3a2f2072540dd2723b2d9263a39e38736ab26a3"
EXPECTED_PREVIEW_PID = 28040


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_text(*args: str) -> str:
    return subprocess.check_output(args, text=True)


def safe_directory(directory: Path) -> None:
    info = directory.lstat()
    assert stat.S_ISDIR(info.st_mode), f"not a real directory: {directory}"
    assert not directory.is_symlink(), f"unexpected symlink: {directory}"
    assert directory.resolve() == directory, f"directory resolves elsewhere: {directory}"


def http_read(url: str) -> dict[str, object]:
    request = urllib.request.Request(
        url,
        headers={"Cache-Control": "no-cache", "Pragma": "no-cache", "Connection": "close"},
    )
    with urllib.request.urlopen(request, timeout=20) as response:
        body = response.read()
        return {"status": response.status, "bytes": len(body), "sha256": sha256(body)}


def listener_pid() -> int:
    values = run_text("lsof", "-nP", "-t", "-iTCP:8768", "-sTCP:LISTEN")
    pids = sorted({int(value) for value in values.split()})
    assert pids == [EXPECTED_PREVIEW_PID], f"unexpected preview listeners: {pids}"
    return pids[0]


def tcp_rows(pid: int) -> list[str]:
    output = run_text("lsof", "-nP", "-a", "-p", str(pid), "-iTCP")
    return [line for line in output.splitlines()[1:] if line.strip()]


def open_target_handles() -> list[str]:
    result = subprocess.run(
        ["lsof", "-nP", "--", str(TARGET)],
        text=True,
        capture_output=True,
        check=False,
    )
    assert result.returncode in (0, 1), result.stderr
    return [line for line in result.stdout.splitlines()[1:] if line.strip()]


def wait_for_quiescence(pid: int) -> tuple[list[str], list[str], int]:
    for attempt in range(150):
        tcp = tcp_rows(pid)
        handles = open_target_handles()
        listener_only = len(tcp) == 1 and "(LISTEN)" in tcp[0] and "->" not in tcp[0]
        if listener_only and not handles:
            return tcp, handles, attempt
        time.sleep(0.1)
    raise AssertionError(
        f"preview did not quiesce: tcp={tcp_rows(pid)}, targetHandles={open_target_handles()}"
    )


def write_json_exclusive(destination: Path, value: dict[str, object]) -> None:
    with destination.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def main() -> None:
    preflight_path = EVIDENCE / "preflight.json"
    result_path = EVIDENCE / "sync-result.json"
    assert not preflight_path.exists(), f"refusing to overwrite {preflight_path}"
    assert not result_path.exists(), f"refusing to overwrite {result_path}"
    assert run_text("git", "rev-parse", "origin/main").strip() == EXPECTED_MAIN

    source = SOURCE.read_bytes()
    merged_source = run_text("git", "show", f"origin/main:{SOURCE_PATH}").encode()
    assert source == merged_source, "worktree source differs from merged origin/main"
    assert sha256(source) == EXPECTED_SOURCE_SHA256
    source_text = source.decode("utf-8")
    assert "PR #759 Rebase and merge" in source_text
    assert "72/213" in source_text and "C11.4는 미완료" in source_text

    tailscale = json.loads(run_text("tailscale", "status", "--json"))
    assert tailscale.get("BackendState") == "Running"
    safe_directory(PREVIEW_ROOT)
    safe_directory(BACKUP_PARENT)
    assert not BACKUP_TASK_ROOT.exists(), f"refusing to reuse {BACKUP_TASK_ROOT}"
    assert not BACKUP_TASK_ROOT.is_symlink()

    target_info = TARGET.lstat()
    assert stat.S_ISREG(target_info.st_mode) and not TARGET.is_symlink()
    previous = TARGET.read_bytes()
    assert sha256(previous) == EXPECTED_PREVIOUS_SHA256
    server_source = (PREVIEW_ROOT / "tailnet-preview-server.mjs").read_text(encoding="utf-8")
    assert "fs.stat(file," in server_source and "fs.createReadStream(file)" in server_source

    pid = listener_pid()
    assert pid == EXPECTED_PREVIEW_PID
    server_command = run_text("ps", "-p", str(pid), "-o", "command=").strip()
    assert "tailnet-preview-server.mjs" in server_command, server_command
    local_before = http_read(LOCAL_URL)
    tailnet_before = http_read(TAILNET_URL)
    assert local_before["status"] == tailnet_before["status"] == 200
    assert local_before["sha256"] == tailnet_before["sha256"] == EXPECTED_PREVIOUS_SHA256
    tcp_before, handles_before, wait_before = wait_for_quiescence(pid)

    preflight = {
        "sourceHead": EXPECTED_MAIN,
        "sourcePath": SOURCE_PATH,
        "sourceBytes": len(source),
        "sourceSha256": sha256(source),
        "previewRoot": str(PREVIEW_ROOT),
        "previewPid": pid,
        "previewCommand": server_command,
        "previewServerRestarted": False,
        "tailscaleBackendState": tailscale["BackendState"],
        "targetPath": str(TARGET),
        "targetWasRegularFile": True,
        "previousBytes": len(previous),
        "previousSha256": sha256(previous),
        "localRouteBefore": local_before,
        "tailnetRouteBefore": tailnet_before,
        "listenerRowsBefore": tcp_before,
        "targetOpenHandlesBefore": handles_before,
        "quiescenceWaitPolls": wait_before,
        "backupRoot": str(BACKUP_ROOT),
        "backupRootWasAbsent": True,
        "selectedRouteCount": 1,
    }
    write_json_exclusive(preflight_path, preflight)

    BACKUP_TASK_ROOT.mkdir()
    BACKUP_ROOT.mkdir()
    backup_path = BACKUP_ROOT / "roadmap.html"
    with backup_path.open("xb") as stream:
        stream.write(previous)
        stream.flush()
        os.fsync(stream.fileno())
    os.chmod(backup_path, stat.S_IMODE(target_info.st_mode))
    backup_bytes = backup_path.read_bytes()
    assert backup_bytes == previous and sha256(backup_bytes) == EXPECTED_PREVIOUS_SHA256

    temporary = PREVIEW_ROOT / f".roadmap.html.codex-sync-{uuid.uuid4()}.tmp"
    try:
        with temporary.open("xb") as stream:
            stream.write(source)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, stat.S_IMODE(target_info.st_mode))
        os.replace(temporary, TARGET)
        directory_fd = os.open(PREVIEW_ROOT, os.O_RDONLY)
        try:
            os.fsync(directory_fd)
        finally:
            os.close(directory_fd)
    finally:
        if temporary.exists():
            temporary.unlink()

    installed = TARGET.read_bytes()
    assert installed == source and sha256(installed) == EXPECTED_SOURCE_SHA256
    local_after = http_read(LOCAL_URL)
    tailnet_after = http_read(TAILNET_URL)
    assert local_after["status"] == tailnet_after["status"] == 200
    assert local_after["bytes"] == tailnet_after["bytes"] == len(source)
    assert local_after["sha256"] == tailnet_after["sha256"] == EXPECTED_SOURCE_SHA256
    tcp_after, handles_after, wait_after = wait_for_quiescence(pid)
    assert listener_pid() == pid, "preview listener changed during sync"

    result = {
        "sourceHead": EXPECTED_MAIN,
        "sourcePath": SOURCE_PATH,
        "sourceBytes": len(source),
        "sourceSha256": sha256(source),
        "targetPath": str(TARGET),
        "backupPath": str(backup_path),
        "backupBytes": len(backup_bytes),
        "backupSha256": sha256(backup_bytes),
        "replacement": "same-directory atomic replace after byte-exact backup",
        "replacedRoutes": ["roadmap.html"],
        "otherRoutesChanged": False,
        "filesDeleted": False,
        "previewServerRestarted": False,
        "previewPidBeforeAndAfter": pid,
        "localRouteAfter": local_after,
        "tailnetRouteAfter": tailnet_after,
        "listenerRowsAfter": tcp_after,
        "targetOpenHandlesAfter": handles_after,
        "quiescenceWaitPolls": wait_after,
        "allRoutesHttp200": True,
        "allRouteHashesMatch": True,
    }
    write_json_exclusive(result_path, result)
    print(json.dumps({"preflight": preflight, "result": result}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
