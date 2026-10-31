# C11.4 numeric author `tab-size` 구현/runtime 실패 관점 20개

구현 후 별도의 20개 관점으로 Rust normalization·intrinsic sizing, computed-style 수집, 두 플랫폼 fixture, simulator evidence를 검토했다. 이 결과는 C11.4/C11 전체 완료 판정이 아니다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | authored declaration이나 scene ID가 runtime tab 배수를 직접 결정하는가 | **PASS** — runtime collector는 computed-style snapshot의 `tab-size` 문자열을 `TabSize`로 바꾸고 scene별 기대값과 일치하는지만 검사한다. 실 입력은 snapshot에서 온 typed 값이다. |
| 2 | snapshot 값이 없거나 scene 기대값과 다를 때 default 8로 진행하는가 | **PASS** — missing 값과 `tab-size:4` 장면에 computed `2`를 준 단위 검사가 모두 style 오류를 확인한다. |
| 3 | `0`, fractional, `<length>` 또는 임의의 serialization이 통과하는가 | **PASS** — `0`, `1.5`, `20px`은 Android·iOS 각 3/3 기대 거부이며 frame 없이 끝난다. |
| 4 | 허용 값의 type이 유효하지 않은 stop 크기를 만들 수 있는가 | **PASS** — `TabSize`는 1·2·4·8만 표현하고 stop 계산은 0으로 나누지 않는다. |
| 5 | 각 numeric stop에서 leading tab의 공백 수가 틀리는가 | **PASS** — Ahem 입력 `\tX`가 size 1/2/4에서 각각 1/2/4칸으로 정규화되고 root 폭 20/30/50px를 만든다. |
| 6 | 이미 stop에 놓인 tab이 현재 위치를 유지하는가 | **PASS** — `X\tX`, `XXXX\tX` exact-stop 장면이 1·2·4에서 다음 strict stop으로 이동한다. size 4 결과는 90px다. |
| 7 | 두 tab이 같은 stop을 재사용하는가 | **PASS** — `XX\t\tX` size 2 결과는 `XX    X`, 두 탭 각각 2칸, 70px다. |
| 8 | forced LF 뒤 이전 줄의 advance가 새 줄 tab 위치에 남는가 | **PASS** — `XX\tXX\nX\tX`는 두 줄 advance 60/30px, frame 60×20px다. |
| 9 | CRLF가 두 forced break를 만들거나 두 번째 source range가 유실되는가 | **PASS** — `X\t\r\nX\tX`는 20/30px 두 줄이다. CR/LF 각각은 같은 normalized newline range를 가리킨다. |
| 10 | tab 확장 뒤 UTF-16 source와 UTF-8 output range가 어긋나는가 | **PASS** — Rust test와 Android/iOS capture validator가 tab 한 source code unit을 확장 공백 범위 전체에 연결한다. |
| 11 | 여러 `TextSource`가 이어질 때 inline cursor 또는 tab owner가 초기화되는가 | **PASS** — text-layout regression은 첫 source `XX` 다음 source의 tab이 이어서 2칸 이동하고 원본 node/source range를 유지함을 검사한다. |
| 12 | min-content가 preserved tab에서 임의 줄바꿈해 폭이 줄어드는가 | **PASS** — size 2 `XX\tXX`의 min/max-content는 모두 60px이고 한 줄이다. |
| 13 | 고정 25px track에서 `pre` line이 wrap되거나 track 폭과 intrinsic advance가 혼합되는가 | **PASS** — track/frame은 25px, paragraph advance는 60px로 분리되고 한 줄을 유지한다. |
| 14 | non-Ahem font, 다른 문자, nonzero spacing 또는 inline style이 조용히 허용되는가 | **PASS** — tab profile 검증은 고정 typography·writing mode·direction·spacing·inline topology를 요구하고 Rust intrinsic/final layout에서 같은 profile 밖 입력을 닫는다. |
| 15 | 새 white-space variant가 shaping의 `nowrap` 또는 preserved-space 정책에서 빠지는가 | **PASS** — text engine은 `PreWithTabSize`를 `NoWrap`과 `Preserve`에 연결하고 67개 text-layout 테스트가 통과했다. |
| 16 | 기본 `tab-size:8`이나 별도 `PreWrap` 탭 동작이 바뀌는가 | **PASS** — 기본 8 regression은 두 simulator matrix에 포함됐고 `PreWrap` 경로는 기존 별도 지원 profile과 default stop을 사용한다. |
| 17 | layout collect/compute 중 새 mode를 거부하거나 unrelated HostDocument 경로까지 fixture 허용을 넓히는가 | **PASS** — layout collect와 compute의 허용 분기를 명시적으로 갱신했고 32개 익명-text layout tests가 통과했다. |
| 18 | Rust, FFI, Android, iOS의 scene catalog나 기대 frame이 서로 달라지는가 | **PASS** — 55개 catalog를 fixture-boundary Node 검사로 고정했고 두 simulator에서 같은 55 ID가 모두 검증됐다. |
| 19 | fixture-only scene ID나 C ABI가 기본 archive에 누출되는가 | **PASS** — 기본 FFI archive는 numeric scene ID 0/16, C11.4 fixture symbol 0개; opt-in archive는 16/16과 symbol 2개다([artifact 결과](./artifact-boundary.json)). |
| 20 | stale 설치 앱·누락된 draw/present·scene retry가 성공으로 오인되는가 | **PASS** — 두 플랫폼은 각 55/55, positive 52/52 draw/present, negative 3/3, retry 0이며 build/install SHA-256이 일치한다([simulator 결과](./simulator-matrix-verification.json)). |

전체 workspace Clippy에서 신규 단위 테스트의 외부 `vec!` lint 하나가 발견되어 배열로 수정했고, default workspace 및 C11.4 fixture-feature Clippy를 다시 실행해 통과했다.

## 남은 범위

두 결과는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 나온다. Android 실기기, iOS 실기기, hardware GPU, compositor 이후 glyph pixel equality, arbitrary CSS cascade, 전체 WPT, C11.4 또는 C11 완료는 검증하지 않았다. 공식 `72/213`은 유지한다.
