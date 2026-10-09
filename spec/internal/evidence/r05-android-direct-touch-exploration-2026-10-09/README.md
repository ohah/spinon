# R05 · Android 실기기 직접 입력 탐색 근거

## 판정

2026-10-09 12:18 KST의 Android 실기기 탐색 실행에서 GPU 사각형을 손가락으로 탭한 다섯 입력이 raw `sec_touchscreen` release부터 앱의 `ACTION_UP`, WGPU 제출, 같은 surface generation의 transaction callback, 사용 가능한 비동기 fence까지 각각 연결됐다. 화면의 활성화 횟수도 5였다. 직접 입력 경로가 동작한다는 실행 근거지만, 기존 앱 프로세스에서 얻은 n=5 탐색 표본이므로 R05.3의 사전 고정 10 × 30 표본에는 넣지 않았다.

별도의 새 프로세스 무접촉 시도는 R05 화면과 실험 extra를 확인하고 수집을 시작했으나 60초 안에 raw 접촉과 앱 입력이 모두 없어 무효였다. 해당 시점의 표본 수는 0/300이었다. 사용자가 뒤에 보고한 추가 다섯 탭은 그 raw recorder가 이미 닫힌 뒤의 동작이므로 어떤 표본에도 포함하지 않았다.
후속 bounded collector는 가짜 ADB에서 정상 자동 종료와 무접촉 timeout 정리를 통과했고, 실제 Android 16 기기에서도 2초 no-contact smoke의 host/device collector 정리를 확인했다. 이 smoke는 직접 입력 양성 sample이나 phase-2 block이 아니다. 자세한 결과는 [실기기 collector smoke](collector-device-smoke/README.md)에 있다.

## 환경과 출처

| 항목 | 값 |
|---|---|
| 기기 | Samsung SM-S731N, Android 16 / API 36 / `SDK_INT_FULL=36.1` |
| 화면 | 1080 × 2340, portrait, 실제 주사율 60Hz |
| 그래픽 | WGPU / Vulkan / Xclipse 940 |
| 앱 | `dev.spinon.bootstrap`, R05 실험 extra `spinon_r05_async_fence_wait=true` |
| 실험 APK SHA-256 | `b87a5cdd1d3589a5ffea66b784d9bf40f64843b69ad4aeb876086412f4db5669` |
| 실험 APK source commit | `364bbc3105300da4a45ea8d3badd12c814cd7942` |
| 탐색 process | PID 28243, 시작 시 이미 실행된 시간 56분 24초 |
| raw 입력 기기 | `sec_touchscreen`, `INPUT_PROP_DIRECT`; `dumpsys input`의 active viewport·Rotation0 변환 확인 |

기기 일련번호는 이 근거에 기록하지 않는다. 캡처 로그의 event 시각은 Android monotonic 기반이며 앱의 `ACTION_UP.event_time_ns`와 raw release 시각이 다섯 건 모두 정확히 일치했다. 앱 로그의 `input_source=unknown`은 raw device 이름을 앱 자체가 판별하지 못했음을 뜻한다. 물리 터치 판정은 동시 수집한 `INPUT_PROP_DIRECT` raw stream과 timestamp join, 사용자의 손가락 입력 확인을 함께 근거로 한다.

## 관측 결과

| 구간 | 관측 | 판정 |
|---|---:|---|
| raw touchscreen contact group | 8 | 원본에 모두 보존 |
| raw release와 앱 `ACTION_UP` 정확 일치 | 5/5 | 입력 sequence 1–5 |
| WGPU submit | 5/5 | 각 입력 sequence에 연결 |
| 현재 generation callback 및 usable async fence | 5/5 | generation 3, fence wait 오류 없음 |
| 연결되지 않은 추가 raw group | 3 | 성공으로 승격하거나 버리지 않음 |
| 화면 활성화 횟수 | 5 | 캡처 후 GPU 사각형이 주황색 |
| `target_vsync_id` | -1 | frame ID·실제 scanout 연결 없음 |

