# PR #202 변경 적대 검토 20개

검토 대상은 PR #202의 전체 변경 diff, PR 본문, `spec/STATUS.md`, C13.3 계획, 실행 증거와 Agent Browser 결과다. 제품 완료 범위와 실험 증거를 구분하고 각 경계를 별도 확인했다.

| # | 실패 관점 | 대조 결과 |
|---|---|---|
| 1 | 부분 진행을 공식 C13.3 완료로 올려 roadmap 수를 부풀리는가 | `spec/STATUS.md`의 C13.3 상위 checkbox와 작업 ID는 그대로 미완료다. 새 완료 ID를 만들지 않아 현재 완료 수 61/197을 임의로 바꾸지 않는다. 통과. |
| 2 | AVD 결과가 실기기 또는 외장 peripheral 증거로 표현되는가 | 계획, 상태 대장, PR 본문, 실행 README 모두 API 37 ARM64 AVD/uinput으로 한정하고 physical composite device 미검증을 명시한다. 통과. |
| 3 | iOS gate를 우회해 adapter를 구현 완료로 보이는가 | UIKit callback/sign/adapter는 이번 diff에 없고 `loginwindow` 상태의 미관찰 gate가 유지된다. 통과. |
| 4 | public `WheelEvent`·DOM scroll 계약이 생겼다고 오해하는가 | 내부 계약 0063과 공개 명세는 변경하지 않았다. 계획·PR 본문에 공개 event/API 범위 밖이라고 썼다. 통과. |
| 5 | 테스트-only helper가 제품 runtime으로 편입되는가 | 변경 코드는 `tools/css-reference/` 아래 runner와 fixture C helper뿐이다. Android app/runtime source와 Cargo feature 설정은 diff에 없다. 통과. |
| 6 | 공식 status, 계획, internal index가 서로 다른 증거 경로를 가리키는가 | 세 문서 링크가 실제 composite README와 fresh matrix README로 연결되고 C13.3 미완료 문구가 유지된다. 통과. |
| 7 | 기본 APK와 opt-in fixture 경계를 새 결과가 확대 주장하는가 | 이번 PR은 composite 이벤트 분류의 opt-in 실행만 주장한다. 기본 artifact 미포함은 이미 기록된 별도 근거로만 연결하며 release 검증을 주장하지 않는다. 통과. |
| 8 | 다른 Android serial이 병렬 목록에서 선택되는가 | CLI에서 `emulator-N`을 요구하며 모든 ADB command가 동일 serial을 지정한다. 실행 manifest는 `emulator-5554`와 `qemu=1`을 기록한다. 통과. |
| 9 | AVD 기종/API 차이로 결과를 잘못 일반화하는가 | 모델 `sdk_gphone16k_arm64`, API 37, Android 17, 1080×2400, density 420이 script guard와 evidence에 있다. 통과. |
| 10 | 한 장치의 합성 capability가 아닌 별도 장치의 값을 합쳐 composite로 오인하는가 | 같은 uinput 이름/descriptor와 같은 InputReader block에서 `CURSOR` 및 `TOUCHPAD`가 확인됐다. 두 scenario마다 등록/제거도 확인했다. 통과. |
| 11 | device source union을 event source로 대체해 분류 근거를 흐리는가 | callback은 `event_source_bits=0x2002`, pointer `mouse`, device union `0x10200a`를 각각 보존한다. 두 bit 집합이 모두 있다는 assertion도 있다. 통과. |
| 12 | touchpad 분류가 통과했는데 화면 이동만 mouse 경로라고 잘못 해석하는가 | callback line의 classifier result가 `MouseWheel`, target 7, admission sequence 1이다. PR 본문과 JSON에서 같은 이벤트를 대조한다. 통과. |
| 13 | normalized delta·target과 실제 draw가 일치하지 않는가 | factor 168 px / surface scale 2.625에서 64 CSS px, target `(132.09375,111.43936)`, revision 2·outer X=64와 presented draw를 로그·JSON으로 대조했다. 통과. |
| 14 | 화면 밖 입력이 queue 후에 거부돼 부수 효과가 생기는가 | `point-outside-surface` log, admission/process 부재, revision 1, 초기 offsets를 검사한다. 통과. |
| 15 | screenshot 비교가 캡처 위치 변화나 전체 화면을 scrollport 범위로 착각하는가 | SurfaceView bounds를 캡처마다 정렬하고 1050×630 크기를 검사한 뒤 CSS root scrollport를 px로 변환했다. positive outside 0 / inside 50,774, negative both 0이다. 통과. |
| 16 | 미완료 draw를 성공 화면으로 제출하는가 | `READY` 후 500ms 대기와 draw/bounds 재확인을 적용했다. 첫 조기 캡처 실패 원인과 수정도 구현 검토에 남겼다. PR 본문 이미지 두 장은 Agent Browser에서 각각 1080×2400으로 로드됐다. 통과. |
| 17 | helper가 임의 toolchain 또는 ELF 정렬을 사용하는가 | pinned NDK version의 clang/readelf를 쓰고 모든 LOAD segment의 16 KB 정렬을 검사한다. JSON에는 raw header와 hash, text sidecar에는 whitespace 정리본이 있다. 통과. |
| 18 | 검증 실패 후 가상 장치나 원격 helper가 남는가 | C helper의 정상·오류 cleanup, 각 scenario 뒤 InputReader 제거 확인, runner `finally`의 remote binary 제거와 temp directory 정리를 점검했다. 통과. |
| 19 | 이번 문서 링크가 Rspress dead-link 오류를 새로 만들었는가 | `docs:build`는 저장소 기존 dead links 때문에 실패했다. 캡처한 오류 목록에서 새 C13.3 composite/follow-up 경로는 발견되지 않았으며 추가 링크는 모두 저장소에서 확인했다. 통과; 전체 docs build는 여전히 차단. |
| 20 | PR이 요구된 검토·라벨·병합 정책 없이 바로 합쳐질 수 있는가 | `spike`, `area: android`, `area: testing`, `area: docs`가 설정됐다. PR은 main 대상이고 `MERGEABLE`; 현재 `gh pr checks`는 보고된 check가 없다. 병합 직전 상태를 다시 확인하고 Rebase and merge를 사용한다. 통과. |

미해결 구현 범위는 실제 외장 wheel/trackpad, iOS callback/sign, public event dispatch, S04.10 displayed-frame targeting, hardware GPU/compositor scanout이다. 이 PR은 C13.3 전체 완료를 선언하지 않는다.
