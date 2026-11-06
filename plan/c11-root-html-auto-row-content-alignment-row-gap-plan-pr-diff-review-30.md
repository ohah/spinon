# C11.4 implicit auto-row alignment + row-gap 계획 PR diff 검토 30개

계획 PR diff에 대한 검토다. 이는 계획 실패 관점 20개 검토 및 추후 구현/runtime review와 별개다.

| # | Diff에서 확인한 실패 경로 / 계약 경계 | 결과 |
|---:|---|---|
| 1 | Plan title과 C11.4 parent link가 정확한가 | 통과: C11.4 intrinsic track sizing을 가리킨다. |
| 2 | 공식 `STATUS.md` anchor가 유효한가 | 통과: `#c11-css-grid`를 사용한다. |
| 3 | Plan 상태가 구현 전 후보로 표시되는가 | 통과: implementation complete를 주장하지 않는다. |
| 4 | 선행 PR #1237의 merge state를 새 작업과 혼동하는가 | 통과: 기존 zero-gap slice를 선행 근거로만 쓴다. |
| 5 | Public CSS support와 fixed runtime profile을 같은 것으로 표현하는가 | 통과: 일반 author CSS와 WPT를 명시적으로 제외했다. |
| 6 | Contract version이 계획만으로 올라가는가 | 통과: `0.1.0` 고정을 규정한다. |
| 7 | Normative Grid gutter source가 연결됐는가 | 통과: 2025-03-26 Grid CRD §10.1을 직접 링크한다. |
| 8 | Positional/distributed/fallback alignment source가 연결됐는가 | 통과: CSS Box Alignment §§4.3–4.4와 §5.1.4를 연결한다. |
| 9 | Fixture route가 구현 소스와 같은 topology인가 | 통과: explicit row 1개, auto rows 2개, direct item 3개다. |
| 10 | Fixture source hash가 기대값과 일치하는가 | 통과: SHA-256 `12f0cdd1…979eeb99`를 기록한다. |
| 11 | Chrome user agent/version/viewport/DPR를 분리 기록하는가 | 통과: observation metadata에 각각 있다. |
| 12 | Expected arithmetic이 browser output에서 복사되지 않았는가 | 통과: `40 - 30 - 2g` independent formula를 둔다. |
| 13 | 모든 gap 값과 alignment 조합이 관측되는가 | 통과: 4×10, 총 40 scenes다. |
| 14 | Subpixel `space-around` tolerance가 실제 측정 오차보다 작은가 | 통과: max 0.026042px, tolerance 0.03125px다. |
| 15 | Gap-zero control이 들어 있는가 | 통과: 기존 #1237 reference와 동일한 10 values가 보존된다. |
| 16 | Zero free-space 경계가 들어 있는가 | 통과: row-gap 5px에서 expected free space 0이다. |
| 17 | Negative free-space 경계가 들어 있는가 | 통과: row-gap 7.5px에서 expected free space -5다. |
| 18 | 실제 out-of-root frames가 expected에 적혔는가 | 통과: `end` -5px 및 `center` -2.5px y를 기록한다. |
| 19 | Distributed negative fallback이 positional unsafe와 구분되는가 | 통과: `space-*` start fallback을 별도 row로 기록한다. |
| 20 | Chrome screenshot SHA/dimensions, PR body image loading 및 browser close 상태가 evidence와 맞는가 | 통과: 1280×633 이미지 3장 모두 PR #1239에서 load됐고 browser process가 남지 않았다. |
| 21 | Chrome screenshot을 Spinon UI로 오해할 수 있는가 | 통과: reference-only 한계를 README와 plan에서 명시한다. |
| 22 | Verifier가 40개 scene 수와 frame extents를 검사하는가 | 통과: Node verifier가 count·track·position·item size를 비교한다. |
| 23 | 계획이 fixture/gap 범위를 일반 CSS 값으로 넓히는가 | 통과: 7.5px 초과·percentage·math를 제외한다. |
| 24 | Column axis가 row-only profile에서 열리는가 | 통과: column-gap 0과 justify-content normal을 요구한다. |
| 25 | `place-content`가 inline content alignment를 우회하는가 | 통과: computed longhand 각각을 검사한다. |
| 26 | Snapshot owner/revision 및 failure atomicity가 빠졌는가 | 통과: 그대로 필수 gate다. |
| 27 | Android/iOS 검증이 실제 app route를 요구하는가 | 통과: 같은 fixture/source hash의 AVD 및 Simulator를 요구한다. |
| 28 | 밖으로 나온 box가 단순 layout log만으로 확인되는가 | 통과: screenshot과 가능한 PixelCopy/Metal readback을 별도 요구한다. |
| 29 | Base/candidate의 broad failure가 green으로 합쳐지는가 | 통과: 동일 실패와 새 실패를 분리 기록하도록 했다. |
| 30 | Official checkbox/count가 제한 slice만으로 승격되는가 | 통과: C11.4/C11 및 `78/214` 유지 조건을 명시한다. |

## 결론

계획 변경은 고정 Root HTML auto-row profile에 row-gap 범위와 positive/zero/negative free-space 관문을 추가한다. Browser source, independent expected values 및 visual reference를 구현 전 evidence로 함께 보존한다. 구현 코드는 이 PR에서 다루지 않는다.
