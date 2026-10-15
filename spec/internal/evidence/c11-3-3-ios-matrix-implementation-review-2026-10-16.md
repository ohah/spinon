# C11.3.3 iOS Simulator matrix 구현 실패 관점 검토 · 2026-10-16

대상은 iOS capture/검증기와 `C0410RuntimeGpuDemo.swift`의 C11.3.3 진단이다. 각 관점은 다른 입력 누락·상태 혼동·실패 경계를 확인한다. 첫 재검토에서 paint ordinal과 paint node 집합만 세고 순서를 비교하지 않는 공백을 발견했다. 검증기를 visible DOM source order와 정확히 대조하도록 고치고 node/ordinal 순서 변조 negative control을 추가했다. 수정 후 전체 iOS 행렬을 다시 실행했다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 다른 schema/profile의 runtime matrix가 같은 자료로 오인되는가 | schema와 `RuntimeBlockFlexGridTemplateV1` 불일치 입력을 거부한다. PASS |
| 2 | scene/node/outcome 수가 바뀌어도 고정 fixture로 통과하는가 | 26 scene·65 node·19/5/2 결과 분포를 고정하고 일치시킨다. PASS |
| 3 | scene 순서 변경이나 중복 ID가 Chrome case를 잘못 연결하는가 | 연속 index·유일 scene ID·scene별 Chrome ID 존재를 확인한다. PASS |
| 4 | scene root ID가 Chrome fixture root와 달라도 통과하는가 | 각 scene의 root ID를 고정 Chrome DPR 1 case와 비교한다. PASS |
| 5 | runtime node의 DOM 순서/이름이 기준 inventory와 어긋나는가 | scene별 node ID 목록과 Chrome node 배열의 순서를 비교한다. PASS |
| 6 | 누락된 Chrome case 또는 node rect를 성공으로 처리하는가 | 성공 각 node의 Chrome rect를 요구한다. 전체 26-scene 실행에서 누락은 없었다. PASS |
| 7 | 보이는 node frame을 숨기거나 hidden node를 불필요한 paint로 세는가 | positive-size frame은 필수이고 hidden zero-size frame은 frame 비교에 보존한다. visible 비교는 61개다. PASS |
| 8 | 좌표·크기 drift가 허용치를 넘어도 통과하는가 | 각 축 0.5 CSS px 초과 mutation은 거부한다. 실제 최대 오차는 0 CSS px다. PASS |
| 9 | 중복·미등록 runtime node ID가 geometry 집계를 오염시키는가 | 중복 ID와 scene 범위 밖 node ID를 거부한다. PASS |
| 10 | 누락·역순 frame index가 연속 결과처럼 보이는가 | frame index를 `0..n-1`과 대조하고 frame summary count를 실제 frame 수와 맞춘다. PASS |
| 11 | 평가가 실패했거나 layout 준비 전인 로그를 성공 처리하는가 | 성공 summary에 `status=0 layout=ready`와 완전한 frame marker를 요구한다. PASS |
| 12 | 이전 scene의 paint marker가 현재 scene 결과에 붙는가 | paint log를 run ID와 scene ID로 scope한다. PASS |
| 13 | 중간 토큰·누락 ordinal이 paint 순서로 파싱되는가 | canonical `ordinal:nodeId` 형식과 0부터 연속한 ordinal을 요구한다. malformed 입력은 거부한다. PASS |
| 14 | paint node가 빠지거나 순서가 뒤집혀도 개수만으로 통과하는가 | Chrome 기준 visible DOM source order와 node ID 전체 배열을 비교한다. node/ordinal swap negative control은 거부됐다. PASS |
| 15 | layout box 수와 실제 WGPU 제출 수가 다른가 | paint count, runtime boxes, 성공 draw의 box count가 같아야 한다. PASS |
| 16 | 기대 거부 코드·CSS property·target node가 바뀌어도 성공하는가 | 두 거부의 정확한 code/property/node를 각각 확인한다. unknown area는 `grid-row-start`, runtime node 2다. PASS |
| 17 | 거부 뒤 부분 layout frame이 남는가 | 기대 거부 로그에 node frame과 frame marker가 없어야 한다. PASS |
| 18 | 거부 전에 draw 또는 Metal readback이 일어나는가 | 두 기대 거부에서 WGPU draw와 Metal pixel begin 부재를 확인한다. PASS |
| 19 | 미등록 scene이 runtime route를 타거나 GPU surface에 도달하는가 | argument rejection 뒤 eval/summary/draw/readback이 없어야 한다. simulator negative control 통과. PASS |
| 20 | status `-12`를 실제 오류로 숨기거나 반복 재실행하는가 | 동일 run/scene의 superseded `-12`만 인식하고 단회 retry만 허용한다. 80자 run ID 경계도 검사한다. 실제 최신 행렬은 retry 0회다. PASS |

## 실행 근거

- iOS Simulator matrix 검증기 10/10, 기존 C11.3.3 runtime 회귀 12/12 통과.
- iPhone 17 Pro / iOS 26.2 Simulator에서 최신 검증기로 26 scene·65 node 전체 재실행, visible frame 61개 최대 geometry 오차 0 CSS px, Metal 표본 2/2 exact.
- 상세 로그·fixture/oracle/source/app SHA-256: [2026-10-16 iOS matrix evidence](c11-3-3-grid-template-areas-runtime-2026-10-16/README.md).
- 제한: 이 fixture는 source-order paint를 검사하며 z-index/stacking-context, 실기기, compositor 이후 픽셀, Spinon WPT 적합성의 근거가 아니다.
