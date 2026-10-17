# C13.1 전체 물리 표면 비교 DPR 정렬 계획 · 실패 관점 검토 20개 · 2026-10-18

최초 Android PixelCopy 비교는 AVD app scale `2.625`와 Chrome DPR `2.626666…`을 섞어 11,720 texel 차이를 냈다. Chrome DPR을 `2.625`로 맞춘 재비교도 11,281 texel 차이가 남았고, 경계의 부분 coverage 값이 서로 달랐다. 기존 계획의 Android `788×263`과 iOS `900×300`을 서로 다른 pixel lattice로 비교하는 조건을 수정해, 다음 실행 전에 같은 3× lattice로 정렬하는 계획을 독립 실패 관점 20개로 검토했다. 이 검토는 실행 성공을 보증하지 않는다.

| # | 새로운 실패 관점 | 계획 판정 |
|---:|---|---|
| 1 | Chrome에 요청한 DPR과 `devicePixelRatio` 보고값이 달라진 상태에서 캡처 | 보고값 3, viewport `300×100`, visual viewport scale 1을 모두 검사한다. 불일치는 run 무효. |
| 2 | Agent Browser 일반 screenshot API가 CSS 크기 `300×100`을 저장해 고DPI 기준으로 오인 | live Agent Browser tab에서 CDP `Page.captureScreenshot`을 호출하고 실제 PNG `900×300`을 검사한다. |
| 3 | CDP target detach 시 `Emulation.setDeviceMetricsOverride`가 초기화된 뒤 캡처 | DPR 설정과 capture를 같은 연결된 page session 안에서 수행한다. |
| 4 | CDP가 다른 탭 또는 이전 fixture URL에 연결 | capture 전에 URL, Chrome revision, fixture node 수를 검증한다. |
| 5 | 3× 물리 크기의 반올림 결과를 임의 scale/crop으로 대체 | CSS `300×100`에서 PNG `900×300` exact dimensions만 허용한다. |
| 6 | 분수 DPR의 경계 coverage 불일치를 tolerance로 감춰 renderer 차이를 통과 처리 | DPR 3 정렬 후에도 tolerance 없이 모든 RGBA byte exact를 요구한다. mismatch가 남으면 구현/범위 실패로 둔다. |
| 7 | Android density 변경이 AVD 물리 display size도 바꾸는 것으로 오해 | `wm size`는 읽기만 하고 수정하지 않는다. density override만 기록한다. |
| 8 | 기존 `wm density` override가 있는 AVD에서 끝에 `reset`해 사용자 값을 잃음 | 시작 시 물리·override density를 저장하고 기존 override 숫자를 복원한다. override가 없을 때만 reset한다. |
| 9 | 비교 실패·build 오류 뒤 임시 density 480 dpi가 남음 | density 설정 뒤에는 실패 경로를 포함해 cleanup을 호출하고 복구 보고값을 원본과 비교한다. |
| 10 | density를 복구하는 동안 fixture 앱이 configuration change로 다시 뜸 | cleanup에서 Spinon fixture process를 force-stop한 뒤 density를 복구한다. |
| 11 | 다른 connected AVD가 API 37 대상 serial 대신 선택됨 | `emulator-5554`와 API 37, ARM64, qemu를 모두 검사한다. |
| 12 | `localhost:5555` AVD의 density·앱 상태를 함께 바꿈 | 모든 `adb` 명령에 선택 serial `emulator-5554`를 고정한다. 다른 serial은 수정하지 않는다. |
| 13 | 480 dpi 설정에도 앱 surface가 900×300이 아니거나 renderer scale이 3이 아님 | PixelCopy begin 로그에서 크기와 양 축 scale을 검증하고 다르면 run을 실패시킨다. |
| 14 | 전체 화면 캡처를 WGPU surface PixelCopy로 잘못 비교 | 앱 screenshot은 증빙으로만 저장하고 comparator에는 PixelCopy PNG 한 장만 전달한다. |
| 15 | PixelCopy readback이 이전 surface generation 또는 app process 결과를 재사용 | run ID, generation, process ID를 PixelCopy begin/summary와 연결하고 매번 앱을 재시작한다. |
| 16 | 900×300 비교에서 일부 texel/channel이 생략 | `270,000` texel과 `1,080,000` channel 수 및 mismatch 0을 함께 고정한다. |
| 17 | fixture paint가 컨테이너 border 바깥으로 나온 부분이 crop 밖이라 누락 | 동일 `900×300` surface 전체를 비교하고 DPR3 외부 표본을 JSON으로 보존한다. |
| 18 | 이전 30,000 center 또는 52 표본 회귀가 full-surface gate 대체 | 둘을 독립 비교로 유지하고 각 결과 cardinality를 모두 요구한다. |
| 19 | 두 번째·세 번째 run이 같은 process 또는 같은 generation을 재사용 | 3회 각각 새 PID와 surface generation을 증명하고 density 설정·복구 로그를 run별로 둔다. |
| 20 | simulator/AVD의 pixel exact 결과를 일반 CSS, 실기기 또는 hardware GPU 지원으로 확대 | 결과는 고정 Chrome 155와 API 37 AVD/iOS 26.2 Simulator의 고정 fixture로 한정하고 C13.1/C13은 미완료로 유지한다. |

이 검토는 계획 변경의 환경·원본·복구 경계만 다룬다. Android DPR3 전체 surface 실행과 3회 반복은 별도 실행 증거가 있어야 통과로 판정한다.
