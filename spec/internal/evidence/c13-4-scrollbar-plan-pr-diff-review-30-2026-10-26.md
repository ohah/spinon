# C13.4 scrollbar 계획 PR 변경 실패 관점 검토 30개 · 2026-10-26

**대상 변경:** C13.4 첫 계획, 내부 계약 draft 0083, 내부 명세 index와 C13 단계 링크, Chrome 156 사전 관찰 및 계획/PR review 기록
**범위:** 계획·계약·근거 문서만 변경했다. Rust 구현 코드는 포함하지 않는다.

| # | PR 변경에서 확인한 실패 경로 | 검토 결과 |
| --- | --- | --- |
| 1 | C13.4 계획 추가를 공식 완료 수 증가로 오해 | `spec/STATUS.md`와 `72/213`을 변경하지 않았다. |
| 2 | C13 상위가 활성/완료로 바뀐 것처럼 게시 | C13 상위 checkbox를 바꾸지 않았다. |
| 3 | C12.5가 C13.2 기반 없이 가능하다고 오해 | scrollport/offset/clip 선행 관계를 계획 상단에 남겼다. |
| 4 | C13.4 하위 계획이 C13 전체 상태에서 발견되지 않음 | C13 umbrella plan의 단계 링크와 internal README index에서 연결한다. |
| 5 | 계약 번호 충돌 | 부모가 예약한 0083만 사용하고 기존 ID를 덮지 않았다. |
| 6 | 미완성 계획을 지원 계약으로 취급 | 0083 상태를 `0.1.0-draft`, 제품 지원 선언 아님으로 표시한다. |
| 7 | CSS Overflow 원본을 wrong draft로 인용 | 고정 2025-10-07 WD §4.2 anchor를 명시했다. |
| 8 | stable이 scrollbar 자체를 표시한다고 해석 | gutter와 scrollbar visibility가 별개라는 규칙을 기록했다. |
| 9 | overlay scrollbar가 gutter를 소비한다고 계획 | overlay geometry는 0이며 시뮬레이터로 확인하도록 gate를 둔다. |
| 10 | classic UA 폭을 상수로 가정 | used width는 platform/browser 환경 값으로 둔다. |
| 11 | Chrome 156 결과를 repository pinned 155와 합침 | 156 관찰은 버전별 보조 자료이고 pinned 155 결과가 아니라고 제한했다. |
| 12 | `155.0.8059.85`를 `155.0.8059.40`과 동일시 | exact patch mismatch와 설치 framework 목록을 분리해 썼다. |
| 13 | screenshot에서 native bars가 숨겨진데 visible이라 기술 | browser run에 `--hide-scrollbars false`가 있고 screenshot에 tracks가 보인다. |
| 14 | screenshot과 입력 fixture provenance mismatch | structured JSON의 input/screenshot SHA-256을 현재 파일과 대조했다. |
| 15 | scrollport의 outer border-box가 변함 | border-box를 고정하고 client/scrollport 내부만 영향받는다고 명시한다. |
| 16 | `stable both-edges`가 첫 gutter만 예약 | 반대 inline edge에도 같은 폭을 두는 표본을 포함한다. |
| 17 | `overflow:auto` 확장을 간단히 추가해 min-size 회귀 | 현재 Taffy mapping 위험을 기록하고 첫 slice에서 auto를 제외한다. |
| 18 | `overflow:scroll`와 hidden/clip semantics 혼합 | 첫 profile과 no-owner negative control을 분리한다. |
| 19 | Taffy `scrollbar_width`가 모든 overflow에 적용된다고 가정 | `Overflow::Scroll`만 width field를 읽고 Spinon auto는 Hidden으로 매핑됨을 확인했다. |
| 20 | computed `scrollbar-gutter`가 이미 runtime에 전달된다고 가정 | cascade snapshot과 LayoutStyle projection이 아직 이 값을 보존하지 않음을 적었다. |
| 21 | layout client box와 C13.2 runtime scrollport 폭 불일치 | source boundary와 일치성 관문을 명시했다. |
| 22 | gutter 변화와 scroll range가 서로 다른 revision으로 게시 | atomic immutable snapshot 및 scroll revision 조건을 계약했다. |
| 23 | offset이 새로운 작은 range에 맞춰 clamp되지 않음 | 기존 C13.2 preserve/clamp/reset 규칙을 그대로 적용한다. |
| 24 | clip/hit 영역이 예전 scrollport 크기를 사용 | geometry와 함께 clip, paint, hit-test를 검증한다. |
| 25 | root/body overflow propagation에서 gutter를 잘못 적용 | root viewport/body propagation을 별도 후속으로 제외했다. |
| 26 | RTL/vertical writing mode의 edge를 LTR로 추정 | 첫 profile은 LTR이며 edge 판정 불능을 fail-closed 처리한다. |
| 27 | scrollbar width/color, thumb drag가 은근히 완료됨 | 해당 기능은 범위 밖이며 구현 code도 없다. |
| 28 | overlay UI가 안 보여도 custom scrollbar를 만들어 pass | 실제 AVD/iOS 표시 gate를 통과하기 전 구현하지 않는다고 명시했다. |
| 29 | Rust 12/12 기존 테스트를 C13.4 구현 통과로 보고 | 명령은 C13.2 baseline이며 gutter 동작은 시험하지 않았다고 기록했다. |
| 30 | plan-only PR에 implementation review/test/platform proof가 완료된 것처럼 표시 | plan review 20과 PR diff 30만 기록하며 implementation review와 Simulator 검증은 pending이다. |

**결론:** 이 PR의 문서 상태와 근거 경계는 일관된다. 새 runtime behavior를 주장하지 않는다. Android/iOS mode가 overlay로 확인되면 user-visible classic gutter 구현을 추가하지 않고 no-gutter geometry 결론만 남긴다.
