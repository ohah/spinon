# C11.4 반복 intrinsic row text 계획 PR diff 검토 · 30 관점

이 검토는 계획 PR의 실제 변경분만 다룬다. 계획 자체의 선행 20관점 검토 및 이후 구현/runtime 검토와 별도다.

| # | 독립 변경 실패 관점 | 판정 |
|---:|---|---|
| 1 | 공식 상태 대장의 C11.4 미완료 표시를 잘못 바꾸는가? | PASS — `spec/STATUS.md`와 완료 checkbox를 수정하지 않는다. |
| 2 | 공식 `78/214`를 구현 전 계획을 이유로 증가시키는가? | PASS — roadmap과 plan에 그대로 유지한다. |
| 3 | roadmap이 병합된 #1286을 여전히 진행 중이라 표시하는가? | PASS — #1286 merge SHA·시간과 지원 profile로 갱신한다. |
| 4 | 현재 작업을 완료됐다고 오해하게 하는가? | PASS — 다음 계획과 구현 전 상태를 분리해 표시한다. |
| 5 | #1286의 정확한 track multiset과 30순서 범위를 축약하다 왜곡하는가? | PASS — 계획과 roadmap에 같은 다섯 후행 member를 쓴다. |
| 6 | Chrome 관측을 Spinon 구현 결과로 서술하는가? | PASS — evidence와 plan 모두 제품 구현 전 기준으로 한정한다. |
| 7 | 브라우저 종류·버전·DPR를 재현할 수 없게 하는가? | PASS — Agent Browser, HeadlessChrome 157, UA, DPR를 기록한다. |
| 8 | fixture SHA가 expected JSON과 맞지 않는가? | PASS — HTML SHA `bac3d41d...e9f0b`를 reducer와 README에 고정한다. |
| 9 | 텍스트가 표준 줄바꿈 외에 inline style이나 폰트 상태를 물려받는가? | PASS — payload style을 명시하고 exact text source만 계획 범위로 둔다. |
| 10 | 9개 direct item과 8개 payload topology가 fixture와 다르게 기술되는가? | PASS — fixture·계획·JSON 모두 동일한 DOM 순서를 쓴다. |
| 11 | 줄 수 측정이 screenshot의 픽셀 추정에 의존하는가? | PASS — Range rect y 좌표와 JSON 수치를 사용한다. |
| 12 | 30개 unique order 생성기가 duplicate 또는 누락 순서를 허용하는가? | PASS — multiset counts 2/2/1을 고정하고 결과 수를 저장한다. |
| 13 | 220px 표준 scene 수 산식이 틀리는가? | PASS — 30 × 4 cap × 2 align = 240으로 기록한다. |
| 14 | 100px overflow scene이 표준 220px 조건과 섞이는가? | PASS — 20px/start인 별도 30 scene으로 기록한다. |
| 15 | 0%, 120%, 직접 percentage의 definite basis 경계가 누락되는가? | PASS — 네 cap을 각각 30 tail order에 적용한다. |
| 16 | start/stretch에서 `auto` stretch 차이를 볼 수 없는가? | PASS — 동일 cap·order의 두 alignment scene을 모두 저장한다. |
| 17 | `gridRows` 외에 item/payload frame이 빠져 overflow를 놓치는가? | PASS — 9 item 및 8 payload의 `[x,y,width,height]`를 저장한다. |
| 18 | subpixel 수치가 문서화 없이 임의 반올림되는가? | PASS — 1e-5 CSS px까지 보존하고 허용 오차를 0.01로 제한한다. |
| 19 | 축약 JSON이 DOM frame 순서를 모호하게 만드는가? | PASS — README에서 fixture 순서와 각 배열 필드를 설명한다. |
| 20 | capture script가 저장된 fixture와 다른 스타일로 캡처하는가? | PASS — script는 fixture의 root variables만 바꾸고 수치를 직접 만든다. |
| 21 | reduction filter가 전체 frame 수치를 빼서 재검토를 막는가? | PASS — 모든 track, Grid, item, payload geometry를 보존한다. |
| 22 | 확대 screenshot이 기준 수치 이미지로 오해되는가? | PASS — 확대는 시각 안내일 뿐이며 JSON이 숫자 기준임을 명시한다. |
| 23 | outline이 실제 Grid box geometry에 포함됐다고 오해하는가? | PASS — outline은 overflow 캡처에만 비레이아웃 장식으로 설명한다. |
| 24 | fixture나 evidence 화면을 Tailnet route에서 찾지 못하는가? | PASS — RSPress `/spinon/` 상대 경로와 README 링크를 쓴다. |
| 25 | plan review의 20개 관점을 PR diff 30개와 합산하는가? | PASS — 두 문서의 책임과 독립성을 명시한다. |
| 26 | implementation/runtime 20관점 검토가 미리 통과한 것처럼 보이는가? | PASS — plan과 roadmap에서 implementation review 전임을 명시한다. |
| 27 | Android/iOS 폰트 동작을 Chrome 결과만으로 단정하는가? | PASS — 각 Simulator에서 같은 hash를 별도로 비교하며 mismatch 시 fail-closed한다. |
| 28 | screenshots를 surface readback이나 전체 CSS 적합성으로 주장하는가? | PASS — visual smoke와 platform/browser 범위를 구분한다. |
| 29 | 공개 API, 내부 숫자 버전 또는 일반 Grid admission을 변경했다고 보이는가? | PASS — API/ABI 추가 없이 `0.1.0`을 유지한다고 적는다. |
| 30 | 계획 PR merge 후 구현 순서와 Tailnet/Agent Browser 정리가 불명확한가? | PASS — 구현·runtime·PR review·merge 후 sync와 browser 종료 관문을 둔다. |

계획 PR diff는 30개의 서로 다른 문서·입력·근거·범위 경계 관점에서 검토했다. 제품 구현 및 runtime 동작은 아직 검증하지 않았다.

## 문서 검증 결과

`git diff --cached --check`는 통과했다. `mise exec -- bun run docs:build`는 저장소 전체 1,490개 문서의 dead link 검사에서 종료됐다. 재실행 로그에는 이번 plan·evidence 경로가 더 이상 포함되지 않았고, 변경하지 않은 기존 evidence 문서도 dead link 항목에 있다. 이를 전체 문서 빌드 통과로 기록하지 않는다.
