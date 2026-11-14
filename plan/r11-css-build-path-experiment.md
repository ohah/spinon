# R11 · 모바일 CSS 변환 경로 증거 범위 확인

**상태:** 제한 조사 실행 완료 · 증거 공백과 미완료 경계 명시 · R11 공식 항목 미완료 유지
**기준 revision:** `401911e6b78b543e3c9553c3e6e162e43ca381b2`
**공식 항목:** [R11 CSS 빌드 경로 실험](../spec/STATUS.md)
**연결 계약:** [0003 웹 표면](../spec/0003-web-surface.md)
**기존 변환기:** [`spikes/style-layout`](../spikes/style-layout/README.md)
**기존 Tailwind 대조:** [R14 계획](./r14-tailwind-css-experiment.md) · [R14 실행 근거](../spec/internal/evidence/r14-tailwind-css-2026-10-16/README.md)
**제한된 조사 실행 근거:** [R10/R14 source hash, current converter 재현, coverage matrix와 남은 한계](../spec/internal/evidence/r11-css-build-path-2026-11-14/run-20261114T124515Z/README.md)
**PR 변경분 검토:** [서로 다른 30개 실패 관점](../spec/internal/evidence/r11-css-build-path-2026-11-14/run-20261114T124515Z/pr-diff-review-30.md)<br>
**계획 실패 관점 검토:** [서로 다른 20개](./r11-css-build-path-experiment-plan-review-20.md)

## 목표

R11의 문구는 Lightning CSS AST를 제한된 모바일 스타일 데이터로 바꿀 수 있는지 조사하라고 한다. 저장소에는 이미 R10 개발용 변환기와 그 입력 거부 검사가 있고, R14는 실제 Tailwind 생성 CSS를 그 변환기에 넣어 전체 파일과 일부 규칙의 결과를 기록했다. 이 계획은 이 증거들이 R11의 조사 질문을 이미 답하는지, R11에만 남은 검증 공백이 있는지 구분한다.

이번 조사는 기존 변환기의 제품 채택이나 CSS 지원 범위를 정하지 않는다. 조사 결과만으로 R11을 완료 표시하지 않는다. 모든 R11 판정은 별도 실행 근거와 PR 검토가 끝난 뒤 상태 대장 변경으로 제안한다.

## 기존 증거와 재사용 경계

| 기존 자료 | 이미 관찰한 내용 | R11에서 재사용할 수 있는 근거 | R11 완료 증거로 그대로 간주하지 않을 점 |
|---|---|---|---|
| `spikes/style-layout/compile-css.mjs`, `verify.mjs`, `sample.css`, `styles.json` | Lightning CSS `1.33.0` AST visitor가 단일 클래스의 좁은 선언 집합을 JSON으로 바꾸고, 미지원 입력과 stale output을 거부한다. JSON은 R10 Rust/Taffy spike에서 소비한다. | 허용 선언·출력 필드, 기존 positive/negative case, 입력과 출력의 분리 경계 | 각 역사적 결과에 기록된 source revision에서 이 파일들의 blob과 lockfile을 복원해 현재 파일 hash와 비교한다. 당시 source revision을 확인할 수 없으면 역사적 실행과 현재 코드를 동일시하지 않고 `미확인`으로 둔다. R10의 Taffy 동등성 결과는 CSS 변환 의미의 검증이 아니다. |
| `spec/internal/evidence/taffy-r10-2026-09-28.md` | 변환 결과를 읽는 제한 Taffy fixture의 노드·Flex·합성 텍스트·반올림 경계를 확인했다. | 출력 JSON 뒤쪽 소비자의 제한과 실험 경계 | 제품 Vite/Rspack, Stylo, Android/iOS CSS cascade 또는 공개 지원을 증명하지 않는다. |
| `plan/r14-tailwind-css-experiment.md`, `spec/internal/evidence/r14-tailwind-css-2026-10-16/tailwind-v4.3.3-generated.css`, `converter-results.json` | 고정 Tailwind 출력의 layer·variable·Preflight 구조와 변환기의 전체 파일 거부, 단순 `.flex` positive control 및 미지원 입력 거부를 기록했다. | 실제 생성 CSS, 결과 JSON, 각각의 과거 source revision을 확인한 뒤 일반 생성 입력과 Tailwind-specific 입력 차이를 비교한다. | R14 완료는 R11 완료나 Tailwind/CSS 지원 완료를 뜻하지 않는다. R14가 R11을 대신한다고 가정하지 않는다. 당시 converter source revision이나 출력 hash가 복원되지 않으면 기존 관찰은 참고 자료로만 두고 재현 결과와 분리한다. |

## 조사 입력과 출력

