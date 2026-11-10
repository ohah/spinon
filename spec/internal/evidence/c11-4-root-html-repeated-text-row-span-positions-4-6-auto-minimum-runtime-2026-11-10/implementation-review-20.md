# C11.4 row-span positions 4–6 자동 최소 크기 구현·런타임 검토 20개

검토 대상은 기본 parsed Root HTML의 single-fit repeated-text row-span 제한 profile이다. C11.4/C11 전체, 일반 CSS Grid, 공개 API 지원으로 넓혀 해석하지 않는다. 각 항목은 별도 실패 경계다.

1. **Profile 진입 전 모양 검사** — 6-member track list와 span topology 검사를 통과한 경우만 새 automatic-minimum candidate에 도달한다. 지원 밖 목록은 기존 `UnsupportedGridAutoRows`/container-style 오류 경계를 유지한다.
2. **행 수와 fit 위치** — 세 implicit row는 authored positions 1–3, 여섯 implicit row는 4–6만 허용한다. 이 slice에서 six-row position 3을 negative control로 거부한다.
3. **Fit 개수** — 새 후보는 single-fit 목록에 한정한다. 기존 two-fit row-span profile의 admission과 `ResidualRounding::Up` 경로를 변경하지 않는다.
4. **Track multiset** — 여섯 member 안에 `min-content ×2`, `max-content ×2`, `auto ×1`, `fit-content ×1`이 있어야 한다. 반복 수·유연 track·길이가 아닌 임의 member를 허용하지 않는다.
5. **Cap 범위** — cap은 유한한 non-negative px 또는 percentage뿐이다. 음수, NaN/무한대, `calc()`는 profile에서 빠진다.
6. **자식 minimum 일관성** — 첫 explicit-row control 뒤 span text item 모두의 computed min-height가 `auto`여야 한다. item마다 `auto`/`0`을 섞는 입력은 거부한다.
7. **Control 보존** — 첫 empty explicit-row item은 계속 min-height `0`이어야 한다. 이를 `auto`로 바꾸면 profile에 들어오지 않는다.
8. **Fit 위치 판정의 단일 출처** — style projection의 broadened item helper는 `span_profile`과 fit 개수를 확인하며, 실제 positions 4–6 경계는 공용 layout profile 검사가 결정한다.
9. **명시·암시 행 경계** — 성공 trace에서 explicit track 하나와 positive implicit track 여섯 개를 검사한다. negative implicit row나 임의 길이 행 배열은 새 분포 규칙에 진입하지 않는다.
10. **Text grammar 제한** — 직접 자식 text payload의 고정 ASCII word grammar, monospace 10px, line-height 10px 조건을 유지한다. Unicode·임의 폰트·중첩 자식은 이 변경에서 열지 않는다.
11. **Automatic minimum 원인** — rounding gate의 trace는 `Minimum` contribution 중 `AutomaticMinimum`이 auto-min track을 span하고, 여러 track을 span하며, flexible max track 없이 content-based인 경우만 인정한다.
12. **예상 floor** — trace test는 각 eligible item의 contribution을 관측한 line rectangle 수 × 10 CSS px와 비교한다. item의 줄 수가 기대 최소보다 낮아지는 값을 허용하지 않는다.
13. **Owner와 revision** — layout trace는 실제 단일 Grid owner 및 동일 revision의 sizing output을 확인한다. stale revision이나 다른 owner의 trace를 현재 결과에 재사용하지 않는다.
14. **행 수 기반 양자화** — 새 residual policy는 결과가 정확히 7 used rows이고 single-fit span automatic minimum이 trace에 있을 때만 선택한다.
15. **기존 반올림 정책** — 새 조건 밖은 `Nearest`, 기존 two-fit은 `Up`을 유지한다. 한정 조건을 일반 행 크기 분포 규칙으로 승격하지 않는다.
16. **Caps 아래 잔여 단위** — fit-content track에서 최소 기여가 cap보다 큰 경우에도 `UpLaterTrack`은 rounded size가 기존 base보다 최대 1/64 CSS px 위인 경우만 후보로 받는다.
17. **포화 track** — 기존 nearest/up 경로의 cap 포화 거부 및 0.01px tolerance 검사는 유지한다. 관련 unit test는 cap을 넘는 saturated row를 remainder 후보에서 제외한다.
18. **잔여 단위 순서** — 동률 growth potential에서 `UpLaterTrack`만 authored 뒤쪽 row를 우선한다. 다른 모드의 auto-track tie-break 순서는 유지한다.
19. **Alignment stretch** — pre-alignment 양자화 뒤 stretch 단계는 auto row에만 free units를 분배한다. paired matrix는 non-auto track이 변하지 않고 auto row가 줄지 않는지 검증한다.
20. **거부·복구와 oracle 범위** — mixed minimum, position 3, control `auto`의 atomic rejection과 같은 RuntimeSession의 light→dark reject→light recovery를 둔다. 4,320 paired Chrome scenes는 행렬 근거이며 WPT, hardware GPU, real device 또는 compositor pixel parity를 주장하지 않는다.

집중 실패·복구·trace·0/20px cap rounding 5개 테스트는 `focused-runtime-tests.log`에 기록한다. 전체 paired matrix 결과와 세부 수량은 완료 후 `README.md` 및 `six-row-matrix-final.log`에 연결한다.
