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


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
APP_SUPPORT = Path.home() / "Library" / "Application Support"
PREVIEW_ROOT = APP_SUPPORT / "SpinonPreview"
BACKUP_PARENT = APP_SUPPORT / "SpinonPreviewBackups"
BACKUP_TASK_ROOT = BACKUP_PARENT / "roadmap-tailnet-sync-pr696-2026-10-27"
BACKUP_ROOT = BACKUP_TASK_ROOT / "pass01"
TARGET = PREVIEW_ROOT / "roadmap.html"
SERVER_SOURCE = PREVIEW_ROOT / "tailnet-preview-server.mjs"
SOURCE = ROOT / "spec/public/roadmap.html"
LOCAL_URL = "http://127.0.0.1:8768/spinon/roadmap.html"
TAILNET_URL = "https://macstudio.tailed42f2.ts.net/spinon/roadmap.html"
EXPECTED_MAIN = "d019db91d9f592af99d9cfed22bcd1b1600f1445"
EXPECTED_PREVIOUS_SHA256 = "aa46929adb6856c3a45ed7aa04f493563917fbcf25d05f39f6ee9f9ecd1c8cd8"
EXPECTED_SOURCE_SHA256 = "7f8091cb06785e7c93c4c86fe17c2156a904251c7e01b24e8fe9599dc654ce64"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_text(*args: str) -> str:
    return subprocess.check_output(args, text=True)


def safe_directory(path: Path) -> None:
    info = path.lstat()
    assert stat.S_ISDIR(info.st_mode), f"not a real directory: {path}"
    assert not path.is_symlink(), f"unexpected symlink: {path}"
    assert path.resolve() == path, f"directory resolves elsewhere: {path}"


def http_read(url: str) -> dict[str, object]:
    request = urllib.request.Request(
        url,
        headers={"Cache-Control": "no-cache", "Pragma": "no-cache", "Connection": "close"},
    )
    with urllib.request.urlopen(request, timeout=15) as response:
        data = response.read()
        return {
            "status": response.status,
            "bytes": len(data),
            "sha256": sha256(data),
            "body": data,
        }


def public_response(value: dict[str, object]) -> dict[str, object]:
    return {key: value[key] for key in ("status", "bytes", "sha256")}


def write_json_exclusive(path: Path, value: dict[str, object]) -> None:
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def listener_pid() -> int:
    values = run_text("lsof", "-nP", "-t", "-iTCP:8768", "-sTCP:LISTEN")
    pids = sorted({int(value) for value in values.split()})
    assert len(pids) == 1, f"expected exactly one preview listener, got {pids}"
    return pids[0]


def tcp_rows(pid: int) -> list[str]:
    output = run_text("lsof", "-nP", "-a", "-p", str(pid), "-iTCP")
    return [line for line in output.splitlines()[1:] if line.strip()]


def wait_listener_only(pid: int) -> list[str]:
    for _ in range(150):
        rows = tcp_rows(pid)
        if len(rows) == 1 and "(LISTEN)" in rows[0] and "->" not in rows[0]:
            return rows
        time.sleep(0.1)
    raise AssertionError(f"preview process has non-listener TCP connections: {tcp_rows(pid)}")


