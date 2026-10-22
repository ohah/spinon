#!/usr/bin/env python3
"""전체 RSPress 빌드 전까지 Tailnet 미리보기의 생성 산출물을 갱신한다."""

import hashlib
import json
import os
import re
import shutil
import subprocess
from pathlib import Path


repository = Path.cwd()
preview = Path(os.environ["SPINON_PREVIEW_ROOT"])
backup_root = Path(os.environ["SPINON_PREVIEW_BACKUP_ROOT"]) / "route-bundle-correction-pass49"
site = repository / "build/spinon/c11-4-cyclic-minmax-calc-cross-axis-span-groups-tailnet-site-pass49"
runtime_rel = Path("internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22")
source_readme = repository / "spec" / runtime_rel / "README.md"
source_text = source_readme.read_text(encoding="utf-8")

route_chunks = [
    Path("static/js/async/route-f81cc1f3adc5.9fa16e0f1b.js"),
    Path("static/js/async/route-4a17fa161b97.7ad731f867.js"),
]
search_index_rel = Path("static/search_index.ko.4e0aa793.json")
markdown_rels = [runtime_rel / "README.md", Path("spec") / runtime_rel / "README.md"]
relative_files = route_chunks + [search_index_rel] + markdown_rels


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def assert_unopened(path: Path) -> None:
    result = subprocess.run(["lsof", "-nP", "-t", str(path)], capture_output=True, text=True)
    if result.returncode == 0 and result.stdout.strip():
        raise RuntimeError(f"open file handles for {path}: {result.stdout.strip()}")
    if result.returncode not in (0, 1):
        raise RuntimeError(f"lsof failed for {path}: {result.stderr.strip()}")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one {label}; found {count}")
    return text.replace(old, new, 1)


def patch_route(path: Path) -> bytes:
    text = path.read_text(encoding="utf-8")
    for old, new, label in [
        ("남은-검증", "검증-상태", "outline anchor"),
        ("남은 검증", "검증 상태", "outline title"),
        (
            "Android definite forward/reverse renderer draw and PixelCopy samples, plus a complete 4/4 AVD run.",
            "Android definite forward/reverse 장면의 renderer draw·PixelCopy 표본과 4개 전체 AVD scene 실행.",
            "Android pending check",
        ),
        (
            "Existing C11.4 regression matrix after this exact fixture build.",
            "이 fixture build를 포함한 기존 C11.4 회귀 matrix.",
            "C11.4 regression check",
        ),
        (
            "Implementation/runtime failure review, 20 perspectives",
            "구현·runtime 실패 관점 검토 20개 완료",
            "implementation review label",
        ),
        (
            "PR diff review, 30 perspectives",
            "PR diff 실패 관점 검토 30개 완료",
            "PR review label",
        ),
    ]:
        if old not in text:
            raise RuntimeError(f"missing {label} in {path}")
        text = text.replace(old, new)

    for route_prefix in ("/spec", ""):
        old_separator = (
            ')," and ",(0,n.jsx)(s.a,{href:"'
            + route_prefix
            + "/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/pr-diff-review-30.html\""
        )
        if old_separator in text:
            text = text.replace(old_separator, old_separator.replace(')," and ",', ')," · ",', 1), 1)

    old_avd = (
        '(0,n.jsxs)(s.li,{children:["AVD startup needs host free space above the emulator\'s current preflight requirement. '
        'The threshold was not printed; do not infer an exact minimum from the ",'
        '(0,n.jsx)(s.code,{children:"1.3 GiB"})," observation."]})'
    )
    new_avd = (
        '(0,n.jsxs)(s.li,{children:["AVD 재시작이 필요하다. host free space ",'
        '(0,n.jsx)(s.code,{children:"1.3 GiB"}),"에서 emulator preflight가 거부됐으나 필요 공간의 정확한 기준은 출력되지 않았다. ",'
        '(0,n.jsx)(s.code,{children:"1.3 GiB"}),"를 기준값으로 추정하지 않는다."]})'
    )
    text = replace_once(text, old_avd, new_avd, "AVD limitation")
    if "Implementation/runtime failure review, 20 perspectives" in text:
        raise RuntimeError(f"stale review label remains in {path}")
    return text.encode("utf-8")


