# 구현·runtime 실패 관점 검토 20개

범위는 C11.1 Root HTML auto-repeat percentage-minimum 구현 diff, 기본 RuntimeSession,
Android API 37 ARM64 AVD 및 iOS 26.2 Simulator 실행이다. 계획 자체 검토 표와 분리했다.

1. **Grammar admission** — auto-repeat fragment의 `minmax()`와 정확한 `1fr` 최대값, 1~2 track 제한이 유지된다. Percentage는 유한하고 음수가 아닌 minimum에만 추가됐다. Projection 14/14 및 cascade test 1/1 통과.
2. **Typed value 보존** — count용 basis 환산을 Taffy track 입력에 복사하지 않는다. Projection test가 authored 0%, 10%, 120% percentage typed value 보존을 확인한다.
3. **반복 축 basis** — columns는 definite width, rows는 definite height를 사용한다. 양축 percentage layout/runtime test가 각 축을 독립 대조한다.
4. **Definite-axis 조건** — Root HTML basis helper는 definite preferred size와 기본 min/max 조합만 인정한다. clamp 또는 indefinite 축은 fail-closed이고 percentage regression 13/13 통과.
5. **0% 의미** — count 계산만 1 CSS px floor를 쓰며 실제 minimum은 0%로 유지한다. 34-track boundary와 sizing regression을 확인했다.
6. **100% 초과 처리** — 120% minimum은 240px로 유지하고 200px grid 밖으로 40px overflow한다. layout track test와 runtime fixture가 모두 검증한다.
7. **Fractional percentage 계산** — 10%, 5%, 20%의 definite basis 환산과 count boundary를 독립 기대값에 비교한다. 2-track pair는 5회 확장된다.
8. **Gap 산식** — 반복 fragment 안·사이 gutter 항을 track 수에 맞게 계산한다. 1/2-track, gap 0/2/5px 사례와 denominator 검사를 확인했다.
9. **Auto-fill / auto-fit 차이** — auto-fill은 empty track을 유지하고 auto-fit은 placement 후 empty track을 collapse한다. 기존 layout suite와 새 pair/fit test 통과.
10. **Count ceiling** — repeat expansion 전에 `GRID_MAX_TRACKS`를 적용하고 정확한 ceiling 및 초과를 검사한다. Layout 10/10, projection 14/14 통과.
11. **정수 변환 경계** — f64 percentage 산식 결과가 f32로 표현 가능한지 확인한 뒤 Taffy에 전달한다. non-finite 및 f32 overflow rejection test 통과.
12. **Gap·곱셈 overflow** — pattern, gutter, 다른 축 track 수 곱이 finite/checked arithmetic을 벗어나면 layout error를 반환한다. boundary rejection tests 통과.
13. **Unsupported maximum** — percentage minimum을 허용해도 `auto`, intrinsic, percentage maximum, calc 등 profile 밖 maximum을 새로 허용하지 않는다. admission pattern과 rejection controls 확인.
14. **Unsupported template combination** — 두 축 repeat, named lines/areas, neighbor tracks, dense placement, invalid pattern은 기존 fail-closed grammar에 남는다. projection rejection tests 통과.
15. **Style/layout count 일치** — cascade projection과 layout contract 각각 같은 definite-axis basis·1px count floor·track cap을 검사한다. 6-scene RuntimeSession expected geometry 통과.
16. **Partial result 게시** — invalid count/layout이 track expansion 한도를 넘을 때 정상 snapshot을 게시하지 않는다. 기존 overflow rejection layout tests 통과.
17. **다른 percentage-auto-repeat profile 격리** — 새 예외는 `RootHtmlAutoRepeatV1` placement mode에만 적용한다. 기존 일반/C11.4 percentage layout 13/13 및 cascade 6/6 통과.
18. **기본 runtime 경로** — Android/iOS route는 새 FFI fixture adapter를 추가하지 않고 기존 root-HTML host 생성 경로를 사용한다. 두 앱에서 `layout=ready`, 27 boxes와 같은 HTML hash 확인.
19. **기본 artifact 제외** — opt-in asset은 Android/iOS feature flag가 켜질 때만 복사된다. feature on APK/app의 asset hash 일치와 feature off 양쪽 bundle 제외를 확인.
20. **플랫폼 인자 충돌** — Android/iOS에서 zero-minimum과 percentage-minimum 동시 사용, page-scale override, 허용 목록 밖 인자를 거부하는 코드 경계를 확인했다. 각 플랫폼 opt-in build와 simulator launch 통과.

남은 범위: 두 개의 현재 failing Root HTML auto-row style assertion은 변경 경로 밖이며 green suite로 표시하지 않는다. Simulator visual smoke는 실기기·hardware GPU 또는 pixel parity를 대신하지 않는다.
