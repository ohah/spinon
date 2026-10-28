# C11.4 two-track span overflow runtime 후속 계획 PR diff 검토 30개

| # | PR 변경분에서 확인한 별도 오류 위험 | 판정 |
|---:|---|---|
| 1 | 새 계획 문서 경로가 plan 디렉터리 외 다른 feature를 가리킴 | 통과 — 변경은 C11.4 runtime 후속 계획이다. |
| 2 | 부모 C11.4 계획이 빠져 상위 범위를 알 수 없음 | 통과 — 상위 계획으로 연결한다. |
| 3 | 이전 test-only PR #756의 역할이 삭제됨 | 통과 — runtime 증거가 아님을 유지한다. |
| 4 | Chrome precomparison이 계획에서 누락됨 | 통과 — 관측, expected JSON, PNG를 각각 연결한다. |
| 5 | 독립 expected value가 browser observation에서 생성됨 | 통과 — 기존 CSS Grid 계산을 기준으로 재사용한다. |
| 6 | 새 profile 목표가 기본 production profile에 직접 합쳐짐 | 통과 — fixture-only computed-style/projection 경계를 규정한다. |
| 7 | generic author CSS admission이 변경 범위에 포함 | 통과 — 일반 앱 document는 제외한다. |
| 8 | profile 이름이 fixture-only 목적을 드러내지 않음 | 통과 — test-only runtime gate에 목적이 드러나야 한다. |
| 9 | feature-off 제거 확인이 빠짐 | 통과 — 기본 빌드에서 새 route/symbol을 확인한다. |
| 10 | fixture-test positive control이 빠짐 | 통과 — opt-in build/run을 별도로 요구한다. |
| 11 | authored `overflow-y`의 secondary computed value를 무시 | 통과 — 축 간 conversion을 기록한다. |
| 12 | cross-axis computed-value 변환을 normative 근거 없이 가정 | 통과 — CSS Overflow §3.1을 근거로 고정한다. |
| 13 | track CSS가 `minmax(auto,max-content)`가 아님 | 통과 — 두 columns를 정확히 지정한다. |
| 14 | item이 한 track 또는 세 track을 span | 통과 — 정확히 두 columns span을 요구한다. |
| 15 | definite Grid dimensions가 빠져 결과가 달라짐 | 통과 — `80×20px` definite size를 고정한다. |
| 16 | row size가 목표 결과에 영향을 주도록 변경 | 통과 — 단일 20px row와 span 비대상을 고정한다. |
| 17 | Ahem font가 로드되지 않아 intrinsic width가 달라짐 | 통과 — Ahem 10px, 10 X, 100px min/max 기준을 명시한다. |
| 18 | visible과 clip을 overflow automatic minimum zero로 잘못 처리 | 통과 — 두 값 모두 content-based 100px을 요구한다. |
| 19 | hidden/auto/scroll 중 하나가 content-based로 남음 | 통과 — 세 값 모두 zero automatic minimum을 요구한다. |
| 20 | maximum contribution이 minimum source에 의해 바뀜 | 통과 — 100px MaxContent를 별도로 유지한다. |
| 21 | source/value 외에 track geometry를 검사하지 않음 | 통과 — 두 final bases와 frames를 포함한다. |
| 22 | CSS-pixel tolerance가 기존 helper 기준으로 넓어짐 | 통과 — zero tolerance를 계획 판정에 명시한다. |
| 23 | render evidence가 layout trace를 대신함 | 통과 — geometry, trace, draw/present를 구분한다. |
| 24 | draw/present가 trace와 다른 revision일 수 있음 | 통과 — 동일 run/revision lineage를 verifier가 확인해야 한다. |
| 25 | Android AVD 조건이 달라 matrix 비교가 불가능 | 통과 — API 37 ARM64 AVD를 고정한다. |
| 26 | iOS simulator 실행이 없어 두 플랫폼 검증으로 확장 | 통과 — iPhone 17 Pro/iOS 26.2 Simulator 5/5를 요구한다. |
| 27 | Simulator evidence가 실기기 결과로 오해됨 | 통과 — 실기기 검증을 제외한다고 적는다. |
| 28 | overflow clipping/scrolling pixel을 검증했다고 오해 | 통과 — 해당 paint/scroll 기능은 명시적으로 범위 밖이다. |
| 29 | app screenshot이 Chrome screenshot으로 대체되거나 반대로 혼합 | 통과 — PR body에서 app Simulator image와 Chrome baseline을 따로 표시한다. |
| 30 | planning review를 implementation review로 재사용 | 통과 — 추후 구현/runtime 20개와 PR diff 30개를 새로 생성한다. |

이 검토는 계획 PR의 scope, 링크, 입력 계약과 실행 판정만 확인한다. 구현 동작·플랫폼 성공·C11.4 완료를 선언하지 않는다.
