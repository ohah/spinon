# C11.4 겹치는 row span runtime 구현 실패 관점 20개

2026-10-22에 고정 fixture 구현과 Android API 37 ARM64 AVD·iPhone 17 Pro iOS 26.2 Simulator 동작을 계획 리뷰와 독립적으로 검토했다. 두 번의 초기 capture가 런타임 scene registry 누락과 행 축 trace assertion의 검증기 오류를 드러냈다. 각 문제를 수정하고 최종 여섯 scene을 양 플랫폼에서 다시 실행했다.

| # | 별도 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 scene ID가 FFI allowlist와 런타임 registry 중 한 곳에서 빠짐 | 첫 AVD 실행이 `unregistered overlapping span scene`을 재현했다. 여섯 고정 ID를 runtime registry에 추가했고 이후 Android/iOS 각각 6/6 통과했다. |
| 2 | 허용 목록 밖의 비슷한 ID가 일반 scene으로 처리됨 | FFI 미등록 ID negative assertion과 runtime prefix 거부 경계를 확인했다. 여섯 ID 외 변형은 공개하지 않는다. |
| 3 | root의 DOM owner·tag·marker 또는 child 수가 다른 입력을 허용 | runtime은 XHTML `div`, document root 소유, 빈 `data-node`, 직접 자식 3개를 검사한다. |
| 4 | item 순서가 DOM과 allowlist 순서에서 달라짐 | 각 scene의 ID, 직접 text node, parent와 실제 DOM 순서를 대조한다. forward/reverse 모두 통과했다. |
| 5 | row track 지정이 column track으로 잘못 계산됨 | typed `grid.rows`만 세 개 calc-minmax로 인정하고 `grid.columns`는 고정 10px 하나로 요구한다. trace에서 Rows `[42,47,47]` 또는 `[32,32,32]`, Columns `[10]`을 확인했다. |
| 6 | cyclic root height가 fixed height처럼 취급됨 | `height:max-content` 두 scene의 root는 `14×144px`, 최종 Rows는 `[42,47,47]`이다. |
| 7 | definite 140px control이 cyclic 결과와 달라짐 | forward/reverse 두 scene 모두 Rows `[42,47,47]`, root `14×144px`다. |
| 8 | definite 100px에서 calc minimum·free space 배분이 틀림 | 두 small control 모두 Rows `[32,32,32]`, root `14×104px`다. |
| 9 | 겹치는 span-2 group의 source order가 결과를 바꿈 | 기대 kind별 node frame을 source order와 분리해 대조했다. forward/reverse 두 그룹의 track과 item geometry가 일치한다. |
| 10 | span별 기여 단계가 잘못 묶이거나 순서가 바뀜 | style-to-layout trace는 intrinsic span batch `[2,3]`과 각 item의 Minimum/MinContent/MaxContent 기여를 확인한다. |
| 11 | 세로 text를 inline intrinsic 값으로 오독 | runtime paragraph는 행 scene에 `word-break:break-all`, Ahem 10px/10px을 적용한다. intrinsic block contribution 70/80/140px을 양 플랫폼 로그에서 확인했다. |
| 12 | forward/reverse에서 item별 intrinsic source가 바뀜 | 정방향은 70/80/140, 역방향은 140/80/70 순서로 DOM에 대응하고 각 node frame과 intrinsic input이 일치한다. |
| 13 | row placement가 column span으로 검사됨 | item 검증은 scene axis에 맞춰 row 또는 column line과 span 길이를 확인한다. 행 배치는 각각 `1/3`, `2/4`, `1/4`다. |
| 14 | 고정 Ahem source 대신 host system font가 사용됨 | runtime은 system font 없이 `Ahem.ttf`를 등록한다. simulator intrinsic source의 font revision은 1이다. |
| 15 | 100px control에서 span-2 text와 frame 경계를 혼동 | item frame은 66/66px이고 text intrinsic 높이는 70/80px다. 내부 AVD PixelCopy·iOS drawable 표본을 exact 색으로 확인했다. |
| 16 | span-3 text가 100px root를 넘지 않거나 target node가 틀림 | span-3 frame은 100px, text intrinsic은 140px다. `(7,126)`에서 span-3 text 색이 두 플랫폼 모두 exact로 관측됐다. |
| 17 | overflow 이후 host clear 여부를 text와 혼동 | `(7,150)`은 span-3 text 뒤 지점이며 AVD PixelCopy와 iOS drawable에서 검정 clear가 exact다. |
| 18 | pixel sample 좌표 변환 또는 scene/generation 연결 오류 | AVD는 2.625× drawable에서 표본 6/6, iOS는 3× Metal drawable에서 6/6을 정확히 확인했다. 두 표본은 small scene 두 개의 세 좌표씩이다. |
| 19 | capture harness가 빈 Columns 기여를 Rows 기여로 판정 | 첫 실행에서 검증기가 양 축을 같은 text contribution 목록으로 비교하는 오류를 확인했다. 행 전용 span trace 비교를 추가한 뒤 두 플랫폼 여섯 장면이 모두 통과했다. |
| 20 | opt-in fixture가 기본 FFI product archive로 유출됨 | `spinon-ffi` default release archive에서 새 scene ID/error marker가 없고 `c04-runtime-gpu-fixture-test` positive control에는 두 marker가 있다. 실기기나 Android hardware GPU 결과는 포함하지 않는다. |

## 판정 범위

Runtime·style-to-layout·FFI focused test는 각 1/1 통과했다. 세 수정 crate의 fixture feature Clippy `-D warnings`, `cargo fmt --all -- --check`, Node syntax check가 통과했다. 스크린샷과 로그는 같은 증거 디렉터리의 Android AVD·iOS Simulator run에 있다. CSS border stroke, Android 실기기/hardware GPU, iOS compositor 이후 pixel, WPT, 일반 CSS admission과 전체 C11.4는 검증하지 않았다.
