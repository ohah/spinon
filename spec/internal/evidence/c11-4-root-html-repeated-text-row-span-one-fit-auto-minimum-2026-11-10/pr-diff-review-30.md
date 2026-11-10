# 구현 PR diff 실패 관점 검토 30개

실제 변경 diff를 코드·fixture·증거·지원 범위 경계별로 검토했다. 독립 implementation/runtime 검토 20개와는 별도의 30개 관점이다. 최초 검토에서 드러난 회귀 테스트·rounding·runtime line-count·fixture parsing·기본 테스트 비용 문제를 반영한 뒤 변경 영역을 다시 확인했다. 현재 수정분에서 미해결 제품 동작 결함은 찾지 못했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | one-fit 이외의 track list가 새 후보로 들어오는가 | 새 후보가 fit-content track 한 개를 검사한다. |
| 2 | six-member list의 지원되지 않는 fit 위치가 허용되는가 | three-row profile은 1–3 위치만 받아들인다. |
| 3 | three-row 이외의 span 길이가 자동 최소 크기에 연결되는가 | 후보 row 수를 정확히 세 개로 제한한다. |
| 4 | explicit-row control의 minimum이 달라지는가 | 후보 control은 `min-height:0`이며 invalid control 거부가 있다. |
| 5 | text item 사이에 auto와 zero minimum이 섞이는가 | all-auto일 때만 admission되고 mixed 입력은 거부된다. |
| 6 | CSS와 layout 사이에서 Grid item 수가 달라지는가 | single owner와 visible child 개수 검증이 유지된다. |
| 7 | text payload가 여러 개거나 다른 grammar를 사용하는가 | 기존 direct-text payload와 child count 제한이 적용된다. |
| 8 | span topology, source order, static positioning 범위를 넘는가 | 기존 placement profile과 명시적 topology allowlist가 유지된다. |
| 9 | scroll container 또는 flexible track에서 자동 minimum 조건이 바뀌는가 | visible overflow 및 flexible-track 배제 검사가 유지된다. |
| 10 | layout predicate와 typed style projection predicate가 다르게 통과시키는가 | 두 계층이 one-fit·row count·minimum profile을 각각 검증한다. |
| 11 | fit-content maximum cap이 minimum contribution을 잘라내는가 | Taffy helper test가 minimum base를 cap 너머까지 늘린다. |
| 12 | maximum contribution이 cap을 넘어가는가 | 같은 helper test가 maximum growth를 cap에서 멈춘다. |
| 13 | row 전용 동작이 다른 축 공유 로직에 숨어서 다르게 적용되는가 | 축 인수 없는 공용 Taffy 분배 helper에서 min/max 분기를 직접 검증한다. Full column layout은 이번 Root HTML 범위 밖이며 일반 CSS Grid 완성을 주장하지 않는다. |
| 14 | nearest rounding이 cap 근처에서 잘못 허용되거나 거부되는가 | 0.01px tolerance 안쪽 반올림과 rounded-down case를 고정한다. |
| 15 | 반올림 불가 행 때문에 다른 행의 잔여 단위 분배가 막히는가 | 다중 행 테스트가 0.098 행을 건너뛰고 0.105 행에 남은 단위를 배분한다. |
| 16 | 남은 1/64px 배분이 정수 단위 총합과 맞지 않는가 | layout track trace `blink_` 경계 테스트 9/9가 통과했다. |
| 17 | zero cap 또는 overlapping spans에서 min contribution이 사라지는가 | zero-cap RuntimeSession trace가 두 span의 contribution과 최종 row 합을 확인한다. |
| 18 | minimum contribution의 source·owner·stage가 잘못 기록되는가 | trace가 `AutomaticMinimum` flags, node owner와 final row stage를 대조한다. |
| 19 | Grid item frame은 맞지만 실제 Runtime text wrapping이 다른가 | glyph baseline으로 센 Runtime line count를 Chrome `lineRects`와 비교한다. |
| 20 | payload가 item bottom 밖으로 그려지는가 | frame·payload frame과 Chrome visible bottom overflow를 함께 검증한다. |
| 21 | 실패한 revision이 이전 published output을 부분 교체하는가 | rejection test에서 failed revision output이 비어 있음을 확인한다. |
| 22 | reject 이후 동일 session의 정상 revision이 복구되지 않는가 | color-scheme 변경을 이용한 same-session recovery가 통과했다. |
| 23 | 고정 precomparison fixture가 손상되어도 잘못된 position을 기본값으로 해석하는가 | 누락된 legacy key만 위치 1로 처리하고 존재하는 malformed 값은 fail-closed한다. |
| 24 | 2,880-pair 행렬이 기본 runtime test에 수 분 비용을 추가하는가 | 신규 큰 matrix 모듈을 `c11-4-grid-track-sizing-fixture-test` feature로 제한한다. |
| 25 | source generator 변경이 과거 960-scene profile을 깨뜨리는가 | 기존 repeated-text row-span matrix 960 scene이 통과했다. |
| 26 | existing two-fit 또는 one-fit auto-placement 제한 경로가 바뀌는가 | candidate-specific legacy-floor guard를 적용했고 관련 prior regression 결과를 README에 기록했다. |
| 27 | vendored Taffy 수정의 의미가 코드와 다르게 문서화되는가 | `vendor/taffy/PATCHES.md`가 minimum base와 maximum cap의 차이를 설명한다. |
| 28 | test-only trace나 fixture payload가 기본 제품 API로 노출되는가 | 새 matrix 코드는 `#[cfg(test)]`와 opt-in fixture feature 아래에 둔다. 공개 API·FFI 선언은 추가하지 않았다. |
| 29 | simulator 증거가 잘못된 HTML/build와 연결되거나 손상되는가 | baseline/candidate source SHA, 기본 asset 복원, Android/iOS logs/screenshots 및 SHA256SUMS를 연결했다. |
| 30 | 변경을 C11.4/C11 전체 완료 또는 깨끗한 broad suite로 과장하는가 | 계획·README는 공식 `78/214`와 상위 항목 미완료를 유지하고 broad suite 실패를 별도 보고한다. |

PR 생성 전 로컬 diff 검토 결과다. PR 생성 후 Agent Browser에서 고정 raw screenshot 네 장이 모두 로드됨을 확인했다(Android light/dark 1080×2400, iOS light/dark 1206×2622). GitHub check-run 0개, status 0개, review 0개였고 CI green으로 해석하지 않았다. PR [#1326](https://github.com/ohah/spinon/pull/1326)은 2026-11-10T05:39:19Z Rebase and merge됐으며 merge SHA는 `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`다. 정확한 session/process 정리와 simulator 상태는 [실행·화면 근거](./README.md)에 기록했다.