앱 로그의 후보 `signal_time_ns - event_time_ns` 값은 27.890714, 44.724253, 44.734261, 44.886565, 36.387073ms다. n=5에서 범위는 **27.890714–44.886565ms**, 중앙값은 **44.724253ms**다. 이는 event에서 transaction fence signal까지의 탐색 후보 구간이다. 화면 광자 시각이나 제품 input-to-photon 지연이 아니며 p95도 계산하지 않는다. Android log의 nanosecond 표기는 clock API의 표현 단위이고 측정 정확도가 나노초임을 뜻하지 않는다.

## 앞선 phase-2 무접촉 시도

- 새 process PID 7244와 정확한 APK hash, R05 실험 extra, WGPU/Vulkan 화면, portrait / 60Hz를 확인했다.
- `GO`는 12:27:06 KST였고 raw `sec_touchscreen` 및 해당 PID의 accepted 앱 입력은 60초 동안 0건이었다.
- 정해둔 자동 제한이 실행되지 않아 수동으로 12:29:13 KST에 캡처를 끝냈다. 실제 창은 127초였고 계획의 60초 cutoff보다 67초 늦었다. 따라서 절차 편차로 기록하며 block은 유효 표본이 아니다.
- 종료 뒤 host collector와 기기 `getevent`가 남지 않은 것은 별도 확인했다. 종료가 늦었던 사실을 정상적인 제한 시간 동작으로 간주하지 않는다.
- 이 실패를 latency 값·성공 입력·추가 접촉으로 보충하지 않았다.

## 새 supervisor · phase-2 no-contact 재시도

2026-10-09 13:05 KST 새 R05 process에서 60초 direct-touch capture를 시작했다. GO 뒤 raw `sec_touchscreen` contact 0건, 앱 accepted input 0건으로 `no_contact_timeout` 자동 종료됐다. 두 host ADB client는 SIGINT 뒤 종료했고 device `getevent` 잔존 0개, `captureError=null`이었다. 이는 supervisor timeout·teardown 경로를 확인한 것이며 유효 표본은 아니다.

[`phase2-block01-supervised-no-contact/`](phase2-block01-supervised-no-contact/)에 redacted manifest, raw/app 기록, 시작·종료 화면과 digest를 보존했다.

## phase-2 block 01 · 직접 입력 수집

2026-10-09 13:09 KST 새 process에서 bounded collector를 시작했다. `sec_touchscreen` raw 32개 중 첫 30개를 scoring set으로 고정하고 drain 중 추가된 두 개는 제외했다. 첫 30개에는 raw release→app `ACTION_UP`→WGPU submit→현재 surface generation callback→같은 request의 usable fence가 28개 정확히 연결됐다. 점수 집합의 raw 두 건은 app `ACTION_UP`과 exact join되지 않았다. 앱 로그에는 `cancelled`와 `outside_target` 제외 이벤트가 각각 하나 있지만 event timestamp가 없어 이 둘을 해당 raw contact에 귀속할 수 없다. drain에서 수집된 추가 raw 두 건은 scoring에서 제외됐고, 이들과 exact timestamp가 일치하는 accepted `ACTION_UP` 두 건도 분석기에서 별도 집계했다. raw protocol error·active/pending contact·collector 잔존은 없었다.

28개 유효 transaction-fence candidate interval의 block 범위는 28.230–61.515ms다. p95를 계산하지 않았고, `target_vsync_id=-1`이므로 제품 입력 지연·VSync·scanout·광자 시각을 뜻하지 않는다. 이전 exploratory n=5와 별도 no-contact 시도는 phase-2 표본에 넣지 않는다. block 01 시점 누적은 **28/300**이었다.

