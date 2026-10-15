# C13.3 typed style projection PR 변경 적대 검토 30개

검토 대상은 이번 branch의 source, test, contract, status 및 evidence delta다. 구현 리뷰 20개와는 다른 PR 경계/회귀 관점으로 각각 확인했다.

| # | PR delta 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | `ComputedStyleProfile` variant 누락 | snapshot enum과 cascade allowlist 모두 C13.3 variant를 추가했다. |
| 2 | 소비자가 새 profile을 외부에서 고를 수 없음 | `spinon-style`와 `stylo_dom` 공개 re-export에 compute 함수와 profile/value type이 연결돼 있다. |
| 3 | overscroll enum이 snapshot 밖에만 존재 | `ComputedElementStyle`에 x/y typed field가 있고 element projection에서 설정한다. |
| 4 | touch-action output field가 profile 구분 없이 채워짐 | 해당 profile만 세 input field를 채운다. 다른 computed style 생성 경로는 새 값의 default `None`을 유지한다. |
| 5 | runtime rendering에 새 CSS 값이 암묵 연결됨 | 변경은 style-to-layout projection까지이며 renderer input wiring을 추가하지 않았다. |
| 6 | C13.3을 C13 전체 완료로 표시 | `STATUS.md`, plan, contract는 여전히 C13.3 전체 미완료다. |
| 7 | 이전 C13.1 allowlist가 넓어짐 | C13.1 함수는 수정하지 않았고 기존 profile에서 touch-action 거부 회귀 테스트를 둔다. |
| 8 | 새 allowlist에서 C13.1 property 누락 | 기존 C13.1 property list를 확장해 중복 제거 후 새 속성만 더한다. |
| 9 | logical axes를 실수로 public support에 포함 | 추가 allowlist에는 physical longhand와 shorthand만 있다. |
| 10 | error type 없이 string으로만 실패 | `UnsupportedTouchActionValue`는 source ID, inline node 선택값, 원 token을 보존한다. |
| 11 | view rebuild error가 panic으로 바뀜 | cascade API는 `StyloViewRebuild` error를 반환한다. |
| 12 | inline preflight의 기존 `expect`가 숨겨짐 | 이 함수는 이미 검증된 view root/snapshot 재빌드를 내부 불변식으로 한정한다. 오류 이유를 `expect`에 명시했다. |
| 13 | scanner가 stylesheet 전역 중첩 규칙을 놓침 | `StyleSheetParser`/`RuleBodyParser`로 grouping 및 nested rule body를 재귀 순회한다. |
| 14 | excessive nested CSS가 무한 재귀 | 최대 깊이 64에서 현재 component values만 drain한다. |
| 15 | scanner가 문자열/괄호 안의 가짜 `pan-left`를 선언으로 읽음 | parser token type으로 선언 value만 판별하고 nested component values는 별도 drain한다. |
| 16 | C13.3 parser 경계에서 기존 C10 baseline CSS 필터 사라짐 | 기존 `compute_cascade`/registry append path를 그대로 통과한다. |
| 17 | UA stylesheet source order 변동 | `compute_cascade_with_reuse`의 UA-first, author registration order를 유지한다. |
| 18 | error diagnostic source order 바뀜 | 새 preflight는 선언 손실 위험만 앞서 거부하며 나머지는 기존 registry/cascade ordering을 쓴다. |
| 19 | style revision 또는 document revision을 새로 만들지 않음 | 호출 인자와 기존 snapshot 생성 경로를 그대로 사용한다. |
| 20 | 새 profile의 incremental API가 잘못된 cache hit을 허용 | `runtime_properties_for_profile`에서 profile을 빼 full recompute fallback을 유지한다. |
| 21 | profile에 background paint가 빠짐 | runtime paint profile switch에 새 profile을 추가했다. |
| 22 | layout projection이 새 profile에서 빠짐 | overflow/positioning projection을 사용하는 기존 profile match에 C13.3 variant를 추가했다. |
| 23 | `ComputedElementStyle` 새 필드 일부 export/import 실패 | workspace 전체 compile/tests와 Clippy 통과. |
| 24 | test source가 기본 제품 runtime에 포함 | C13.3 unit tests와 style-to-layout test는 `cfg(test)` 아래 등록했다. |
| 25 | app fixture ABI/payload를 default artifact에 추가 | 이 delta는 FFI, Cargo feature, Android/iOS build flag를 변경하지 않는다. |
| 26 | PR 설명의 app screenshot이 실제 구현 근거인 것처럼 보임 | style-only slice에는 app route가 없어 Simulator/screenshot을 만들지 않았다고 기록한다. |
| 27 | Chrome oracle 화면을 Spinon 동작으로 오인 | oracle screenshot은 별도 precomparison 자료에 남고 이 PR의 runtime 증거로 쓰지 않는다. |
| 28 | spec status가 focused test 수를 과거 값으로 표시 | 이번 focused style 결과를 8/8로 갱신하고 전체 workspace 결과도 추가했다. |
| 29 | 구현 계약과 처리 가능한 Stylo 값의 차이가 문서에서 누락 | 계약/계획에 `Unsupported` 보존 값과 방향 pan/`var()` profile error를 구분했다. |
| 30 | unrelated original worktree changes가 branch로 유입 | 별도 `/private/tmp` worktree에서만 편집했다. 원본 checkout은 읽기 전용으로 확인했고 수정하지 않았다. |

`README.md`의 test, Clippy, rustfmt, diff-check log와 수정된 spec를 다시 대조했다. 이 PR은 typed style projection slice만 제공하며 native gesture 동작을 주장하지 않는다.
