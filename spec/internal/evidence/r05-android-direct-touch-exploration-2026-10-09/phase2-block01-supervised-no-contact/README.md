# Phase-2 block 01 · 새 supervisor 무접촉 종료

## 판정

2026-10-09 13:05:31 KST에 fresh R05 process에서 60초 physical-touch capture를 시작했다. GO 뒤 raw `sec_touchscreen` contact와 앱 accepted input이 각각 0건이어서 `no_contact_timeout`으로 자동 종료됐다. 종료 시 `captureError=null`, host getevent/logcat client는 SIGINT로 종료, device `getevent` 잔존 0개였다. 수동 종료로 시간이 넘어갔던 이전 실행과 달리 자동 제한 뒤 약 0.73초 내 cleanup을 완료했다.

이는 capture 종료 경로를 확인한 무접촉 시도다. raw touch 0건을 latency 값으로 보지 않으며 phase-2 block으로 채점하지 않는다. 유효 입력은 계속 0/300이다. recorder 종료 뒤 별도로 보고된 다섯 탭은 이 실행에서 수집된 것으로 가정하지 않는다.

## 환경과 결과

| 항목 | 결과 |
|---|---|
| 기기 | Samsung SM-S731N · Android 16/API 36 · WGPU/Vulkan/Xclipse 940 |
| process | fresh PID, GO 시점 age 약 10.86초 · PID는 redacted manifest에서 숨김 |
| 화면 | R05 title과 파란 GPU 사각형 표시 · portrait / 60Hz |
| APK | 지정/설치 digest 모두 `b87a5cdd1d3589a5ffea66b784d9bf40f64843b69ad4aeb876086412f4db5669` |
| GO/stop | 13:05:31.155 → 13:06:31.881 KST |
| raw | 0 contact group · active/pending/protocol error 0 |
| 결과 | `no_contact_timeout` · `captureWindowValid=false` · `analysisStatus=not_analyzed_join_required` |

## 보존 자료

serial과 devices 원문을 제외했다. `capture-manifest.redacted.json`에서 serial·PID·child process identity를 지웠다. checksum은 폴더 안의 보존 파일 전체를 확인한다.