- 기준 입력·버전은 `spikes/style-layout/package.json`, `bun.lock`, `compile-css.mjs`, `verify.mjs`, `sample.css`, `styles.json` 및 R14의 `tailwind-v4.3.3-generated.css`, `converter-results.json`이다. 각 파일의 repo-relative 경로와 실제 바이트 SHA-256을 manifest에 한 줄씩 기록한다.
- 역사적 증거에는 해당 결과가 실행된 commit/source revision을 우선 사용한다. `git show <revision>:<path>`로 converter·lockfile·입력의 historical blob hash를 계산하고 보존 결과의 hash와 연결한다. 기록된 실행 revision이 없거나 경로가 당시 commit에 없으면 `과거 source 미확인`으로 남긴다. 현재 실행은 별도 worktree에서 하되, 전체 checkout이 아니라 converter 관련 source path가 기준 HEAD와 byte-for-byte 같은지 검사한다. branch, 전체 status와 staged/unstaged/untracked source 관련 경로 및 file hash도 남긴다.
- source identity를 먼저 캡처한 뒤 실행 입력·출력·로그는 저장소 worktree 밖의 새 `mktemp -d` run directory에 둔다. 생성된 temp 경로와 `git rev-parse --show-toplevel` 결과를 각각 `realpath`로 정규화한 뒤, path-component 기준으로 temp directory가 repository root 아래가 아닌지 확인하고 canonical path를 manifest에 기록한다. `TMPDIR`이 worktree 안을 가리키거나 경로를 판정할 수 없으면 중단한다. `spikes/style-layout` 관련 파일의 staged/unstaged diff가 없고 working-copy hash와 `git show HEAD:<path>` blob hash가 같은지 확인한다. 계획·review 문서는 현재 worktree에 있을 수 있으므로 전체 checkout을 clean이라고 부르지 않고, 실행에 사용한 관련 source path 집합을 별도로 고정한다. run temp directory는 생성 시 이미 있거나 symlink인 경로를 거부한다.
- 모든 실행·검증 자료를 기록한 뒤 새 evidence subdirectory `spec/internal/evidence/r11-css-build-path-2026-11-14/<run-id>/`로 옮긴다. 해당 경로가 이미 있으면 중단하고 자동 덮어쓰기·병합을 하지 않는다. evidence copy 전후 파일 목록과 SHA-256이 일치해야 한다. 정리는 검증된 evidence copy 이후 해당 실행이 만든 temp directory에만 적용하며, 실패 입력·stdout/stderr·unexpected output을 복사하기 전에는 지우지 않는다.
- `compile-css.mjs`는 실행 직후 지정 출력 경로를 삭제한다. 저장소 파일을 입력으로 직접 넘기지 않고 run-local copy만 사용하며, 새 case directory 안에 아직 없는 `output.json`만 출력 인자로 준다. 실행 전 `lstat`으로 output path 부재와 symlink 부재를 확인한다. `realpath`한 output parent에 output basename을 붙여 얻은 canonical output path가 `realpath(input)`와 다른지, 그 parent와 모든 입출력 경로가 새 run directory 안에 있는지 검사한다. 기존 `styles.json`, R14 산출물, 증거 파일 및 symlink target과 canonical output path가 겹치거나 출력 경로가 이미 있으면 실행을 중단한다. 경로 대소문자·symlink alias negative control은 폐기 가능한 run-local fixture에만 적용한다.
- 새 변환 결과를 사용하면 입력 CSS, output JSON, stdout/stderr, exit code, output 존재/크기/SHA-256을 함께 저장한다. 비정상 종료·경고·partial output은 성공 결과로 읽지 않는다. 실패 output이 생성되면 `unexpected-output`으로 보존해 검사하고 해당 case의 scratch 안에서만 정리한다. JSON은 exit code 0, warning 없음 또는 모두 설명된 warning, schema·expected field 검증을 통과할 때만 success로 판정한다.
- manifest에는 기준 revision, source snapshot을 만든 시점의 전체 status 요약과 converter 관련 path의 staged/unstaged diff hash, working-copy/HEAD blob SHA-256, 실행 시각, working directory, 명령행, `bun --version`, `node --version`, 실제 `lightningcss` package version과 resolved module path를 기록한다. 이 source snapshot은 run temp/evidence output을 만들기 전에 캡처한다. `package.json`/`bun.lock`과 resolved module version이 일치하지 않으면 실행 결과를 인정하지 않는다. 과거 R10/R14 증거도 동일한 파일별 hash 목록으로 source revision과 출력물을 특정한다.

## 사전 판정 기준

Coverage matrix에서 아래 질문 각각을 `기존 근거로 충족`, `현재 revision 재현 필요`, `미확인` 중 하나로 표시하고 링크·파일 hash로 근거를 붙인다.

