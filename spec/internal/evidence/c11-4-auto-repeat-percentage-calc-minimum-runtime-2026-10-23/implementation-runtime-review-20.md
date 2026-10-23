# 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 분리해 구현 diff 및 실제 Android/iOS 실행 결과를 검토했다. 검토 대상은 cascade preflight, definite-axis layout gate, Taffy auto-repeat fixed-component 변경, native frame assertion, fixed fixture 및 simulator/archive evidence다.

| # | 별도 실패 경로 | 검토 및 판정 |
| ---: | --- | --- |
| 1 | 임의의 math function이 fixed minimum으로 승인됨 | cascade와 layout 모두 정확히 2항 `Sum(LengthPx, Percentage)`만 승인한다. `max(20px, 5%)` 거부 테스트 통과. |
| 2 | `Percentage + LengthPx` 항 순서가 거부됨 | 양 순서를 구현에서 허용한다. Stylo가 이 fixture를 percentage-first로 보존하는 cascade 테스트 통과. |
| 3 | 길이 또는 백분율에 음수·비유한 값이 들어옴 | 두 계층에서 finite 검사, positive length, nonnegative percentage 조건을 확인했다. `calc(0px + 0%)`와 negative percentage의 cascade/layout 거부 테스트가 포함됐다. |
| 4 | 세 번째 0px 항이 추가되어도 승인됨 | 길이 정규화 이후에도 정확히 2항만 받는다. layout의 추가 항 거부 테스트 통과. |
| 5 | `min()`/`max()` 결과가 calc minimum으로 잘못 허용됨 | `Sum` 이외 variant는 fail-closed다. `max()` cascade 거부 테스트 통과. |
| 6 | Calc ID가 누락되거나 stale해 다른 수식이 사용됨 | layout이 ID로 math binding을 찾아야 승인한다. 누락 binding 회귀 테스트 통과. |
| 7 | 다른 node의 수식 binding을 재사용함 | `node_id`가 grid node와 일치해야 한다. 주변 auto-repeat binding 회귀 테스트가 잘못된 node를 거부한다. |
| 8 | columns binding을 rows property에 재사용함 | `LayoutCssMathProperty`가 property 문자열에서 도출한 axis와 일치해야 한다. 기존 wrong-property binding 테스트 통과. |
| 9 | column percentage가 block size를 기준으로 계산됨 | columns fixture는 definite 112px inline size만 바꾸고 48px row control을 둔다. observed tracks `[112/3; 3]`. |
| 10 | row percentage가 inline size를 기준으로 계산됨 | rows fixture는 definite 112px block size, 48px column control이다. observed tracks `[112/3; 3]`. |
| 11 | cyclic/indefinite axis가 repeat admission을 우회함 | preflight는 `DefiniteAxes`의 해당 축을 요구한다. indefinite rows 직접 layout 테스트는 거부된다. |
| 12 | border/padding/gap 포함 축 크기를 inner percentage basis로 오인함 | 고정 입력은 border/padding/gap 0이고 root sizing을 112px로 둔다. 이 결과를 nonzero inner-edge 조건에 확대하지 않는다. |
| 13 | Taffy의 calc breadth가 auto-repeat sizing 전에 fixed 판정을 못 받음 | vendored `has_fixed_component()`에 calc feature 인식을 추가했고 standalone Taffy test가 3 repetitions를 확인한다. |
| 14 | `auto-fill`이 31.2px minimum에서 잘못된 repetition count를 가짐 | calc와 `31.2px` control 모두 columns/rows에서 세 track을 반환하고 결과가 일치한다. |
| 15 | `auto-fit`이 collapse 후 repeat count를 1로 덮어씀 | trace와 final tracks에서 세 반복 흔적과 `[112,0,0]` used/collapsed 결과를 구분해 2/2 축을 확인했다. |
| 16 | `auto-fit` item의 남은 폭이 이중 차감됨 | 첫 capture가 마지막-track residual 적용 오류를 드러냈다. residual 계산을 auto-fill로 제한했고 두 축 auto-fit item frame은 root와 일치한다. |
| 17 | float rounding으로 track/frame 허용 오차를 넘김 | ideal `112/3` 대비 실제 frame/track 오차는 1/64px보다 작다. Android와 iOS 관찰 geometry 간 최대 차이는 0px. |
| 18 | native 예상 문자열의 `48`/`48.0` 차이가 성공 판정을 깨뜨림 | 두 번째 capture에서 재현했다. integer-valued Float는 정수 문자열, 분수는 Float 문자열을 써 최종 양 플랫폼 실행이 통과한다. |
| 19 | 앱 host scene allowlist나 fixture total이 새 여섯 장면을 누락함 | Android/iOS allowlist, harness selection, 12-scene FFI ID 검사와 348-scene catalog boundary test를 확인했다. |
| 20 | fixture-test scene이 기본 제품 archive에 노출됨 | default/product static archive에서 scene marker/module symbol 부재, opt-in positive control에서 둘 다 존재한다. APK/IPA bundle exclusion으로 확대하지 않았다. |

최종 수락 실행은 Android API 37 ARM64 AVD 6/6, iPhone 17 Pro/iOS 26.2 Simulator 6/6이다. scene별 frame·track·trace·draw/present 기록은 [runtime README](./README.md)와 `spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-2026-10-23/` 아래의 Android/iOS `result.json`에 있다. C11.4는 미완료다.
