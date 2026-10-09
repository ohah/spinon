# C04.10 · resize 왕복 구현 적대 검토

- 대상: C04.10 현재 working tree의 Android/iOS resize 경계, JNI·FFI·WGPU surface, resize fixture/tests와 실행 증거.
- 계획: [`C04.10 구현 계획`](../../../plan/c04-runtime-css-to-gpu.md) · SHA-256 `e6893181941a2820f1db29782a7520703aa2e30d9af3288546ec5a478b79d199`.
- 비교 기준: [Chromium resize precomparison](c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md).
- 실행 증거: [Android·iOS resize 왕복](c04-runtime-css-to-gpu-resize-simulators-2026-10-09.md), [Android 로그](c04-runtime-css-to-gpu-resize-android-roundtrip-2026-10-09.log), [iOS 로그](c04-runtime-css-to-gpu-resize-ios-26.2-2026-10-09.log).

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 판정 |
|---:|---|---|
| 1 | Android `surfaceCreated`가 크기를 알리기 전에 renderer가 0 크기로 생성되는가 | 코드상 생성 입력의 양수 검사를 확인했다. 최초 surface 생성 뒤 `surfaceChanged`에서 실제 크기가 들어온 다음 생성한다. API 37 화면 시작 로그에서도 0 크기 생성은 없었다. |
| 2 | Android callback이 0 또는 음수 크기를 보고해도 기존 환경이 오염되는가 | `surfaceChanged`에서 무시하고 기존 revision을 유지한다. 이 입력은 별도로 주입하지 않았다. |
| 3 | 같은 크기의 중복 `surfaceChanged`가 새 surface generation을 만들거나 renderer를 파괴하는가 | 크기 비교에서 generation 증가·재생성을 건너뛴다. 같은 크기의 중복 callback 실행은 주입하지 않았다. |
| 4 | renderer 생성 작업 중 크기가 바뀌면 처음 크기의 renderer가 현재 handle로 남는가 | 생성 전후 generation을 확인하고 stale renderer를 파괴한 뒤 최신 크기로 재시도한다. 실제 생성 중 callback 경합은 별도 주입하지 않았다. |
| 5 | 이전 크기 draw가 resize 뒤 render queue에서 실행되는가 | draw admission은 captured surface generation을 다시 확인하므로 낡은 작업은 반환한다. 확대·축소 왕복 로그는 각 draw의 최신 texture를 보인다. |
| 6 | Android에서 새 renderer를 만들기 전에 이전 renderer가 surface를 계속 쓰는가 | 동일 serial render queue에서 이전 renderer 파괴가 새 생성 전에 수행됨을 코드와 renderer 로그로 확인했다. |
| 7 | 연속 resize에서 이전 recreate callback이 새 크기를 다시 덮는가 | recreate task와 main callback에서 generation을 비교한다. 세 번의 정상 간격 왕복은 실행했지만 빠른 callback 폭주는 주입하지 않았다. |
| 8 | JNI context가 `ANativeWindow`를 renderer보다 먼저 release하는가 | context destroy에서 Rust renderer 파괴 완료 뒤 native window를 release하고 context를 해제한다. |
| 9 | Android dp 정수 반올림이 viewport 오차 한계를 넘는가 | API 37 density `2.625`에서 895 px/2.625=`340.9524` CSS px를 확인했다. Chromium 기준 root와의 최대 차이는 `0.5 CSS px` 이하다. |
| 10 | iOS point 좌표와 Metal drawable pixel을 혼동하거나 scale을 두 번 곱하는가 | CSS viewport는 point, drawable은 point×3이다. `301×100 → 341×128 pt`와 `903×300 → 1023×384 px` 왕복 로그를 확인했다. |
| 11 | iOS renderer 생성 중 layout 크기가 바뀌면 구 generation surface가 ready로 남는가 | 생성 전·후 generation을 확인하고 stale renderer를 파괴·재시도한다. 실제 생성 경합 주입은 하지 않았다. |
| 12 | iOS resize 중 draw admission이 열린 상태로 layer를 재구성하는가 | 새 generation 증가 뒤 draw generation 일치가 깨지고 renderer surface generation을 비운다. 시뮬레이터 로그는 configure와 새 generation draw를 순서대로 보인다. |
| 13 | iOS `CAMetalLayer` configure가 UIKit 허용 스레드 밖에서 실행되는가 | configure 경로는 main queue에 고정되어 있다. 현재 iOS build와 simulator 화면 실행은 통과했다. |
| 14 | iOS resize configure와 이미 예약한 draw가 서로 경합하는가 | resize 전에 serial render queue barrier를 통과하고 configure 이후 generation을 재검사한다. 크기 왕복 시 오래된 화면은 캡처되지 않았다. |
| 15 | 화면 크기 전환 중 이전 CSS environment가 새 viewport scene으로 제출되는가 | presentation sequence와 surface generation을 각 단계에서 검사한다. 로그에서 environment revision과 root frame이 매 단계 함께 갱신됐다. |
| 16 | 요청 크기·OS surface·acquired texture·scene 중 한 항목만 새 크기인 채 성공 처리되는가 | 네 경계를 로그·캡처로 대조했다. Android root bbox는 `790×263`과 `895×336`, iOS 최종 bbox는 `1023×384`로 각각 surface texture와 일치했다. |
| 17 | Android renderer 재생성이 CSS host·V8 document를 리셋하는가 | 왕복 내내 host generation `1`, document revision `12`, render tree revision `4`를 유지하고 environment revision만 증가했다. |
| 18 | 큐 포화 시 오래된 resize/draw 작업이 무제한 누적될 수 있는가 | **미완료 결함/요건.** Android `newSingleThreadExecutor`와 iOS serial `DispatchQueue`의 대기열은 무제한이며 최신 pending scene 하나로 제한하는 admission/coalescing은 구현되지 않았다. C04.10 완료를 막는다. |
| 19 | renderer 생성/resize/draw가 실패해도 이전 frame이 성공으로 보고되거나 재사용되는가 | 오류가 status로 분리되는 코드를 확인했지만 WGPU 실패를 실제 주입하지 않았다. 실패 복구는 미검증이다. |
| 20 | shutdown 중 pending draw/resize가 view, native window 또는 host보다 오래 살아남는가 | Android lifecycle 실행의 drain 순서는 별도 증거에 있다. 이번 resize 실행에서는 shutdown과 pending draw를 동시에 강제하지 않았다. queue drain timeout 및 강제 draw 실패 검증도 남는다. |

## 음성 대조

resize 소스 계약 테스트가 잘못된 변경을 잡는지 확인하기 위해 두 조건을 임시로 반전했다. iOS 자동 왕복 횟수 `3→2`, Android stale surface 비교 `!=→==` mutation은 각각 해당 정적 assertion에서 실패했다. 원본을 복원한 뒤 전체 `test:css-reference` 23개가 통과했다. 앞선 잘못된 queue 호출 방식으로 실행한 두 시도는 test-runner 밖 실행 오류였고, mutation 증거로 세지 않았다.

## 수정·상태

resize 화면의 성공 여부를 API status만으로 판정하지 않고 Android native window, WGPU acquired texture, CSS layout/render frame, screenshot root bbox를 함께 대조하도록 증거를 보강했다. iOS 자동 재현은 확대·축소·재확대 세 번의 변경을 수행한다. Android renderer는 기존 V8 host를 유지한 채 resize 때만 재생성한다.

대기열 coalescing/backpressure, pending 작업과 동시 종료, WGPU 오류 주입은 완료로 표시하지 않는다. 따라서 C04.10은 계속 미완료이고 PR merge 준비 상태가 아니다. 내부 계약·제품 버전은 출시 결정 전 `0.1.0`으로 유지한다.
