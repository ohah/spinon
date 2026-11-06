# Normal whitespace intrinsic-row plan review · 20 failure perspectives

계획 [C11.4 parsed Root HTML normal whitespace for intrinsic rows](./c11-root-html-intrinsic-normal-whitespace-multiple-text.md)의 구현 전 실패 검토다. 각 항목은 다른 입력·계약·실행 경계를 확인한다.

1. **공백만 있는 source가 빈 token으로 허용되는가?** — token을 제거한 뒤 최소 `A` 하나를 요구하고, empty/whitespace-only를 거부하도록 계획했다.
2. **연속 공백에서 empty split token을 오판하는가?** — ASCII collapsible separator runs를 건너뛰고 실제 token이 모두 `A`인지 별도 조건으로 판정한다.
3. **선행·후행 공백이 원문에서 삭제되는가?** — source는 trim하지 않고 기존 `white-space: normal` text-layout 처리에 전달한다.
4. **여러 ASCII 공백이 한 줄의 break opportunity를 보존하는가?** — Chrome repeated/edge-space scene과 사용 열·행 및 line position 비교를 요구한다.
5. **탭의 tab-size가 무심코 적용되는가?** — 허용 스타일은 `white-space-collapse: collapse`; 보존 탭은 거부하고 collapse 처리만 확인한다.
6. **LF segment break가 보존 줄바꿈으로 잘못 취급되는가?** — `normal` collapse의 영어 fixture에서 Chrome 기준 line count·frame과 비교한다.
7. **segment-break 변환의 언어 의존성은?** — UA 정의 여지를 표시하고 `lang=en` fixture 밖으로 일반화하지 않는다.
8. **CRLF를 두 개의 break로 세는가?** — HTML parser의 newline preprocessing과 CR character-reference 입력을 서로 다른 입력으로 유지한다.
9. **DOM 안의 CR character reference가 space처럼 처리되는가?** — CSS Text §4.3.3 근거와 전용 Chrome control을 둔다.
10. **form feed를 일반 HTML whitespace로 과잉 허용하는가?** — form feed는 허용 separator에서 제외하고 Chrome min-content 폭 negative control로 차이를 기록한다.
11. **vertical tab가 탭과 혼동되는가?** — U+0009만 허용하고 U+000B는 별도 reject control로 둔다.
12. **NBSP가 collapsible whitespace로 오인되는가?** — U+00A0를 허용 집합에서 제외하고 min-content width 대조를 기록한다.
13. **NEL 등 다른 Unicode separator를 ASCII whitespace로 처리하는가?** — NEL을 거부하고 Unicode token/text 지원은 별도 후속 범위로 둔다.
14. **`AA`가 두 개의 허용 token으로 오인되는가?** — 모든 분리 뒤 token은 정확히 단일 ASCII `A`여야 한다.
15. **다른 글자·혼합 token이 통과하는가?** — `B`, `A B`, fullwidth `Ａ` 및 non-ASCII 입력을 negative tests에 포함한다.
16. **여러 text item에 확장이 일관되지 않는가?** — 긴/짧은 item 모두 같은 source validator를 통과하며 reverse DOM order도 검사한다.
17. **item order 또는 contribution owner가 합쳐지는가?** — 각 source의 NodeId, original text, row contribution, frame, paint order를 별도로 유지한다.
18. **stale revision/invalid 2번째 item이 부분 게시를 만드는가?** — 실패 source·owner/revision mutation이 frame/render snapshot 전에 원자 거부되는지 검사한다.
19. **10,000 track resource bound가 바뀌는가?** — whitespace parser는 item 수나 checked implicit-row preflight를 변경하지 않으며 기존 10,000/10,001 회귀를 다시 실행한다.
20. **Chrome 관측만 제품 검증으로 과장되는가?** — 계획은 precomparison을 기준 입력으로만 취급하고 Rust RuntimeSession·Android AVD·iOS Simulator 검증을 별도로 요구한다.

검토 중 수정이 필요한 계획 결함은 남지 않았다. 구현 PR은 별도 20개 구현/runtime 검토와 실제 코드 diff 30개 검토를 새로 수행한다.