def main() -> None:
    preflight_path = EVIDENCE / "preflight.json"
    result_path = EVIDENCE / "sync-result.json"
    for path in (preflight_path, result_path):
        assert not path.exists(), f"refusing to overwrite evidence: {path}"

    assert run_text("git", "rev-parse", "origin/main").strip() == EXPECTED_MAIN
    source = SOURCE.read_bytes()
    merged_source = run_text("git", "show", "origin/main:spec/public/roadmap.html").encode()
    assert source == merged_source, "worktree roadmap differs from merged origin/main"
    assert sha256(source) == EXPECTED_SOURCE_SHA256
    source_text = source.decode("utf-8")
    assert "pull/695" in source_text
    assert "d6dd9fd15d2897804142e6bb119149a633952a9b" in source_text
    assert "72/213" in source_text and "C11.4는 미완료" in source_text

    tailscale = json.loads(run_text("tailscale", "status", "--json"))
    assert tailscale.get("BackendState") == "Running"
    safe_directory(PREVIEW_ROOT)
    safe_directory(BACKUP_PARENT)
    assert not BACKUP_TASK_ROOT.exists(), f"backup path already exists: {BACKUP_TASK_ROOT}"
    assert not BACKUP_TASK_ROOT.is_symlink()

    target_info = TARGET.lstat()
    assert stat.S_ISREG(target_info.st_mode) and not TARGET.is_symlink()
    previous = TARGET.read_bytes()
    assert sha256(previous) == EXPECTED_PREVIOUS_SHA256
    server_source = SERVER_SOURCE.read_text(encoding="utf-8")
    assert "fs.stat(file," in server_source and "fs.createReadStream(file)" in server_source

    pid = listener_pid()
    rows_before = wait_listener_only(pid)
    server_command = run_text("ps", "-p", str(pid), "-o", "command=").strip()
    assert "tailnet-preview-server.mjs" in server_command, server_command
    local_before = http_read(LOCAL_URL)
    tailnet_before = http_read(TAILNET_URL)
    for label, response in (("local", local_before), ("Tailnet", tailnet_before)):
        assert response["status"] == 200, f"{label} route unavailable before sync"
        assert response["sha256"] == EXPECTED_PREVIOUS_SHA256, f"{label} route changed before sync"

    handles = subprocess.run(
        ["lsof", "-nP", "--", str(TARGET)],
        text=True,
        capture_output=True,
        check=False,
    )
    assert handles.returncode in (0, 1), handles.stderr
    handle_rows = [line for line in handles.stdout.splitlines()[1:] if line.strip()]
    parsed_handles = []
    for line in handle_rows:
        columns = line.split()
        assert len(columns) >= 4
        assert int(columns[1]) == pid, f"non-preview process has roadmap open: {line}"
        assert columns[3].endswith("r"), f"non-read roadmap handle: {line}"
        parsed_handles.append({"command": columns[0], "pid": int(columns[1]), "descriptor": columns[3]})

    preflight = {
        "sourceHead": EXPECTED_MAIN,
        "sourcePath": "spec/public/roadmap.html",
        "sourceBytes": len(source),
        "sourceSha256": EXPECTED_SOURCE_SHA256,
        "tailscaleBackendState": tailscale["BackendState"],
        "tailnetHostName": tailscale.get("Self", {}).get("HostName"),
        "previewRoot": str(PREVIEW_ROOT),
        "previewPid": pid,
        "previewCommand": server_command,
        "previewServerRestarted": False,
        "targetPath": str(TARGET),
        "targetWasRegularFile": True,
        "previousBytes": len(previous),
        "previousSha256": EXPECTED_PREVIOUS_SHA256,
        "targetReadHandles": parsed_handles,
        "activeClientConnections": 0,
        "listenerRowsBefore": rows_before,
        "localRouteBefore": public_response(local_before),
        "tailnetRouteBefore": public_response(tailnet_before),
        "backupRoot": str(BACKUP_ROOT),
        "backupRootWasAbsent": True,
        "selectedRouteCount": 1,
    }
    write_json_exclusive(preflight_path, preflight)

    BACKUP_TASK_ROOT.mkdir()
    BACKUP_ROOT.mkdir()
    backup_path = BACKUP_ROOT / "roadmap.html"
    with backup_path.open("xb") as backup:
        backup.write(previous)
        backup.flush()
        os.fsync(backup.fileno())
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
    for label, response in (("local", local_after), ("Tailnet", tailnet_after)):
        assert response["status"] == 200, f"{label} route unavailable after sync"
        assert response["bytes"] == len(source), f"{label} route byte count mismatch"
        assert response["sha256"] == EXPECTED_SOURCE_SHA256, f"{label} route content mismatch"
    rows_after = wait_listener_only(pid)
    assert listener_pid() == pid, "preview listener changed during sync"

    result = {
        "sourceHead": EXPECTED_MAIN,
        "sourcePath": "spec/public/roadmap.html",
        "sourceBytes": len(source),
        "sourceSha256": EXPECTED_SOURCE_SHA256,
        "targetPath": str(TARGET),
        "backupPath": str(backup_path),
        "backupBytes": len(backup_bytes),
        "backupSha256": sha256(backup_bytes),
        "replacement": "same-directory atomic rename after byte-exact backup",
        "replacedRoutes": ["roadmap.html"],
        "otherRoutesChanged": False,
        "filesDeleted": False,
        "previewServerRestarted": False,
        "previewPidBeforeAndAfter": pid,
        "listenerRowsAfter": rows_after,
        "localRouteAfter": public_response(local_after),
        "tailnetRouteAfter": public_response(tailnet_after),
        "allRoutesHttp200": True,
        "allRouteHashesMatch": True,
    }
    write_json_exclusive(result_path, result)
    print(json.dumps({"preflight": preflight, "result": result}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
