# C11.2.1 모바일 runtime 행렬 구현 실패 관점 검토 · 2026-10-12

**대상:** 고정 fixture 생성기, Android/iOS capture runner, runtime 비교기, 실제 AVD·Simulator 실행 증거.
**범위 판정:** 20개 실패 경계를 코드·회귀 테스트·원시 실행 결과와 대조했다. 이 검토는 실기기, surface pixel parity, WPT 적합성 또는 전체 CSS 지원의 완료 판정이 아니다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | inventory에 없는 scene을 앱에 주입한다 | 고정 scene allowlist가 아닌 입력을 생성기와 FFI에서 거부한다. 알 수 없는 scene을 넘기는 회귀 테스트가 통과했다. |
| 2 | fixture 문자열로 실행 JavaScript를 탈출한다 | 생성기는 고정 HTML 선언만 읽고 문자열을 JSON escaping한다. 호출자 제공 JavaScript는 받지 않는다. escape·입력 편집 negative test 3/3 통과. |
| 3 | scene 순서·fixture node preorder를 바꿔도 전체 개수만 맞춘다 | inventory·HTML·reference case 순서와 node 관계, source hashes를 생성 단계에서 대조한다. 생성물 `--check` 통과. |
| 4 | 기준 파일 변경 후 예전 결과를 재사용한다 | 두 플랫폼 결과가 같은 inventory/HTML/Chrome reference hashes를 기록하며 앱 binary hash는 플랫폼별로 기록한다. |
| 5 | 기대 거부를 성공 후보 수에 포함한다 | 10 scene을 성공 6, 거부 4로 따로 집계한다. 두 플랫폼 모두 두 합계가 각각 일치했다. |
| 6 | 지원 scene의 frame 하나를 빠뜨리거나 중복시킨다 | node별 preorder·ID·유한 좌표·전체 frame 수를 검사한다. 누락·변이·서로 다른 중복 report negative test가 통과했다. |
| 7 | geometry 허용치를 결과 확인 뒤 넓힌다 | 고정 기준은 Chrome 154 DPR 1, 0.5 CSS px다. 실제 양 플랫폼 최대 오차는 0 CSS px. |
| 8 | 이전 실행의 PID/log를 새 case 결과로 사용한다 | 각 case는 앱 process를 재시작한다. Android/iOS 수집은 run ID와 scene ID를 요구하며 process ID를 결과에 기록한다. |
| 9 | Android logcat 전체를 비워 사용자의 로그를 지우거나 stale row를 읽는다 | 수집은 전체 logcat 삭제 없이 새 PID·run·scene가 일치하는 marker만 비교한다. |
| 10 | iOS 이전 app process가 scene을 계속 소유한다 | runner가 이전 process 종료를 기록하고 종료 상태를 기다린 뒤 새 launch PID를 수집한다. 최신 10개 case가 개별 PID로 완료됐다. |
| 11 | `simctl launch` 응답 시간 초과를 앱 실행 실패로 단정한다 | 이전 시도에서 launch 응답 지연과 실제 실행을 구별했다. runner는 새 process가 확인될 때만 PID 복구를 허용하고 이후 run marker를 기다린다. |
| 12 | iOS surface 재구성 중 superseded된 평가를 최신 결과로 사용한다 | ready environment 뒤의 run-bound summary/frame/paint marker를 비교한다. 최신 전체 matrix 10/10에서 scene 결과가 귀속됐다. |
| 13 | summary보다 먼저 생긴 이전 draw를 새 장면의 제출로 쓴다 | 비교기는 성공 scene에서 summary 뒤의 기대 box 수 draw를 요구한다. 실제 6개 성공 scene 모두 제출 기록이 있다. |
| 14 | 기대 거부가 timeout/crash/disabled fixture로 위장한다 | runner 오류·timeout과 CSS 진단을 별도 실패로 처리한다. 네 거부 scene은 각각 새 process에서 지정된 오류 code를 냈다. |
| 15 | named-line 거부의 소유 node/property를 임의로 보고한다 | 양 플랫폼 실행에서 `cascade_failed`, owner/property `None`, detail `grid-template-columns`를 확인해 계약을 바로잡았다. |
| 16 | 실패 전 partial frame이나 draw가 남았는데 성공 거부로 처리한다 | 거부 scene은 frame 0, positive draw 0이어야 한다. 비교기 4개 거부 fixture가 이 조건을 확인한다. |
| 17 | 겹치는 item은 frame이 같으므로 paint 순서 오류를 놓친다 | `overlap-source-order`는 geometry 외에 paint snapshot node 순서를 별도 확인한다. 두 플랫폼에서 고정 순서 `[1,2,3,4]`가 기록됐다. |
| 18 | 전체 화면 screenshot을 renderer pixel 동일성으로 설명한다 | 비교 JSON과 README에서 screenshot을 실행 확인용으로만 분류했다. 실제 표면 parity는 별도 PixelCopy/Metal 표본만 근거로 삼는다. |
| 19 | AVD·Simulator를 물리 기기나 GPU 성능 결과로 확대한다 | Android target은 qemu/API 37이며 ANGLE/Vulkan/SwiftShader다. iOS target은 고정 Simulator다. 양쪽 모두 실기기 성능 주장을 하지 않는다. |
| 20 | targeted test만 통과한 상태를 전체 회귀 통과로 표시한다 | 생성기 3/3, 비교기 4/4, CSS reference 327/327, FFI feature test 37 passed/2 ignored, 전체 workspace·release depth·Clippy·rustfmt·diff 검사 로그를 함께 보존한다. |

## 검토 후 남긴 한계

현재 연결된 Android는 API 37 AVD뿐이다. 전체 10-scene surface pixel matrix, Android 실기기 실행, Spinon WPT runner와 공개 CSS 지원은 이 변경에서 증명하지 않았다. Chrome WPT oracle, frame geometry, WGPU 제출, 표면 pixel 비교는 서로 다른 관문으로 유지한다.
