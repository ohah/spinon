# C01.4 계획 적대 검토 20개

**검토 대상:** [`plan/c01-4-selector-at-rule-surface.md`](../../../plan/c01-4-selector-at-rule-surface.md) · 구현 전 검토

| # | 실패 관점 | 계획 반영 |
|---:|---|---|
| 1 | CSS parser가 반환한 rule 존재를 실제 style 적용 지원으로 오해할 수 있음 | 산출물 설명과 미포함 범위에서 parser surface와 Spinon 지원을 분리한다. |
| 2 | selector 하나의 match가 전체 매칭 의미를 대표한다고 과장할 수 있음 | selector 행은 고정 DOM fixture 내 관찰 ID 집합에 한정하고 completeness를 partial로 둔다. |
| 3 | 여러 fixture 노드가 같은 id/selector에 걸려 결과를 구별하지 못할 수 있음 | fixture는 고정 `data-c01-node` 소유 ID를 사용하고 capture 시 등록된 ID만 허용한다. |
| 4 | pseudo-element를 `querySelectorAll` 결과가 비었다는 이유로 거부로 기록할 수 있음 | pseudo-element는 별도 stylesheet parse 관찰로만 저장한다. |
| 5 | `:is()` forgiving parsing 때문에 잘못된 내부 branch를 전체 selector 거부로 오분류할 수 있음 | invalid branch가 섞인 `:is()`는 독립 행으로 관찰하고 결과 분류를 별도로 둔다. |
| 6 | 유효하지 않은 selector를 `false` match와 syntax error로 합칠 수 있음 | exception name과 empty match를 distinct observation state로 기록한다. |
| 7 | pseudo-class 상태·문서 언어·viewport가 host마다 달라 결과가 바뀔 수 있음 | selector 행에서 동적 상태를 제외하고 viewport/media/locale를 고정·재관찰한다. |
| 8 | `@media` 결과가 viewport/emulation 편차로 바뀔 수 있음 | CSS viewport와 media feature 값을 입력 계약에 고정하고 output에 재기록한다. |
| 9 | `@container` 조건의 미충족이 parse 실패로 오해될 수 있음 | 평가 결과 대신 rule constructor/tree를 수집하고 fixture에 container context를 고정한다. |
| 10 | unknown at-rule이 무시된 결과와 syntax exception을 혼동할 수 있음 | rule 없음, retained rule, thrown exception을 각각 다른 상태로 저장한다. |
| 11 | CSSRule numeric type가 버전 간 같은 의미라고 가정할 수 있음 | constructor, numeric type, canonical text를 모두 보존하고 Chromium revision별 snapshot을 분리한다. |
| 12 | `cssText` canonicalization의 공백 차이를 selector 의미 차이로 오판할 수 있음 | source text와 CSSOM `cssText`를 별도 필드로 정확 보존한다. |
| 13 | nested rule 순서를 정렬해 cascade/source order를 훼손할 수 있음 | CSSOM에서 관찰한 rule 순서를 원문 순서대로 기록한다. |
| 14 | `@import`, font-face 등 외부 요청이 로컬/원격 네트워크를 발생시킬 수 있음 | fixture는 외부 자원을 두지 않고 at-rule samples는 외부 네트워크를 호출하지 않는 입력만 쓴다. |
| 15 | 실행 환경이 지정한 Chrome 155 또는 CDP revision이 아니어도 산출물을 생성할 수 있음 | CLI/CDP version과 CDP가 실제 반환하는 commit-only revision을 각각 고정하고 reference identity에 넣는다. upstream branch ref는 별도 provenance로 적는다. |
| 16 | Agent Browser screenshot과 자동 CDP capture가 서로 다른 Chrome build 또는 locale/time zone/media를 쓸 수 있음 | version/revision이 다르면 확정하지 않고, 고정 환경을 재현하지 못한 screenshot·관찰값은 시각 확인에만 사용한다. |
| 17 | 임시 Chromium 프로세스를 이름으로 종료해 기존 브라우저를 닫을 수 있음 | 기존 harness의 소유 process group/profile 확인·bounded cleanup을 재사용한다. |
| 18 | 기존 reference JSON을 새 실행 결과로 덮어써 과거 근거가 바뀔 수 있음 | 새 경로만 생성하고 directory 충돌은 실패시키며 원자 저장한다. |
| 19 | schema에 지원 상태 같은 필드를 덧붙여 partial oracle을 제품 약속으로 바꿀 수 있음 | exact-key schema와 `completeness: partial`, 고정 `uncovered` 목록을 강제한다. |
| 20 | 작은 selector/rule 표본을 C01 전체 또는 Chromium CSS 적합성으로 보고할 수 있음 | 상태 대장 부모 C01은 미완료로 유지하고 fixture screenshot도 oracle 표시로만 명시한다. |

## 검토 후 보완

- Rule matching, computed value, geometry, paint를 이번 산출물에서 제외해 selector/rule parser 관찰을 제품 스타일 동작과 혼동하지 않도록 했다.
- Agent Browser의 시각 확인과 자동 capture의 브라우저가 같은 version/revision인지 확인하지 못하면 결과를 확정하지 않도록 했다.
- Agent Browser의 host locale/time zone/media가 계획 조건과 다를 수 있으므로 screenshot 관찰을 fixed-environment JSON과 분리하도록 했다.
- 시뮬레이터 검증은 앱 변경이 없는 기준 수집에 해당하지 않아 계획의 검증 범위에서 제외했다.
- 실제 CDP revision 문자열은 upstream branch suffix 없이 commit hash만 반환해 고정값을 그 관찰 형식으로 보정했다.
