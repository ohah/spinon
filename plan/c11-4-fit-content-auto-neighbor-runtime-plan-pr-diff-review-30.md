# C11.4 fit-content 옆 auto implicit row 런타임 계획 · PR diff 적대 검토 30관점

**검토 기준:** `a23a48402dddf04ab1041ce16656c720f8a399d1` (`HEAD`와 일치) 대비 전용 worktree 변경. 검토 당시 계획·사전 비교 파일 12개가 staged 상태였다. 이 보고서는 계획 내용·규범 산술을 검토한 [별도 20관점 문서](./c11-4-fit-content-auto-neighbor-runtime-plan-review-20.md)와 겹치지 않도록 diff의 파일 경계, 상태·링크 동기화, 증거 번들 무결성 및 계획 게이트의 추적성을 확인했다.

**실행 범위:** 문서·JSON·이미지를 읽고 링크, Git diff, checksum을 확인했다. 테스트, 빌드, 앱, 시뮬레이터는 실행하지 않았다. 제품 소스와 다른 문서는 수정하지 않았다.

## 실패 경로별 검토

1. **기준 commit이 달라 예상 밖 변경이 diff에 숨을 수 있음 — PASS.** 검토 당시 `HEAD`와 지정 base가 모두 `a23a48402dddf04ab1041ce16656c720f8a399d1`이었다. 따라서 이 판정은 그 base에서 시작하는 전용 worktree의 변경만 대상으로 한다.

2. **다른 작업 파일이 C11.4 변경에 섞일 수 있음 — PASS.** 검토 당시 12개 경로는 계획·검토 문서 3개와 해당 날짜 사전 비교 증거 9개(README, manifest, 브라우저 버전·정리 기록, 이미지 2개, 원시 관찰, 기대값, source pins)로 한정됐다.

3. **계획 PR이 구현 또는 완료 증거를 이미 만든 것처럼 보일 수 있음 — PASS.** 변경 파일에는 Rust·Kotlin·Swift 제품 소스나 테스트 코드가 없다. 새 runtime 계획은 구현 전 작업으로 쓰였고 테스트·앱 실행 결과를 주장하지 않는다.

