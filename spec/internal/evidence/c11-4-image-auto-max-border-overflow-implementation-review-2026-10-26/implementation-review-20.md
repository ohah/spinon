# C11.4 image auto/max border overflow 구현 적대 검토 20개

**검토 대상 source:** `29449e4f794e53255909b58ed8fb2fd4995dd5d2` + 아래 고정 diff<br>
**검토 diff SHA-256:** `dd2d90c1a434203c25ffddc25419ff51b6d73ee2a19cdac63d130062f9dea291`<br>
**계획 blob SHA-1:** `ac08230270d78971c561fc9f7001d2ded0019082` · [계획 20관점 검토](../c11-4-image-auto-max-border-overflow-plan-review-2026-10-26/plan-review-20.md)<br>
**검토 경계:** 정적 코드와 focused Rust test. simulator 동작은 이 판정에 포함하지 않는다.

**후속 상태:** 이 문서는 Android build/AVD runtime 이전의 고정된 구현 검토 기록이다. 이후 PixelCopy scene allowlist 보완과 API 37 AVD 실행은 [별도 검토](./android-pixelcopy-scene-allowlist-review-20.md) 및 [runtime README](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)에 기록했다. 아래 미검증 항목은 당시 검토 범위를 가리킨다.

| # | 독립 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | 새 fixture가 기본/product feature에 들어가 사용자 표면을 넓힘 | C11.4 image case는 기존 `c11-4-grid-track-sizing-fixture-test` 경계 안이고 FFI adapter는 `c04-runtime-gpu-fixture-test` 경계 안이다. 새 production layout 경로는 추가하지 않았다. 정적 PASS. |
| 2 | 제한 scene을 공식 완료 또는 C11.4 전체 완료로 표시 | `spec/STATUS.md`와 roadmap은 diff에 없다. 계획과 변경 범위는 `72/213` 및 C11.4 미완료를 유지한다. PASS. |
| 3 | scene ID 또는 root/item ID 충돌로 다른 입력이 선택됨 | 새 ID는 `image-auto-max-mixed-tracks-border-overflow`, root/item은 `c114-image-auto-max-border-overflow-grid/item`으로 Rust·FFI·앱·runner가 일치한다. Runner는 scene ID 중복도 거부한다. PASS. |
| 4 | 고정 PNG의 자연 크기 대신 authored 크기나 다른 자산 사용 | 새 `ImageCase`는 기존 direct PNG `ImageSizing::Intrinsic` 경로를 사용한다. FFI source도 기존 `c14-1-png-basic.png` URL을 쓰고 authored width/height 없이 생성한다. Runtime focused test PASS; 앱 실행 여부는 미검증. |
| 5 | border-box root에서 border를 빼지 않아 content width가 바뀜 | case geometry는 12×40, 네 방향 border 1px이며 `box-sizing:border-box` validator를 유지한다. 계획 산술은 content width 10px이다. 정적 PASS. |
| 6 | track/gap/row 차이 때문에 overflow 기하가 달라짐 | 새 case와 FFI scene은 `minmax(auto,max-content) 17px`, `auto`, column gap 5, row gap 0으로 고정한다. Runner는 columns `[24,17]`, row `[16]` 및 frame `(1,1,24,16)`을 기대한다. 정적 PASS; 실기하 결과는 iOS/Android capture 전. |
| 7 | 기존 root style 검증 helper 변경이 border 없는 control을 깨뜨림 | `mixed_root_style`은 `case.grid_border`를 사용한다. 기존 control은 0px이고 신규 case만 1px이다. Runtime image focused test 9/9 PASS. |
| 8 | root border가 없거나 content-box인데 신규 case가 성공 처리됨 | 신규 test는 정상 1px border를 수락하고 border 제거와 content-box mutation을 각각 거부하는지 확인한다. PASS. |
| 9 | 명시 배치, 두 번째 자식 또는 다른 asset이 auto-placement 기대를 바꿈 | 신규 입력은 `Placement::Auto`와 intrinsic image sizing을 사용하며 기존 prepare guard가 direct single `<img>` 및 asset identity를 확인한다. 신규 test의 case shape assertions PASS; 기존 guard tests도 focused suite에서 PASS. |
| 10 | product sizing algorithm을 고쳐 이 fixture에만 맞춤 | Rust production sizing algorithm 변경은 없다. Scene은 기존 image auto/max mixed-track adapter와 trace를 재사용한다. 변경 범위 PASS. |
| 11 | FFI scene이 기존 image-track list의 count/behavior를 바꿈 | 새 scene은 별도 `C11_4_IMAGE_BORDER_OVERFLOW_SCENES` 1개 배열과 source route에 둔다. 기존 image-track list의 19개 count는 유지된다. FFI focused test PASS. |
| 12 | generated HTML에 의도하지 않은 clipping/overflow CSS 또는 다른 paint 값이 들어감 | FFI source는 border-box 12×40, 1px green border, `#101820` root, 기존 red image-box marker와 고정 track/gap을 만든다. 전용 source assertion은 CSS `overflow`가 없는지 확인한다. PASS. |
| 13 | FFI scene 누락·중복 또는 다른 scene source와 혼동 | fixture test는 신규 배열 길이 1, root/item IDs, dimensions, border, background, tracks, gaps 및 source dispatch를 확인한다. `exposes_only_bounded_intrinsic_track_sizing_scenes` 1/1 PASS. |
| 14 | Android app에서 scene ID가 admission 또는 intrinsic input gate를 통과하지 못함 | ID가 scene allowlist와 image intrinsic input case에 추가됐다. 신규 root 12×40 및 item 24×16 결과가 별도로 지정됐다. Java compile/runtime는 아직 미검증. |
| 15 | Android node frame의 item 원점이 border 안쪽이 아님 | node-frame gate는 신규 item 원점을 `(1,1)`로 지정하고 root를 `(0,0,12,40)`으로 고정한다. 정적 기대값 PASS; PixelCopy 근거 전. |
| 16 | Android diagnostics가 신규 track을 기존 mixed scene과 다르게 계산하거나 느슨하게 승인 | 신규 scene은 `c114ImageAutoMaxMixedTrackTraceMatches`를 재사용한다. 이 gate는 새 scene의 track expectation 자체를 별도로 완화하지 않는다. 정적 PASS; 앱 trace 미실행. |
| 17 | Android sample 배열이 누락/중복/잘못된 target 또는 좌표를 통과 | gate는 정확히 4개 ID, target ID, 정수 CSS index를 확인하고 중복/추가 항목을 거부한다: `[12,8]`, `[26,8]`, `[0,30]`, `[11,30]`. 정적 PASS. |
| 18 | Android 색 표본이 box marker·배경·border를 혼동하거나 surface index를 1px 이동 | expected RGB는 item `[229,72,77]`, outside `[16,24,32]`, 두 border `[34,197,94]`다. 기존 `SurfaceViewPixelCapture`는 `floor((origin + index + 0.5) * scale)`을 적용하고 이 fixture는 origin `(0,0)`을 준다. 정적 PASS; PixelCopy 실행 전. |
| 19 | iOS Swift gate에서 Android와 다른 geometry, sample identity 또는 scale을 사용 | iOS scene ID, intrinsic image inputs, trace, frame origin `(1,1)`과 네 개 `[id,node,index,RGBA]` tuple이 일치한다. decoder의 고정 C114 sample gate는 count와 tuple을 확인하고 readback은 `floor((index+0.5)*drawableScale)`을 사용한다. 정적 PASS; Metal capture 전. |
| 20 | Runner가 control을 빼먹거나 잘못된 simulator/AVD에 실행해 근거를 과장 | runner는 base control과 신규 scene 둘을 함께 선택하는 경로, 498개 고유 scene count, iOS model identifier `iPhone18,1` guard 및 분리된 evidence directory를 추가한다. `--list-scenes`는 선택 2개를 확인했다. S03의 task-owned C9 clone에서만 iOS capture를 진행할 수 있고 Android AVD는 공간 부족으로 미검증이다. simulator 동작·스크린샷·PR 이미지 render는 이 정적 검토에서 승인하지 않는다. |

## 검증 결과

- `cargo fmt --all -- --check`: 통과.
- Runtime image focused tests: 9/9 통과.
- FFI source/scene focused test: 1/1 통과 (`CARGO_INCREMENTAL=0`).
- Runner `--list-scenes`: 498개 고유 ID 중 control + 신규 scene 2개 선택 확인. simulator 명령은 실행하지 않았다.
- Android Java/iOS Swift build, iOS Simulator capture, Android AVD capture는 이 보고서 작성 시점에 실행하지 않았다.

**정적 구현 판정: PASS.** 실제 iOS runtime 및 Android runtime 동작은 별도 simulator evidence 전까지 미확인이다. 이 보고서는 그 미확인 결과를 통과로 간주하지 않는다.
