# C13.3 Android Control-wheel PR 변경 적대 검토 20개 · 2026-10-16

검토 대상은 이번 PR 전체 diff다: native wheel 계획 갱신, STATUS 근거 문구, Android AVD uinput helper/capture runner, 실행 산출물과 계획·구현 검토 기록. 앱 product runtime code는 변경하지 않았다.

| # | 실패 관점 | diff 대조 | 판정 |
|---:|---|---|---|
| 1 | C13.3을 전체 완료로 승격 | 계획·STATUS·README가 iOS native callback과 C13.3 전체 미완료를 유지하는지 확인했다. | 통과. 상위 완료 체크와 전체 61/197 수를 변경하지 않는다. |
| 2 | ADB 원인 설명이 모호함 | 기존 별도 `input keyevent`/`input mouse scroll` 시도가 modifier를 전달하지 않은 사실과 같은 virtual uinput device 사용을 구분했다. | 통과. 원인과 해결 경로가 문서에 연결됐다. |
| 3 | 좌우 Ctrl을 같은 raw bit로 기록 | runtime JSON과 로그의 left/right meta를 비교했다. | 통과. left 12288, right 20480, normalized modifier 2다. |
| 4 | 수직 wheel 부호를 제품 기대와 혼동 | C helper REL event와 Android callback raw axis를 대조했다. | 통과. 이 거부 시험은 수집 raw `vscroll=1`을 기록하며 normalized CSS delta를 주장하지 않는다. |
| 5 | 잘못된 hit point로 Control 거부 통과 | logs의 SurfaceView origin, CSS target, physical point와 callback source/reason을 비교했다. | 통과. 네 run 모두 target `(132,112)` CSS px의 MouseWheel callback이다. |
| 6 | reject 후 숨은 queue work | native enqueue/process marker와 draw report를 확인했다. | 통과. 4/4에서 enqueue/process 0건이다. |
| 7 | reject 후 상태 변형 | 모든 case의 latest draw revision과 root/outer/inner offsets를 비교했다. | 통과. revision 1과 `[0,0;0,0;86,106]`이 유지된다. |
| 8 | 화면 비교 범위 누락 | SurfaceView bounds와 PNG dimensions, changed pixel count를 비교했다. | 통과. case별 661,500 pixel 전체의 changed count는 0이다. |
| 9 | PNG 이미지가 전후 불일치 | before/after 파일 SHA-256을 비교했다. | 통과. 대표 screenshot 두 파일은 byte-identical이다. |
| 10 | 항상 reject하는 구현의 false pass | modifier 없는 ordinary wheel queue/process/draw를 확인했다. | 통과. modifier 0, target 7, revision 2, outer X 64다. |
| 11 | 가상 device 잔존 | capture 로그의 destroy 결과, InputReader dump와 원격 파일 정리를 확인했다. | 통과. 각 run device/key/meta가 정리되고 runner는 helper binary도 제거한다. |
| 12 | 실기기나 alias 조작 | emulator serial 검사와 qemu/API/ABI/page-size guard를 확인했다. `localhost:5555`는 ADB 호출 전에 거부된다. | 통과. 명시된 API 37 ARM64 AVD만 허용한다. |
| 13 | 잘못된 NDK 사용 | runner가 저장소 고정 NDK 버전과 세그먼트별 ELF align을 검사한다. | 통과. NDK 27.1.12297006, 모든 LOAD align 0x4000이다. |
| 14 | build feature 혼선 | fresh worktree에서 사용한 Android build flags와 installed APK hash를 JSON과 비교했다. | 통과. opt-in C04 runtime GPU 및 C13.3 fixture flags가 기록됐다. |
| 15 | main source와 실행 산출물 혼동 | HEAD, APK, helper source/binary와 capture script hash를 각각 기록한다. | 통과. 산출물과 재현 도구 identity를 서로 다른 필드로 보존한다. |
| 16 | 앱 코드 변경이 기록되지 않음 | PR diff에서 `platforms/android/app`, Rust runtime/FFI 변경 여부를 확인했다. | 통과. product callback guard는 기존 구현이며 이 PR은 harness/evidence만 추가한다. |
| 17 | 과거 근거와 새 근거 모순 | 기존 방향 행렬 설명을 당시 ADB 실패로 한정하고 후속 Control runtime 결과를 연결했다. | 통과. 이전 실행의 raw history는 유지하고 현재 follow-up과 구분한다. |
| 18 | 재현 절차가 빌드 조건을 생략 | README의 pinned V8/NDK 준비, opt-in APK build, emulator-only runner를 검토했다. | 통과. 명령과 필수 build flags가 문서에 있다. |
| 19 | 실행 결과의 플랫폼 범위 확대 | Android AVD, external trackpad, iOS callback, public DOM event, scanout 제한을 확인했다. | 통과. 미검증 경계가 README와 STATUS에 남는다. |
| 20 | PR 화면 링크 오류 | merged main 경로로 이어질 PNG를 본문에 넣고 Agent Browser에서 렌더링을 확인할 절차를 대조했다. | 브라우저 확인은 PR 생성 후 실행 예정이며 통과 판정은 링크 확인 뒤 갱신한다. |

현재 결과: 코드·실행·상태 문서에서 미해결 의미 모순을 찾지 못했다. 20번의 화면 렌더링은 PR 생성 후 Agent Browser 검증으로 닫는다. 계획 review와 구현 review는 별도 기록이다.