def markdown_plain_tail(text: str) -> str:
    section = text.split("## 검증 상태\n", 1)[1]
    lines = []
    for line in section.splitlines():
        if line.startswith("- "):
            line = line[2:]
        elif not line.strip():
            lines.append("")
            continue
        line = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", line)
        line = re.sub(r"`([^`]+)`", r"\1", line)
        lines.append(line)
    return "검증 상태\n\n" + "\n".join(lines).strip()


def patch_search_index(path: Path) -> bytes:
    entries = json.loads(path.read_text(encoding="utf-8"))
    route_suffix = "/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/README"
    found = 0
    tail = markdown_plain_tail(source_text)
    for entry in entries:
        if entry.get("routePath", "").endswith(route_suffix):
            found += 1
            content = entry["content"]
            old_heading = "남은 검증\n\n"
            index = content.rfind(old_heading)
            if index < 0:
                raise RuntimeError(f"old search-index section missing for {entry['routePath']}")
            entry["content"] = content[:index] + tail + "\n"
            toc = entry["toc"]
            if not toc or toc[-1].get("id") != "남은-검증":
                raise RuntimeError(f"old TOC entry missing for {entry['routePath']}")
            toc[-1] = {
                **toc[-1],
                "id": "검증-상태",
                "text": "검증 상태",
                "charIndex": entry["content"].rfind("검증 상태\n\n"),
            }
    if found != 2:
        raise RuntimeError(f"expected two route aliases in search index; found {found}")
    return (json.dumps(entries, ensure_ascii=False, separators=(",", ":")) + "\n").encode("utf-8")


def atomic_write(path: Path, data: bytes, mode: int) -> None:
    temporary = path.with_name(path.name + ".pass49-correction.tmp")
    temporary.write_bytes(data)
    temporary.chmod(mode)
    os.replace(temporary, path)


preview_files = {rel: preview / rel for rel in relative_files}
site_files = {rel: site / rel for rel in relative_files}
for rel in relative_files:
    if not preview_files[rel].is_file() or not site_files[rel].is_file():
        raise RuntimeError(f"missing generated/served route asset: {rel}")

source_bytes = source_readme.read_bytes()
planned: dict[Path, bytes] = {}
for rel in route_chunks:
    planned[rel] = patch_route(site_files[rel])
planned[search_index_rel] = patch_search_index(site_files[search_index_rel])
for rel in markdown_rels:
    planned[rel] = source_bytes

before = {}
for rel in relative_files:
    served = preview_files[rel]
    built = site_files[rel]
    before[rel.as_posix()] = {
        "served_sha256": digest(served.read_bytes()),
        "build_sha256": digest(built.read_bytes()),
        "served_bytes": served.stat().st_size,
    }

for rel in relative_files:
    assert_unopened(preview_files[rel])

for rel in relative_files:
    backup_path = backup_root / rel
    backup_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(preview_files[rel], backup_path, follow_symlinks=False)
    if digest(backup_path.read_bytes()) != before[rel.as_posix()]["served_sha256"]:
        raise RuntimeError(f"backup mismatch for {rel}")

for rel in relative_files:
    target = preview_files[rel]
    assert_unopened(target)
    mode = target.stat().st_mode & 0o777
    atomic_write(site_files[rel], planned[rel], mode)
    atomic_write(target, planned[rel], mode)

after = {}
for rel in relative_files:
    served = preview_files[rel].read_bytes()
    built = site_files[rel].read_bytes()
    if digest(served) != digest(built):
        raise RuntimeError(f"served/build mismatch after correction for {rel}")
    after[rel.as_posix()] = {
        "sha256": digest(served),
        "bytes": len(served),
        "backup_sha256": digest((backup_root / rel).read_bytes()),
    }

report = {
    "reason": "HTML만 수정한 뒤에도 RSPress가 이전 route chunk를 hydrate해 나머지 생성 자산을 정정했다",
    "changed_file_count": len(relative_files),
    "target_deletions": 0,
    "open_handles_at_write": 0,
    "full_site_tree_hash_note": "pass49 전체 사이트 tree hash는 이 생성 route 정정 전 값이다",
    "before": before,
    "after": after,
}
report_path = repository / "spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-tailnet-sync-2026-10-22/runtime-assets-correction-pass49.json"
report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(json.dumps(report, ensure_ascii=False, indent=2))
