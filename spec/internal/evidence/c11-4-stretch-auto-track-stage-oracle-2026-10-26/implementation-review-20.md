# C11.4 §11.8 auto-track stretch oracle · 구현 실패 관점 검토 20개

검토 대상은 새 test-only `spinon-layout` module, `lib.rs`의 `#[cfg(test)]` 등록, 계획·실행 근거다. 검토 중 테스트를 실행하거나 코드를 수정하지 않았다.

1. **제품 코드 경계** — 새 파일은 `#[cfg(test)]`로만 등록돼 제품 library/API 경로를 바꾸지 않는다. **PASS**
2. **fixture boundary 오염** — 앱용 FFI/adapter나 opt-in fixture feature를 추가하지 않았다. **PASS**
3. **규범 단계 식별** — 대상은 §11.8 `Stretch auto Tracks`이며 §11.6 `Maximize Tracks`와 구분돼 있다. **PASS**
4. **definite axis 조건** — root height와 viewport height가 모두 100px이고 root sizing은 `Match`다. **PASS**
5. **trace 활성화 조건** — `GridPlacementMode::TrackSizing`을 설정해 기존 trace 경로를 활성화한다. **PASS**
6. **pass 축 선택** — `Rows` pass만 선택하고 현재 단일 Grid 입력에서 pass가 하나인지 검사한다. **PASS**
7. **trace owner 확인** — `rows_pass`는 trace 수를 1개로 고정하고 `LayoutGridTrackSizingTrace.node == id(1)`을 assert한 뒤 해당 root의 row pass만 선택한다. **PASS**
8. **track 수와 배치** — one-column Grid에 세 direct child를 DOM 순서대로 두고 세 explicit row를 제공한다. 입력 topology가 고정돼 있다. **PASS**
9. **contribution 안정성** — item heights `[10,20,5]`는 고정값이며 text wrapping, font, replaced content에 의존하지 않는다. **PASS**
10. **gutter 산술** — 세 row 사이에 내부 row gap 두 개가 있고 각각 10px이며 free-space 계산 `100 - 35 - 20 = 45px`와 일치한다. **PASS**
11. **eligible track 식별** — 두 `auto` maximum tracks만 늘어나며 third `LengthPx(5)` track은 `[5]`로 검사된다. **PASS**
12. **공평 분배** — 45px 잔여 공간이 두 auto tracks에 22.5px씩 더해져 `[32.5,42.5,5]`가 된다. **PASS**
13. **선행 stage snapshot** — `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks` 각각에서 `[10,20,5]`를 검사한다. **PASS**
14. **stage 순서** — 위 세 stage 뒤에 유일한 `StretchAutoTracks`가 오는지 확인한다. **PASS**
15. **stage 적용 flag** — stretch 입력은 `{ applied:true }`, `start` 대조 입력은 `{ applied:false }`를 정확히 구분한다. **PASS**
16. **negative control 결과** — `start` 입력에서 rows `[10,20,5]`와 남은 45px를 확인해 분배가 과도하게 적용되는 회귀를 잡는다. **PASS**
17. **CSS track/gutter 구분** — snapshot vector 길이를 고정하고 `is_gutter`의 alternating pattern, 두 boundary gutter의 `is_collapsed` sentinel flag, gutter 순서·크기를 검사한다. README의 collapsed-boundary 설명과 assertion이 일치한다. **PASS**
18. **stage values의 유한성** — `close()`는 각 검사값이 finite이고 오차 `1e-4` 이하인지 요구한다. **PASS**
19. **frame 계산** — positive/negative root frame과 세 child의 x/y/width/height를 각각 고정 기대값으로 검사한다. **PASS**
20. **범위·상태 과장** — 실행 근거는 test-only 결과로 한정하고 simulator/browser parity를 주장하지 않는다. 계획과 parent index는 구현 완료·20/30 리뷰 진행 중이라고 기록하며 C11.4와 공식 `72/213`은 미완료로 유지한다. **PASS**

## 판정

**최종 판정: PASS — 20/20, 차단 항목 0개.** 첫 검토에서 PARTIAL이던 #7과 #17을 코드 보완 뒤 재검토했다. trace owner와 단일 trace 수, gutter alternation 및 collapsed boundary flags가 이제 직접 검사된다. 마지막 focused-test log는 1/1, fmt-check와 all-target Clippy 로그도 통과를 기록한다. 계획·상위 index의 구현 상태, 검토 진행 상태 및 C11.4 미완료·공식 `72/213` 유지 문구는 결과를 과장하지 않는다.
