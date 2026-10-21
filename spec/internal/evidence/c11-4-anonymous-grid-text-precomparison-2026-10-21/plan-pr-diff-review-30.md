# C11.4 anonymous Grid text 계획 PR diff 적대 검토 30개

계획 자체의 실패 관점 검토 20개와 별도로, 계획 PR 변경 전체와 근거 파일 연결을 30개 관점에서 검토한다. 기능 구현 검토로 재사용하지 않는다.

| # | 실패 관점 | 공격 검토와 판정 |
| ---: | --- | --- |
| 1 | 사전 비교 README에서 fixture 상대경로가 깨지는가 | 처음에는 `../../../tests`라 존재하지 않는 저장소 내부 `spec/tests`를 가리켰다. `../../../../tests`로 고쳤고 대상 fixture가 존재한다. |
| 2 | fixture 안 Ahem URL이 잘못 풀리는가 | `../../resources/c15/Ahem.ttf`가 `tests/fixtures/resources/c15/Ahem.ttf`로 해석되고 파일이 존재한다. |
| 3 | 계획의 상위 계획 링크가 깨지는가 | 같은 `plan/` 안의 `c11-4-grid-track-sizing.md`를 가리킨다. |
| 4 | 계획의 내부 명세 링크가 깨지는가 | 저장소의 `spec/internal/0059-c11-grid-layout.md`를 가리킨다. |
| 5 | 공식 상태 링크가 존재하지 않는 C11 anchor로 깨지는가 | `STATUS.md`에는 C11 heading/anchor 대신 목록 항목이 있어 fragment를 제거하고 공식 상태 문서 전체로 연결했다. |
| 6 | Chrome 관찰 JSON 링크가 빠지거나 엉뚱한 산출물을 가리키는가 | 계획과 README가 같은 evidence 디렉터리의 `chrome155-observation.json`을 지목한다. |
| 7 | 화면 캡처 링크가 빠지거나 시각 근거와 혼동되는가 | 계획과 README가 해당 fixture의 Chrome 캡처 PNG를 지목한다. |
| 8 | fixture 수정 뒤 기록한 입력 hash가 낡았는가 | 공백 입력과 viewport 기록을 포함한 최종 fixture hash `7f105fae…b4860bd`를 다시 계산해 기록했다. |
| 9 | Ahem 파일이 바뀌었는데 옛 hash를 유지하는가 | 저장소 Ahem 파일의 SHA-256 `b719ecb3…94b8448`과 README가 일치한다. |
| 10 | JSON을 다시 캡처한 뒤 옛 hash가 남았는가 | 최종 관찰 JSON SHA-256 `44211bc4…3a9f8e`를 다시 계산해 기록했다. |
| 11 | 캡처 파일이 바뀌었는데 screenshot hash가 틀리는가 | PNG SHA-256을 다시 계산했다. 화면은 공백 문자만 바뀐 입력에서 동일하게 렌더링됐다. |
| 12 | 고정 viewport가 기록되지 않아 좌표를 재현할 수 없는가 | fixture가 `innerWidth/innerHeight`를 관찰에 넣고 README에 `1280×633` CSS px를 기록한다. |
| 13 | Chrome 버전·플랫폼·DPR 정보가 빠졌는가 | README와 JSON에 Chrome 155, user agent, MacIntel, DPR 1을 남겼다. |
| 14 | Ahem fallback인데 폰트 준비를 통과로 오인하는가 | `document.fonts.ready` 이후 `ahemReady=true`를 관찰했고 화면에서도 Ahem 블록 glyph를 확인했다. |
| 15 | Grid 직접 텍스트를 여러 DOM node item으로 잘못 세는가 | 고정 입력에는 `XX XX` 단일 text node가 있으며 min-content track에서 두 줄 Range fragment를 기록한다. |
| 16 | comment가 텍스트 item 경계를 만든다고 오인하는가 | comment가 text node 사이 DOM child로 기록되지만 두 fragment는 한 행의 x 0·20px에 이어진다. |
| 17 | block child가 양쪽 텍스트 run을 합쳐 source order를 깨는가 | text/block/text direct-child 순서와 20·30·40px track 및 각 frame이 함께 기록된다. |
| 18 | ASCII space만 보고 모든 공백 문자를 입증했다고 과장하는가 | fixture whitespace run을 U+0020, U+0009, U+000A로 구성하고 계획은 이 고정 profile 범위만 판정한다. |
| 19 | whitespace Range가 비어도 anonymous item이 셀을 소비하는가 | 공백 Range rect가 비고 양쪽 block은 첫 두 track에 위치한다. |
| 20 | page 좌표와 grid-local 좌표를 혼동하는가 | JSON은 page 좌표를 보존하고 README는 Grid 원점을 빼는 변환을 명시한다. |
| 21 | Range fragment를 anonymous item box 또는 glyph ink로 과대해석하는가 | 계획과 README가 Range 관찰을 inline fragment로 한정하고 앱 geometry와 paint 근거를 분리한다. |
| 22 | CSSOM computed track과 기대값의 단위가 모호한가 | columns·rows는 CSS px 문자열이며 fixture의 고정 Ahem font와 track 입력이 함께 남는다. |
| 23 | screenshot이 네 장면 전부를 포함하지 않는가 | Agent Browser가 전체 fixture의 네 장면을 캡처했고 화면에서 각 label과 렌더링을 확인했다. |
| 24 | Chrome 관찰을 Spinon 결과로 오인하는가 | README는 독립 사전 oracle이며 앱 runtime·WPT 결과가 아니라고 명시한다. |
| 25 | 계획 PR을 구현 완료로 표시하는가 | 계획은 구현 전 상태라고 명시하고 런타임 검증을 다음 단계로 둔다. |
| 26 | 공식 C11.4 checkbox나 완료 수를 앞당기는가 | diff에는 계획 note만 추가하고 C11.4 checkbox와 완료 수는 바꾸지 않는다. |
| 27 | 한정 구현을 일반 CSS Grid 지원으로 넓혀 해석하는가 | inline element, nested/non-leaf, `pre`, 기타 placement와 WPT 한계를 계획에 열거한다. |
| 28 | 공개 API나 내부 계약 버전을 조용히 변경하는가 | 이 PR은 계획·fixture·근거만 추가하며 API 선언이나 계약 버전을 수정하지 않는다. |
| 29 | Android 실기기 serial과 model을 혼동하는가 | 계획에서 serial `R5KYB06SMPE`와 model `SM-S731N`을 분리했다. 현재 ADB 목록에는 emulator 둘뿐이라 실기기 실행을 주장하지 않는다. |
| 30 | PR 화면에서 screenshot이 실제 표시되는지 확인하지 않고 병합하는가 | PR #421의 commit-pinned PNG를 Agent Browser에서 확인했다. 이미지가 로드됐고 natural size는 1280×633이다. `gh pr checks --required`는 체크 없음, main required-status-checks API는 비활성(404), repository rulesets는 빈 목록이며 PR merge state는 `CLEAN`이다. |

## 판정

계획 파일과 연결된 Chrome oracle diff에서 세 결함(README fixture 경로, 기기 serial/model 표기, 없는 상태 anchor)을 발견해 수정했다. 최종 해시·viewport·공백 입력을 다시 기록했다. PR #421 본문의 실제 screenshot 렌더링을 Agent Browser에서 확인했다. 현재 GitHub에는 이 PR의 required check이 설정되어 있지 않으며 merge state는 `CLEAN`이다.
