# C11.4 `white-space:pre` preserved tab 계획의 실패 관점 검토 20개

**검토 대상:** [`c11-4-anonymous-grid-text-pre-tabs.md`](./c11-4-anonymous-grid-text-pre-tabs.md)와 Chrome 156 사전 비교 fixture. 구현 코드 검토가 아니다. 각 항목은 계획에서 요구할 별도 실패 경로와 이를 잡을 관문이다.

| # | 적대적 관점 | 필요한 판정/방어 | 계획 판정 |
|---:|---|---|---|
| 1 | tab origin을 line item 또는 Grid cell 시작으로 잘못 잡음 | CSS Text의 nearest block container origin을 기록하고, 이번 fixture에서 ancestor/Grid content origin이 x=0으로 일치하는 조건과 offset ancestor 제외를 고정 | 통과 |
| 2 | initial `tab-size`를 4 또는 폰트 크기로 혼동 | CSS Text §4.2 initial 8 및 Ahem space advance 배수로 80px stop을 독립 계산 | 통과 |
| 3 | exact stop x=80에서 tab이 폭 0이 됨 | `XXXXXXXX\tX`로 strict subsequent stop x=160 검증 | 통과 |
| 4 | leading tab이 빈 값이라 제거됨 | `\tX`가 90px line/track을 만드는 장면 고정 | 통과 |
| 5 | consecutive tab에서 두 번째 tab이 첫 번째 이전 위치 기준으로 계산됨 | `XX\t\tX`에서 stop x=80, x=160을 따로 계산 | 통과 |
| 6 | trailing tab이 trim되어 intrinsic contribution에서 빠짐 | `XX\t`가 80px max-content를 유지해야 함 | 통과 |
| 7 | forced break 뒤 tab column이 이전 줄에서 이어짐 | 두 줄의 advance 100px/90px 장면으로 line start 재설정 확인 | 통과 |
| 8 | CRLF를 두 forced break로 해석 | 한 LF로 normalize하고 CR·LF source range 둘 다 보존할 테스트를 계획 | 통과 |
| 9 | CR과 LF가 서로 다른 `TextSource`에 나뉘어 들어옴 | flattened paragraph 처리 후 source별 UTF-16 provenance와 단일 line reset 확인 | 통과 |
| 10 | form-feed/CR forced break가 LF와 다른 cursor reset을 함 | 허용된 segment-break class를 고정하고 각각 같은 reset 규칙 확인 | 통과 |
| 11 | `min-content`가 preserved tab을 soft-wrap opportunity로 취급 | `minmax`와 min-content track 100px, 한 줄 기대값을 고정 | 통과 |
| 12 | `max-content` contribution에서 expanded spaces 일부가 누락 | 2 columns 뒤 tab 장면 100px 독립 계산과 Chrome 교차 확인 | 통과 |
| 13 | definite 25px track에서 `pre` text가 wrap되어 overflow가 사라짐 | root/track 25px와 line advance 100px를 분리해 검증 | 통과 |
| 14 | proportional font 또는 `letter-spacing`/`word-spacing`이 tab advance를 바꿈 | Ahem·normal letter spacing·0 word spacing으로 한정하고 다른 profile을 거부 | 통과 |
| 15 | U+0009 하나를 여러 space byte range로 바꾼 뒤 source mapping이 한 칸만 덮음 | tab의 source range는 한 UTF-16 unit, output range는 확장된 전체 space range로 검증 | 통과 |
| 16 | paragraph가 여러 text node로 나뉘어 tab cursor 초기화 | cursor는 paragraph 전체에서 이어가고 각 text node의 source identity를 보존 | 통과 |
| 17 | authored `tab-size:2`가 지원값 8처럼 처리됨 | computed longhand를 검사해 2를 고정 거부하며 Chrome의 60px 음성 경계를 보존 | 통과 |
| 18 | `tab-size:<length>` 또는 offset ancestor가 숫자 profile처럼 오인됨 | 사용자 지정 숫자/길이와 offset origin을 제외하고 허용하지 않는 profile은 fail-closed | 통과 |
| 19 | tab 입력 오류가 intrinsic/final layout 중 한 경로에서만 발생 | supported Ahem profile 밖의 tab은 두 API에서 모두 같은 오류로 닫고 partial snapshot을 금지 | 통과 |
| 20 | opt-in scene gate 또는 fixture ABI가 새 tab 입력을 기본 product로 노출 | exact scene/style allowlist와 product/fixture archive negative/positive controls를 계획 | 통과 |

## 계획 보정 기록

검토 뒤 계획은 다음 근거를 포함한다: tab-stop origin과 고정 x=0 fixture 제한, exact-stop/leading/consecutive/trailing/break-reset 입력, `tab-size` computed longhand 및 2 거부, source mapping, min-content와 25px overflow, artifact boundary, simulator draw/present 한계. 검토는 향후 구현이 통과했다는 뜻이 아니다.
