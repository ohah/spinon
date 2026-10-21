# C11.4 depth-three nested Grid 구현·runtime 실패 관점 검토 20개

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 새 fixture adapter가 허용 ID를 임의 depth까지 넓혔는가? | FFI adapter는 기존 8개 control과 신규 2개만 선언한다. 미등록 depth-three 단축 ID와 depth-four ID가 `None`이다. 통과. |
| 2 | Grid chain이 네 Grid 순서를 잘못 붙이는가? | FFI source와 runtime fixture가 root → middle → branch → inner 순서의 정확한 ID 배열을 비교한다. 통과. |
| 3 | 중간 branch 역할이 inner/middle과 혼동되는가? | Runtime fixture가 depth별 역할을 명시적으로 매핑하며 잘못된 branch identity는 거부하는 회귀 사례가 통과했다. |
| 4 | root나 중간 Grid에 두 번째 직접 자식이 추가돼도 부분 layout이 나오는가? | root에 unexpected direct child를 붙인 입력이 `layout.is_err()`로 거부됐다. 통과. |
| 5 | root ID만 허용 scene처럼 바꾸어 depth-four가 들어오는가? | `nested-depth-four-min-content-two-axis` root가 fail-closed 처리됐다. 통과. |
| 6 | 한 단계만 다른 intrinsic keyword를 써도 같은 결과로 통과하는가? | 세 번째 Grid의 min/max keyword를 뒤집은 malformed style fixture가 거부됐다. 통과. |
| 7 | root가 HostRoot 직속이 아니거나 marker가 없는 경우를 놓치는가? | runtime adapter의 parent, tag와 빈 `data-node` 검사를 확인했고 정상 10-case만 통과했다. |
| 8 | 특정 Grid에서 width/height와 한 축 track keyword가 달라지는가? | 각 Grid에서 width·height와 단일 Columns·Rows template을 함께 검사한다. 두 축 런타임 결과는 양 플랫폼 모두 기대값과 같다. |
| 9 | margin·padding·border·gap 또는 auto track 값이 합산되는가? | runtime allowlist는 0 edges/gaps, 단일 intrinsic track과 기본 auto rows/columns만 받아들인다. 통과. |
| 10 | leaf가 직접 text가 아니거나 text item 소유자가 inner Grid가 아닌가? | inner Grid의 단일 자식, HTML `div`, exact leaf ID와 부모 ownership, 유일한 `XX XXXXXXX` text node를 검사한다. 통과. |
| 11 | Ahem fallback이나 line-height 차이가 기대값에 섞이는가? | fixture는 시스템 글꼴 없는 엔진에 저장소 Ahem revision 1을 등록하고 10px font/line-height를 고정한다. Simulator diagnostics의 direct-leaf intrinsic source도 Ahem revision 1이다. 통과. |
| 12 | min-content의 text width 또는 row height가 한 단계에서 누락되는가? | 네 Grid와 text frame 및 두 축 final track이 모두 70×20px이다. Android/iOS 각 depth-three min scene 통과. |
| 13 | max-content에서 공백 포함 폭 또는 단일 줄 높이가 틀리는가? | 네 Grid와 text frame 및 두 축 final track이 모두 100×10px이다. Android/iOS 각 depth-three max scene 통과. |
| 14 | contribution이 바로 아래 자식이 아니라 leaf에서 root로 직접 건너뛰는가? | Rust test와 simulator trace verifier가 각 부모의 direct child, source order, span 1 및 contribution kind/value를 순서대로 확인한다. 통과. |
| 15 | trace가 Grid 하나를 빠뜨리거나 같은 축 pass가 중복되는가? | Runtime test는 Grid별 Columns/Rows pass와 trace 개수를 확인하고 Android/iOS verifier도 4개 trace를 요구한다. 통과. |
| 16 | text intrinsic 입력이 잘못된 Grid 또는 node ID에 귀속되는가? | 최내부 Grid 4와 text item 5, source order 0, min/max inline/block 값과 font revision을 검사한다. 통과. |
| 17 | 앱 allowlist만 넓고 화면 validator는 이전 node 수를 계속 요구하는가? | Android/iOS가 새 ID, root frame, 5-box count, node 1..5 frames와 intrinsic input을 검사한다. 양쪽 실행 10/10 통과. |
| 18 | layout 성공 후 실제 draw/present가 되지 않아도 실행을 통과하는가? | 10개 Android와 10개 iOS scene 모두 `drawPresented=true`; 각 실행의 track trace chunks도 기록됐다. 통과. |
| 19 | capture matrix가 신규 ID를 포함하지 않거나 전체 scene 수 상수를 놓치는가? | capture tool `--list-scenes`가 194개 ID를 보고하고 두 새 ID가 포함된다. 전체 실행에서 각 Simulator 10/10이 통과했다. |
| 20 | fixture payload가 기본 제품 archive에 들어가거나 검증 artifact가 앱과 다를 수 있는가? | 기본 FFI archive에서 새 ID가 없고 opt-in archive에는 있다. Android local/installed APK SHA-256이 일치하고 source revision/diff SHA가 두 simulator manifest에 기록됐다. 통과. |

## 판정

이 검토는 계획의 규범 기대값 검토를 반복하지 않고, 구현 경계·malformed input·두 축 trace·플랫폼 validator와 artifact/source binding을 확인했다. 10 scene Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 실행은 [runtime 근거](../spec/internal/evidence/c11-4-nested-grid-depth-three-runtime-2026-10-22/README.md)에 기록한다. 계획 PR diff 검토 30개는 구현 검토를 대신하지 않는다.
