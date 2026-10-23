# PR diff 독립 실패 관점 30개

**검토 범위:** 구현 후보의 최종 diff 전체. 각 항목은 별도 계약 경계 또는 실패 입력이다. 30개 관점의 초안 검토에서 놓친 empty item-rect host mapping은 runtime에서 재현·수정한 뒤 아래 후속 검토로 다시 확인했다.

| # | 별도 diff 실패 관점 | 확인 내용과 판정 |
| ---: | --- | --- |
| 1 | 단일 축 13개 또는 기존 양 축 9개 fixture를 제거한다. | JSON scene 수 29와 기대 ID 순서를 Node test로 검사한다. 통과. |
| 2 | 7개 추가 ID 중 하나가 FFI fixed allowlist에서 빠진다. | Rust validator source와 Node ID matrix를 대조했다. 통과. |
| 3 | 새 ID가 기존 id와 충돌하거나 문자열 오타가 있다. | JSON 29개 unique 검사와 앱 route 문자열을 대조했다. 통과. |
| 4 | scene root의 크기가 비교 입력과 달라 repeat count가 달라진다. | 새 7 scene root는 모두 52×52이며 Node test가 고정한다. 통과. |
| 5 | column/row gap 하나가 4px가 아니다. | 신규 JSON 7개 모두 각 gap 4px다. 통과. |
| 6 | `grid-auto-columns`가 area-added column에 적용되지 않는다. | 신규 scene과 projection test 기대가 16px를 고정한다. test source 정적 검토 통과. |
| 7 | `grid-auto-rows`가 area-added row에 적용되지 않는다. | 신규 scene과 projection test 기대가 12px를 고정한다. test source 정적 검토 통과. |
| 8 | empty areas를 item 없는 empty scene과 혼동한다. | `fit-fit-area-empty`만 `items=[]`; 다른 여섯 scene은 hero를 둔다. JSON/host maps 대조 통과. |
| 9 | item을 added-area cell 대신 auto-placement에 맡긴다. | 여섯 hero scene의 `gridArea: hero` 및 named area rectangle을 대조했다. 통과. |
| 10 | area 문자열이 직사각형이 아니거나 row 길이가 다르다. | 3×3, 1×3, 3×1 문자열을 Chrome precomparison fixture의 class와 대조했다. 통과. |
| 11 | full-span scene이 3×3보다 작은 영역을 span한다. | `fit-fit-area-span` item frame 64×60, track arrays, areas가 전체 3×3이다. 통과. |
| 12 | mixed-axis scene에서 auto-fill/auto-fit 방향이 반대다. | `fit-fill`은 columns fit/rows fill, `fill-fit`은 반대다. JSON과 host id를 대조했다. 통과. |
| 13 | scroll extent가 item overflow를 감춘다. | span, bottom-only, right-only expected extent를 Node test로 확인했다. 통과. |
| 14 | Rust layout test의 child size가 fixed여서 stretch를 숨긴다. | item style을 `LayoutDimension::Auto`로 설정했고 7 frames를 실제 layout test가 확인했다. 통과. |
| 15 | test helper가 기존 generic grid fixture 상태를 공유해 case 간 오염된다. | 각 loop iteration에서 새 `LayoutInput`을 만들고 case별 container/metadata를 설정한다. 통과. |
| 16 | collapsed count에서 gutter를 repeat track으로 세거나 반대로 센다. | 기존 `collapsed_track_count` helper가 `!is_gutter && is_collapsed`만 세며 테스트에서 각 축 수를 단언한다. 통과. |
| 17 | used track 비교가 collapsed zero를 live track으로 반환한다. | 기존 `final_axis_track_sizes`가 collapsed track을 거른다. 기대 lists와 collapse count를 함께 쓴다. 통과. |
| 18 | projection 테스트가 AutoFill 또는 한 축만 검사한다. | 신규 projection Cargo test가 두 축 AutoFit mode, 3×3 rectangle, auto track sizes를 검사해 1/1 통과했다. |
| 19 | allocation-failure 테스트 fixture를 바꿔 기존 negative path가 무력화된다. | 기존 allocation-failure fixture를 수정하지 않고 독립 test를 추가했다. 통과. |
| 20 | FFI validator가 empty scene을 허용하지 않는다. | opt-in fixture feature FFI test에서 29 scene·empty case 검증이 1/1 통과했다. |
| 21 | Android scene allowlist는 있지만 root-frame dispatch가 빠진다. | Node guard가 29 area ID를 Android root-frame helper에서 확인하고 full AVD 결과도 41/41 통과했다. |
| 22 | iOS scene allowlist는 있지만 root-frame dispatch가 빠진다. | 같은 Node guard가 iOS root-frame helper를 확인하고 full simulator 결과도 41/41 통과했다. |
| 23 | Android item names/rects의 scene 순서가 FFI node 순서와 다르다. | 첫 full run에서 empty-area rect 누락을 발견했다. 빈 배열 case와 helper membership guard를 추가하고 full run 후 frame·PixelCopy를 통과했다. |
| 24 | iOS item names/rects의 scene 순서가 FFI node 순서와 다르다. | 동일한 omission을 Swift에서 수정했다. helper membership guard, 단독 scene, full run 및 Metal drawable 표본을 통과했다. |
| 25 | overflow sample count가 1-item 일반 scene의 세 점 규칙과 어긋난다. | empty scene 2점과 overflow four-point 예외를 포함해 29개 area sample set, 플랫폼별 116/116 exact다. |
| 26 | span이 root sample을 덮는데 expected root color가 회색이다. | Android PixelCopy 및 iOS Metal drawable에서 root/hero/clear sample이 모두 기대값과 일치한다. |
| 27 | outside-root item 색과 item 이후 clear 색이 같은 좌표를 쓴다. | span/bottom/right overflow의 item과 바깥 clear point를 각 플랫폼 116 samples 안에서 exact로 검사했다. |
| 28 | capture builder가 새 scene을 22개에서 자르거나 전체 C11.4 route count가 stale하다. | Node boundary test 7/7, full route 336개, 정확한 matrix 41개가 두 플랫폼에서 실행됐다. |
| 29 | simulator 앱 fixture가 product FFI default path에 포함된다. | product archive에 scene marker/host symbol이 없고 fixture-test archive에 둘 다 있다. Rust LLVM 22.1.2 `llvm-nm` 및 raw marker scan으로 확인했다. |
| 30 | local diff가 official support count나 Tailnet 상태를 조기에 올린다. | `spec/STATUS.md`와 Tailnet은 아직 갱신하지 않았다. merge 뒤에만 공식 집계와 Tailnet sync를 진행한다. 통과. |