1. 고정된 Lightning CSS 버전이 CSS를 AST로 읽고 명시된 visitor만 실행하는가?
2. 지원하는 CSS 구문·선택자·속성·값의 경계가 구체적으로 열거되어 있는가?
3. 성공한 변환이 버전이 표시된 모바일 스타일 데이터(JSON)에 어떤 필드와 값을 쓰는가?
4. 지원 범위 밖 구문이 부분 성공·규칙 누락·stale JSON 없이 명시적으로 실패하는가?
5. 변환 산출물을 소비하는 기존 fixture가 데이터 형식 오류를 탐지하는가? 그렇다면 그 검증이 R10/Taffy에 한정됨을 분리하는가?
6. Tailwind 생성 CSS의 전체 거부와 `.flex` positive control이 기존 제한의 성격을 설명하는가? 이를 제품 CSS 동작이나 R11 판정으로 과장하지 않는가?
7. 증거가 서로 다른 시점·revision에서 왔다면 각 source/input/output hash를 연결하고, 실행되지 않았거나 과거 source를 복원할 수 없는 부분을 통과로 표시하지 않는가?
8. 입력 CSS의 top-level rule/declaration과 출력 JSON 사이에 누락·부분 변환이 없는지 확인할 수 있는가? `transform()` warning 또는 진단을 모두 보존하고, 진단을 수집할 수 없으면 해당 경우는 `미확인`으로 남기는가?

R11 문구를 기존 자료만으로 충분히 판정할 수 없으면 결과는 확인됨·재현됨·미확인을 구분하고, 부족한 질문만 후속 계획으로 제안한다. 새 converter 문법을 임의로 추가해 공백을 채우지 않는다. Positive input은 expected JSON과 정확히 비교하고, mixed valid/unsupported control은 성공 코드·부분 JSON·경고가 있어도 성공으로 분류하지 않는지 확인한다. transform warning을 CLI가 출력하지 않거나 반환 metadata를 노출하지 않아 완전성을 확인할 수 없으면 그 경계를 미확인으로 기록한다.

## 실패 사례와 격리 요구

- converter 또는 oracle 버전이 lockfile과 다르면 결과를 비교하지 않는다. Node/Bun 버전, 실제 모듈 resolved path/version, working directory, argv를 기록하지 못해도 실행 결과를 인정하지 않는다.
- CSS 파싱 실패와 AST 해석 실패를 같은 오류로 뭉치지 않는다. 가능한 경우 원본 입력과 첫 진단 위치를 보존한다.
- at-rule, 복합/비클래스 선택자, 중복 규칙, `!important`, 변수·백분율, 음수/비유한 값, 알 수 없는 속성, 문법 오류를 성공으로 흡수하지 않는다. 이미 R10/R14에 있는 각 case를 재사용할지, 필요한 별도 control인지 matrix에서 구분한다.
- 기존 JSON이 남아 있는 상태에서 실패했는데도 이를 새 결과로 읽는 경우는 실패다. 비정상 종료 뒤 부분 JSON이 있으면 유효 결과로 parse/consume하지 않고 output 존재·크기·hash·exit 상태를 기록한다.
- converter가 파일을 쓰기 전에 기존 출력 경로를 지우므로 `realpath`/`lstat`으로 input-output alias, symlink target, 기존 파일과 evidence path 중첩을 거부한다. 새 evidence run directory와 각 scratch case directory는 이미 있으면 중단한다. 변환에 쓰는 복사 입력·출력 경로는 새 run directory의 canonical path 아래 있어야 한다.
- cleanup은 새 run directory 안에서 명시한 scratch case directory에만 적용한다. 실패 원본과 예기치 않은 결과는 evidence에 복사하고 hash를 남기기 전에는 정리하지 않는다.
- Chrome 캡처가 추가되더라도 CSS parser/converter 결과의 oracle로 세지 않는다. Android/iOS simulator는 이 문서·변환기 범위의 판정에 필요하지 않다.

## 범위 제외

- Lightning CSS를 제품 CSS parser/cascade, Vite/Rspack 모바일 실행 경로, Stylo 대체재로 선택하지 않는다.
- 사용자 CSS API, 지원 속성·선택자 표, Tailwind 지원, CSS 호환성 또는 WPT 적합성을 추가·승격하지 않는다.
- 기존 R10/R14 converter·evidence와 제품 source는 수정하지 않는다. 실행 뒤 공식 상태 대장 R11 행에는 완료 근거가 없다는 점과 이번 제한 조사 링크만 덧붙일 수 있다. 체크박스와 완료 수는 유지하고 architecture 및 공개 roadmap 동작은 바꾸지 않는다.
- 플랫폼 앱·시뮬레이터·실기기 빌드/설치/실행, 시각 비교와 성능 주장은 제외한다.
- 별도의 구현 계획·20개 계획 실패 관점 검토가 완료되기 전에는 converter source를 변경하지 않는다.

