# computed `word-spacing` 계획 실패 관점 검토 20개

대상 계획: [`c11-4-anonymous-grid-text-computed-word-spacing.md`](./c11-4-anonymous-grid-text-computed-word-spacing.md). 사전 관측은 [Chrome/Ahem evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/README.md)와 고정 fixture를 별도 대조했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11.4 전체 완료로 범위를 넓히는가 | 통과. opt-in fixture slice만 목표다. |
| 2 | `word-spacing`이 현재 ParagraphStyle에 이미 있다고 가정하는가 | 통과. 현행 모델에는 필드가 없다. |
| 3 | Parley 연결이 불가능한데 신규 의존성 도입을 요구하는가 | 통과. 고정 Parley 0.12.0의 WordSpacing property를 사용한다. |
| 4 | `normal`을 nonzero spacing으로 해석하는가 | 통과. computed 0 기준을 명시했다. |
| 5 | authored `0px`와 `normal`의 serialization 차이를 숨기는가 | 통과. 두 입력을 따로 관찰하고 CSSOM computed 값 검증을 요구한다. |
| 6 | 음수 길이에서 폭 감소를 버리거나 절댓값 처리하는가 | 통과. `-1px`을 signed value로 포함했다. |
| 7 | repeated spaces를 separator 여러 개로 중복 계산하는가 | 통과. collapse 후 `XX   XX XX`는 두 separator로 계산한다. |
| 8 | line-edge trailing whitespace를 남겨 max-content 폭에 더하는가 | 통과. source의 후행 공백은 normal 처리 후 제거한다. |
| 9 | min-content에 word spacing을 무조건 더하는가 | 통과. longest unbreakable word만으로 width를 계산한다. |
| 10 | max-content가 separator 개수에 비례하지 않는가 | 통과. `s × (10px + d)`를 사용한다. |
| 11 | separator 없는 문자열을 누락하는가 | 통과. `XX` control에서 spacing 무효를 확인한다. |
| 12 | row height를 track width로부터 추정해 wrap 결과를 놓치는가 | 통과. min-content 단어 수와 max-content 한 줄을 독립 판정한다. |
| 13 | 일반 proportional font의 kerning 등 다른 변수가 섞이는가 | 통과. Ahem 10px/400으로 고정한다. |
| 14 | writing-mode, direction, ligature가 다른 결과를 유도하는가 | 통과. horizontal-tb/LTR와 고정 font profile이다. |
| 15 | CSSOM 결과를 규범 기대값으로 오인하는가 | 통과. 규범식과 font table로 expected JSON을 별도 생성한다. |
| 16 | Chrome 관측을 Spinon runtime 증거로 취급하는가 | 통과. browser-only observation임을 명시한다. |
| 17 | computed style serialization이 scene allowlist 바깥 값을 통과시키는가 | 통과. exact scene/profile validation을 요구한다. |
| 18 | default text path가 무심코 비영 spacing을 받는가 | 통과. default 0과 기존 regression 보존을 명시한다. |
| 19 | static archive scan을 전체 모바일 release로 확대하는가 | 통과. archive 경계만 주장하게 제한했다. |
| 20 | simulator/status screenshot이 glyph pixels를 증명한다고 하는가 | 통과. paint·PixelCopy·drawable·compositor 경계를 명시했다. |

## 결과

계획 단계에서 규범·독립 계산·scene 경계·negative controls·platform/artifact evidence의 누락을 찾지 못했다. 구현은 이 계획 PR이 병합된 뒤 시작한다.
