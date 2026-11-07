# C11.4 safe/unsafe center PR diff 실패 관점 검토 30개

검토 대상은 제한 구현 코드, RuntimeSession/Layout tests, HTML fixtures, 내부 계약·계획 변경, simulator runner, raw evidence와 이 문서다. 항목마다 서로 다른 소유권·실패·범위 경계를 확인했다.

| # | 검토 관점 | 판정 |
|---:|---|---|
| 1 | style allowlist의 정확한 value 경계 | 정확한 `safe center`·`unsafe center` 두 값만 추가했다. |
| 2 | projection test의 기존 10 alignments regression | 기존 값은 유지하고 safe/unsafe center만 accepted 쪽으로 이동했다. |
| 3 | invalid positional safety value matrix | safe start, unsafe end/flex-start와 center-safety 순서 변형을 계속 거부한다. |
| 4 | Layout profile validator와 projection validator drift | 같은 두 값만 인정하는 layout-side helper가 두 검증 경로에서 공유된다. |
| 5 | mode-specific relaxation leak | helper의 추가 values는 `RootHtmlAutoRowStretchV1`일 때만 인정한다. |
| 6 | Taffy safety adapter 변경 누락 | 기존 typed Taffy conversion은 Safe/Unsafe 각각 보존하고 새 코드는 그 입력만 연다. |
| 7 | positive control의 단위/기준 | 0px row-gap과 40px box에서 독립 `[5,15,25]` 좌표를 쓴다. |
| 8 | overflow 계산 오류 | 두 gutter 46px, track 합 30px, free space -82px를 독립 expected JSON과 맞춘다. |
| 9 | track/gutter sizing 변화 | Layout trace에서 row 3개와 gutter 2개를 분리 assert한다. |
| 10 | plain center behavior drift | overflow plain center도 별도 test case로 둔다. |
| 11 | RuntimeSession source fixture drift | 기존 percentage-row-gap fixed source만 mutate하고 topology를 보존한다. |
| 12 | output assertion이 layout state를 건너뜀 | RuntimeSession은 Ready, cascade/layout revision key equality, root frame을 확인한다. |
| 13 | failure/recovery test가 새 session으로 재현 | 같은 session의 media 환경만 바꾸어 failure와 recovery를 확인한다. |
| 14 | HTML fixtures 간 의도하지 않은 차이 | positive/safe/unsafe는 고정 shape에서 align-content와 row-gap만 의도대로 바꾼다. |
| 15 | fixture text가 geometry를 바꿈 | direct `A`/`B`, 10px monospace, 10px line height 그대로다. |
| 16 | positive fixture에 unsafe 동작을 잘못 표시 | positive simulator fixture는 safe center이고 3값 평등은 Rust RuntimeSession matrix에서 검증한다. |
| 17 | runner가 현재 user asset을 덮어씀 | 기존 bootstrap source와 generated build artifacts를 임시 백업하고 종료 trap에서 복원한다. |
| 18 | simulator에 실행 중인 앱을 무단 교체 | Android/iOS target의 기존 app process를 검사해 실행 중이면 중단한다. |
| 19 | 다른 기기/OS/ABI에서 결과를 오분류 | API 37, ARM64, 고정 iPhone 17 Pro UDID와 iOS 26.2 boot 상태를 사전 검사한다. |
| 20 | base install의 split APK 손실 | 단일 base APK가 아닌 Android install은 완전 백업이 불가능해 실행 전 중단한다. |
| 21 | fixture app ABI가 기본 runtime에 포함 | C11 fixture flags는 전부 끄고 C04 runtime GPU만 켠다. |
| 22 | 앱 ready와 draw/present를 혼동 | 로그는 boot layout-ready와 두 text draw/present marker를 각각 확인한다. |
| 23 | screenshots가 성공 scene과 연결되지 않음 | 각 run은 fixture SHA, artifact SHA, environment, runtime logs, PNG SHA를 같은 pass 폴더에 둔다. |
| 24 | restoration 실패인데 성공으로 기록 | source/build/install restoration hash를 대조하고 실패 시 임시 recovery backup을 보존한다. |
| 25 | PixelCopy/Metal surface evidence라고 표현 | README와 JSON은 OS screenshot bounds로 분류하고 surface readback을 부인한다. |
| 26 | positive-control root origin이 overflow screenshot에서 가려짐 | 각 platform pass01의 40×40 positive screenshot을 같은 앱 root origin 기준으로 사용한다. |
| 27 | unsafe 첫 item이 안 보이는 것을 음수 좌표 실패로 오해 | status area occlusion을 기록하고 negative frame 근거는 RuntimeSession/Layout gates에 둔다. |
| 28 | PR image link가 raw evidence보다 다른 파일을 가리킴 | PR 본문은 이 commit의 pass02/pass03 Android/iOS raw PNG 경로와 full SHA를 사용한다. |
| 29 | 공식 상태·로드맵 집계가 조기 증가 | C11/C11.4 checkbox와 `78/214`는 유지하고 병합 후 문서 PR에서 bounded slice만 기록한다. |
| 30 | CI·WPT·실기기 범위 확대 | checks가 보고되지 않으면 CI green으로 주장하지 않고 WPT·real device/hardware GPU는 미검증으로 남긴다. |

판정: blocking finding 없음. PR 본문과 공식 상태는 C11.4/C11 완료로 표현하지 않으며 OS screenshot을 surface parity로 표시하지 않는다.