[`phase2-block01-direct-touch/`](phase2-block01-direct-touch/)에 serial과 좌표를 제거한 raw/contact summary, 앱 R05 로그, APK provenance, 기기 조건, cropped screenshot과 checksum을 보존했다. 저장소 사본에서도 분석기를 다시 실행해 **28/30** exact join을 재현했다. 과거 탐색 로그에서도 raw X/Y 값을 제거했다.

## phase-2 block 02 · 사전 입력 없음

2026-10-09 13:48 KST 새 process에서 block 02 수집을 시작했지만 `GO` 뒤 60초 동안 raw `sec_touchscreen` 접촉이 없어 collector가 자동으로 `prestart_no_contact` 종료했다. raw contact와 앱 입력 marker는 모두 0건이며 host client와 device getevent가 정리됐다. 창이 닫힌 뒤 도착한 탭 완료 응답은 이 capture에 연결하지 않았다. 이 시도는 block 또는 표본이 아니다. 상세 자료는 [무접촉 시도 근거](phase2-block02-prestart/README.md)에 있다.

## phase-2 block 02 · 직접 입력 수집

2026-10-09 13:56 KST 별도 fresh process에서 새 capture를 시작했다. raw direct touchscreen 접촉 30개 중 사전 고정한 첫 30개 전부가 앱 `ACTION_UP`, WGPU submit, 현재 surface generation callback, 같은 request의 usable async fence 및 clock bracket까지 exact join됐다. raw overlap·protocol error·앱 제외 입력은 0건이다. 30개 bracket 후보 interval의 block envelope는 **28.070127–45.156253ms**다. 이는 transaction-fence 후보이며 `target_vsync_id=-1`로 제품 input-to-photon·VSync·scanout 수치가 아니다. 화면은 GPU 도형 활성화 30회를 표시한다. [실행·분석 근거](phase2-block02-direct-touch/README.md).

이 block 종료 뒤 phase-2 누적은 **58/300**, 유효 block **2/12**다. 13:48 KST의 무접촉 재시도는 별도 무효 자료이며 후보 수에 포함하지 않는다. R05.3은 미완료이고 10개 유효 block이 남는다.

## 증거 파일

- [`exploratory-5tap/summary.json`](exploratory-5tap/summary.json): 접촉 수, exact join, 후보값 요약
- [`exploratory-5tap/raw-touch.log`](exploratory-5tap/raw-touch.log): 원본 raw input 이벤트
- [`exploratory-5tap/app.log`](exploratory-5tap/app.log): PID별 입력·submit·callback·fence 로그
- [`exploratory-5tap/screen-after.png`](exploratory-5tap/screen-after.png): 활성화 횟수 5와 주황색 GPU 사각형
- [`phase2-block01-prestart/`](phase2-block01-prestart/): 무접촉으로 무효 처리된 capture의 raw/app 요약·종료 사유·화면·시계 기록. 다른 앱 메타데이터가 포함될 수 있는 전체 Android system dump는 공개 저장소 사본에서 제외
- [`phase2-block01-direct-touch/README.md`](phase2-block01-direct-touch/README.md): fresh process 직접 입력 28/30 join, bracket candidate 범위, 공개 저장소용 redacted evidence
- [`phase2-block02-prestart/README.md`](phase2-block02-prestart/README.md): block 02의 무접촉 사전 입력 시도. 표본에는 포함하지 않음
- [`phase2-block02-direct-touch/README.md`](phase2-block02-direct-touch/README.md): block 02의 새 capture, 30/30 exact join, 후보 interval과 redacted evidence
- [수집기 구현 검토](../r05-android-capture-supervisor-review-2026-10-09.md): collector의 서로 다른 실패 경로와 수정
- [PR 통합 검토](../r05-android-direct-touch-pr-review-2026-10-09.md): 계획·근거·GitHub 첨부·표본 수의 일치 확인
- [`SHA256SUMS`](SHA256SUMS): 보존 파일 checksum

공개 미리보기에는 집계값과 화면 이미지까지만 공유하며 raw 좌표 로그와 기기 일련번호는 포함하지 않는다.
