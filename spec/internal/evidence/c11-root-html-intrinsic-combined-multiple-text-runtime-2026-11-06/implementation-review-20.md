# C11.4 구현·런타임 실패 관점 검토 · 2026-11-06

계획 검토와 분리해 후보 코드, 기본 RuntimeSession, Android/iOS simulator 결과를 살폈다. 이 결과는 intrinsic columns + multiple direct text rows 조합에만 한정한다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | `grid-auto-rows`가 singleton `min-content`가 아니어도 새 경로가 선택된다 | 후보 selector가 정확히 singleton `TrackList([MinContent])`를 요구하고, max-content 및 다중 track 목록 음성 회귀가 있다. **통과.** |
| 2 | 고정 explicit tracks의 기존 여러 text 경로가 달라진다 | intrinsic explicit track이 없는 경우 기존 후보 조건과 같은 결과를 반환한다. 기존 고정 column 여러 text 회귀가 통과했다. **통과.** |
| 3 | intrinsic row도 combined intrinsic-column 경로로 잘못 허용된다 | combined 후보는 intrinsic explicit columns가 있어야 하며 explicit rows는 단일 fixed px여야 한다. 런타임 intrinsic-row 음성 입력은 layout을 거부한다. **통과.** |
| 4 | explicit row의 `0px` 또는 음수·비유한 값이 implicit row 조합에 들어간다 | 후보 함수는 단일 유한 양수 px만 수락한다. `0px`와 여러 explicit rows 단위 음성 케이스를 확인했다. **통과.** |
| 5 | explicit row가 둘 이상인데도 한 행 계약으로 처리된다 | computed rows slice가 단일 원소인지 검사하고 2개 row를 거부한다. **통과.** |
| 6 | `fit-content()`가 intrinsic track 확장으로 유입된다 | combined 후보가 양 축의 fit-content track을 거부한다. runtime fit-content 입력은 partial output 없이 실패한다. **통과.** |
| 7 | auto-repeat이 combined route 안으로 들어온다 | 양 축 auto-repeat을 helper에서 거부한다. 현재 CSS computed profile은 auto-repeat declaration 자체를 cascade에서 실패시키며 runtime에 완성 layout을 내지 않는다. **통과, parser 단계 거부.** |
| 8 | `grid-template-areas`가 implicit-row owner와 섞인다 | candidate predicate와 runtime negative 모두 named area를 거부한다. **통과.** |
| 9 | row flow에서 implicit column sizing이 silently 적용된다 | candidate predicate는 `grid-auto-columns:auto`만 허용한다. runtime `grid-auto-columns:8px` negative가 atomic failure를 확인한다. **통과.** |
| 10 | dense 또는 column auto-flow에서 순서가 달라진 결과를 내놓는다 | combined candidate는 sparse row flow로 제한된다. 기존 C11 intrinsic track 음성 회귀와 새 helper matrix를 확인했다. **통과.** |
| 11 | 여러 Grid owner 중 일부만 검증되고 나머지가 publish된다 | 공통 validator는 visible Grid owner 하나만 허용한다. 기존 multiple-owner rejection과 신규 단일 owner default-session 경로가 있다. **통과.** |
| 12 | anonymous item, span, hidden node 또는 nested markup이 text item으로 간주된다 | 공통 validator가 visible direct HTML div와 기존 bounded topology만 허용한다. 익명·비div·nested/다중 child 기존 negative 회귀가 있다. **통과.** |
| 13 | 임의 문자, 탭, 여러 공백 또는 지원하지 않는 source를 토큰으로 읽는다 | 텍스트 source parser의 ASCII `A`와 단일 공백 문법은 그대로 유지되고, invalid second text source runtime negative가 통과한다. **통과.** |
| 14 | 텍스트 item의 width/height, font, wrapping 또는 positioning 범위가 넓어진다 | 공통 validator가 기존 auto sizing, monospace 10px, line-height 10px, normal wrapping, LTR/static/auto placement 계약을 계속 검사한다. nowrap negative가 atomic failure다. **통과.** |
| 15 | 두 text item의 min-content 기여가 합쳐지거나 다른 item에 귀속된다 | layout trace에서 두 NodeId의 MinContent 기여를 각각 검사하며 값과 source identity를 보존한다. **통과.** |
| 16 | explicit column 해석 전 text wrap 폭을 고정 추정치로 사용한다 | min-content/max-content/auto/minmax 대표값에서 used column trace와 행 sequence를 검사한다. 각 시나리오의 line count와 frames가 일치한다. **통과.** |
| 17 | text DOM 순서를 뒤집으면 기여와 frame 순서가 분리된다 | 네 column mode 모두 forward/reverse 순서를 검사하고, 서로 다른 행 높이와 item identity가 함께 움직인다. **통과.** |
| 18 | trace 예산 또는 10,000-track ceiling 실패가 이미 계산한 일부 frame을 반환한다 | combined intrinsic-column fixture가 10,000 tracks에서 마지막 frame을 보존하고 optional trace만 생략한다. 10,001번째는 계산 전 거부된다. **통과.** |
| 19 | runtime paint가 text owner/revision/order를 잃거나 Grid 밖 overflow를 자른다 | 두 `RuntimeText`의 generation/document/render-tree/style revision, background→text→next box draw order와 40px Grid 아래 마지막 item frame을 검사한다. Android/iOS 화면 smoke에서도 overflow가 보인다. **통과.** |
| 20 | simulator·fixture 결과를 제품 전체 호환성이나 성공한 전체 테스트로 과장한다 | 기본 Root HTML route와 fixture flag 0을 확인했다. screenshot은 visual smoke로만 기록했다. 전체 crate suite 실패는 원래 base commit에서도 재현했고 C11/C11.4 완료 주장은 없다. **통과, 남은 범위는 README에 기록.** |

근거: [simulator/runtime 요약](README.md), [Android log](android/app.log), [iOS log](ios/app.log), [Rust focused test 결과](README.md). 전체 crate 실패와 base commit 대조 결과는 README에 구분했다.
