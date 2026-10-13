# C11.2.2 sparse row placement 구현 실패 관점 검토

> 이 문서는 allocator fault injection 이전의 구현 검토를 보존한 시점 기록입니다. 아래 Android 실기기 행렬은 당시 실행이며 최종 격리 PR 후보와 같은 tree임을 증명하지 않습니다. 현재 후보의 Android API 37 AVD·iOS Simulator 결과는 [최종 후보 구현 검토](./c11-2-2-pr-candidate-implementation-review-2026-10-13.md)와 [후속 allocator 결과](./c11-2-2-grid-allocation-failure-2026-10-13/README.md)를 참조하세요. 다섯 Vec 예약 지점의 후속 주입·복구 검증은 [후속 구현 검토](./c11-2-2-grid-allocation-failure-implementation-review-2026-10-13.md)에 기록했습니다.

이 검토는 계획 검토와 별개로 현재 구현 diff, 고정 Chrome 154 fixture, Android·iOS 앱 로그 및 실제 surface 표본을 대조한다. 대상은 제한 `RuntimeBlockFlexGridAutoPlacementV1`이며 C11 전체·WPT 적합성 검토가 아니다.

## 구현 검토

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 profile 없이 기존 C11.1/C11.2.1 결과를 바꾸는가 | `RuntimeBlockFlexGridAutoPlacementV1`와 `SparseRow` 모드를 분리했다. 기존 fixture와 23개 layout Grid 검사 통과. |
| 2 | `grid-auto-flow`를 CSS 문자열이나 미검증 기본값으로 추측하는가 | Stylo computed typed 값을 투영하고 `row`만 통과시킨다. `dense`·`column` 거부 scene 2개가 Android/iOS에서 정확한 진단으로 종료됐다. |
| 3 | 암시 행의 computed sizing이 손실되거나 다른 속성에 덮이는가 | typed `grid-auto-rows`를 보존한다. 초기값 `auto`는 암시 행이 없을 때만 허용하며, 반복 pattern과 암시 `auto` sizing은 거부한다. Stylo 검사 3개와 runtime scene 2개 확인. |
| 4 | 숨김 자식·손자나 중첩 Grid가 바깥 점유에 섞이는가 | `display:none` subtree를 제외하고 중첩 container마다 독립 분류한다. hidden·nested scene 및 layout 회귀 검사 통과. |
| 5 | positioned item이나 지원하지 않는 `order`가 auto-placement에 조용히 섞이는가 | positioned item과 `order != 0`는 배치 성공으로 가장하지 않고 실패한다. 제한 profile 진단 검사 통과. |
| 6 | definite item을 auto item 뒤에 등록해 셀 충돌을 놓치는가 | 모든 definite row+column item을 먼저 occupancy에 등록한다. `definite-first-mixed`가 Chrome 기준과 일치한다. |
| 7 | definite row/auto column이 앞서 배치된 definite item을 덮는가 | row-locked item을 행별 column cursor와 occupancy로 배치한다. `row-locked-auto-column` 및 span 검사 통과. |
| 8 | definite column이 이전 cursor보다 작아졌을 때 같은 행을 다시 쓰는가 | cursor 행을 먼저 증가시킨다. 기준의 `column-cursor-wrap` 장면에서 두 item 좌표가 Chrome과 일치하고 implicit row가 생긴다. |
| 9 | sparse 모드가 앞서 지나간 빈칸을 다시 채우는가 | 다음 cursor 위치에서만 검색한다. `sparse-hole-not-backfilled`에서 빈칸은 남고 나중 item은 Chrome과 같은 다음 셀로 간다. |
| 10 | item span이 여러 행·열 점유를 빠뜨리는가 | 행별 interval occupancy에 전체 span 범위를 등록한다. `auto-column-span`, 충돌 및 gap 장면이 통과했다. |
| 11 | 역순 line, 같은 line 또는 양 끝 span 충돌을 잘못 해석하는가 | endpoint conflict 정규화 뒤 양 끝·span을 계산한다. reversed/equal/span 조합 Rust 회귀와 고정 scene 비교 통과. |
| 12 | 음수 explicit line을 implicit track으로 오해하는가 | explicit track 기준 line 정규화를 수행한다. 음수·양수 line fixture 통과; explicit grid 바깥은 거부한다. |
| 13 | 큰 span·line 산술이 overflow하거나 범위를 잘라 성공 처리하는가 | checked arithmetic, line 검증, track 상한을 Taffy 호출 전에 적용한다. cap 회귀와 잘못된 placement 거부 통과. |
| 14 | definite column이 explicit 열 밖일 때 암시 열을 만들어 범위를 넓히는가 | 암시 열 요청을 전체 계산 오류로 거부한다. 양 플랫폼에서 해당 장면은 frame 0·양수 draw 0이다. |
| 15 | `grid-auto-rows:auto`나 반복 pattern을 고정 높이로 취급하는가 | 자동·반복 sizing은 unsupported 진단으로 닫는다. 거부 scene이 Android/iOS 모두 기대 결과다. |
| 16 | 단일 px implicit row가 gap·track 확장과 다르게 배치되는가 | 양의 implicit row를 고정 px track으로 확장하고 기존 row gap을 유지한다. `implicit-track-size-and-gap` Chrome 비교 및 surface sample 통과. |
| 17 | nested container, hidden item 또는 실패 scene 뒤에 paint order가 바뀌는가 | source-order NodeId와 paint order를 성공 frame에서 비교한다. nested·hidden 장면 및 다섯 거부 장면의 무출력 결과 통과. |
| 18 | 오류 scene이 부분 frame을 내거나 이후 scene을 오염시키는가 | 각 scene은 새 process/HostDocument에서 실행한다. 기대 거부 5개는 frame·양수 draw가 없고; 후속 계산 회복 unit test도 통과한다. 다만 Grid reservation allocator failure 주입은 아직 하지 않아 이 오류 경로의 복구는 미검증이다. |
| 19 | FFI·플랫폼 보고서가 scene 경계를 무시하거나 표본 JSON을 자르는가 | 미등록 scene, null host, 잘못된 출력 인수를 거부한다. 실행 중 iOS 1 KiB 보고서 버퍼에서 표본 JSON이 잘리는 문제가 드러나 Android/iOS 버퍼를 16 KiB로 늘린 뒤 두 행렬을 재실행했다. FFI 5/5 통과. |
| 20 | screenshot을 surface parity로 오인하거나 한 플랫폼 결과를 다른 플랫폼에 일반화하는가 | Android 실기기 PixelCopy와 iOS Simulator Metal drawable에서 각각 11 scene·51 sample이 Chrome DPR 1과 exact, 최대 채널 오차 0이다. screenshot은 보조 자료이며 iOS 최종 compositor, iOS 실기기, 다른 Android 기기 및 WPT 적합성은 증명하지 않는다. |

