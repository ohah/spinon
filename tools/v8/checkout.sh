#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
depot_revision="$(cat "$repo_root/tools/v8/depot-tools-revision.txt")"
v8_revision="$(cat "$repo_root/tools/v8/v8-revision.txt")"
depot_dir="$repo_root/build/depot_tools"
source_root="$repo_root/build/v8-source"
v8_dir="$source_root/v8"

mkdir -p "$repo_root/build"
new_depot_checkout=false
if [[ ! -d "$depot_dir/.git" ]]; then
  git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git "$depot_dir"
  new_depot_checkout=true
fi
if [[ "$(git -C "$depot_dir" rev-parse HEAD)" != "$depot_revision" ]]; then
  if [[ "$new_depot_checkout" == true ]]; then
    git -C "$depot_dir" fetch origin "$depot_revision"
    git -C "$depot_dir" checkout --detach "$depot_revision"
  else
    echo "depot_tools checkout이 고정 커밋과 다릅니다. 기존 파일을 덮어쓰지 않습니다." >&2
    echo "필요 커밋: $depot_revision" >&2
    exit 1
  fi
fi

export DEPOT_TOOLS_UPDATE=0
export PATH="$depot_dir:$PATH"
"$depot_dir/ensure_bootstrap"
mkdir -p "$source_root"
new_v8_checkout=false
if [[ ! -d "$v8_dir/.git" ]]; then
  new_v8_checkout=true
  if [[ -f "$source_root/.gclient" ]]; then
    python3 - "$source_root/.gclient" <<'PY'
import ast
from pathlib import Path
import sys

source = Path(sys.argv[1]).read_text()
try:
    tree = ast.parse(source)
except SyntaxError as error:
    raise SystemExit(f"기존 .gclient를 해석할 수 없습니다. 덮어쓰지 않습니다: {error}")
solution_writes = [
    node for node in ast.walk(tree)
    if isinstance(node, ast.Name) and node.id == "solutions" and isinstance(node.ctx, ast.Store)
]
if len(solution_writes) != 1:
    raise SystemExit("기존 .gclient에 단일 solutions 설정이 없습니다. 덮어쓰지 않습니다.")
solutions_statement = next((
    statement for statement in tree.body
    if isinstance(statement, ast.Assign)
    and len(statement.targets) == 1
    and statement.targets[0] is solution_writes[0]
), None)
if solutions_statement is None:
    raise SystemExit("기존 .gclient의 solutions 설정을 안전하게 확인할 수 없습니다. 덮어쓰지 않습니다.")
try:
    solutions = ast.literal_eval(solutions_statement.value)
except (ValueError, TypeError):
    solutions = None
if (
    not isinstance(solutions, list)
    or len(solutions) != 1
    or not isinstance(solutions[0], dict)
    or solutions[0].get("url") != "https://chromium.googlesource.com/v8/v8.git"
):
    raise SystemExit("기존 .gclient가 V8 checkout 설정이 아닙니다. 기존 파일을 덮어쓰지 않습니다.")
PY
    (cd "$source_root" && gclient sync --no-history --with_branch_heads --revision="v8@$v8_revision")
  else
    # 빌드에 필요한 소스만 받고 저장소 전체 Git 이력은 복제하지 않는다.
    (cd "$source_root" && fetch --no-history v8)
  fi
fi

actual_revision="$(git -C "$v8_dir" rev-parse HEAD)"
if [[ "$actual_revision" != "$v8_revision" ]]; then
  if [[ "$new_v8_checkout" == true ]]; then
    git -C "$v8_dir" fetch --depth=1 origin "$v8_revision"
    git -C "$v8_dir" checkout --detach "$v8_revision"
  else
    echo "V8 checkout 커밋이 다릅니다. 기존 파일을 덮어쓰지 않습니다." >&2
    echo "필요 커밋: $v8_revision" >&2
    echo "현재 커밋: $actual_revision" >&2
    exit 1
  fi
fi
if [[ -n "$(git -C "$v8_dir" status --porcelain)" ]]; then
  echo "V8 checkout에 수정된 파일이 있습니다. 덮어쓰지 않고 중단합니다." >&2
  git -C "$v8_dir" status --short >&2
  exit 1
fi

python3 - "$source_root/.gclient" <<'PY'
from pathlib import Path
import ast
import re
import sys

path = Path(sys.argv[1])
source = path.read_text()
tree = ast.parse(source)
target_os_writes = [
    node for node in ast.walk(tree)
    if isinstance(node, ast.Name) and node.id == "target_os" and isinstance(node.ctx, ast.Store)
]
if len(target_os_writes) > 1:
    raise SystemExit("기존 .gclient에 중복 target_os 설정이 있습니다. 덮어쓰지 않습니다.")
match = re.search(r"(?m)^target_os\s*=\s*(\[[^\]\n]*\])\s*$", source)
target_os = ["android", "ios"]
if match:
    configured = ast.literal_eval(match.group(1))
    if not isinstance(configured, list) or not all(isinstance(item, str) for item in configured):
        raise SystemExit(".gclient target_os 설정을 읽을 수 없습니다.")
    target_os = sorted(set(configured) | set(target_os))
    source = source[:match.start()] + source[match.end():]
elif target_os_writes:
    raise SystemExit("기존 .gclient의 target_os 형식이 단일 행 목록이 아닙니다. 덮어쓰지 않습니다.")
source = source.rstrip() + f"\ntarget_os = {target_os!r}\n"
path.write_text(source)
PY

(cd "$v8_dir" && gclient sync --no-history --revision="v8@$v8_revision")
echo "V8 및 DEPS 동기화 완료: $v8_revision"
