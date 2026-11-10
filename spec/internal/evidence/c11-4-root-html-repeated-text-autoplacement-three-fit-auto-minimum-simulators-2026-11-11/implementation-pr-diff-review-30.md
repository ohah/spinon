# C11.4 three-fit auto-placement 구현 PR diff 적대 검토 30개

검토 대상은 현재 구현 branch diff 전체(계획 진행 상태, style/layout gates, Rust tests, Root HTML test fixture, simulator scripts/logs/screenshots, evidence README, implementation/runtime review)다. 계획 PR의 30관점 검토와 기능 구현/runtime 20관점 검토와 별도로 작성했다.

| # | 서로 다른 실패 관점 | 실제 diff 확인 및 방어 | 판정 |
|---:|---|---|---|
| 1 | 새 admission이 일반 CSS Grid로 노출되는가 | 조건은 parsed Root HTML fixed profile 내부에서만 평가되고 public CSS 지원 선언은 추가하지 않았다. | 통과 |
| 2 | public API/FFI surface가 바뀌는가 | 공개 타입·함수·header·FFI 파일 변경이 없다. | 통과 |
| 3 | 내부 interface numeric contract가 올라가는가 | 계약 `0.1.0`을 유지한다. | 통과 |
| 4 | official completion denominator가 바뀌는가 | plan과 roadmap 공식 완료 `78/214`, C11.4/C11 incomplete 표기를 유지한다. | 통과 |
| 5 | 계획 상태가 공식 완료와 혼동되는가 | 계획 문구는 bounded implementation, 검토 및 merge 관문을 별도로 적는다. | 통과 |
| 6 | auto-placement list 길이가 넓어지는가 | 두 layer에서 정확히 여섯 member만 허용한다. | 통과 |
| 7 | fit-count validation이 duplicate/missing tail을 놓치는가 | exact 3 fit 및 min/max/auto 각각 1개를 요구하고 invalid multiplicity를 거부한다. | 통과 |
| 8 | cap 검사가 position별로 누락되는가 | 각 FitContent member에 대해 finite non-negative direct px/% helper를 호출한다. | 통과 |
| 9 | cap test가 `calc` 또는 비유한 값 admission을 놓치는가 | projection/layout tests에 negative, NaN, infinity, Calc cases가 있다. | 통과 |
| 10 | 120 허용 permutation이 중복 또는 덜 생성되는가 | fit-position triple 20개와 tail 순서 6개를 조합해 total 120을 assert한다. | 통과 |
| 11 | 신규 predicate가 기존 one-fit/two-fit을 대체하는가 | 기존 helper 경로를 유지하고 full baseline/candidate runtime regressions를 별도 실행했다. | 통과 |
| 12 | row-span allowlist에 auto-placement semantics가 합쳐지는가 | span predicate 자체를 바꾸지 않았고 별도 11,520-scene row-span test를 돌렸다. | 통과 |
| 13 | test module이 production test harness 또는 artifact에 들어가는가 | 신규 pure predicate tests는 `#[cfg(test)]`; runtime tests는 기존 test module 아래 등록했다. | 통과 |
| 14 | test-only fixture feature가 신규 제품 ABI를 만드는가 | Cargo feature 또는 FFI symbol을 추가하지 않았다. Trace test는 기존 opt-in feature를 사용한다. | 통과 |
| 15 | RuntimeSession golden data가 미추적 로컬 파일에 의존하는가 | 관측 NDJSON은 이미 plan PR에서 고정·추적되었고 `include_str!` 경로가 이 file을 사용한다. | 통과 |
| 16 | fixture placeholder replacement가 잘못된 source를 조용히 받는가 | `replace_once`는 fragment 존재를 assert하고 single replacement만 수행한다. | 통과 |
| 17 | scene generator가 metadata row를 scene으로 셈하는가 | metadata record 및 5,760 scene 수를 각각 검증한다. | 통과 |
| 18 | min-height cohorts가 같은 key로 pair되지 않는가 | track list, alignment, DOM order key를 별도 set으로 대조해 2,880 pair 동등성을 assert한다. | 통과 |
| 19 | reverse DOM frame을 정렬 때문에 source order와 혼동하는가 | scene item list와 payload node IDs를 사용해 expected owner/frame을 확인하며 corpus에 두 order가 모두 있다. | 통과 |
| 20 | row cycle에서 explicit row와 implicit row를 혼합하는가 | control item-0과 implicit item ID를 분리하고 trace에서 explicit 1 + implicit 8 rows를 검사한다. | 통과 |
| 21 | baselines가 overflow의 positive control을 잃는가 | zero-minimum은 2,880 overflow scene, auto candidate는 0으로 별도 집계한다. | 통과 |
| 22 | item 내부 overflow와 Grid boundary overflow를 같은 결과로 취급하는가 | payload/item frame과 220px Grid end를 각각 비교한다. | 통과 |
| 23 | test helper가 payload node 대신 parent node를 읽는가 | expected payload IDs 및 `frame_for_payload`를 사용하며 이전 ID mismatch를 5,760 matrix 실행으로 교정했다. | 통과 |
| 24 | 실패 revision이 이전 layout/render/trace를 stale output으로 게시하는가 | rejection은 `completed=None`; 이후 revision은 새 key와 완성 render로 회복한다. | 통과 |
| 25 | trace assertions가 wrong Grid 또는 잘못된 floor를 받아들이는가 | 한 trace, owner node, current cascade key, automatic-minimum source/value, final row base size와 eligible set을 확인한다. | 통과 |
| 26 | simulator app source를 복원하지 않고 남기는가 | build script는 원본 두 source SHA를 확인하고 temp backup/trap 복원 뒤 hash log를 남긴다. | 통과 |
| 27 | Android/iOS capture가 fixture/diagnostic gate로 제품 완료를 과장하는가 | 기본 Root HTML app route에서 ready/draw-present만 검증하고 fixture-test flags는 off다. | 통과 |
| 28 | simulator 결과가 hardware/pixel parity로 표현되는가 | README는 API 37.2 emulator llvmpipe, simulator visual smoke, pixel/hardware 미검증 한계를 명시한다. | 통과 |
| 29 | screenshot/앱/시뮬레이터/browser artifact가 재사용 뒤 남는가 | 네 simulator 화면과 logs는 이 PR의 재현 근거로 보존하고 앱·기기 종료를 기록한다. PR image check용 Agent Browser는 고유 session으로 열고 확인 후 종료할 관문이다. | 통과 |
| 30 | PR 이미지/roadmap/status가 merge 전후 혼동되거나 링크가 깨지는가 | PR body에는 commit-pinned raw screenshot을 넣고 Agent Browser image load를 검증한 뒤 merge한다. merge 후 STATUS/spec/roadmap/Tailnet served hash를 따로 확인하며 이 slice로 공식 count를 바꾸지 않는다. | 통과 |

## 검토 결과

적대 검토 과정에서 수정한 것은 test fixture marker/실제 Grid owner mismatch, duplicate indefinite-height mutation, test-only Clippy range-loop warnings와 누락된 7-member/RTL/invalid-text rejection case다. 현재 diff에 해결되지 않은 blocker를 찾지 못했다. 이 기록은 실제 implementation PR diff의 30관점 검토이며 plan review와 implementation/runtime 20관점을 대체하지 않는다.