## 재실행 결과

- Chrome 154 scene/node 기준: 16 scene·69 node, 11개 지원·5개 거부.
- Android 16/API 36 SM-S731N 실기기: 16/16, geometry 최대 오차 0 CSS px, PixelCopy 51/51 sample exact. Renderer는 Xclipse 940/Vulkan 1.3.279 경로다.
- iPhone 17 Pro / iOS 26.2 Simulator: 16/16, geometry 최대 오차 0 CSS px, Metal drawable 51/51 sample exact. readback은 OS compositor 제출 전이다.
- Rust: `spinon-layout` Grid 23개, `spinon-style-to-layout` 2개, `spinon-style` 3개, `spinon-runtime` 진단 1개, `spinon-ffi` 5개 통과.
- JavaScript 도구: runtime matrix 1개, runtime 비교 27개, pixel matrix 9개 통과.
- 이 검토에서 확인한 배치 좌표·진단·표면 표본 불일치는 없다. 명시적 allocator failure injection, WPT runner 적합성, 전체 CSS Grid, iOS 실기기와 OS compositor 이후 화면은 미검증이다.

## 열린 경계

이 subset의 `Vec::try_reserve` 오류는 `GridAllocationFailure`로 전달되도록 작성됐지만 fault-injection으로 실행하지 않았다. `BTreeMap`·`BTreeSet` 등 다른 할당과 전역 OOM을 복구한다고 주장하지 않는다. 해당 주입 테스트가 끝날 때까지 C11.2.2의 실패 원자성 관문과 상태 체크는 미완료로 유지한다. 전체 Grid/CSS 지원이나 WPT pass를 뜻하지 않는다.
