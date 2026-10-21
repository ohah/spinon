# 동시 cyclic percentage 다중 track 구현/runtime 검토 20개

검토 대상은 구현 commit `3bbdbd4c379564e98663dee892588c4d4f44b706`의 고정 C11.4 fixture, runtime adapter, layout assertions, Android/iOS harness 및 clean-source Simulator 결과다. 계획 검토 20개와 별개의 구현 검토다. 각 행은 다른 실패 경로 또는 경계 하나를 확인한다.

| # | 실패 관점 | 검토 및 판정 |
| --- | --- | --- |
| 1 | percentage basis를 Chrome 결과에서 역산했는가 | CSS Grid 2025 CRD와 Ahem intrinsic size로 `40×20px` root와 최종 track을 사전 계산했다. Chrome 156은 보조 비교로만 남겼다. 통과. |
| 2 | 두 축의 cyclic percentage가 서로의 미확정 basis에 의존하는가 | `max-content × max-content` scene에서 두 축의 final track을 따로 확인했다. `[20,10,20]` / `[10,5,10]`이며 root는 `40×20px`다. 통과. |
| 3 | inline axis만 cyclic인 경우를 양축 결과와 구분하는가 | columns-only scene은 columns `[20,10,20]`, 고정 rows `[10,5,5]`를 확인한다. 통과. |
| 4 | block axis만 cyclic일 때 columns-first sizing 경계가 깨지는가 | rows-only scene은 고정 columns `[20,10,10]`, rows `[10,5,10]`을 확인한다. 통과. |
| 5 | definite percentages가 cyclic 경로와 우연히 같은 처리로 덮이는가 | definite control은 `40×20px` containing size에 percentage를 적용해 `[20,10,20]` / `[10,5,10]`을 확인한다. 통과. |
| 6 | 네 scene ID가 서로 다르거나 다른 fixture 경로와 충돌하는가 | FFI source, runtime adapter, Android/iOS allowlist 및 simulator manifest에서 같은 네 ID를 사용한다. 미등록 ID는 거부된다. 통과. |
| 7 | extra child, 잘못된 parent 또는 다른 DOM order가 fixture로 받아들여지는가 | root의 direct child 두 개, text item의 direct `XX` node, overflow leaf, owner와 node marker를 검사한다. 정해진 두 child 외에는 준비되지 않는다. 통과. |
| 8 | system font나 폰트 fallback이 intrinsic input을 바꾸는가 | 내장 Ahem font를 `TextLayoutEngine::new_without_system_fonts`에 등록한다. 환경 폰트에 의존하지 않는다. 통과. |
| 9 | root box의 gap·padding·border·alignment가 basis 계산을 바꾸는가 | zero spacing, zero gap/border, `border-box`, start alignment 및 크기를 runtime adapter가 검사하며 fixture source에도 고정돼 있다. 통과. |
| 10 | 50%와 25%가 서로 바뀌거나 length track이 percentage로 projection되는가 | typed computed track을 순서대로 0.5, 0.25, 20px/10px와 대조한다. unit 또는 개수가 달라지면 거부한다. 통과. |
| 11 | Ahem text의 wrapping·line-height·minimum size가 intrinsic contribution을 바꾸는가 | `XX`, 10px font/line-height, wrapping·whitespace·direction 관련 profile, zero minimum과 spacing을 고정한다. 통과. |
| 12 | overflow cell이 고정 크기나 3행·3열 placement를 잃는가 | leaf item은 `20×10px`, column/row start line 3, end auto를 검사한다. 통과. |
| 13 | node frame이 root-relative 값과 세 track의 합을 혼동하는가 | Rust layout test와 simulator 기대값은 root `40×20`, text `(0,0) 20×10`, overflow `(30,15) 20×10`을 비교한다. root 바깥 폭 10·높이 5가 일치한다. 통과. |
| 14 | trace 검사가 중간 pass나 gutter를 final track으로 오인하는가 | 마지막 axis pass와 마지막 stage만 읽고 gutter/collapsed track을 제외한 뒤 세 track의 base size를 비교한다. 통과. |
| 15 | 새 text fixture가 기존 한 direct-text child fallback 동작을 넓게 바꾸는가 | 네 scene 전용 adapter를 generic fallback보다 먼저 시도하며 기존 fallback 로직은 수정하지 않았다. 통과. |
| 16 | 잘못된 computed style에서 부분 fixture input이 반환되는가 | profile, computed property, typed tracks, dimensions와 placement 불일치는 오류로 끝난다. malformed DOM 전반의 별도 고장 주입은 이 slice에서 수행하지 않았다. 정적 allowlist 및 지원 경계 검사는 통과; 한계 기록. |
| 17 | DOM node allocation 순서가 JS·native frame 기대값과 다른가 | text node가 node 3을 소비하므로 overflow element는 node 4다. Rust/Android/iOS 로그에서 node 4 frame을 확인했다. 초기 node 3 기대값은 수정 후 네 scene에서 재확인했다. 통과. |
| 18 | Android가 layout만 준비하고 surface에 실제 overflow를 내지 않는가 | API 37 AVD의 네 scene 모두 draw/present 성공. scene당 PixelCopy 표본 5개, 총 20개가 text-item 배경 및 root 내부/오른쪽/아래/두 경계 밖에서 exact다. 통과. |
| 19 | iOS readback 좌표 scale 또는 drawable 수명이 경계 pixel을 다른 위치에서 읽는가 | iPhone 17 Pro/iOS 26.2, 960×720 drawable/3×에서 네 scene 모두 draw/present 및 Metal 표본 20/20 exact다. readback은 drawable 제출 전이다. 통과; compositor 이후 화면은 범위 밖. |
| 20 | artifact 격리 또는 capture가 실제 source를 고정하지 못하는가 | 기본 debug staticlib에는 새 ID가 없고 fixture feature positive control에는 있다. 초기 untracked adapter capture는 최종 근거에서 제외하고, clean source commit `3bbdbd4c`에서 두 simulator를 재실행했다. 양쪽 `trackedDirtyPaths=[]`다. 통과. |

## 발견 사항과 종결

- 최초 Android capture의 scene allowlist가 신규 ID를 몰라 요청을 거부했다. Android/iOS allowlist와 expected-frame 매핑에 네 ID를 추가한 뒤 전체 4-scene capture가 통과했다.
- 기존 runtime text adapter는 direct Grid child 한 개만 허용했다. 이 slice에서만 두 direct item을 받는 adapter를 추가해 generic fallback을 그대로 유지했다.
- 첫 node-frame expectation은 overflow element ID를 3으로 적었지만 DOM text node가 그 ID를 사용했다. node 4로 고친 뒤 Rust·AVD·iOS 네 scene 결과가 일치했다.
- 최초 iOS 재실행은 Metal pixel-test build flag 없이 만들어 surface sample이 비활성화됐다. `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 rebuild해 최종 20/20을 확인했다.
- all-target Clippy가 redundant closure 한 곳을 지적했다. `.all(zero_spacing)`으로 수정하고 Clippy와 세 focused Rust test를 재실행했다.
- 캡처 source metadata는 신규 adapter가 untracked였던 첫 실행에서 완전한 출처를 고정하지 못했다. 구현 commit 뒤 양쪽 simulator를 재실행했고 결과 JSON이 commit/tree와 빈 tracked diff를 기록한다.

최종 결과는 fixed fixture 범위에서 통과했다. 실제 text glyph paint, 실기기 Android, hardware GPU, iOS compositor 이후 pixel, WPT와 전체 CSS Grid/C11.4는 확인하지 않았다.
