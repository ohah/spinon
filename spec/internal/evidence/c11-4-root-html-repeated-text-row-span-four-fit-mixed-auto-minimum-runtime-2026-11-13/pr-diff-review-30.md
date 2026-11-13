# Four-fit mixed minimum PR diff 실패 관점 검토 30개

대상은 계획 PR #1374 병합본 위의 C11.4 four-fit row-span candidate 전체 diff다. Rust admission/layout, RuntimeSession fixture, 실행 근거와 공개 로드맵을 각각 다른 경계로 확인했다. 전체 Chrome oracle 138,240개와 RuntimeSession 표본 3,840개는 분리해서 판단했다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 다른 fit 개수가 mixed four-fit path로 들어옴 | Style과 layout 모두 여섯 track 중 정확히 네 `fit-content()`를 요구한다. 세·다섯·여섯 fit 경로는 기존 분기로 남는다. |
| 2 | `min-content`/`auto` tail multiplicity가 늘거나 바뀜 | 두 admission 계층에서 singleton `min-content`와 singleton `auto`를 모두 요구한다. Tail의 다른 조합은 four-fit gate를 통과하지 않는다. |
| 3 | cap multiset에 중복·누락·다른 단위가 들어옴 | 네 typed cap `0px`, `10px`, `20%`, `30px` 각각 한 번인 기존 helper만 승인한다. |
| 4 | authored cap 순서가 정렬·정규화 중 유실됨 | 입력 track vector를 재정렬하지 않고 720 order를 비교한다. Chrome full oracle과 uniform endpoint 회귀가 전체 순서 분모를 유지한다. |
| 5 | percentage cap이 indefinite basis에서 평가됨 | 기존 Root HTML profile의 definite `40×220px` owner 조건을 재사용한다. Runtime matrix는 모든 selected track-list context의 computed/used 결과를 비교한다. |
| 6 | Style projection과 layout validator의 track 판정이 갈라짐 | projection이 승인한 four-fit row-span list와 layout의 exact cap helper를 대조했다. 둘 다 member 수·tail·cap 조건이 동일하다. |
| 7 | auto-placement/nested Grid가 row-span profile을 우회함 | 기존 repeated-text span owner/topology 검증 뒤 새 mask gate가 계산된다. 기존 auto-placement matcher를 변경하지 않았고 5,760-scene regression을 통과했다. |
| 8 | implicit row 수 또는 explicit row 높이가 profile 밖으로 확장됨 | simulator는 한 explicit row와 여섯 implicit row를 검사한다. Chrome records의 used rows와 RuntimeSession trace가 일치했다. |
| 9 | 첫 빈 explicit-row control이 `auto`가 됨 | Style/layout predicates 모두 첫 child를 computed `0px`로 고정한다. RuntimeSession은 control을 `auto`로 바꿔 원자적 거부와 회복을 확인한다. |
| 10 | span item의 임의 `min-height`가 mask로 간주됨 | 새 profile은 정확한 `0px` 또는 `auto`만 허용한다. 실제 computed `1px` mutation은 실패하고 output을 게시하지 않는다. |
| 11 | child 수·direct-text payload·owner 경계가 넓어짐 | 기존 two-topology repeated-text parser/owner 검사 안에서만 활성화된다. 추가 item, nested owner와 바뀐 payload는 기존 rejection 경계에 남는다. |
| 12 | zero-only endpoint가 새 Blink replay로 바뀜 | four-fit replay는 spanning item과 mixed zero/auto mask가 함께 있을 때만 열린다. 기존 all-zero/all-auto endpoint matrix 11,520 scenes가 통과했다. |
| 13 | five-fit mixed replay의 기존 조건이 약화됨 | 분기 선택은 fit count별로 분리되어 있고, 공유 stretch trace helper의 이름만 generalize했다. 기존 five-fit mixed tests 결과를 유지한다. |
| 14 | four-fit replay가 비-span 또는 잘못된 trace를 사용함 | `has_spanning_item`, mixed minimum 상태, six-row input, 4–6 fit count, growth-limit trace 조건이 확인된다. Replay 불가 시 candidate는 수용되지 않는다. |
| 15 | `auto` eligibility가 scroll/flexible 조건 밖으로 확대됨 | 지원 profile은 visible non-scroll overflow와 고정 track topology만 허용한다. eligibility assertion은 각 span의 `auto|fit-content()` 최소 함수에 한정한다. |
| 16 | 새 branch가 기존 residual rounding 선택 순서를 바꿈 | 기존 rounding mode 선택은 같은 precedence로 보존되고 새 four-fit mixed gate에만 simulator가 연결된다. 기존 uniform/trace 회귀가 통과했다. |
| 17 | replay가 Grid total의 1/64px residual을 버림 | total target을 기존 rounding mode에서 계산하고 simulated total이 부족할 때만 fallback residual 복구를 시도한다. mixed sample과 세 지정 rounding boundary가 통과했다. |
| 18 | residual 복구가 임의 행을 늘리거나 초과 증가함 | fallback vector 길이와 총합을 먼저 검증하고 fallback이 허용한 양까지만 뒤에서 복원한다. 남은 residual이 있으면 `None`으로 닫는다. |
| 19 | 잘못된 residual fallback에서 partial rows가 노출됨 | 복구·최종 track 대조 중 하나라도 맞지 않으면 layout candidate가 거부된다. RuntimeSession 실패 snapshot은 `completed=None`이다. |
| 20 | `align-content:stretch`가 제한 없는 행 외의 track을 바꿈 | auto tail residual은 singleton stretch auto row에만 적용하고 나머지 row는 final track trace와 대조한다. 전체 alignment pair가 통과했다. |
| 21 | source-order 반전으로 semantic item frame이 바뀜 | 20개 track-list 각각 96 forward/reverse pair에서 used rows와 semantic ID별 item/payload frame을 비교했다. 허용 차이는 `0.01 CSS px`다. |
| 22 | sample 누락·중복·재정렬을 정상 실행으로 오인함 | metadata test가 sample index 0–3839, 원본 index 오름차순, 20개 list당 192 scenes와 Chrome oracle hash를 고정한다. |
| 23 | mask/topology/order coverage가 균형을 잃음 | sample은 15개 fit-position set, 선택된 20 cap order, 양 tail order, 32/16 mask의 forward/reverse topology를 확인한다. |
| 24 | auto-minimum floor 결과를 zero baseline과 합산함 | expected/actual shortfall을 zero/auto 두 cohort로 따로 집계한다. auto cohort는 0이고 zero control에서 1,024건이다. |
| 25 | Grid-end overflow를 floor shortfall 또는 무조건 0으로 덮음 | overflow는 payload별 별도 expected/actual 비교이며 zero/auto 결과 `[8, 0]`을 유지한다. |
| 26 | span row frame·payload frame·used rows 중 하나만 비교함 | 각 selected scene에서 `observe_current`와 row vector 비교를 수행하고 paired test에서 semantic item 및 payload map을 함께 비교한다. |
| 27 | 실패 revision이 이전 layout을 새 결과처럼 보임 | control auto와 item `1px` rejection 둘 다 `completed=None`을 요구하고 같은 `RuntimeSession`의 valid revision으로 회복한다. |
| 28 | fixture가 기본 제품 artifact/API에 노출됨 | 변경된 harness는 기존 opt-in `c11-4-grid-track-sizing-fixture-test` 아래 RuntimeSession tests에 등록된다. 공개 FFI/API와 product admission 선언은 추가되지 않았다. |
| 29 | 모바일 startup을 mixed feature 검증이나 pixel parity로 확대 보고함 | iOS/Android 증거는 기본 Root HTML route의 `layout-ready`/draw-present smoke뿐이다. Android headless 성공과 cold-boot/GUI 실패를 분리하고 mixed Grid 모바일 실행은 미검증으로 표기했다. |
| 30 | 임시 환경 흔적·roadmap 숫자가 완료 판정을 오염시킴 | task-owned AVD와 임시 iOS profile을 제거하고 공유 API 37 AVD만 재사용 목적으로 남겼다. 저장된 emulator log의 ADB key/host label은 가렸고 roadmap·contract에서 공식 `78/214`, C11.4/C11 미완료를 유지한다. |

검토 중 source admission의 범위 확대, partial layout, matrix coverage 또는 문서의 completion overclaim은 발견되지 않았다. `runtime-mixed.log`는 6 passed/0 failed와 3,840 scene을 기록한다. `cargo fmt --all -- --check`와 변경 crate strict Clippy 결과는 runtime README에 연결되어 있다. 전체 workspace strict Clippy의 6개 기존 lint는 변경하지 않은 테스트 파일에서 발생해 이 candidate의 통과 근거로 세지 않았다.
