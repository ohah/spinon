# C13.3 계획 PR 변경 30개 적대 검토 · 2026-10-15

- **검토 종류:** 계획/oracle PR의 변경분 적대 검토 30개
- **기준:** C13.3 계획·계약·Chrome fixture/reference, C13.2 이력 보완과 상태 대장 변경 전체
- **기준 commit:** `edf705786ed3a438e53fda71efd5f366e1b27d7a`
- **계획 관점 검토:** [별도 20개 검토](c13-3-scroll-gesture-plan-review-20-2026-10-15.md)
- **실행 근거:** focused C13.3 oracle 5/5, CSS reference 전체 447/447, 두 Node 파일 `--check`, `git diff --check`
- **판정:** 차단 결함 없음. 실제 Android/iOS 앱 입력과 C13.3 runtime은 아직 구현·검증하지 않았다.

| # | PR 변경의 실패 경로 | 적대 확인과 판정 |
| --- | --- | --- |
| 1 | 계획만 들어갔는데 공식 상태 대장이 구현 완료로 올라간다. | `spec/STATUS.md`는 C13.3을 사전 계획이라고 기록하고 Android/iOS runtime과 S04.10을 미검증으로 남긴다. |
| 2 | 상위 C13 roadmap이 새 계획의 축·입력 경계를 요약하다 다른 계약을 말한다. | C13.3 표와 세부 계획이 normalized wheel, per-axis remainder, touch-action, cancel/multipointer 및 미지원 event 범위에서 맞는다. |
| 3 | C13.2 follow-up PR #176의 표면 결과가 미실행 또는 C13.2 전체 완료로 과장된다. | 기존 C13.2 계획/근거는 run5 PixelCopy/Metal 각 5/5로 갱신하면서 ink-only overflow와 전체 완료를 미지원으로 유지한다. |
| 4 | C13.2 구현 근거를 C13.3 gesture 근거로 재사용해 시작점·입력·owner 범위를 혼합한다. | 새 `0063`과 Chrome evidence는 C13.3 입력을 별도 고정하고, 기존 `0062` 변경은 이전 C13.2 범위만 보충한다. |
| 5 | C13.1 fixture boundary가 Cargo manifest 변경 뒤 이전 feature 이름 때문에 실패한다. | `fixture-module-boundary`의 기존 확인은 manifest의 `dep:sha2`를 반영한다. 새 oracle 스크립트는 Rust feature나 제품 ABI를 더하지 않는다. |
| 6 | package script가 이름을 겹치거나 capture/test 실행을 혼동한다. | `css:reference:c13-3-scroll-gesture-precomparison`은 capture, `test:css-c13-3-scroll-gesture-precomparison`은 offline reference assertion으로 각각 지정돼 있다. |
| 7 | Chrome DOM fixture를 Spinon 앱 runtime fixture로 보인다. | HTML은 `tests/fixtures/css/c13` 자료실이고 README/JSON/PR 본문은 Chrome oracle이며 앱 화면이 아니라고 명시한다. |
| 8 | 캡처 재생성이 PATH의 임의 Chrome 결과를 조용히 oracle에 고정한다. | capture 도구가 Chrome `155.0.8059.40`과 WebKit revision을 assert하고 reference test가 다시 고정한다. |
| 9 | 사전 비교가 user가 금지한 terminal-browser로 수행됐다고 오해한다. | 도구는 `agent-browser` session·CDP를 직접 사용한다. terminal-browser 호출은 포함하지 않는다. |
| 10 | desktop/real-device 수치를 simulator 실행이라고 보고한다. | oracle metadata에 macOS/viewport/touch emulation을 기록하고 AVD/iOS 앱 실행은 미래 관문으로 분리한다. |
| 11 | Chrome viewport가 모바일 simulator viewport로 섞이거나 CSS px과 DPR을 혼동한다. | oracle는 `400×240`, DPR 1로 고정하며 screenshot header/JSON test가 크기를 검사한다. 앱 evidence에는 실제 surface 크기와 scale을 요구한다. |
| 12 | PR의 시각 자료가 다른 fixture나 잘못된 픽셀 크기를 가리킨다. | PNG를 agent-browser로 다시 캡처하고 하단 label clipping을 수정했다. reference는 screenshot path, 400×240 dimensions와 SHA-256을 pin한다. |
| 13 | CSS fixture가 조용히 바뀌어 이전 JSON trace가 계속 통과한다. | test는 checked-in fixture SHA와 screenshot SHA를 현재 파일 bytes에서 다시 계산한다. 마지막 캡션 수정 뒤 capture와 전체 reference 검사를 재실행했다. |
| 14 | observation output이 capture 환경·도구를 특정할 수 없다. | JSON은 Chrome build, WebKit revision, agent-browser version, host platform/OS, viewport와 max touch contacts를 기록한다. |
| 15 | 대상 target이 아닌 sibling/ancestor에서 event나 offset을 읽고 결과를 잘못 연결한다. | fixture는 inner/outer/root owner를 명시하고 offset/event를 owner id와 target별로 저장한다. wheel trace에는 point/target 정보가 남는다. |
| 16 | `touch-action` 조상 교집합 fixture가 실제 hit target 경로를 시험하지 않는다. | inner-content pan-x case와 nearest inner scroller를 고정하고 바깥 outer policy는 경로 밖임을 README와 test에서 확인한다. |
| 17 | root chain trace가 단일 inner/outer의 변화로 끝나거나 초기 max 값이 틀린다. | fixture는 inner·outer y max 시작과 root remainder를 따로 기록하고 assertion이 각 owner의 기대 offset을 확인한다. |
| 18 | overflow `hidden` direct-input boundary를 `clip`/`visible` offset과 합친다. | Chrome oracle의 세 입력 trace를 분리해 저장하고 계획 해석은 각각 hidden programmatic-only, clip/visible no owner로 명시한다. |
| 19 | touch-action 테스트 변경이 active gesture 정책 freeze를 증명하지 않는다. | 사전 none, touchstart 뒤 변경, pan 시작 뒤 변경, pan-x/y 양 방향 trace가 따로 있고 expected offsets와 pointer events를 pin한다. |
| 20 | direct touch의 mixed-axis discrepancy를 normative expectation으로 승격한다. | JSON과 README는 Chrome diagonal `(0,0)`과 per-axis contract의 차이를 분리하고 test는 차이 보존을 검사한다. |
| 21 | after-cancel move 오류를 browser가 late event를 drop했다는 제품 증거로 쓴다. | CDP가 `TouchStart first`로 dispatch를 거부한 raw 오류를 저장한다. 계획과 README는 queued stale Move 검증을 core/Simulator 미완료로 남긴다. |
| 22 | simultaneous two-contact trace를 active-pan 중 second-pointer insertion의 검증으로 말한다. | trace input metadata가 simultaneous start임을 고정하고 evidence/test/plan이 active-pan 삽입은 앱 gate라고 분명히 한다. |
| 23 | `touch-action:none`가 wheel까지 막는다고 오해하거나 Chrome DOM event를 public API로 읽는다. | wheel-under-none case는 offset change를 저장하고 JSON/contract/README는 DOM event가 oracle-only라고 제한한다. |
| 24 | screenshot만으로 offset, chaining 또는 event를 증명한다. | fixture screenshot은 시각 참고로만 쓰고 JSON trace가 owner offset, scroll/pointer/wheel event를 separately pin한다. |
| 25 | 테스트가 reference metadata만 확인하고 중요한 관찰을 빠뜨린다. | 다섯 focused tests가 hash/provenance, boundary/overflow, touch-action, cancel/multipointer, per-axis discrepancy를 독립 assertion한다. |
| 26 | full CSS reference 실행의 실패가 이 변경 또는 의존성 문제인지 숨겨진다. | 첫 전체 실행의 C11 실패는 isolated worktree에 `parse5`가 없는 import error였다. lockfile 고정 install 후 447/447 통과했다. tracked dependency files는 변경하지 않았다. |
| 27 | Markdown link scan의 이전 dead link를 새 C13.3 링크 문제로 간주하거나 숨긴다. | 새 C13.3 local paths가 resolve한다. 전체 changed-doc scan에 남은 `/roadmap.svg` 및 C11/C12 historical artifact 경로는 기존 STATUS 항목으로 분리해 새 링크 오류라고 표시하지 않는다. |
| 28 | 내부 계약 version을 계획 중에 올리거나 제품 CSS 지원을 선언한다. | `0063`은 `0.1.0-draft`이며 “제품 CSS 지원 선언 아님”을 표기한다. 구현 전 oracle에서 숫자 버전을 올리지 않는다. |
| 29 | plan PR의 simulator 관문을 이미 실행된 platform proof로 쓴다. | plan review와 STATUS는 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator를 후속 구현 gate로만 남긴다. |
| 30 | CI가 없거나 required checks가 없다는 사실을 merge 가능성/제품 검증으로 바꿔 말한다. | GitHub 상태와 mergeability는 PR 직전 확인한다. CI 미설정이어도 위 local oracle/reference 검증을 별도로 보고하고 앱 동작 완료를 주장하지 않는다. |

## 남은 경계

이 PR의 결과물은 계획, Chrome 사전 oracle, fixture와 고정 reference다. Rust workspace/Clippy는 Rust 코드 변경이 없어 실행하지 않았다. Android/iOS runtime gesture, 실제 simulator input/screenshots, feature artifact negative/positive controls, public events 및 S04.10 presentation 연계는 구현 PR의 독립 관문이다.
