# C11.4 cross-axis fixed-repeat + auto-repeat + template areas PR diff 검토 30개

검토 대상은 브랜치 feat/c11-4-cross-axis-fixed-auto-repeat-areas-20261025, 기준 HEAD 96efeebd9e816110ac981b6c24eecd3e88b620a8이다. 최초 코드 diff를 SHA-256 81fa7e7865a9e6ef0d08273c5bc67b3dbe4f7d8e9b0496775c39848504505870에 고정해 검토했고, 최종 README/staticlib evidence 수정은 이 review 파일을 제외한 staged diff SHA-256 d21be3e0a758c55a5eb3f47a4913258b9d09459c826e06df53c4e27701669808로 다시 대조했다. 구현·runtime 실패 관점 20개 검토와 별도로, 코드·fixture·플랫폼 경계·문서 변경을 대상으로 30개 실패 관점을 확인했다. 기존 시뮬레이터·빌드 결과는 runtime README의 증거를 참조했으며 본 리뷰에서 테스트를 다시 실행하지 않았다.

## 검토 항목

1. **예외 활성 조건이 fixed-repeat과 auto-repeat이 서로 다른 축에 함께 있을 때로 한정되는가 — PASS.** predicate는 cross-axis 조합에서만 새 metadata를 요구한다. grid.rs:129–145.
2. **columns-fixed/rows-auto와 rows-fixed/columns-auto가 같은 제한을 적용하는가 — PASS.** 양방향 branch가 fixed 축 1 group, auto 축 0 fixed group 및 bounded 조건을 각각 요구한다. grid.rs:132–145; 양방향 projection test는 c11_4_track_sizing_areas_tests.rs:446–475.
3. **fixed 축의 repeat group이 둘 이상일 때 예외가 열리지 않는가 — PASS.** group count를 보존하고 정확히 1을 요구한다. grid.rs:484–505, 132–145.
4. **auto-repeat 축에 추가 fixed-repeat group이 끼면 거부되는가 — PASS.** auto 축 group count가 0이어야 한다. fixed_repeat_auto_repeat.rs의 unsupported combination cases.
5. **fixed-repeat 안의 pattern 크기가 1–2 track으로 제한되는가 — PASS.** single_bounded_fixed_repeat의 개수 범위와 negative matrix가 이를 고정한다. grid.rs:509–529; fixed_repeat_auto_repeat.rs.
6. **auto-repeat pattern이 단일 breadth인지 확인하는가 — PASS.** bounded auto-repeat predicate가 track_sizes 길이 1과 list.values 길이 1을 요구한다. auto_repeat.rs:262–269.
7. **track breadth eligibility가 computed finite nonnegative LengthPx로 제한되는가 — PASS.** 양 helper가 track_size의 결과 타입, finite 여부, 0 이상을 확인한다. grid.rs:519–527; auto_repeat.rs:265–273.
8. **fixed 축에 repeat 바깥 ordinary track이 섞이면 허용하지 않는가 — PASS.** single_bounded_fixed_repeat은 list.values가 repeat 하나뿐이어야 한다. grid.rs:509–517.
9. **auto-repeat 축에 prefix/suffix ordinary track이 섞이면 허용하지 않는가 — PASS.** list.values 길이가 1이어야 한다. auto_repeat.rs:262–269; 계획의 negative boundary는 c11-4-cross-axis-fixed-auto-repeat-areas.md:42.
10. **percentage, flex, intrinsic, 미정규화 calc, minmax fixed breadth의 회귀 거부가 있는가 — PASS.** 양방향 fixed-axis negative matrix가 각 grammar를 점검한다. fixed_repeat_auto_repeat.rs.
11. **fixed-repeat을 두 auto-repeat 축에 혼합하는 조합이 계속 거부되는가 — PASS.** 두 방향의 exact root/property rejection test가 추가됐다. fixed_repeat_auto_repeat.rs의 fixed_repeat_mixed_into_both_auto_repeat_axes_is_rejected_in_both_orientations.
12. **template areas 없는 cross-axis 조합이 예외를 통과하지 않는가 — PASS.** template_areas.is_some()가 eligibility의 필수 조건이고 no-area negative control이 있다. grid.rs:132–145; fixed_repeat_auto_repeat.rs.
13. **비대상 runtime profile로 예외가 새지 않는가 — PASS.** cascade caller가 integer repeat·intrinsic·auto-repeat을 모두 허용하는 profile을 RuntimeBlockFlexGridTrackSizingV1 하나로 제한한다. element.rs:185–202.
14. **fixed축 area dimension mismatch가 명시적으로 fail-closed되고 문서화됐는가 — PASS.** projection test는 작거나 큰 area dimension을 root의 grid-template-areas 오류로 확인한다. 계획과 계약은 동일 경계를 명시한다. c11_4_track_sizing_areas_tests.rs:477–511; plan/c11-4-cross-axis-fixed-auto-repeat-areas.md:33,40,55,64; spec/internal/0059-c11-grid-layout.md.
15. **auto-repeat 반복 수와 area-added track이 구분되는가 — PASS.** fixtures는 area로 더해진 row/column을 grid-auto-* 12px로 별도 기대하고 계획은 auto-repeat count를 늘리지 않는다고 명시한다. scene JSON의 area-adds-two-rows/area-adds-one-row/area-adds-one-column cases; 계획 :25–33.
16. **area 문법의 rectangularity와 parser 오류 경계가 유지되는가 — PASS.** 새 경로는 기존 computed_grid_template_areas parser 결과를 사용하고 새 raw area parser를 만들지 않는다. grid.rs:84–89.
17. **공개 layout type 또는 내부 contract version이 불필요하게 바뀌지 않는가 — PASS.** eligibility metadata는 private ComputedGridTracks 필드이며 contract 0059는 0.1.0을 유지한다. grid.rs:27–35; spec/internal/0059-c11-grid-layout.md.
18. **track 및 line/area metadata resource boundary가 새 코드에서 우회되지 않는가 — PASS.** 변경은 private eligibility 계산에 한정되고 existing expansion/reservation 경로를 호출한다. grid.rs:495–505; plan/c11-4-cross-axis-fixed-auto-repeat-areas.md:49.
19. **기존 dual-auto, same-axis fixed+auto 및 no-area negative 조합이 새 예외와 분리되는가 — PASS.** 새 predicate는 fixed-repeat과 반대축 auto-repeat의 동시 존재를 먼저 요구하며 기존 regression matrix가 유지된다. grid.rs:129–165; fixed_repeat_auto_repeat.rs.
20. **positive projection이 두 축 전치에서도 expected frame을 확인하는가 — PASS.** fixed columns와 fixed rows scene의 root item frame을 각각 검증한다. c11_4_track_sizing_areas_tests.rs:446–475.
21. **fixture registry의 총량, cross-axis 개수 및 ID 유일성이 고정되는가 — PASS.** 40 area scenes, cross-axis 11개, ID 중복 방지와 명시 목록을 확인한다. c11_4_template_area_auto_repeat.rs:91–169.
22. **fixture schema가 빈 area·단일 item·두 item을 허용하면서 기대 track 배열을 고정하는가 — PASS.** 0/1/2 item count와 columns/rows 배열을 장면별로 비교한다. c11_4_template_area_auto_repeat.rs:170–220.
23. **과거 41-scene 회귀 집합이 새 11개와 섞여 결과를 부풀리지 않는가 — PASS.** capture manifest는 과거 template-area ID 29개를 유지하고 새 ID를 별도 11-scene selection으로 확인한다. fixture-module-boundary.test.mjs:280–350.
24. **새 장면만 선택했을 때 전용 evidence 경로와 run kind가 분리되는가 — PASS.** capture script가 전 장면 prefix를 검사하고 전용 evidence directory를 선택한다. capture-c11-4-auto-repeat-neighbors-simulator.mjs:146–200.
25. **Android route가 새 prefix만으로 임의 장면을 허용하지 않는가 — PASS.** BuildConfig fixture flag와 canonical scene allowlist를 모두 확인한다. C0410RuntimeGpuDemo.java:618–624, 3501–3521; boundary test는 route를 고정한다.
26. **iOS route가 동일 장면을 fixture build에서만 노출하는가 — PASS.** 11개 Set은 compile condition 아래 있고 cross-axis prefix는 해당 allowlist를 요구한다. C0410RuntimeGpuDemo.swift:53–68, 371–380.
27. **C header 선언 조건이 fixture ABI의 native build gate와 맞는가 — PASS.** C11.4 constructor declaration은 core runtime과 fixture-test define을 모두 요구한다. spinon_ffi.h:455–461; header matrix 결과는 runtime README:31–32에 기록돼 있다.
28. **scene/sample ID 길이 증분이 소비 계층 전체와 맞는가 — PASS.** FFI byte length, Android sample ID regex, capture case ID 제한은 128이다. scene JSON에서 계산되는 최장 신규 sample ID는 112 ASCII bytes다. c11_4.rs:40–58; C0410RuntimeGpuDemo.java:11297–11310; SurfaceViewPixelCapture.java:159–164.
29. **Android/iOS 결과 검사가 track·collapse·trace·surface sample을 서로 구분하는가 — PASS.** 플랫폼 adapter는 root/item frames, 양 축 final trace, collapsed track과 shared gutter를 따로 검사하고, overflow 표본은 root 밖 hero 및 hero 뒤 첫 clear 위치에 연결한다. C0410RuntimeGpuDemo.java:3764–3930, 4023–4100, 9214–9248; C0410RuntimeGpuDemo.swift:2382–2460, 3600–3678. runtime 증거는 Android/iOS 각각 scene 11/11, 표면 표본 35/35다(README:7–25).
30. **PR evidence 축소 뒤 검사 결과와 경계 주장이 보존되는가 — FOLLOW-UP PASS.** git diff --cached --check는 캡처 원본을 유지한 avd-startup.log와 ios-build.log에서 11개 공백 지적을 계속 보고한다. 최종 runtime README는 이 11건과 원본 보존 사유를 명시하고, staged source/docs에 추가 지적이 없음을 구분한다. staticlib README에는 두 feature 조합의 재현 빌드·llvm-nm·검증기 명령, 파일 크기·SHA, 필터된 host-symbol 일치 결과와 JSON 설명이 있으며, full symbol dump는 재생성 가능하다. 로컬 full dumps 및 원본 archive의 stage 제외·보존 여부와 staged filtered result는 아래 후속 delta 확인을 참조한다.

## 판정

## 최종 evidence delta 확인

기능 코드·fixture 경계·플랫폼 route 판정은 그대로 유효하다. 최종 staged patch에서는 27,479,917-byte 및 28,393,121-byte full llvm-nm 정의 심볼 덤프와 660,361,184-byte 및 696,066,136-byte 정적 archive가 제외됐고, 네 파일 모두 로컬에 남아 있다. PR stage에는 verifier, 재현/경계 설명, filtered C11.4 host-symbol match files 및 fixture-free/positive-control result JSON이 포함됐다. README는 재현 절차를 통해 전체 심볼 dump를 다시 생성할 수 있음을 설명하며, 이 자료가 최종 APK/IPA fixture exclusion을 증명하지 않는다고 유지한다. 따라서 증거 축소는 주장의 경계를 바꾸지 않는다.

이 문서는 diff 검토이며 GitHub PR의 live checks, merge 상태와 병합 후 main 검증은 다루지 않는다.
