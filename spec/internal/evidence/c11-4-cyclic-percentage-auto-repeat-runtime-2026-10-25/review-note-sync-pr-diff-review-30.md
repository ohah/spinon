# C11.4 cyclic auto-repeat 검토 기록 정정 PR diff 적대 검토 30개

**검토 대상:** `implementation-runtime-review-20.md`의 최종 판정 문구 및 근거 연결 정정
**범위:** 문서 diff와 이미 커밋된 runtime evidence만 확인. 생산 코드·API·fixture·roadmap 상태 변경 없음.
**판정 기준:** 최종 PASS가 기존 evidence의 연결 관계를 정확히 설명하고 한계를 감추지 않아야 한다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 초기 결과를 최종 결과처럼 표시하는가 | 19 PASS / 1 PARTIAL을 초기 결과로 남긴다. PASS |
| 2 | 표의 최종 집계가 행별 판정과 다른가 | 20행은 모두 PASS이고 최종 집계는 20/0/0이다. PASS |
| 3 | #20이 처음 PARTIAL이었던 이유를 잃는가 | 후속 evidence commit 전에는 source-bound 자료가 부족했다는 이유를 남긴다. PASS |
| 4 | evidence commit SHA가 저장소에 없거나 잘못된가 | `ea06d8ff86729d582b4821554cf87d22170aefc0` 객체가 존재한다. PASS |
| 5 | evidence commit parent가 다른 구현 revision인가 | parent는 `c5b8ef75804e39d36a37aa61b82b1f028115fefd`다. PASS |
| 6 | full source SHA가 README와 맞지 않는가 | review 표와 README의 구현 revision이 일치한다. PASS |
| 7 | README 구간 citation이 source binding을 가리키는가 | README의 source revision, 실행/해시 구간을 지목한다. PASS |
| 8 | README가 evidence commit tree에 빠져 있는가 | evidence commit tree에 해당 README가 포함된다. PASS |
| 9 | Android/iOS runtime logs가 빠져 있어 결과를 추적할 수 없는가 | 두 플랫폼의 source/runtime logs가 commit된 evidence에 있다. PASS |
| 10 | 전체 화면 캡처를 pixel assertion으로 오해하는가 | 화면은 보조 자료고 표본 판정은 readback samples로 제한한다. PASS |
| 11 | Android PixelCopy 입력 자료가 evidence에 없는가 | PixelCopy sample JSON/base64와 raw capture를 보존한다. PASS |
| 12 | iOS Metal sample 입력 자료가 evidence에 없는가 | Metal sample JSON/base64와 관련 실행 로그를 보존한다. PASS |
| 13 | 두 target/control run을 하나로 합쳐 세는가 | cyclic/definite, Android/iOS 네 run ID를 각각 열거한다. PASS |
| 14 | Android run ID가 cyclic/definite 역할과 바뀌는가 | Android의 두 ID와 각 trace/frame을 별도로 기록한다. PASS |
| 15 | iOS run ID가 cyclic/definite 역할과 바뀌는가 | iOS의 두 ID와 각 trace/frame을 별도로 기록한다. PASS |
| 16 | Android 표본 수가 README와 어긋나는가 | 두 Android scene의 PixelCopy samples는 각 3/3이다. PASS |
| 17 | iOS 표본 수가 README와 어긋나는가 | 두 iOS scene의 Metal drawable samples는 각 3/3이다. PASS |
| 18 | APK digest를 iOS나 archive artifact에 붙이는가 | Android APK SHA는 README의 debug APK SHA와 일치한다. PASS |
| 19 | iOS executable digest를 APK나 archive artifact에 붙이는가 | iOS Simulator executable SHA는 README의 값과 일치한다. PASS |
| 20 | 앱 바이너리 부재를 숨겨 digest의 재현성을 과장하는가 | 앱 바이너리가 evidence tree에 없고 commit에서 digest를 재계산할 수 없다는 한계를 명시한다. PASS |
| 21 | 과거 SHA 재계산을 이번 PR에서 재실행한 것처럼 쓰는가 | 재계산은 후속 reviewer가 당시 보유한 산출물로 수행한 역사적 확인이라고 구분한다. PASS |
| 22 | default archive digest가 앱 executable hash와 섞이는가 | default static archive를 앱 산출물과 별도로 기록한다. PASS |
| 23 | positive fixture archive가 negative control처럼 기록되는가 | opt-in archive의 marker와 fixture symbols 존재를 별도 기록한다. PASS |
| 24 | static archive 결과를 release APK/IPA 제외 증거로 확대하는가 | README의 APK/IPA 한계를 그대로 유지한다. PASS |
| 25 | iOS drawable 표본이 compositor 이후 결과로 표시되는가 | compositor 제출 전 Metal drawable 한계를 명시한다. PASS |
| 26 | Android 결과가 실기기나 hardware GPU로 확대되는가 | software-rendered Android AVD 범위를 유지한다. PASS |
| 27 | 이번 PR이 실기기 검증을 새로 수행했다고 오해되는가 | 이번 변경은 문서 정정이며 실기기 실행을 주장하지 않는다. PASS |
| 28 | 후속 read-only review를 새 test/build/CI 실행으로 읽는가 | 후속 reviewer가 추가 실행을 하지 않았음을 기록한다. PASS |
| 29 | 구현/runtime review를 PR diff review나 GitHub approval과 합산하는가 | 각 review 유형 및 GitHub 상태를 별도로 취급한다. PASS |
| 30 | 문서 정정으로 C11.4 완료나 완료 수가 승격되는가 | C11.4 미완료와 공식 `72/213`을 그대로 유지한다. PASS |

**결과:** 30개의 서로 다른 source binding, artifact identity, platform evidence 및 범위 해석 경로를 확인했다. 앱 바이너리 자체가 evidence tree에 없다는 한계도 본문에 명시한다.
