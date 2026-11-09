# C11.4 PR #1302 병합 후 상태·로드맵 diff 검토 30개

검토 대상은 PR #1302 병합 뒤 수정한 `spec/STATUS.md`, 내부 계약 `0059`, 집중 계획, 공개 `roadmap.html`이다. 각 관점은 별도의 상태 오기·경계 확대·링크·표시 실패 가능성을 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 PR 번호와 merge SHA를 구현 PR과 혼동한다. | PASS — #1301과 #1302를 분리해 기록했다. |
| 2 | 구현 PR의 merge SHA를 head commit SHA로 잘못 쓴다. | PASS — `3c403dc265933407dcd34154012ceeec871d71c8`를 GitHub merge SHA와 대조했다. |
| 3 | merge timestamp를 로컬 시간과 UTC 사이에서 바꿔 적는다. | PASS — GitHub UTC timestamp `2026-11-09T03:03:35Z`를 사용했다. |
| 4 | C11.4 공식 체크박스를 제한 slice 병합만으로 완료 처리한다. | PASS — checkbox는 계속 미체크다. |
| 5 | 상위 C11 체크를 C11.4 구현과 함께 완료 처리한다. | PASS — 상위 checkbox도 변경하지 않았다. |
| 6 | 공식 완료 수에 bounded slice를 더한다. | PASS — `78/214`를 보존했다. |
| 7 | 내부 숫자 계약 버전을 구현 범위만으로 올린다. | PASS — `0.1.0`을 유지했다. |
| 8 | 새 공개 API나 FFI가 생긴 것처럼 쓴다. | PASS — 기존 제한 profile의 확장으로 기록했다. |
| 9 | `fit-content` 하나만 허용한다는 multiplicity 경계가 빠진다. | PASS — 단일 항목 조건을 문서화했다. |
| 10 | px 또는 percentage 외의 cap을 지원한다고 과장한다. | PASS — 유한·비음수 `px|%` 범위를 적었다. |
| 11 | 위치 1–6 authored 순서 보존이 빠진다. | PASS — STATUS·0059·roadmap에 범위를 명시했다. |
| 12 | 나머지 track multiplicity가 다른데도 같은 profile로 보인다. | PASS — `min-content ×2`, `max-content ×2`, `auto ×1`을 함께 기록했다. |
| 13 | text fixture topology 밖 임의 child를 허용하는 것처럼 쓴다. | PASS — direct-text 제한 profile과 고정 item 범위를 명시했다. |
| 14 | definite block-size basis 조건이 사라진다. | PASS — `40×220px` Grid 경계를 0059에 적었다. |
| 15 | row-span profile 범위까지 위치 1–6으로 확장한 것처럼 보인다. | PASS — row-span trace/profile 경계를 별도로 유지한다고 기록했다. |
| 16 | span 위치 4–6 rejection 경계를 새 auto-placement 지원과 혼동한다. | PASS — 두 경로를 별도 회귀/계약으로 구분했다. |
| 17 | 1,440-scene matrix의 차원을 빠뜨린다. | PASS — position/order/cap/alignment 결과를 evidence에 연결했다. |
| 18 | fixture-trace variant를 기본 route 결과와 동일 실행으로 합친다. | PASS — 구현 근거는 두 matrix를 별도 표기한다. |
| 19 | 기존 row-span 1,920-scene 회귀가 미실행인 것처럼 표시한다. | PASS — 수정 뒤 전체 통과한 근거를 연결했다. |
| 20 | scene 292에서 찾은 회귀와 수정 내역을 감춘다. | PASS — 실행 근거의 발견·수정 문서로 연결했다. |
| 21 | Android/iOS 화면 smoke를 pixel readback으로 부른다. | PASS — PixelCopy/Metal readback이 아니라고 명시했다. |
| 22 | simulator sentinel을 전체 native geometry parity로 과장한다. | PASS — 8개 scene smoke와 native matrix 부재를 분리했다. |
| 23 | 비어 있는 GitHub check rollup을 CI 성공으로 주장한다. | PASS — CI green으로 기록하지 않았다. |
| 24 | 기존 position 1–3 row-span 검증 경로가 사라진다. | PASS — 별도 trace 경로의 회귀 확인을 적었다. |
| 25 | public roadmap에서 구현 전의 `위치 2–6` NEXT 카드가 남는다. | PASS — 해당 `detailNext` 항목을 제거했다. |
| 26 | 다음 범위가 정해지지 않았는데 구현 계획이 있는 것처럼 표시한다. | PASS — 다음 bounded 후보 미선정으로 표시했다. |
| 27 | 완료된 PR #1302가 roadmap 최신 기록에서 빠진다. | PASS — 최신 C11.4 progress 카드에 merge 정보를 추가했다. |
| 28 | roadmap의 PR/계획/명세/근거 링크가 Tailnet에 없는 로컬 route를 가리킨다. | PASS — 새 카드의 세부 문서는 GitHub main 링크를 사용한다. |
| 29 | 과거 roadmap history를 새 카드 삽입 중 덮어쓴다. | PASS — PR #1299 다음에 새 항목만 삽입했다. |
| 30 | status·plan·internal spec·roadmap 중 하나만 갱신돼 상충한다. | PASS — 네 파일에 같은 merge SHA, 범위, 미완료 상태를 반영했다. |

이 검토는 문서 diff의 상태·범위·링크 오류를 다룬다. Tailnet 제공본의 source/served hash와 화면은 병합 후 별도 확인한다.
