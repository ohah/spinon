# 실행 후 독립 실패 관점 검토

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 연결된 기기가 실기기가 아니라 emulator | ADB 목록의 단일 대상은 Samsung SM-S731N USB 기기였다. |
| 2 | Android/API 버전 식별이 잘못됨 | release 16, API 36, SDK_INT_FULL 36.1을 기기에서 읽었다. |
| 3 | ABI나 화면 크기를 다른 기기 값으로 기록 | `arm64-v8a`, 1080×2340, density 450을 확인했다. |
| 4 | 다른 APK를 설치하고 기준 hash와 혼동 | 빌드 APK와 설치 APK를 각각 추출해 SHA-256 `1639…` 일치를 확인했다. |
| 5 | APK signing key 불일치가 설치를 바꾸거나 실패 | 테스트/보관 APK signer SHA-256이 같았고 `install -r`가 성공했다. |
| 6 | 원래 앱을 보존하지 못해 복구 불가 | 테스트 전 APK를 별도 보관했고, 종료 뒤 설치본 SHA-256이 원래 `2ab…`와 다시 같았다. |
| 7 | 데이터 초기화나 설정 변경이 테스트에 섞임 | `pm clear`·uninstall·display 설정 변경을 하지 않았다. timeout·밝기·자동 회전 값이 동일했다. |
| 8 | 화면이 잠겼거나 다른 앱에 입력이 전달 | 화면은 Awake·잠금 해제였고 테스트 중 Spinon Activity가 foreground였다. |
| 9 | 오래된 프로세스 로그를 새 실행 결과로 셈 | 각 구간에서 앱을 force-stop 후 cold launch했고 별도 PID를 기록했다. |
| 10 | debug 전용 probe가 실제 실행되지 않음 | 새 PID에서 `SPINON_R05_FENCE_WAIT_START api=36`이 확인됐다. |
| 11 | 화면이 뜬 것처럼 보이나 GPU backend 초기화 실패 | 앱 로그가 WGPU/Vulkan과 Samsung Xclipse 940을 표시했다. |
| 12 | 합성 음성 대조 좌표가 GPU target 바깥 | 화면 중앙 좌표 540×1170이 target 내부였고 앱 count가 1 증가했다. |
| 13 | ADB input을 손가락 입력으로 오분류 | 앱 action은 1건이었고 동시 raw direct touchscreen event는 0건이었다. |
| 14 | 반복 구간이 앞선 대조 PID와 섞임 | 별도 fresh PID 25050에서 sequence가 다시 1부터 시작했다. |
| 15 | 중복·누락 input을 하나의 성공 수치로 가림 | scored input sequence 1–10과 submit 10개가 각각 정확히 존재했다. |
| 16 | callback을 입력 순서만으로 잘못 join | request ID·input sequence·revision·generation을 각 record에서 대조했고 10개가 일치했다. |
| 17 | transaction listener 등록만으로 callback 성공 주장 | 실제 `outcome=callback_received` record가 각 request에 10개 있었다. |
| 18 | pending/invalid fence를 usable 완료로 오인 | 각 wait가 `signaled`, fence usable true, 오류 없음으로 끝났다. |
| 19 | worker queue 누수 또는 화면 색 변경 실패를 놓침 | 각 완료 queue가 pending/active/depth 0이고, 캡처 count 10 및 색상 토글이 일치했다. |
| 20 | 무입력 구간을 physical pass로 표현하거나 latency·복구를 과장 | 30초 direct 표본은 0건으로 보류 처리했다. VSync는 -1이며 지연 미계산; Chrome·원래 APK·화면 설정 복원을 확인했다. |

검토는 실행 자료에서 독립 실패 경계를 20개 대조한 기록이다. 실제 손가락 입력 표본이 없다는 제한은 남는다.
