# C13.3 Android AVD multi-touch PR 변경 적대 검토 30개

검토 대상은 C13.3 Android 후속 plan/STATUS/public roadmap와 실행 evidence다. 변경은 문서·재현 자료이며 제품 runtime 코드는 바꾸지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 제품 runtime 변경이 문서-only PR에 섞이는가 | staged diff에서 Rust·Kotlin·Swift·build source 변경이 없는지 확인한다. |
| 2 | 공식 구현 상태가 갱신되지 않는가 | `spec/STATUS.md`의 C13.3 항목에 새 AVD 근거와 남은 gate를 기록한다. |
| 3 | 정적 공개 로드맵이 오래된 Android 상태를 유지하는가 | `spec/public/roadmap.html`에서 Android 실기기만 남은 범위로 표시한다. |
| 4 | 작업 계획이 새 증거를 연결하지 않는가 | plan 실행 후속 기록에서 README, script, pixel 비교와 구현 검토를 연결한다. |
| 5 | 전체 C13.3 완료로 잘못 표시되는가 | 완료 checkbox/API 지원 선언은 건드리지 않고 제품 미완료 범위를 유지한다. |
| 6 | AVD 테스트가 실기기 통과로 표현되는가 | 기기명, API 37, ARM64 AVD 및 실기기 한계를 문서와 로그에 남긴다. |
| 7 | evdev를 우회해 앱에 Event를 직접 dispatch한 것처럼 읽히는가 | kernel input과 InputReader의 접촉 상태를 수집했고 synthetic dispatch를 사용하지 않았다. |
| 8 | second-touch가 pan 이후에 들어왔는지 증명되지 않는가 | 첫 pan/revision trace가 `ACTION_POINTER_DOWN`보다 먼저 기록됐다. |
| 9 | 앱 action 5만으로 contact 두 개를 단정하는가 | InputReader `pointerCount=2`, pointer id 0/1 증거를 함께 보존한다. |
| 10 | pointer 수가 2→1→0으로 정리되는가 | 두-contact, one-contact, all-up 상태 파일을 각각 연결한다. |
| 11 | second-pointer cancel reason이 달라지는가 | 앱 trace의 `SecondPointer` cancellation과 scenario 요약을 맞춘다. |
| 12 | cancel 뒤 남은 contact movement가 한 건이라도 수락되는가 | 세 callback 각각 `accepted=false`이고 전후 offset/revision이 유지된다. |
| 13 | second contact up만으로 sequence가 회복된다고 오해하는가 | 첫 contact가 남은 상태의 move도 거부됐다고 기록한다. |
| 14 | all-up 안정 결과를 한 장의 screenshot만으로 추정하는가 | InputReader all-up state와 cancel/all-up crop 비교를 분리해 보존한다. |
| 15 | fresh gesture가 이전 pointer/session을 재사용하는가 | all-up 이후 새 down/pan과 revision 증가를 app log에서 대조한다. |
| 16 | 화면 비교의 crop 좌표가 잘못됐는가 | 1080×2400 screenshot에 적용한 `(15,1026)-(1065,1656)`을 비교 로그에 고정한다. |
| 17 | cancel 후 0 pixel 차이를 전체 화면 무변화로 확대하는가 | 결과를 SurfaceView crop 한정으로 설명한다. |
| 18 | screenshot pixel 변화가 입력과 무관한 노이즈인가 | first-pan/cancel/all-up/recovery 순서와 pixel counts를 연결한다. |
| 19 | compositor 또는 display scanout까지 검사했다고 오해하는가 | PR 범위 설명에서 post-compositor/scanout을 미검증으로 남긴다. |
| 20 | UI evidence에 맞지 않는 이미지 파일을 선택하는가 | cancel 유지 화면과 fresh recovery 화면을 각각 evidence README 및 PR 본문에 사용한다. |
| 21 | 이미지가 존재하지만 0 byte/잘못된 포맷인가 | 각 캡처의 PNG 포맷·1080×2400 크기와 SHA-256을 확인한다. |
| 22 | ADB input `motionevent`로 multi-touch 된 것처럼 기록하는가 | 명령이 DOWN/UP/MOVE/CANCEL만 지원한다는 진단을 보존한다. |
| 23 | non-root 권한 실패가 재현 원인으로 남지 않는가 | AVC denial log에서 `shell`→`input_device` write denial을 연결한다. |
| 24 | event11 같은 비활성 node에서 우연히 입력이 들어가는가 | preflight가 event11 virtual display mapping을 거부하고 event1 display 0을 쓴다. |
| 25 | AVD root 조치가 실기기에도 적용될 수 있는가 | harness는 정확한 emulator serial/API/ABI/root 조건을 검사하고 실행 후 AVD `unroot`를 확인했다. |
| 26 | 경로 인자 삽입으로 shell 명령이 실행되는가 | `/dev/input/eventN` allowlist와 malicious path negative control을 포함한다. |
| 27 | 중간 실패가 접촉을 누른 채 남길 수 있는가 | trap은 두 slot을 release하고 최종 pointer count 0을 확인한다. |
| 28 | 앱 준비 전 캡처가 성공으로 기록되는가 | `SPINON_C133_READY input=MotionEvent`가 없는 상태는 사전 실패로 처리한다. |
| 29 | build 환경 실패와 통과한 opt-in build를 혼합하는가 | Java 미설정 첫 시도와 mise Java 17 재실행 log를 별도 보존한다. |
| 30 | 증거 파일/상태 링크가 병합 후에도 추적되는가 | README·STATUS·plan·roadmap의 상대/정적 링크를 검사하고 PR 및 Tailnet 제공 hash를 후속 절차에서 채운다. |

검토 당시 알려진 ADB 실패 원인과 해결은 `android-adb-diagnosis.md`, 재현 조건은 `android-evdev-multitouch.sh`, 실행 결과는 원본 log/InputReader dump/screenshots에 대조했다. PR URL 및 Tailnet served hash는 생성·병합 뒤 확정한다.