## Runtime defect 후속 검토

| 추가 실패 관점 | 확인 내용과 판정 |
| --- | --- |
| 빈 item list가 `nil`/unknown scene과 혼동되지 않고 zero-item geometry로 전달된다. | 두 host에 explicit empty rect를 추가하고 empty scene을 단독 실행했다. 통과. |
| 새 fixture를 추가할 때 helper 중 일부만 갱신되는 회귀를 검출한다. | Node 경계 test가 29 ID 모두 root/name/rect helper 안에 있는지 검사한다. 7/7 통과. |
| 모든 renderer/backend에서 empty scene sample을 실제 surface에서 읽는다. | Android PixelCopy와 iOS drawable에서 root/clear sample이 exact; 전체 area 116/116씩 통과. |
| runtime 요약 없이 앱 설정 오류가 성공처럼 기록되지 않는다. | 최초 실패는 `runtime summary` 검사 전 config error로 종료됐다. 수정 뒤 전체 41/41에서 summary·trace·present가 확인됐다. 통과. |

최신 source의 focused tests, workspace/fixture-feature Clippy, Android/iOS app build, 41-scene runtime 및 FFI archive 경계는 각각 근거 파일에 기록했다. 구현 PR을 만든 뒤 현재 required checks와 merge state를 다시 조회하고, Agent Browser로 본문 이미지가 렌더링되는지 확인한 다음 task 소유 browser process만 종료한다.
