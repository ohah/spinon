# 반복 text auto-placement fit-content 위치 제한 구현·런타임 검토 20개

검토 대상은 C11.4의 고정 9-item direct-text Root HTML profile이다. `spinon-style-to-layout`, Layout admission·trace 정규화, RuntimeSession matrix와 native smoke harness를 서로 다른 실패 경로로 대조했다. 초기에 20번 회귀 검토가 기존 row-span 보정 경로의 trace 단계를 깨는 문제를 발견했다. 보정 산술을 공유하되 기존 span trace 규약을 유지하도록 분리한 뒤 해당 1,920-scene 회귀를 다시 실행했다.

1. **Track-list 길이** — 다섯·일곱 member list가 새 six-member predicate로 들어가지 않는다. Layout·projection admission에서 거부하는 검사를 확인했다.
2. **Multiplicity** — `fit-content` 1개, `min-content` 2개, `max-content` 2개, `auto` 1개의 개수만 허용한다. 중복·누락 control을 유지한다.
3. **Fit 위치** — `fit-content`의 index 0–5가 모두 projection·Layout에서 보존된다. authored order를 정렬하거나 정규화하지 않는다.
4. **Fit cap 입력** — finite non-negative px와 percentage만 허용한다. 음수, 비유한 값, 계산식과 중복 fit track은 거부한다.
5. **Percentage basis** — definite Grid content block-size를 분모로 사용하고 indefinite basis에서 추측값을 쓰지 않는다.
6. **Zero 및 과대 cap** — `0%`는 0 row 경계를 보존하고 `120%`는 cap 자체를 줄이지 않는다. 0 크기 row 및 visible overflow scene을 확인한다.
7. **Explicit row 분리** — 10px explicit row는 여섯-member implicit cycle에서 빠진다. item row와 authored auto-row index 대응을 재검토했다.
8. **반복 cycle** — 여덟 implicit row에서 위치 1·2의 두 번째 출현을 검증한다. track 위치를 item 간 y 차이로 추정하지 않는다.
9. **Grid 소유자 수** — 둘 이상의 대상 Grid를 한 번의 정규화에 묶지 않고 fail-closed한다.
10. **Revision 일치** — Layout input과 output revision 불일치, stale item/flow frame을 새 결과 보정에 사용하지 않는다.
11. **Visible child shape** — 빈 explicit item 하나와 auto-placement item 여덟 개만 대상이다. hidden subtree와 추가·누락 child 경계가 유지된다.
12. **Text payload shape** — 각 wrapper의 direct payload 하나, 고정 ASCII 문법, 단일 text source 조건을 확인했다. nested/replaced/inline-run payload는 받아들이지 않는다.
13. **Numeric fixture profile** — legacy 및 compact payload-height 배열은 각각 전체 child에서만 선택되고, 항목마다 서로 섞인 배열은 거부된다.
14. **Row trace 소유권** — 신규 profile은 target Grid trace가 하나여야 한다. 누락·중복 trace와 잘못된 최종 sequence는 보정하지 않는다.
15. **Track-state integrity** — gutter를 제외한 track 수·내부 index·collapse flag·base size·growth limit의 유한성과 비음수성을 확인한다.
16. **Used-row serialization** — `used_track_list`의 px 값을 최종 track base와 대조한다. 일부 값만 빠지거나 final trace와 다른 list는 거부한다.
17. **Auto stretch 경계** — `stretch`의 free space가 authored `auto` row에 균등 분배되고 다른 row는 변하지 않는지 확인한다. Overflow에서는 음수 stretch를 만들지 않는다.
18. **Frame 선검증** — 모든 item과 flow frame이 raw track trace의 x/y/width/height에 맞는지 mutation 전에 검사한다.
19. **원자적 보정** — 모든 row 보정량을 먼저 계산한 뒤 descendants의 frame과 flow frame을 갱신한다. 중간 row 실패가 부분 output을 남기지 않는다.
20. **기존 row-span 경계** — 새 strict auto-placement trace 검증이 span profile로 새어 들어가던 회귀를 발견했다. span용 trace extraction은 이전 단계 규약을 유지하고, Chrome 고정 1,920-scene RuntimeSession 행렬로 복구를 확인했다.

검토 결과: 신규 profile 제한과 부정 경계는 유지된다. 제20항 수정 후 projection, Layout, RuntimeSession, 기존 row-span, Clippy 및 rustfmt 최종 실행 로그는 이 폴더의 `rust-verification/`에 기록한다.
