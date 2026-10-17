# C13.1 중첩 stacking descendant full viewport hit 계획 PR 변경 검토 30개

계획, Chrome capture, integrity validator/test, STATUS, 상위 C13 계획과 public roadmap note의 변경을 서로 다른 계약 경계와 실패 조건으로 대조했다. 이 검토는 계획 PR delta 대상이며 이후 runtime 구현의 실패 검토를 대신하지 않는다.

| # | 변경 관점 | 대조 결과 |
|---:|---|---|
| 1 | 작업 목표가 제품 CSS 지원처럼 읽히는가 | plan과 STATUS 모두 고정 fixture-test slice로 한정하고 public CSS/C13 완료를 제외한다. |
| 2 | C13.1의 기존 18개 nested case와 새 69,120 grid의 관계가 분명한가 | 새 grid는 기존 18개 hit/RGBA를 보존하는 추가 판정이며 대체하지 않는다고 적었다. |
| 3 | screenshot이 runtime/app evidence처럼 제시되는가 | evidence README와 public roadmap에서 Chrome oracle-only 화면이라고 구분한다. |
| 4 | viewport 크기와 CSS 좌표 단위가 일치하는가 | `540×128` CSS px 및 integer `(x,y)` convention을 plan, JSON, validator가 공유한다. |
| 5 | half-pixel query 결과가 정수 grid에 섞였는가 | fractional edge는 `null` control로 별도 두고 full grid에서 제외한다. |
| 6 | DPR 또는 page scroll 차이가 기대 좌표를 바꾸는가 | DPR 1, scroll `(0,0)`을 JSON과 validator에서 exact 고정한다. |
| 7 | Chrome patch version이 재현 가능할 만큼 고정됐는가 | fullVersionList에서 `155.0.8059.85`, Agent Browser `0.31.1`, OS/architecture를 기록한다. |
| 8 | fixture 변경 뒤 기존 oracle를 재사용할 수 있는가 | fixture SHA를 data와 validator 양쪽에서 확인하고 mismatch면 실패한다. |
| 9 | HTML DOM node mapping이 source order 변경에 취약한가 | 고정 13-ID order를 명시하고 browser capture와 validator가 정확히 비교한다. |
| 10 | transparent layer hit가 색상 표본으로 잘못 대체되는가 | hit identity는 `elementFromPoint()`에서 직접 캡처하고 color identity 추론을 배제한다. |
| 11 | root fallback을 고정 key로 임의 설정했는가 | 모든 정수 viewport point의 observed topmost ID를 oracle에 기록하고 unknown/null은 거부한다. |
| 12 | RLE row 경계가 화면 폭과 어긋날 수 있는가 | 각 row run 합 540, 전체 row 128, point count 69,120을 각각 검사한다. |
| 13 | RLE run이 누락/중복된 point를 숨길 수 있는가 | gap, overlap, viewport 폭 초과, zero width, unknown index mutation을 tests가 거부한다. |
| 14 | 같은 ID의 연속 run 분리로 digest 표기가 흔들리는가 | canonical adjacent run 병합 요구와 negative test를 추가했다. |
| 15 | 데이터 순서는 맞지만 값이 바뀐 경우 검출되는가 | 확장 ID byte stream SHA-256을 browser JSON, validator와 plan에 고정한다. |
| 16 | grid digest만 일치하고 잘못된 browser metadata가 통과하는가 | Chrome full version, UA, platformVersion, architecture도 별도 검사한다. |
| 17 | screenshot이 다른 viewport나 다른 capture에서 올 수 있는가 | 실제 PNG SHA-256과 `540×128` dimensions를 JSON·validator·README에 교차 고정한다. |
| 18 | agent-browser raw serialization key order를 의미 있는 것으로 취급하는가 | 초기 validator 실패를 재현했고, edge control은 field별 exact 비교로 바꾸어 key order 의존을 제거했다. |
| 19 | fractional edge `null`과 integer edge root가 섞이는가 | 네 정수 edge positive와 세 fractional edge null을 서로 다른 expected entries로 고정한다. |
| 20 | viewport 밖 null control 수와 방향이 누락되는가 | left/right/top/bottom 바깥 좌표 4개를 별도로 보존하고 mutation 검사를 둔다. |
| 21 | capture 스크립트와 committed oracle가 달라질 수 있는가 | 재현용 Agent Browser eval script와 캡처 JSON을 같은 evidence set에 둔다. |
| 22 | integrity validator가 실제로 실행된 근거가 있는가 | validator output, test log, screenshot dimensions 및 SHA manifest를 evidence에 링크한다. |
| 23 | mutation tests가 모든 실패를 하나의 broad assertion으로 뭉치는가 | browser, source, viewport, mapping, row encoding, digest, screenshot, edge controls를 독립 테스트로 분리했다. |
| 24 | plan review가 plan implementation review와 혼동되는가 | 계획 실패 20개와 계획 PR delta 30개를 서로 다른 파일로 연결한다. |
| 25 | 다음 구현의 report가 OS log 용량을 넘길 수 있는가 | app report는 counts/digests/최대 16 samples만 허용하도록 계획한다. |
| 26 | eval 결과와 draw 결과가 다른 revision일 때 stale grid가 성공하는가 | generation/document/tree/style tuple 고정 및 stale rejection을 판정 기준에 넣었다. |
| 27 | Android 실행 범위가 실기기까지 확장되어 읽히는가 | `emulator-5554` API 37 ARM64 AVD로 지정하고 physical device를 제외한다. |
| 28 | iOS 실행 범위가 simulator 밖으로 확장되는가 | 지정 iPhone 17 Pro Simulator UDID만 대상으로 하고 physical device를 제외한다. |
| 29 | repository state checkbox나 aggregate count가 성급하게 증가하는가 | STATUS는 계획 단계만 추가하고 C13.1/C13 체크 및 count를 건드리지 않는다. |
| 30 | Tailnet/static roadmap가 앱 결과를 이미 통과한 것으로 표시하는가 | public note에 plan/oracle only, app grid 미실행, C13.1/C13 미완료를 명시한다. |

**판정:** raw edge JSON key order에 대한 validator 가정을 발견해 수정한 뒤 intact capture와 모든 mutation을 다시 확인했다. 남은 plan PR diff에서 제품 지원 확대, 좌표 convention 혼동, evidence 과장 또는 completion checkbox 변경을 발견하지 못했다. Runtime 구현 PR은 새 20개 implementation review와 별도 30개 PR review를 받아야 한다.
