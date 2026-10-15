# C11.3.3 iOS Simulator matrix PR #211 적대 검토 · 2026-10-16

대상 PR: [#211](https://github.com/ohah/spinon/pull/211), 최신 head `fd597175`. 구현 관점 20개와 별도로 PR diff·증거 경계·배포 표현을 검토했다. 처음에는 generic `SPINON_C0410_DRAW`가 해당 scene의 paint order보다 앞서 있어도 성공으로 인정될 수 있었다. 비교기와 capture 대기를 paint marker 뒤 draw로 제한하고 순서 변조 negative control을 추가했다. 변경 뒤 iOS 전체 행렬을 다시 실행했다. 미해결 P1/P2는 없다.

| # | 독립 실패 관점 | 판정 근거 |
| --- | --- | --- |
| 1 | PR scope가 다른 CSS/Grid 작업까지 완료로 끌어올리는가 | 상태 수정은 C11.3.3 근거에 국한되고 상위 C11 설명은 완료 처리하지 않는다. PASS |
| 2 | candidate를 완료 checkbox로 바꾸었는가 | `spec/STATUS.md`의 C11.3.3 checkbox는 미완료 그대로다. PASS |
| 3 | 상위 C11.3/C11 계약을 이 실행 결과로 완료 처리하는가 | PR과 evidence 모두 제한 profile candidate로 명시한다. PASS |
| 4 | 61 frame 비교 수가 공식 완료 수로 표시되는가 | Agent Browser에서 공식 문구 `197개 표시 · 공식 완료 65/197`를 확인했고 evidence README에 61이 frame 수라고 분리했다. PASS |
| 5 | 내부 인터페이스 버전을 검증 증거만으로 올렸는가 | 내부 계약 0059는 `0.1.0` 유지다. PASS |
| 6 | fixture 결과가 공개 API 지원 주장으로 쓰이는가 | API 없음·fixture 제한·WPT 미검증을 PR 본문에 기록했다. PASS |
| 7 | 다른 schema/profile matrix를 같은 입력으로 허용하는가 | validator가 schema와 `RuntimeBlockFlexGridTemplateV1`을 고정한다. PASS |
| 8 | fixture 장면/node가 빠져도 표본 결과가 통과하는가 | 26 scene·65 node와 scene 순서·고유 ID를 고정한다. PASS |
| 9 | 성공/fallback/rejection 비율이 바뀌어도 통과하는가 | 집계 19/5/2를 고정하고 재실행 결과도 일치한다. PASS |
| 10 | 비교 oracle가 바뀌거나 다른 Chrome 빌드를 가리키는가 | Chrome 154.0.8037.92 reference ID와 source hash를 결과에 기록한다. PASS |
| 11 | DPR 차이 때문에 기준 geometry가 다른가 | 고정 oracle의 DPR 1 observation만 선택한다. PASS |
| 12 | 같은 이름의 scene이 다른 root/node inventory와 결합되는가 | scene ID, root ID와 node ID 순서를 Chrome case와 대조한다. PASS |
| 13 | geometry 오차가 허용치를 넘어도 전체 평균으로 숨는가 | x/y/width/height를 node마다 0.5 CSS px 한도로 대조하며 실제 최대 오차는 0이다. PASS |
| 14 | 숨겨진 node가 누락되거나 visible paint에 섞이는가 | nested hidden node frame 1개는 zero-size로 보존되고 paint list에서 빠진다. PASS |
| 15 | paint node 개수만 맞고 source order는 뒤집힐 수 있는가 | Chrome visible DOM 순서 배열과 node ID 전체 순서를 비교하고 swap 변조를 거부한다. PASS |
| 16 | 이전 또는 순서가 앞선 WGPU draw가 현재 layout 성공으로 오인되는가 | 현재 run/scene paint marker 뒤에 있는 draw만 받고 draw-before-paint 변조를 거부한다. PASS |
| 17 | rejection의 code/property/target node가 틀려도 성공하는가 | 두 rejection의 정확한 code, CSS property, runtime node ID를 확인한다. PASS |
| 18 | rejection 장면에 일부 layout frame이 남아도 통과하는가 | `frames=absent`와 node frame marker 부재를 검사한다. PASS |
| 19 | rejection이 surface draw/readback을 거친 뒤에야 실패 처리되는가 | rejection 로그에서 draw와 Metal pixel begin 모두 없어야 한다. PASS |
| 20 | unknown scene argument가 runtime route로 흘러드는가 | 미등록 scene negative control이 eval, summary, draw와 readback 전에 거부됐다. PASS |
| 21 | 잘못된 sample 좌표가 readback 후 거부되는가 | x 좌표를 1 CSS px 변조한 negative control이 Metal readback 전에 거부됐다. PASS |
| 22 | 이전 실행의 `-12`가 현재 scene 실패를 숨기거나 무한 재실행하는가 | 동일 run/scene superseded만 단회 retry하고 최신 전체 실행의 retry는 0회다. PASS |
| 23 | 실행 ID가 앱 argument 제한이나 허용 문자 경계를 넘는가 | 허용 문자·scene index·80자 상한 및 retry 표기를 검사한다. PASS |
| 24 | 이전 simulator 프로세스 로그가 새 장면 결과에 합쳐지는가 | capture는 fresh launch PID로 로그를 필터하고 frame/paint marker도 run ID와 scene ID로 scope한다. PASS |
| 25 | evidence JSON이 다른 checkout이나 app binary를 가리키는가 | commit `1d00fad0`, fixture/oracle/source/app SHA-256을 저장하고 최신 결과에서 hash 대조가 일치한다. PASS |
| 26 | V8이 다른 source revision인 빌드가 섞이는가 | 빌드 source는 pinned V8 `7b50b62cb18f28617959e8452e2cd18195b38bcf`와 일치한다. PASS |
| 27 | 검증 fixture가 기본 제품 빌드에 켜졌다고 주장하는가 | capture는 `C04 runtime GPU`와 C11 Metal 표본을 명시 opt-in flag로 빌드한다. PASS |
| 28 | simulator 결과를 실기기·hardware GPU·compositor 결과로 확대하는가 | iPhone 17 Pro/iOS 26.2 Simulator 및 compositor 전 Metal drawable 범위만 기록한다. ADB/실기기 검증은 주장하지 않는다. PASS |
| 29 | 실패한 전체 검사나 CI 상태를 숨기고 모두 green으로 표현하는가 | 집중 10/10·runtime 12/12와 전체 CSS reference 470/472(기존 C13.3 tap probe 2 실패), 전체 docs build dead-link 실패, scoped docs build 성공을 분리한다. GitHub checks는 보고되지 않았고 main required status checks도 활성화되지 않았다. PASS |
| 30 | PR 본문 동작 이미지가 깨지거나 다른 run의 화면을 보여주는가 | Agent Browser에서 최신 raw image가 complete, 1206×2622 픽셀로 로드됨을 확인했다. PASS |

## 재검증 결과

- `node --test tools/css-reference/c11-3-3-grid-template-areas-ios-simulator-matrix.test.mjs`: 10/10 통과. paint node/ordinal 변조, draw-before-paint 순서 변조를 포함한다.
- `mise exec -- bun run test:css-c11-3-3-grid-template-areas-runtime`: 12/12 통과.
- iPhone 17 Pro / iOS 26.2 Simulator: 26 scene·65 node, success 19 / fallback 5 / rejection 2, visible frame 61개 geometry 최대 오차 0 CSS px, Metal 표본 2/2 exact.
- PR 이미지와 Tailnet 공식 완료 카운터는 Agent Browser로 확인했다. 61은 화면 frame 수이고 공식 완료 항목은 65/197이다.

## 남은 경계

C11.3.3·상위 C11, Spinon WPT 적합성, iOS/Android 실기기, hardware GPU와 compositor 이후 pixels는 미완료다. 이 문서는 PR diff의 검토 근거이며 지원 범위를 넓히지 않는다.
