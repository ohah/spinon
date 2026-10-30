# C11.4 anonymous Grid text `white-space:nowrap` 계획 PR diff 검토 30개

검토 대상은 새 `white-space:nowrap` 계획·Chrome precomparison 자료, C11.4 상태 대장 추가, C11.4 상위 계획의 auto-repeat 문구 정정이다. 각 관점은 실제 변경 파일·근거·상태 경계를 별도로 대조했다.

| # | 적대 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 후보가 구현된 기능처럼 기록되는가 | 상태 대장과 계획 첫머리에 “계획·oracle, 구현 미시작”이라고 명시했다. |
| 2 | C11.4 checkbox가 잘못 완료 처리되는가 | `[ ] C11.4`를 유지했다. |
| 3 | C11 parent가 완료 처리되는가 | `[ ] C11`을 유지했다. |
| 4 | 공식 완료 수가 바뀌는가 | `72/213` 유지 문구를 명시하고 집계 로직을 바꾸지 않았다. |
| 5 | 신규 status note의 계층이 C11.4 밖으로 빠지는가 | C11.4의 기존 4칸 하위 기록 들여쓰기에 맞췄다. |
| 6 | 정상 syntax인 `white-space:nowrap`를 잘못 unsupported라고 부르는가 | CSS Text §3과 Chrome 156에서 유효한 상속 입력임을 확인했다. |
| 7 | anonymous Grid item 분류가 혼동되는가 | CSS Grid §6의 direct text anonymous block item 입력으로 한정했다. |
| 8 | normal control이 빠져 신규 값만 확인하는가 | 같은 Ahem 문자열과 `min-content` track의 normal control을 포함했다. |
| 9 | 예상 track 값이 CSSOM 문자열과 다르게 기록되는가 | Chrome observation JSON의 `20px`/`50px`를 계획 표와 대조했다. |
| 10 | root 크기와 min-content track 크기를 혼동하는가 | nowrap root 20px, track 50px, overflow 30px를 별도 기입했다. |
| 11 | row height와 line count가 모순되는가 | normal은 20px/두 fragment, nowrap은 10px/한 fragment다. |
| 12 | CSS Text normal과 nowrap 공백 처리 차이를 놓치는가 | 두 값 모두 whitespace collapse를 사용하고 wrapping 허용만 다르다고 기록했다. |
| 13 | 변경 범위가 `pre`·`pre-line`까지 넓어지는가 | 명시적으로 제외하고 `PreLine` 거부 대조를 남겼다. |
| 14 | direct item `order` 의미가 함께 풀리는가 | `order:0`만 허용하며 비영 값 negative control을 계획했다. |
| 15 | style-to-text 일반 CSS cascade 완료를 주장하는가 | `ParagraphStyle` fixture 고정 입력이며 general mapping 제외라고 명시했다. |
| 16 | computed CSS longhand와 fixture ParagraphStyle이 다를 수 있는가 | runtime validator가 `text-wrap-mode`와 paragraph style의 일치를 확인한다. |
| 17 | source node grouping 회귀가 생기는가 | 기존 5 scene을 실행하고 anchor/source provenance를 검사한다. |
| 18 | 기존 anonymous Grid route의 앱 allowlist가 빠지는가 | 계획에 기존 5 + 신규 2 scene을 같은 opt-in route에서 실행하도록 적었다. |
| 19 | item frame이라는 표현이 존재하지 않는 NodeId frame을 요구하는가 | HostDocument 테스트에서 anchor text NodeId frame 및 intrinsic input을 확인하는 기존 관측 구조를 확인했다. |
| 20 | Chrome의 page-level y좌표를 앱 좌표와 비교하는가 | acceptance를 grid-local 크기·track·fragment 상대 offset으로 제한했다. |
| 21 | Range rect를 anonymous box 또는 glyph ink로 오해하는가 | evidence에서 Range는 fragment 관찰값으로만 제한했다. |
| 22 | Ahem font 미로드 상태가 기대값에 섞이는가 | `document.fonts.check('10px Ahem')`가 true이며 asset hash를 보존했다. |
| 23 | fixture hash가 실제 파일과 다르게 기록되는가 | 재계산한 SHA-256이 README·계획 기록과 일치한다. |
| 24 | screenshot 자체가 다른 viewport로 잘못 표기되는가 | 저장된 PNG는 `1280×633`이고 Chrome 관측 DPR은 1이다. |
| 25 | Browser version의 정확도를 과장하는가 | UA에 실제 기록된 `HeadlessChrome/156.0.0.0`만 표기했다. |
| 26 | W3C 근거가 third-party 요약에 의존하는가 | 공식 CSS Grid 2025 CRD와 CSS Text Level 3 링크를 사용했다. |
| 27 | 두 auto-repeat 제한을 기능 backlog로 계속 오해하는가 | CSS Grid §7.2.3.1 최대 1회 규칙을 인용하고 기존 parser rejection test를 연결했다. |
| 28 | auto-repeat의 유효한 fixed-repeat 조합까지 지원한다고 오해하는가 | 문구는 여러 auto-repeat syntax만 invalid로 구분하고 유효한 교차 matrix는 미완료로 남긴다. |
| 29 | simulator 근거를 hardware/WPT로 일반화하는가 | AVD, iOS Simulator, 실기기, hardware GPU, WPT, compositor-after의 증거 경계를 각각 제한했다. |
| 30 | 계획 review와 PR diff review를 같은 검토로 세는가 | 계획 실패 관점 20개와 이 PR 변경 30개를 별도 파일로 보존했다. |

**결론:** 계획 PR 변경 자체의 차단 결함은 발견되지 않았다. 구현 PR에서는 계획 검토를 재사용하지 않고 새 runtime/code 실패 관점 20개를 수행한다.