4. **미완성 계획이 공식 구현 대장을 조용히 바꿀 수 있음 — PASS.** `spec/STATUS.md`는 diff에 없고 기존 공식 C11.4 행도 미수정이다. [공식 상태 대장](../spec/STATUS.md#L273).

5. **C11.4 부모 항목이 계획 추가와 함께 완료 체크될 수 있음 — PASS.** 공식 대장의 C11.4 intrinsic track sizing은 여전히 `- [ ]`이며, 상위 계획도 C11.4를 미완료로 둔다. 계획 추가를 제품 완료로 승격하지 않는다. [C11.4 계획](./c11-4-grid-track-sizing.md#L3-L7).

6. **완료 집계가 계획 변경만으로 증가할 수 있음 — PASS.** 상위 계획과 새 세부 계획은 공식 `72/213`을 그대로 표시하고 변경하지 않는다고 명시한다. 대장 파일도 diff에서 제외됐다.

7. **과거 집계가 현재 값으로 재사용될 수 있음 — PASS.** 상위 계획은 `74/207`, `77/213`을 과거 스냅샷으로 분류하고 현재 값 `72/213`과 구별한다. [상위 계획 상태 머리말](./c11-4-grid-track-sizing.md#L3).

8. **새 날짜 기록이 연혁 순서를 깨거나 오래된 기록을 덮을 수 있음 — PASS.** 2026-10-27 계획 항목은 2026-10-26 기록보다 앞에 추가됐고 기존 기록 본문은 유지됐다. [상위 계획 연혁](./c11-4-grid-track-sizing.md#L5-L11).

9. **PR #662의 Rust-only oracle이 이미 앱 runtime 검증을 했다고 오해할 수 있음 — PASS.** 상위·세부 계획은 #662 결과를 test-only 기준으로 표현하고 새 Android/iOS app fixture 검증을 후속 계획으로 구분한다. 실제 Spinon runtime pass라고 쓰지 않는다. [계획 상태·목표](./c11-4-fit-content-auto-neighbor-runtime.md#L5-L9).

10. **상위 항목에서 새 계획으로 가는 링크가 빠지거나 잘못될 수 있음 — PASS.** 상위 계획의 `후속 계획` 링크가 새 runtime 계획 파일을 가리키며 대상이 존재한다. [링크 위치](./c11-4-grid-track-sizing.md#L7).

11. **상위 계획에서 plan-level 20-view가 유실될 수 있음 — PASS.** 부모 연혁은 20-view 파일에 연결되고 그 파일이 diff에 포함되어 있다. 링크 경로가 전용 `plan/` 디렉터리에서 유효하다.

12. **상위 계획의 30-view 링크가 이 리뷰 산출물과 맞지 않을 수 있음 — PASS.** 요구된 경로에 이 30-view 보고서를 만들었고, 상위 계획의 상대 링크와 일치한다. 보고서 추가 뒤 링크 대상이 존재한다. [상위 링크](./c11-4-grid-track-sizing.md#L7).

13. **세부 계획이 C11.4 상위 문맥과 분리될 수 있음 — PASS.** 새 계획의 첫 줄은 상위 C11.4 track-sizing 계획에 역링크하고 상태·목표를 계획 단계로 제한한다. [세부 계획 머리말](./c11-4-fit-content-auto-neighbor-runtime.md#L1-L9).

14. **상태 대장의 링크가 존재하지 않는 fragment 때문에 깨질 수 있음 — PASS.** 세부 계획은 임의의 `#c11-css-grid` fragment를 쓰지 않고 `../spec/STATUS.md` 전체 문서를 가리킨다. 상대 파일 경로가 존재한다. [세부 계획 링크](./c11-4-fit-content-auto-neighbor-runtime.md#L3).

15. **계획에서 내부 Grid 계약을 찾을 수 없을 수 있음 — PASS.** 세부 계획은 내부 계약 0059를 직접 연결하고 해당 경로가 존재한다. [계약 링크](./c11-4-fit-content-auto-neighbor-runtime.md#L3).

16. **사전 비교 README가 계획의 기준 oracle과 분리될 수 있음 — PASS.** 계획의 Chrome·규범 비교 링크는 날짜가 붙은 precomparison README에 도달한다. README는 Chrome 기록과 규범 유도 내용을 분리해 설명한다. [기준 링크](./c11-4-fit-content-auto-neighbor-runtime.md#L13).

17. **계획이 예상값 JSON 없이 산술 결과를 참조할 수 있음 — PASS.** 별도 `expected.json` 링크가 존재하는 파일로 연결된다. JSON 파싱 확인에서도 구조 오류가 없었다. [기대값 링크](./c11-4-fit-content-auto-neighbor-runtime.md#L13).

18. **README의 화면 링크가 빠지거나 다른 이름의 캡처를 가리킬 수 있음 — PASS.** 전체 WPT 결과 이미지와 target 전용 이미지 모두 상대 경로에서 확인됐다. README는 둘을 별도 목적의 증거로 나눈다. [사전 비교 README](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.md).

19. **원시 관찰, source pins 또는 정리 기록이 탐색 불가능할 수 있음 — PASS.** README의 raw JSON, `source-pins.json`, `expected.json` 링크가 모두 존재하며 같은 evidence 디렉터리에 있다.

20. **Chrome WPT 실행을 Spinon 적합성으로 확대 해석할 수 있음 — PASS.** 상위 계획, 세부 계획, precomparison README, expected JSON은 upstream Chrome testharness 관찰과 Spinon runtime/WPT 결과를 구별한다. README에는 Spinon 실행이 없다고 명시돼 있다. [증거 범위](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.md).

21. **브라우저 결과의 런타임·버전 출처가 빠질 수 있음 — PASS.** `agent-browser-cli-version.txt`는 0.31.1을, 원시 관찰은 HeadlessChrome 156 user agent를 기록하며 계획과 README의 요약이 이 값과 맞는다.

22. **고정 WPT revision과 digest가 문서마다 달라질 수 있음 — PASS.** 계획·README·`source-pins.json`은 revision `c999c58338ee1d223df5ad62e48be1202ced0d37`와 SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`를 동일하게 기록한다. source pins에는 live digest도 같은 값으로 명시돼 있다.

23. **WPT source anchor가 오래된 줄 번호나 다른 요소를 가리킬 수 있음 — PASS.** GitHub의 pinned file은 133줄이다. 현재 README와 20-view의 정정은 setup `11–15`, target DOM `72–75`, target assertion `131–132`를 가리키며 이전 `566–568` 표기는 남지 않았다. [고정 WPT source](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L1-L133), [target DOM](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75), [target assertion](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L131-L132).

24. **원시 관찰값이 요약의 Pass 수·target 결과와 충돌할 수 있음 — PASS.** `chrome-wpt-live-observation.raw.json`은 `allThirteenPass`, `targetAssertionPresent`, `targetResultPass`, `gridHeight=300px`, `rows=10px 290px`를 기록한다. README와 계획 요약이 같은 결과를 쓴다.

25. **화면이 설명과 다른 페이지이거나 파일이 전송 중 손상될 수 있음 — PASS.** 두 캡처를 눈으로 확인했으며 요약 화면은 testharness의 13 Pass를, target 화면은 대상 assertion과 결과 목록을 보여준다. SHA256SUMS에 기재된 8개 번들 파일도 모두 checksum과 일치한다.

26. **기대값 자료가 runtime 결과처럼 오인되거나 upstream 차이를 숨길 수 있음 — PASS.** `expected.json.status`는 독립 oracle이고 Spinon runtime 결과가 없다고 명시한다. `deviations_from_upstream_fixture`는 runtime root width 60px과 upstream의 auto inline size 차이를 기록한다.

27. **계획과 expected fixture가 서로 다른 scene·root·style 입력을 가질 수 있음 — PASS.** 세부 계획의 scene/root/node IDs 및 `display:grid`, `width:60px`, 300px block size는 JSON의 `runtime_identity`·`source_shape`와 대응한다. 이는 두 파일의 입력 계약 동기화를 확인한 것이며 sizing 산술을 다시 판정하지 않는다.

28. **negative-control 설명은 있으나 실제 구현 검증 항목에서 빠질 수 있음 — PASS.** 계획은 각 unknown/mutated/missing/extra/stale negative control을 focused Rust/FFI tests에서 개별 실행하고 rejection 및 partial output 부재를 확인하도록 요구한다. expected JSON의 rejection 목록도 이 요구와 일치한다. 이 보고서 검토 중 해당 테스트는 실행하지 않았다. [계획 gate](./c11-4-fit-content-auto-neighbor-runtime.md#L29).

29. **기본·제품·fixture-test artifact 경계가 모호하거나 잘못된 Cargo 조합을 쓸 수 있음 — PASS.** 계획은 기본, `--features c04-runtime-gpu`, `--features c04-runtime-gpu-fixture-test`의 세 명령을 각각 고정하고 처음 두 artifact의 fixture FFI symbol/string/payload 제외 및 마지막 positive control의 포함을 요구한다. Cargo.toml의 feature graph와 프로젝트 규칙 `35–37`행에 맞으며, 세 빌드는 구현 gate라 이번 검토에서는 하지 않았다. [계획 gate](./c11-4-fit-content-auto-neighbor-runtime.md#L48), [Cargo feature 정의](../crates/spinon-ffi/Cargo.toml#L10-L32).

30. **Agent Browser가 종료되지 않았는데 정리 완료로 과장할 수 있음 — PASS.** 정리 JSON은 session closed 및 후속 프로세스 scan의 0 count를 기록하고, 첫 scan이 shutdown과 경쟁했다는 제한도 함께 밝힌다. 이는 기록된 세션 전용 프로세스 조회 범위의 결과이며 전체 기기 프로세스 상태를 주장하지 않는다. [정리 기록](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/browser-cleanup.json).

## 최종 판정

**PASS (30 PASS / 0 PARTIAL / 0 FAIL).** 계획 PR diff와 연결된 증거·탐색 경로에서 미해결 문서 또는 번들 결함을 찾지 못했다. 실행하지 않은 빌드, 테스트, 앱·시뮬레이터 동작은 이 판정에 포함되지 않는다.
