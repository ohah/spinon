# C13.3 Android API 29 follow-up · PR 변경 적대 검토 30개

검토 범위는 이 PR에서 새로 추가한 API 29 AVD 캡처 runner, uinput helper, evidence, 계획과 상태 기록이다. 제품 Android/iOS native adapter는 변경하지 않았다. 각 항목은 서로 다른 회귀·오판 경로를 검토했다.

| # | 적대적 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 검증 도구 변경이 제품 입력 동작을 몰래 바꾼다. | 통과. 변경 파일은 캡처 도구, test-only C helper, 검증 evidence와 계획·상태 문서다. 기존 제품 adapter 소스는 변경하지 않았다. |
| 2 | 사용자가 실기기 serial을 넘겨 실수로 입력을 보낸다. | 통과. runner는 `emulator-N` 형식, `ro.kernel.qemu=1`과 지정 AVD 이름을 모두 요구한다. ADB 호출마다 해당 serial을 명시하고 실행은 `emulator-5556`으로만 했다. |
| 3 | 다른 AVD/API/ABI/page size의 결과가 API 29 근거로 저장된다. | 통과. API 29, Android 10, ARM64, 4 KB page size, `spinon_r05_api29_fallback`을 실행 전에 각각 검사하고 JSON에 기록한다. |
| 4 | 해상도나 density가 달라 입력점과 crop이 바뀌어도 같은 oracle을 쓴다. | 통과. 1080×1920과 density 420을 검사한다. 실제 SurfaceView bounds로 scale과 입력 screen point를 구하고 callback origin과 bounds 일치도 확인한다. |
| 5 | 다른/빈 APK 또는 일반 앱 실행을 opt-in 결과로 오인한다. | 통과. APK 빈 파일을 거부하고 SHA-256을 저장한다. fresh launch가 `auto-boundary`·기대 initial offsets·revision 1·fixture readiness를 내지 않으면 중단한다. |
| 6 | 호스트에 설치된 임의 compiler가 API 29용 helper를 만든다. | 통과. repository pin의 NDK 버전을 선택하고 API 29 ARM64 clang만 사용한다. `-Wall -Wextra -Werror`와 4 KB max-page-size link option을 적용하며 compiler 경로·helper hash를 저장한다. |
| 7 | 잘못된 좌표·axis·modifier argument가 helper에서 장치 입력으로 변환된다. | 통과. 정수와 허용 범위를 검사하고 H/V 둘 다 0인 입력을 거부한다. Control 값은 `none/left/right`만 허용한다. |
| 8 | helper가 실제 Android 입력 경로 대신 앱 queue에 직접 값을 넣는다. | 통과. Linux uinput에서 relative X/Y와 H/V 축을 생성한다. evidence는 OS `ACTION_SCROLL` callback 및 event source/tool을 요구한다. |
| 9 | diagonal이 두 callback으로 쪼개지거나 한 축이 빠진 채 성공 처리된다. | 통과. helper는 양쪽 wheel 축을 같은 `SYN_REPORT`로 보낸다. runner는 callback과 queue commit이 각기 정확히 하나이고 두 delta가 모두 oracle에 맞는지 확인한다. |
| 10 | 포인터가 화면 밖에 남거나 점이 stale surface 기준으로 계산된다. | 통과. helper가 먼저 포인터를 화면 경계로 옮긴 다음 목표 screen point로 이동한다. ready와 wheel callback의 surface origin을 대조하고 CSS point 오차를 screen pixel 반올림 한계로 제한한다. |
| 11 | Control key가 wheel과 같은 입력 source에 결합되지 않아 rejection을 잘못 검증한다. | 통과. helper 하나에서 left/right Control down, H/V wheel, key-up을 순서대로 보낸다. 네 조합 모두 callback의 Control modifier와 reject reason을 검사한다. |
| 12 | helper 실패 뒤 uinput 장치가 다음 case에 남아 입력을 오염시킨다. | 통과. normal/error/signal 종료에서 `UI_DEV_DESTROY`와 fd close를 수행한다. 각 입력 직후 helper lifecycle 출력과 `getevent -lp` 장치 부재를 확인했다. |
| 13 | 과거 앱 로그가 새 실행 callback으로 집계된다. | 통과. 각 case를 fresh force-stop/start하고 모든 logcat buffer를 비운다. 현재 PID 기록만 모아 callback, process, draw 수를 센다. |
| 14 | 앱 시작 race가 초기 revision 부재를 wheel 성공 또는 실패로 오판한다. | 통과. `pidof`가 준비될 때까지 재시도하고 READY, eval-ready, ready bounds, initial draw revision을 모두 기다린다. scenario와 initial offsets도 고정한다. |
| 15 | 기기 보고 값만 서로 일치하고 Android 정규화 공식은 어긋난다. | 통과. SurfaceView 실제 scale과 callback factor를 읽고 H/V 공식 및 별도 고정 CSS 64 px oracle을 둘 다 비교한다. scale/factor는 양수여야 한다. |
| 16 | 동일 입력이 중복 적용되거나 오래된 target으로 전달된다. | 통과. native callback 하나, process 하나, sequence 1, admitted/Queued, target 7, final revision 2를 각각 검사한다. |
| 17 | 내부 scroll owner가 아닌 root/outer/inner에 remainder가 잘못 전달된다. | 통과. 방향 8개마다 root·outer·inner의 각 X/Y offset을 precomparison oracle과 exact 비교한다. |
| 18 | queue는 성공했으나 화면 draw가 완료되지 않은 상태를 성공으로 기록한다. | 통과. process 뒤 revision 2의 `status=0 presented` draw와 세 owner offset을 확인한다. Control 거부에서는 revision 1과 시작 offsets를 강제한다. |
| 19 | SurfaceView 이동·screenshot crop 오류가 scroll boundary 통과로 숨는다. | 통과. before/after PNG 크기와 SurfaceView bounds를 검사하고 실제 scale로 기대 surface 크기를 확인한다. 화면 밖 crop을 거부하며 root scrollport 외부 변경 0, 내부 변경 양수와 Control no-op 전체 0을 검사했다. |
| 20 | evidence가 코드·APK·renderer 차이를 숨기거나 지원 범위를 과장한다. | 통과. source fingerprint, 개별 소스·helper·APK SHA-256, AVD 식별자와 GL/ANGLE/SwiftShader 정보를 보존한다. 기록은 API 29 software AVD에 한정하고 실기기·hardware GPU·iOS·전체 C13.3은 미완료로 남긴다. |
| 21 | serial 인자를 셸 문자열로 합쳐 악성/공백 입력이 다른 명령을 실행한다. | 통과. `execFileSync`에 executable과 argument 배열을 분리해 전달한다. serial도 emulator 패턴·qemu·AVD identity 검사를 통과해야 한다. |
| 22 | API 29 ADB의 `mouse scroll` 미지원 오류를 앱 구현 실패로 기록하거나 우회 주입을 native 입력으로 과장한다. | 통과. API 29에서는 helper의 uinput relative axes만 쓴다. 앱 callback에서 `ACTION_SCROLL`, `SOURCE_MOUSE`, mouse pointer tool을 받아야 하며 보고서는 가상 장치 경로라고 명시한다. |
| 23 | build cache에 남은 APK의 opt-in fixture 존재를 새 빌드 성공으로 착각한다. | 통과. 이 캡처기는 입력된 APK를 해시하고 실행 중 fixture READY·eval-ready·draw/callback/process 로그를 요구한다. 별도 빌드 성공은 PR의 독립 빌드 결과로 구분한다. |
| 24 | evidence 경로에 symlink가 끼어 저장소 밖 파일을 덮어쓴다. | 통과. evidence root와 각 하위 디렉터리를 `lstat`·`realpath`로 확인하고 symlink 및 저장소 밖 경로를 거부한다. |
| 25 | 같은 이름의 재실행이 성공 결과 일부를 덮어써 이전 로그·신규 screenshot·옛 JSON이 혼합된다. | 통과. 모든 산출물을 새 `.capture-*` staging directory에 모은 다음 matrix 전체가 성공할 때만 unique run directory로 rename한다. 중간 실패면 staging만 제거되고 기존 완료 run은 유지된다. |
| 26 | run directory 게시 직후 index README 갱신이 실패해 성공 근거를 잃거나 불완전한 결과를 가리킨다. | 통과. 완료된 run 폴더를 먼저 게시하고 index는 별도 임시 파일 작성 후 rename한다. index 갱신 전 실패 시 이전 index가 계속 이전 완료 run을 가리킨다. |
| 27 | 새 완료 run이 예전 evidence까지 제거해 재검증 이력을 잃는다. | 통과. run 이름은 캡처 시각 기반으로 고유하며 이전 run directory를 수정·삭제하지 않는다. 안정 경로 README만 최신 완료 run으로 전환한다. |
| 28 | surface crop 경계에서 작은 색상 차이를 허용해 spill 또는 no-op 변화를 감춘다. | 통과. 비교는 aligned RGBA pixel의 channel exact equality를 쓴다. crop bounds와 dimensions를 먼저 검사하고 scroll/거부마다 별도 수용 조건을 적용한다. |
| 29 | runner가 테스트 종료 뒤 임의 emulator·tmux/browser/실기기를 종료한다. | 통과. 캡처기 finally는 입력에 사용한 serial의 app과 remote helper만 정리하고 AVD 자체는 종료하지 않는다. 외부 종료는 사용자가 지정한 `emulator-5556`만 대상으로 한다. |
| 30 | API29 단일 AVD 합격으로 공개 C13.3 또는 iOS 지원을 완료 처리한다. | 통과. 계획·STATUS는 추가 근거만 적고 official checkbox를 바꾸지 않는다. iOS callback/sign, 물리 입력장치, hardware renderer와 전체 C13.3 미완료를 명시한다. |

## 남긴 경계

runner는 자신이 시작한 AVD를 종료하지 않는다. 여러 case 사이 AVD를 유지해 입력 matrix를 수행한 뒤, 세션 정리에서 지정한 `emulator-5556`만 별도로 종료한다. API 29 virtual uinput 결과는 물리 장치나 외장 mouse/trackpad 증거가 아니다. iOS UIKit callback/sign 및 C13.3 전체 완료 판정은 이 diff에서 충족하지 않는다.
