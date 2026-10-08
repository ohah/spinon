# 실행 증거 적대 검토 · debug 기본 경로와 R08 probe-off

2026-10-09 Android 16.1 실기기의 process·intent·log·화면·복구 및 release build preflight를 독립 실패 관점 20개로 대조했다. 이 결과는 두 debug control의 부분 통과이며 release 또는 R05.3 완료 선언이 아니다.

| # | 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | emulator를 실기기로 오인 | [serial을 제거한 연결 확인](connection.txt)에 물리 USB 기기 한 대와 emulator/QEMU process 0개가 기록됐다. |
| 2 | stale intent extra가 default process에 잔류 | force-stop 후 MainActivity를 extra 없이 시작하고 새 PID 28058을 확인했다. |
| 3 | default Activity launch를 bootstrap 성공으로 오인 | PID log에서 `SPINON_BOOTSTRAP_EXECUTION`과 `SPINON_BOOTSTRAP_RESULT`를 모두 확인했다. |
| 4 | bootstrap fixture가 visual UI까지 출력한다고 가정 | 결과에 render report 표시 경로가 없고 화면은 흰색이었다. GPU visual pass로 집계하지 않았다. |
| 5 | 빈 화면에서 보낸 ADB tap을 GPU 입력으로 셈 | R08 surface가 없어 이 tap은 비점수로 제외했다. |
| 6 | default 실행이 R05 worker를 암묵적으로 켬 | process log에서 `SPINON_R05_` marker 0건을 확인했다. |
| 7 | R08 control에 이전 R05 extra가 잔류 | 두 번째 새 process의 명령은 `spinon_r08=true`만 전달했다. |
| 8 | R08 화면은 보이나 WGPU backend는 준비되지 않음 | log에 Vulkan·IntegratedGpu·Samsung Xclipse 940·surface size marker가 있었다. |
| 9 | surface 준비 전 tap을 보내 R08 입력 실패로 오판 | surface-ready log와 resumed Activity 확인 뒤 tap했다. |
| 10 | tap coordinate가 GPU 도형 밖 | 540×1170은 1080×2340 화면의 도형 중앙에 있고 전후 캡처가 색 변경을 보인다. |
| 11 | 자동 입력을 finger touch로 분류 | 입력은 `adb shell input tap`이며 직접 raw touchscreen provenance로 주장하지 않았다. |
| 12 | 화면 색만 바뀌고 앱 touch handler는 실행되지 않음 | PID log에서 `SPINON_R08_TOUCH count=1`을 확인했다. |
| 13 | R08 probe-off에서 R05 worker가 켜짐 | 해당 process log에서 `SPINON_R05_` marker 0건을 확인했다. |
| 14 | 빈 marker 검색 결과가 잘못된 PID의 로그에서 나옴 | 각 새 process를 분리해 PID-filtered log를 저장했다. |
| 15 | 설치본이 다른 hash라 baseline 비교가 무효 | 기기 설치 APK SHA-256을 직전 공식 artifact와 대조해 동일함을 확인했다. |
| 16 | release build에 SDK만 설정하고 release runtime을 주장 | 첫 SDK 설정 오류 뒤 재시도도 pinned V8 source 부재로 `prepareSpinonBootstrap` 단계에서 실패했다. APK는 만들어지지 않았다. |
| 17 | 부재한 V8 source를 임의 checkout해 workspace를 크게 변경 | checkout/download를 하지 않았고 기존 작업물을 덮어쓰지 않았다. |
| 18 | 앱 crash/ANR이 없다고 시스템 전체를 과장 | process-filtered log에는 fatal/ANR marker가 없고 app은 foreground에 유지됐다. OS 전체 crash 상태를 주장하지 않는다. |
| 19 | 테스트 뒤 폰 상태를 바꾸고 두고 옴 | 밝기·timeout·stay-awake·60Hz mode는 유지됐고 Chrome foreground로 복귀했다. |
| 20 | partial debug control을 release 격리·R05.3 완료로 승격 | release 두 시나리오, direct physical touch, VSync, scanout, 성능은 모두 미완료로 남겼다. |

### 검토 결과

debug 기본 bootstrap은 JS fixture 실행 및 R05 marker 부재를 확인했다. R08 probe-off는 synthetic tap 1회에 WGPU 도형 색상 변경과 R08 touch count를 확인했고 R05 marker는 없었다. Release variant는 source precondition을 충족하지 못해 전혀 실행되지 않았다. 따라서 release 격리 계획의 일부 gate만 완료됐다.
