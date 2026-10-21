# C11.4 cyclic `calc()` PR diff 실패 관점 30개

최종 코드, Rust opt-in 경계, Android/iOS simulator adapter와 PixelCopy/Metal 검사, fixture runner, 계획·상태 문서를 서로 다른 실패 경로로 재검토했다. 발견된 harness 결함 세 건은 수정 후 영향 범위 전체를 다시 실행했다. 병합 대기 상태에서 기록한다.

| # | 실패 관점 | 최종 확인 |
| ---: | --- | --- |
| 1 | 새 Rust fixture가 기본 제품 모듈로 컴파일됨 | 모듈은 기존 C11.4 fixture-test feature 경계 안에서만 등록되고 fixture boundary 검사 `6/6` 통과. |
| 2 | default artifact에 새 FFI ABI나 scene payload가 들어감 | 기본·fixture opt-in release `.rlib`을 분리 빌드·scan했다. 기본은 symbol/string 부재, positive control은 symbol과 네 scene string 존재. 최종 앱 번들 크기로 확대 해석하지 않는다. |
| 3 | 네 scene ID가 FFI dispatcher에서 빠지거나 엉뚱한 source를 반환 | 각 ID에서 CSS와 root size를 비교하는 dispatcher test가 통과했고 실제 Android/iOS runtime trace도 기대 scene과 일치. |
| 4 | 등록되지 않은 ID가 fallback scene으로 성공 | FFI source 함수와 runtime adapter가 미등록 cyclic-calc ID를 오류로 닫으며 `cyclic-calc-unlisted` negative assertion 통과. |
| 5 | 다른 prefix를 쓰는 malformed cyclic ID가 일반 C11.4 경로로 빠져 성공 | `c114-cyclic-calc-` root ID의 미등록 항목은 즉시 unsupported 오류를 반환한다. |
| 6 | 허용 scene 수가 늘거나 ID가 중복 | FFI source와 runtime scene table 모두 네 고정 ID이며 unique count와 scene count를 확인한다. runner 전체 fixture count도 `219`로 조정했다. |
| 7 | 다른 root 아래 Grid를 fixture로 수용 | runtime adapter는 root owner가 `HostParent::Root`인지 검사한다. |
| 8 | root의 element/tag marker가 scene 설명과 다름 | root `div`, 예상 ID, empty `data-node`, profile을 고정 검사한다. |
| 9 | Grid 아래 복수 child 또는 중첩 child가 단일 item으로 가장됨 | 정확히 direct child 하나만 허용하고 item의 parent가 root인지 검사한다. |
| 10 | 다른 item ID/tag 또는 marker가 같은 layout을 통과 | `div`, scene별 item ID, parent, empty `data-node`를 모두 검사한다. |
| 11 | Ahem 입력이 누락·변형·간접 text node로 대체 | direct text node 하나와 정확한 `XX` 및 parent identity를 확인한다. |
| 12 | percentage와 px 항의 typed `calc()` 순서/항이 바뀜 | validator는 단일 `ComputedCssMath::Sum([Percentage(0.5), LengthPx])`만 허용하고 axis별 4px/2px를 정확히 비교한다. |
| 13 | 추가 track, auto-repeat, line name 또는 template area가 함께 들어옴 | 축별 단일 track만 허용하고 auto-repeat·areas·line names를 거부한다. |
| 14 | cyclic axis별 percentage basis나 다른 축 계산을 혼용 | column-only, row-only, simultaneous 장면의 independently resolved `[16]/[10]`을 Rust trace 및 양 플랫폼에서 확인했다. |
| 15 | definite-size control이 cyclic 결과를 우연히 재사용 | 별도 `40×20` control에서 Columns `[24]`, Rows `[12]`를 확인했다. |
| 16 | root sizing에 min/max, gap, border, padding 또는 spacing이 섞임 | root dimensions와 min/max 및 0 spacing, border-box 조건을 validator에서 제한하고 네 root frame을 대조했다. |
| 17 | item 크기·font·padding·border가 intrinsic input을 오염 | 고정 `24×16` Ahem item과 computed style/box 조건을 검사하고 text intrinsic trace를 대조했다. |
| 18 | 자동 배치 위치가 첫 번째 cell이 아님 | Grid placement가 column/row line 1과 auto end인지 검증하고 item frame `(0,0)`을 양 플랫폼에서 확인했다. |
| 19 | host system font에 따라 intrinsic size가 달라짐 | system fonts 없는 `TextLayoutEngine`에 저장소 Ahem bytes를 등록하고 직접 `XX`의 `24×16` 크기를 trace와 비교했다. |
| 20 | 다른 Grid node, 이전 pass, 잘못된 축의 trace를 선택 | style test는 root node trace를 찾고 각 axis의 마지막 pass와 마지막 stage를 비교한다. scene별 양 축 trace가 두 simulator에서 통과했다. |
| 21 | gutter 또는 collapsed track을 정상 track으로 계산 | Rust assertion은 gutter/collapsed track을 제외하고 expected count를 검사하며 각 장면의 단일 used track이 일치했다. |
| 22 | frame assertion이 root/item을 바꾸거나 definite control 크기를 놓침 | Android/iOS 기대값에 네 root/item frame을 각각 고정했고 simulator 결과가 모두 일치했다. |
| 23 | Android 새 scene이 앱 fixture allowlist에서 빠짐 | 최초 실행에서 누락을 재현했고 네 scene을 allowlist에 추가한 뒤 AVD matrix `4/4`를 재실행했다. |
| 24 | iOS scene gate가 Android와 불일치하거나 control이 누락 | Swift scene set에 동일 네 ID를 추가했고 iPhone 17 Pro matrix `4/4`를 실행했다. |
| 25 | platform adapter가 box count나 node ordering을 잘못 기대 | 각 scene box count 2, node 1 root/node 2 item frame을 두 앱 adapter와 runtime result JSON에서 비교했다. |
| 26 | Android sample ID가 빠지거나 track 경계 좌표가 잘못 지정 | PixelCopy sample 2개/scene의 ID, target item, inside/past CSS 좌표를 확인했다. track 넘어 paint 표본 총 `8/8` exact. |
| 27 | iOS color sample이 다른 surface/축/기준색을 읽음 | Metal drawable의 item node, exact RGBA `[229,72,77,255]`, inside/past 좌표를 검사했다. sample 총 `8/8` exact이며 compositor 제출 전 측정이다. |
| 28 | 검증 결과가 오래된 APK나 잘못된 simulator에서 나옴 | Android API 37 ARM64 AVD `emulator-5558`의 local/install APK SHA 일치, iPhone 17 Pro/iOS 26.2 고정 UDID를 증거에서 확인했다. 실기기는 연결하지 않았다. |
| 29 | `-two-axis` 보정이 cyclic scene 또는 기존 nested scene을 잘못 변환 | suffix 제거를 `nested-depth-*`에 한정했고 기존 six-scene nested-depth regression이 Android/iOS 각각 `6/6` 통과했다. |
| 30 | 실행 분류·증거 위치·문서가 범위를 과장하거나 완료 수를 올림 | cyclic calc run kind와 evidence group이 고정되고 plan/status 링크·성능/적합성 한계가 기록됐다. C11.4는 미완료, 공식 count는 `72/213`이다. |

## 판정

30개 관점에서 병합을 막는 미해결 발견은 없다. 초기에 발견된 Android scene allowlist 누락, Android RGB sample mapping 누락, iOS `-two-axis` 기대 frame 변환 결함은 수정 후 각 영향 행렬 전체를 재실행했다. 이 결과는 fixed fixture와 simulator 범위다. WPT, 일반 CSS author admission, Android 실기기, hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4 완료를 증명하지 않는다.
