# C11.4 cyclic percentage minmax calc 구현·런타임 적대적 검토 20개

검토 대상은 고정 fixture adapter, typed style matcher, 직접 layout regression, Android/iOS scene validator, PixelCopy/Metal 표본과 두 최종 Simulator 실행이다. 계획 문서의 20개 항목과 다른 구현·실행 실패 관점을 검토했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | fixture 경로가 기본 제품 archive에 들어가는가? | 새 adapter는 기존 opt-in c04-runtime-gpu-fixture-test registry 아래에 연결했다. 기본 archive에서 새 scene 문자열이 빠지고 fixture-test positive control에는 포함됨을 static archive 문자열 검사로 확인했다. |
| 2 | minmax minimum을 자동 최소나 intrinsic 최소로 바꾸는가? | matcher가 typed LengthPx(0) minimum만 허용한다. 다른 breadth는 거부된다. |
| 3 | maximum calc를 bare calc나 fit-content로 받아들이는가? | ComputedCssGridTrackSize::MinMax의 max Calc variant를 검사하므로 다른 track function은 match되지 않는다. |
| 4 | calc AST에서 percentage/length 외 항이나 다른 basis가 허용되는가? | typed sum을 [Percentage(0.5), LengthPx(length)]와 정확히 비교한다. 추가 term 또는 값 변경은 실패한다. |
| 5 | column cyclic percentage가 row 또는 viewport basis를 사용해도 통과하는가? | independent expectation은 50px max-content column basis를 사용하며 최종 track 29px과 root/item frame을 검사한다. |
| 6 | row cyclic percentage가 column pass 전에 확정되는가? | row scene은 20px column과 40px intrinsic root height를 유지하고 final row track 22px을 직접 확인한다. |
| 7 | definite control이 cyclic 기대값을 우연히 만족시키는가? | 40px와 10px 두 definite case를 따로 검사해 24px와 9px 결과를 확인한다. |
| 8 | 작은 한도에서 item 또는 track이 9px를 초과하는가? | Android와 iOS 모두 10px root, 9px column/item을 trace와 frame에서 확인했다. |
| 9 | 최종 frame 대신 초기 layout 후보만 검사하는가? | 직접 layout test는 axis trace의 마지막 pass·stage, root/item 위치와 크기를 검사한다. 앱 실행은 draw/present까지 기록한다. |
| 10 | 예상 frame은 track trace와 모순되는가? | 네 scene에서 Columns·Rows track과 root/item frame이 서로 일치하고 독립 fixture 값과 맞는다. |
| 11 | root 아래 추가 element, 다른 owner, 중첩 text가 fixture를 우회하는가? | runtime adapter는 root identity·직접 child 수·owner·text node 수·text 내용을 고정 확인하고 다르면 거부한다. |
| 12 | 미등록 ID 또는 변형된 scene이 부분 layout으로 fallback되는가? | 알려진 ID prefix에서 scene lookup 실패는 unsupported error로 끝난다. partial fixture output은 허용하지 않는다. |
| 13 | 신규 ID가 기존 fit-content scene과 겹쳐 route되는가? | C11.4 FFI route는 정확한 ID lookup을 사용한다. 기존 fit-content 네 scene도 같은 모듈의 회귀 control로 남는다. |
| 14 | Android PixelCopy의 미등록 sample ID가 다른 기대색으로 통과하는가? | 이전 hostile review에서 small-limit 기본 매핑 fallback을 발견했다. 명시적 scene case와 return false로 고친 뒤 최종 APK에서 네 scene을 재실행했다. |
| 15 | Android sample 등록·색·좌표가 scene matrix와 어긋나는가? | 모든 ID가 유일하고 inside/past-item 색과 small-limit text-overflow 표본이 실제 PixelCopy에서 exact였다. |
| 16 | iOS Metal 표본이 PixelCopy와 다른 scene/좌표를 확인하는가? | Swift allowlist가 동일한 네 ID와 CSS 좌표를 사용한다. 네 scene의 Metal drawable 표본 9/9가 exact였다. |
| 17 | 픽셀 좌표의 drawable scale 적용이 잘못되었는가? | Android 2.625 scale, iOS 3.0 scale과 실제 surface/drawable 크기가 캡처 로그에 기록되고 sample 색이 기대값과 일치했다. |
| 18 | small-limit 빨간 표본을 CSS border stroke로 오인하는가? | 장면에는 border가 없고 sample은 Ahem text 색이다. 근거와 PR 설명을 text overflow로 한정한다. |
| 19 | capture가 stale source 또는 다른 앱 바이너리를 사용했는가? | 최종 양 플랫폼 trackedDiffSha256가 현재 tracked diff와 같고, 설치된 Android APK hash도 빌드 APK와 같다. iOS 앱 실행 파일 hash도 build output과 같다. |
| 20 | 네 fixed scene 통과를 일반 CSS Grid 지원 완료로 과장하는가? | C11.4 공식 checkbox와 72/213 집계는 유지한다. 계획·근거·PR은 fixed fixture와 명시된 미검증 범위만 주장한다. |

**미해결 blocking finding:** 없음. Android sample default 결함은 수정·재빌드·재실행했다. 두 최종 실행의 scene matrix와 pixel samples는 [런타임 근거](./README.md)에 연결했다.
