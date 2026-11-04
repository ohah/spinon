# C11.1 Root HTML direct-text aspect-ratio 값 구현·런타임 검토 20개

**대상:** `impl/c11-root-html-grid-direct-text-aspect-ratio-values-20261105`의 구현 diff와 2026-11-05 실행 근거. 계획의 실패 관점 20개와 분리한 검토다.

| # | 실패 경로·불변 조건 | 확인 및 판정 |
|---:|---|---|
| 1 | 숫자 단일값과 분수 표기가 같은 typed 비율을 내는가 | 기본 cascade 테스트에서 `1.5`→`1.5 / 1`, `3 / 2`→`3 / 2`, 두 typed ratio `1.5`를 확인했다. 통과. |
| 2 | CSSOM 문자열을 수치 출처로 삼아 typed 값과 어긋나는가 | projection은 `layout_aspect_ratio`를 수치로 사용하고 computed serialization은 numeric/auto 판별에만 사용한다. 코드와 cascade 테스트를 대조했다. 통과. |
| 3 | 기본 text profile의 제한을 block profile 또는 다른 profile로 넓히는가 | cascade 검증은 숫자 비율을 기본 text profile에서만 인정하고 block profile은 `aspect-ratio`를 계속 거부한다. 통과. |
| 4 | `auto <ratio>`의 content-box 의미가 box-sizing 비율로 오인되는가 | typed sizing-box가 `BoxSizing`인 경우만 통과하고 기존 `auto <ratio>` 거부 테스트를 유지한다. 통과. |
| 5 | 0·음수·NaN·무한대 ratio가 admission을 통과하는가 | projection의 finite-positive guard, zero-ratio rejection, layout 단위 NaN rejection을 확인했다. 통과. |
| 6 | 유한 ratio transfer와 여러 absolute frame이 끝점 계산에서 범위를 넘는가 | 양 축 transfer 결과에 finite-positive 검사를 두고 `1e-38`, `1e38` 대조를 거부한다. 추가 검토에서 두 개의 개별 `3e38px` row는 각각 유한하지만 두 번째 `y + height`가 무한대가 되는 것을 재현했다. 공통 frame collector가 x/y 위치와 끝점을 모두 확인해 `NonFiniteFrame`으로 거부하도록 보강했고 회귀 테스트가 통과했다. |
| 7 | 고정 width→auto height 전이가 나눗셈 방향을 뒤집는가 | `width:60px` 장면의 `1/2`, `4/3`, `3/2`, `5/2` 결과는 높이 `120/45/40/24px`다. projection/layout/runtime matrix가 독립 기대값과 맞는다. 통과. |
| 8 | 고정 height→auto width 전이가 나눗셈으로 잘못 처리되는가 | height `30px` 장면의 `4/3`, `3/2`, `5/2` 결과는 폭 `40/45/75px`다. 세 matrix가 확인한다. 통과. |
| 9 | 비율 전이 후 auto row가 target 높이를 반영하지 않는가 | 14 scene matrix가 width-locked scene의 row와 target 높이를 각각 검사한다. 오차 허용은 Rust 0.01px, runtime 0.5px다. 통과. |
| 10 | 10:1 ratio가 6px로 줄어 Ahem 한 줄의 automatic minimum을 깨뜨리는가 | 정확한 placement mode·target·nowrap paragraph에서 line-height를 `min-height`로 전달한다. ratio 10은 target/row 10px이며 focused layout/runtime 테스트가 통과했다. 통과. |
| 11 | height-locked 5:2 폭이 60px 또는 Grid 경계로 잘리는가 | target 폭 75px, 70px Grid 밖 visible overflow 15px를 Rust와 runtime frame 검사에서 확인했다. 통과. |
| 12 | `aspect-ratio:auto` 대조 입력에 이전 numeric transfer가 남는가 | width/height auto controls를 별도 matrix scene으로 유지했다. 숫자 ratio가 없는 geometry와 typed admission 경계를 확인했다. 통과. |
| 13 | 기존 2:1 폭 고정·높이 고정 회귀가 바뀌는가 | 두 방향 모두 기존 `60×30px` geometry를 matrix에 포함했고 통과했다. |
| 14 | height 80px 예외가 모든 ratio에 열리는가 | layout/projection은 height 80px을 typed ratio 0.5에만 허용한다. 1/2 positive와 3/2 negative를 각각 검사했다. 통과. |
| 15 | 추가 ratio item이 있는 owner에서 일부만 부분 적용되는가 | owner당 ratio item 하나 조건을 유지하고 두 item ratio negative fixture가 전체 projection 오류를 확인한다. 통과. |
| 16 | Grid root 또는 block root의 ratio가 item slice로 오인되는가 | Grid-root ratio와 Root HTML block-root ratio negative cases가 모두 `aspect-ratio` 오류로 닫힌다. 통과. |
| 17 | 고정 2-column·한 auto-row topology 밖에서 예외 admission이 되는가 | 두 track·한 row·70px owner 조건을 보존한다. named area와 71px/normal-alignment 대조가 fail-closed를 확인했다. 통과. |
| 18 | direct-child 소유권·순서·text 구조가 다른데 target ratio가 연결되는가 | owner map, direct-child 목록, `A`/`AAAA`, text paragraph source 및 target NodeId를 함께 검사한다. mismatch 시 candidate가 되지 않음을 코드 경로로 확인했다. 통과. |
| 19 | min/max·box edge·position·alignment 같은 미지원 조건을 조용히 무시하는가 | 양쪽 projection/layout admission guard를 대조하고 max-height와 Grid/item alignment negative cases를 통과시켰다. 통과. |
| 20 | 플랫폼 폰트 차이 또는 임시 fixture 복원이 geometry 증거를 오염시키는가 | 결정적 Rust 행렬은 Ahem을 등록한다. 앱 runtime은 monospace auto-width만 범위 검사한다. 두 simulator 앱 번들의 fixture hash가 같고 임시 bootstrap 파일은 기록된 원본 hash로 복원했다. 캡처는 layout/draw-present 근거이며 PixelCopy·Metal readback 증거가 아니다. 통과. |

검토 중 `single_element_loop` Clippy 경고 두 건을 발견했다. 하나는 새 negative test, 하나는 새로 허용된 `3 / 2`를 제거한 기존 test에 남은 단일 원소 반복문이었다. 두 곳을 단순 assertion으로 바꾼 뒤 변경 네 crate all-target Clippy와 `cargo fmt --all -- --check`가 통과했다. 절대 끝점 검사를 추가한 뒤 finite-transfer 누적 실패 회귀도 통과했다. 최종 focused style-to-layout 5/5, runtime 1/1, cascade 1/1, layout transfer 1/1도 통과했다. Broad package 결과는 각각 `spinon-layout` 441/442와 `spinon-style-to-layout` 390/400이며, task 범위 밖 실패는 README와 원본 로그에 분리 기록했다.

이 검토는 제한된 Root HTML Grid slice와 시뮬레이터 실행을 확인한다. 일반 CSS Grid 지원, 실기기 GPU, PixelCopy/Metal 표면 동일성, iOS compositor 이후 표시는 포함하지 않는다.
