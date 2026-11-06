# Normal whitespace plan PR diff review · 30 failure perspectives

검토 대상은 계획 PR의 staged diff 전체다: 계획 문서와 별도 20개 계획 검토, Chrome 사전 비교 fixture/expected/evidence, browser capture와 lifecycle 기록. 항목들은 서로 다른 파일·실행·계약 실패를 확인한다.

1. **기준 branch가 최근 main인가?** — 현재 branch `plan/c11-root-html-normal-whitespace-rows-2026-11-06`는 최신 `main` HEAD `990ce5c41`에서 생성했고 별도 worktree는 만들지 않았다.
2. **구현 코드가 계획 PR에 섞였는가?** — staged diff에는 Rust/FFI/runtime 제품 변경이 없고 계획·fixture·기준 근거만 있다.
3. **기능이 완료 상태로 승격됐는가?** — `STATUS.md`, API 목록, 공식 집계와 C11 checkbox를 바꾸지 않았다.
4. **scope가 최근 #1202 계약을 넓게 대체하는가?** — 기존 direct `<div>` text·min-content implicit row admission은 유지하며 text separator만 후속 계획으로 제안한다.
5. **원문 text가 사전 정규화되는가?** — 계획은 trim/normalize/rewrite 금지와 raw source/NodeId 보존을 구현 요구에 명시한다.
6. **허용 whitespace set이 CSS 문서 공백과 일치하는가?** — SPACE, TAB, LF, CR만 구분자로 제안하고 외 문자 집합은 보류한다.
7. **token이 임의 문자열로 늘어나는가?** — 하나 이상의 단독 ASCII `A` token 조건을 유지한다.
8. **빈 token으로 빈/공백-only source가 통과하는가?** — 계획 검토와 구현 test에서 최소 token 1개를 요구한다.
9. **leading/trailing whitespace가 빠졌는가?** — fixture의 반복·가장자리 공백과 short text control이 이를 직접 포함한다.
10. **CSS computed style이 측정과 다른가?** — fixture `white-space:normal`, normal wrapping, monospace 10px, line-height 10px을 고정한다.
11. **line-break transformation이 언어 독립으로 단정됐는가?** — `lang=en` 기준으로 범위를 고정하고 UA 언어/스크립트 일반화를 금지한다.
12. **LF source와 CRLF parser preprocessing이 혼동되는가?** — line-feed variant와 CR character-reference variant를 분리했다.
13. **HTML parser가 CR character reference를 제거하는가?** — Chrome fixture 결과에 `&#x0d;` source를 보존하고 실제 DOM text 결과를 기록한다.
14. **form feed가 공백 separator로 허용되는가?** — Chrome control에서 min-content 폭이 single-space와 다른 14.671875px이고 계획상 reject다.
15. **vertical tab가 tab과 혼동되는가?** — U+000B는 같은 폭 대조로 분리했고 U+0009만 허용한다.
16. **NBSP를 collapsible separator로 처리하는가?** — Chrome min-content 폭 18.0625px와 별도 negative control을 기록한다.
17. **NEL을 segment break로 일반화하는가?** — U+0085 negative control과 16.03125px 폭을 보존하고 계획에서 범위 밖으로 둔다.
18. **여섯 입력 variant 수가 24개 scene count와 맞는가?** — 6 × 2 columns × 2 DOM orders = 24이며 raw observation에서도 24개다.
19. **각 baseline 조합이 존재하는가?** — single-space 기준 2 columns × 2 orders 4개가 있고 나머지 20개와 대응한다.
20. **기준 열 조건이 한 가지뿐인가?** — `min-content`와 definite-container `auto`를 모두 포함해 6.03125px/40px 결과를 비교한다.
21. **DOM order를 무시하는가?** — 긴/짧은 text item의 forward/reverse 각 결과를 고정하고 raw frames/line positions를 남긴다.
22. **text line count를 frame만으로 추측하는가?** — Range line rectangles를 직접 모아 줄 위치를 비교한다.
23. **explicit/implicit row 기대값이 빠졌는가?** — explicit 10px row, text row와 34px overflow control의 전체 used row list를 기록한다.
24. **overflow가 viewport 밖인지 Grid 밖인지 혼동되는가?** — expected control은 Grid 아래 34px item frame이며 screenshot은 reference fixture만 보인다.
25. **baseline 비교가 다른 page 위치 때문에 흔들리는가?** — item frame과 Range line position을 Grid origin 기준 local coordinate로 정규화했다.
26. **summary가 raw observation과 불일치하는가?** — JSON 생성 단계에서 24 cases, 20 non-baseline match, mismatch 0을 assert하고 raw file도 JSON parse했다.
27. **negative-control 요약이 raw source를 잃는가?** — 별도 JSON은 name, source, computed column/row, item width 및 range rectangles를 담는다.
28. **screenshot이 제품 또는 pixel parity evidence로 오해되는가?** — 1280×752 Agent Browser Chrome reference capture이며 README가 제품 구현·시뮬레이터·WPT 검증이 아님을 명시한다.
29. **local markdown link가 끊기는가?** — 검토 문서와 함께 계획/README의 local paths를 다시 검사하며 현재 검토 문서 링크도 존재한다.
30. **Agent Browser process가 남았는가?** — `close --all` 결과와 후속 process filter 결과 0개를 증거 JSON으로 보존했다.

**판정:** 계획 PR diff에서 구현 경계, 기준 입력, raw evidence, source-order/identity, failure boundary와 한계가 서로 일치한다. 발견 결함은 없었다. 이 문서는 plan diff 검토이며 구현 코드 검토가 아니다. 구현 PR은 새 20개 implementation/runtime review와 새 30개 PR diff review를 받아야 한다.
