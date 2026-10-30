# C11.4 anonymous Grid text break-spaces 계획 PR diff 검토 30개

**검토일:** 2026-10-31
**대상:** staged plan, precomparison fixture, expected/observed JSON, screenshot diff. 이 검토 기록 자체는 대상 diff에서 제외한다.

| # | 독립 diff 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 계획이 C11.4 또는 C11 checkbox를 완료 처리하거나 72/213을 증가시키는가? | PASS — 완료 상태 변경을 요구하지 않고 제외 범위를 명시한다. |
| 2 | 계획 fixture 구현을 일반 authored CSS 지원으로 과장하는가? | PASS — opt-in C11.4 anonymous-text route로 범위를 제한한다. |
| 3 | 사용자 root HTML parity 요청이 이 plan으로 끝난 것처럼 보이는가? | PASS — 기존 별도 요청을 미완료로 둔다. |
| 4 | PR diff가 계획과 함께 실제 구현까지 시작했다고 주장하는가? | PASS — 계획 PR 뒤에 별도 implementation change로 시작한다고 명시한다. |
| 5 | CSS Text primary normative link가 존재하고 핵심 동작을 가리키는가? | PASS — W3C TR CSS Text 4 white-space section을 연결한다. |
| 6 | CSS Grid whitespace-only normative boundary가 빠졌는가? | PASS — Grid Level 1 §6을 연결한다. |
| 7 | fixture source 경로가 유효한가? | PASS — tests/fixtures/css/c11/anonymous-grid-text-break-spaces-precomparison.html를 연결한다. |
| 8 | observation JSON의 scene ID가 HTML fixture scene 집합과 어긋나는가? | PASS — 12개 ID와 순서가 일치한다. |
| 9 | independent expected JSON이 Chrome observation을 단순 복사해 oracle 독립성을 잃는가? | PASS — expected values는 Ahem metrics와 CSS Text break/hanging rules로 별도 산출했다. |
| 10 | Ahem file path/hash가 다른 font revision을 가리키는가? | PASS — fixture Ahem SHA-256은 b719ecb3…b8448이다. |
| 11 | browser capture에서 Ahem이 실제 loaded 상태인지 확인하는가? | PASS — observation은 font face loaded, document.fonts.check=true를 기록한다. |
| 12 | browser version과 viewport/DPR evidence가 누락됐는가? | PASS — HeadlessChrome 156 UA와 1280×633, DPR 1이 observation에 있다. |
| 13 | break-spaces computed values가 pre-wrap/preserve와 혼동되는가? | PASS — observed white-space-collapse:break-spaces, text-wrap-mode:wrap이다. |
| 14 | repeated-space max-content expected 60px 계산이 틀렸는가? | PASS — XX 두 개와 preserved U+0020 두 개가 각각 10px advance다. |
| 15 | min-content 30px expectation이 pre-wrap의 20px과 섞이는가? | PASS — 같은 input의 PreWrap control을 별도 scene으로 두었다. |
| 16 | fixed 25px wrap 결과가 root width와 overflowing Range fragment를 같은 값으로 취급하는가? | PASS — expected root/track 25px과 30px text fragment를 분리 기록한다. |
| 17 | forced LF scene이 HTML parser source newline과 output line count를 혼동하는가? | PASS — fixture source LF와 computed tracks/ranges를 함께 관찰한다. |
| 18 | forced break 직전 spaces가 hanging될 수 있는 경우를 놓치는가? | PASS — max-content와 min-content 모두 forced-break source를 포함한다. |
| 19 | comment split scene에서 source node identity가 없어지는가? | PASS — observation은 source index/text를 각 fragment에 기록하고 expected file은 두 source를 열거한다. |
| 20 | leading/trailing source 공백이 max-content에서 누락될 수 있는가? | PASS — 각각 별도 40px scene이다. |
| 21 | whitespace-only sequence를 anonymous item으로 렌더링할 수 있는가? | PASS — 0×0/max-content control과 Grid spec 근거를 둔다. |
| 22 | whitespace run이 following block auto-placement cell을 소비할 수 있는가? | PASS — marker 위치 (0,0)과 20px 두 tracks를 고정한다. |
| 23 | screenshot이 text glyph pixel proof로 오해될 수 있는가? | PASS — README에서 layout-only 화면과 Range fragment 한계를 설명한다. |
| 24 | HTML fixture나 observation capture가 터미널 브라우저를 사용했는가? | PASS — Agent Browser 0.31.1로 실행했고 브라우저를 닫았다. |
| 25 | browser close 뒤 실행 중 Agent Browser/Headless Chrome이 남았는가? | PASS — 해당 세션과 고유 temp-profile Chrome process를 종료했다. |
| 26 | plan review 20개가 implementation/runtime review로 중복 계상되는가? | PASS — 이 파일들은 계획 변경만 검토하며 implementation review는 구현 후 새로 작성한다. |
| 27 | plan PR diff 검토 기록이 자기 자신을 포함해 순환적으로 승인하는가? | PASS — 기록 파일은 대상 staged diff에서 제외한다. |
| 28 | runtime total 26 = 기존 16 + 신규 10을 구성하는가? | PASS — 계획은 양쪽 count와 모든 신규 scene ID를 명시한다. |
| 29 | Android/iOS gates가 hardware renderer, physical device, compositor pixel evidence로 확대 해석될 수 있는가? | PASS — AVD/Simulator와 각 증거 경계를 구분한다. |
| 30 | 새 plan/evidence local links가 repo-relative hierarchy에서 잘못 계산됐는가? | PASS — fixture, JSON, screenshot, plan 링크는 같은 repository path 기준으로 연결한다. |

## 최종 판정

계획 PR diff에 차단 문제가 없다고 판단한다. 기준 자료는 plan/evidence이며 구현·Rust tests·Android/iOS runtime·product archive·제품 지원을 승인하지 않는다. 변경 후 최종 PR head patch가 이 검토 대상과 일치하는지 병합 전에 다시 확인해야 한다.