## 실행 순서와 종료 판정

1. 이 계획과 독립적인 20개 실패 관점 검토를 완료하고 지적사항을 반영한다. **검토와 수정 확인이 끝나기 전에는 아래 조사를 실행하지 않는다.**
2. run directory를 만들기 전에 source identity를 캡처한다. 기준 revision에서 입력 file set, lockfile, R10/R14 근거 파일의 historical hash와 commit ancestry를 수집한다. 과거 실행의 converter revision이 불명확하면 미확인으로 둔다.
3. converter 관련 path의 worktree status, diff hash, working-copy/HEAD blob hash와 runtime/module version·argv를 manifest에 기록한다. 관련 source file이 dirty이거나 HEAD blob과 다르면 중단한다. 전체 worktree에 plan/review 문서가 존재할 수 있으므로 clean 전체 checkout으로 가장하지 않는다.
4. 새 unique temp run directory를 저장소 밖에 만들고 그 canonical path가 repository root 밖인지 확인한 뒤 재실행이 꼭 필요한 공백만 run-local case로 검사한다. 디렉터리 충돌, symlink, path containment 위반, 이미 존재하는 output이면 중단한다.
5. 판정 기준 8개를 R10/R14 자료에 대조하고 coverage matrix를 작성한다. 단순 파일 존재는 동작 재현으로 간주하지 않는다.
6. 새 positive output은 expected JSON, schema, warnings, input/output hashes와 함께 검증한다. 실패/partial output은 소비하지 않고 exit status·diagnostic·file state를 기록한다. 로그와 산출물을 고유 evidence directory로 옮긴 뒤 hashes를 다시 대조한다.
7. 공백이 전혀 없으면 R11의 제한 실험이 기존 증거에 이미 포함되는지와 상태 대장에 별도 연결이 필요한지를 근거와 함께 제안한다. 이 계획 실행만으로 R11 checkbox를 바꾸지 않는다.
8. 공백이 있으면 가장 작은 재현 입력과 출력 경계만 후속 계획으로 제안한다. converter 구현, 지원 범위 확대, 공식 상태 변경은 별도 계획과 검토 이후로 미룬다.
9. 최종 보고에는 `확인됨 / 재현됨 / 미확인`을 구분하고 제품 CSS 경로와 API 지원에는 결론을 확장하지 않는다.

초안에 대한 20개 계획 관점 검토에서 발견된 사항을 반영했다. 보정 후 독립 재검토는 4–20번에서 통과했고, 1–3번은 초기 무발견 뒤 재검토하지 않았다. 따라서 수정 후 20개 전체 통과를 주장하지 않는다. 제한 조사는 4–20번 재검토 뒤 실행됐으며, 이 review gap도 본 PR 검토에서 다시 확인한다. 조사 결과만으로 구현이나 공식 완료를 변경하지 않는다.

## 2026-11-14 제한된 조사 결과

- R10의 고정 sample을 저장소 밖 임시 출력으로 변환해 기존 styles.json과 byte-exact 일치하는 JSON 산출을 재현했다. 현재 converter의 verify.mjs는 12개 입력 경계에서 통과했고, valid display:flex와 unsupported color를 함께 둔 입력은 exit 1 및 output 미생성으로 거부했다. 이 실행은 warning 완전성, CSS parser와 visitor rejection의 진단 차이, 강제 종료 중 partial write를 확인하지 않았다.
- 원래 실행 직전·직후 R10/R14 관련 source hash를 과거 revision과 현재 HEAD 사이에서 대조했다. 후속 문서 동기화는 R11 완료 근거가 없음을 명확히 하고 제한 조사 링크만 추가하며 checkbox·공식 완료 수는 변경하지 않는다.
- Lightning CSS transform 반환 warning metadata를 converter가 소비하지 않아 경고 완전성은 미확인이다. verify.mjs의 저장 결과는 합계 한 줄뿐이라 개별 parser 오류·AST 거부 stderr를 비교할 수 없고, 강제 종료/I/O 실패 partial-write도 실행하지 않았다. output preflight의 실행 전 lstat 결과와 source snapshot-임시 경로 생성 순서는 원본 artifact만으로 재검증할 수 없다.
- 전체 명령·hash, verify.mjs의 12개 입력 boundary, 8개 기존 질문과 6개 실행·진단 경계를 담은 14행 coverage matrix 및 남은 한계는 위 실행 근거를 따른다. R11 공식 항목은 계속 미완료다.
