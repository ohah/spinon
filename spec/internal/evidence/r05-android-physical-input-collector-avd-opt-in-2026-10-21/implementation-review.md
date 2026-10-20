# 수집기 구현 후 검토

| # | 실패 경로 | 확인한 방어 또는 근거 |
|---:|---|---|
| 1 | 새 옵션이 기본 동작으로 적용됨 | 기본값은 `false`; 단일 기기 외 연결은 계속 실패한다. |
| 2 | 옵션 이름의 부분 일치가 opt-in을 켬 | parser는 `--allow-extra-emulators` 전체 토큰만 인식하고 알 수 없는 토큰은 거부한다. |
| 3 | boolean 옵션 뒤의 `true`가 실수로 무시됨 | 값 없는 flag로 처리하며 뒤의 `true`는 알 수 없는 옵션으로 실패한다. |
| 4 | 같은 opt-in을 반복해 모호한 설정이 됨 | 중복 flag는 parser에서 실패한다. |
| 5 | ADB header가 없는데 첫 실제 기기 행을 버림 | 첫 줄이 `List of devices attached`가 아니면 fail-closed 한다. |
| 6 | CRLF 출력에서 정상 header를 거부 | 줄 분리와 `trim()`이 CRLF를 처리하는 회귀 검증이 있다. |
| 7 | 지정 target이 목록에 없음 | 선택 serial 일치 개수가 1이 아니면 실패한다. |
| 8 | target이 두 번 나타남 | 선택 serial 개수 검사와 전체 중복 serial 검사가 둘 다 실패시킨다. |
| 9 | target 상태가 offline 또는 unauthorized | target state가 정확히 `device`가 아니면 실패한다. |
| 10 | AVD 자체를 physical target으로 지정 | `emulator-<port>`는 target serial로 거부한다. |
| 11 | AVD가 default path로 허용됨 | opt-in이 false일 때 online AVD 추가 연결도 거부한다. |
| 12 | `emulator-`와 비슷한 임의 serial이 통과 | 추가 serial은 `^emulator-\d+$` 전체 패턴이어야 한다. |
| 13 | offline AVD가 추가 기기로 숨겨짐 | opt-in이어도 state가 `device`가 아니면 실패한다. |
| 14 | unauthorized AVD가 추가 기기로 숨겨짐 | online 상태와 같은 정확한 `device` 조건을 적용한다. |
| 15 | 다른 physical device가 opt-in 아래 선택됨 | target 외 모든 non-AVD serial은 실패한다. |
| 16 | 중복 AVD row가 목록에서 조용히 사라짐 | 모든 serial에 `Set` 유일성 검사를 적용한다. |
| 17 | 잘못된 expected serial을 조용히 대체 | 일치하는 target이 없어 명시적으로 실패한다. |
| 18 | capture 뒤 ADB 호출이 기본 기기로 돌아감 | live manifest의 collector 명령은 지정 serial을 `-s`로 사용했다. |
| 19 | opt-in 사용 여부가 사후에 불명확 | `allowExtraEmulators`를 capture manifest policy에 기록한다. |
| 20 | no-contact run을 physical 성공으로 승격 | 60초 실행은 `prestart_no_contact`, `captureWindowValid=false`, raw 0건으로 유지했고 phase-2 count를 바꾸지 않았다. |

수정 후 `mise exec -- bun test tools/benchmark/capture-android-physical-touch.test.mjs`에서 17개 테스트가 통과했다. 명시적 opt-in을 적용한 실기기 synthetic control은 app event·submit·callback·async signal 연결과 raw touchscreen 0건을 보였으며 physical positive 결과로 취급하지 않았다.
