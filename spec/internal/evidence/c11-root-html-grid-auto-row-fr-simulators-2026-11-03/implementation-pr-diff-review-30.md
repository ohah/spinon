# C11.1 fractional implicit-row 구현 PR 실제 diff 검토

구현·검증·계약·상태 문서 및 첨부 evidence로 구성된 실제 PR diff를 대상으로 별도의 실패 관점 30개를 검토했다. 각 항목은 파일 간 계약 누수나 변경된 입력 경계를 따로 확인한다.

1. **Cascade option 소유자:** 새 fraction flag는 `computed_element_output` profile 선택에서만 켜져 Runtime Root HTML에 귀속된다.
2. **기존 cascade flags:** px/% 목록 및 template-area flags와 조건식이 섞이지 않았고, 기존 분기 값이 유지된다.
3. **Row parser 배선:** `computed_grid_auto_rows`에만 fraction-list option이 전달된다.
4. **Column parser 비변경:** `computed_grid_auto_columns` 구현과 option은 바뀌지 않았다. 넓은 Root HTML suite의 auto-column 실패도 별도 제한으로 기록했다.
5. **typed 순서 보존:** cascade 결과의 fraction 값 순서가 projection과 layout track list까지 유지된다.
6. **Named-area 격리:** projection에서 implicit rows와 template areas의 결합은 계속 거부된다.
7. **Column-flow 격리:** column-flow profile은 auto rows가 `Auto`여야 하고, 이번 fraction row list로 우회할 수 없다.
8. **No-area profile 경계:** direct row admission은 제한된 Root HTML placement mode 외에 일반 CSS Grid admission을 열지 않는다.
9. **Projection 숫자 검증:** finite/non-negative factor check가 direct fraction 값에도 적용된다.
10. **Projection list 형태:** empty, unsupported, non-direct, over-limit 목록은 새 허용 경로에도 통과하지 않는다.
11. **Layout type whitelist:** RootHtml implicit-row mode의 accepted variants에만 fraction이 추가됐다. 다른 placement mode 분기는 변경되지 않았다.
12. **잘못된 factor 오류 종류:** negative/NaN/infinity가 unsupported-value나 초기값 fallback이 아니라 `InvalidGridTrack`으로 실패한다.
13. **Expanded list 순서:** implicit expansion은 명시 행을 보존한 뒤 pattern을 cycle한다.
14. **Expanded row cap:** placement 확장 전 existing `GRID_MAX_TRACKS` 및 reservation 실패 경계가 유지된다.
15. **Percentage basis validation:** `%` 또는 `fr`가 섞인 목록은 definite finite height를 요구한다.
16. **Height constraint:** min-height/max-height/aspect-ratio 검사도 relative row list일 때 기존 fail-closed 경계를 따른다.
17. **Individual percentage overflow:** resolved percentage가 f32 범위를 넘으면 node/property 오류로 frame 공개 전 거부한다.
18. **Aggregate overflow:** fixed/% expanded rows와 row-gap 합이 f32를 넘으면 전체 scene publication 전에 거부한다.
19. **Fraction range:** direct fraction은 validator에서 먼저 finite/non-negative 확인을 받고 sizing 단계에서 사용된다.
20. **Gap arithmetic:** expanded range에서 gutter 개수를 `tracks - 1`로 세어 explicit/implicit 사이를 포함하고 percentage basis에는 gap을 차감하지 않는다.
21. **Taffy 재사용:** 새 flex sizing 알고리즘이나 별도 app/test route를 추가하지 않고 기존 Grid track sizing 입력으로 전달한다.
22. **Overflow semantics:** percentage/fraction used size 및 fixed item minimum은 root로 clamp하지 않는 기존 visible-overflow 의미를 유지한다.
23. **Explicit/scalar compatibility:** 단일 px `LengthPx`와 기존 px/% track list route를 보존한다.
24. **Cascade fallback controls:** `-1fr` row syntax fallback test가 기존 invalid-declaration 제어를 고정한다.
25. **Layout oracle coverage:** cycle, mixed, zero, zero-factor, subunit, fixed-item-minimum 및 overflow tests가 Chrome expected geometry와 연결된다.
26. **Runtime DOM identity:** runtime fixture tests가 id/class 변경과 source reorder를 별도 조작해 selector 이름에 의존하지 않는지 확인한다.
27. **Runtime failure atomicity:** invalid height constraint 입력은 error를 남기고 completed scene/frame snapshot을 발행하지 않는다.
28. **Committed fixture linkage:** runtime unit tests가 committed `root-html-grid-auto-row-fr-runtime.html`을 `include_str!`로 불러오며 app-run source SHA도 같은 hash다.
29. **Mobile build isolation:** app resource 복사는 빌드 중에만 했고 Android/iOS 원본 assets를 복구했으며 tracked asset diff가 없다. AVD software renderer와 iOS Simulator GPU, readback 여부를 구분했다.
30. **Docs/status contract:** 계약 0059와 STATUS는 bounded candidate와 남은 test failure를 기록하지만 C11 checkbox, `72/213`, API version `0.1.0`은 바꾸지 않는다. Plan은 PR/merge/roadmap sync가 끝났다고 과장하지 않는다.

넓은 style-to-layout Root HTML 36/37 결과, PR check 미확인 상태 및 screenshot이 surface-readback 근거가 아니라는 점을 실제 PR 설명에도 유지해야 한다.
