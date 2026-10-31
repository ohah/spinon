# C11.4 anonymous Grid text `text-indent` 계획 실패 관점 검토 20개

대상: [`c11-4-anonymous-grid-text-computed-text-indent.md`](./c11-4-anonymous-grid-text-computed-text-indent.md). 기준 oracle: [Chrome 156/Ahem 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-precomparison-2026-11-01/README.md). 구현 코드는 아직 검토 대상이 아니다.

| # | 별도 실패 관점 | 판정 |
|---:|---|---|
| 1 | 제한 fixture slice가 C11.4 또는 C11 전체 완료로 표시되는가 | 통과. 공식 체크와 72/213을 유지한다. |
| 2 | property 적용 대상을 block container가 아닌 root text node로 오인하는가 | 통과. Grid root의 computed 값을 anonymous direct-text item이 상속하는 경계를 고정한다. |
| 3 | inherited computed value와 원래 HostDocument revision이 분리되는가 | 통과. 동일 snapshot의 property와 revision 검사를 요구한다. |
| 4 | 일반 CSS author input까지 허용하는가 | 통과. opt-in fixture, exact scene allowlist만 허용한다. |
| 5 | `0px`, `5px`, `10px` 외의 computed px 값이 조용히 통과하는가 | 통과. allowlist 밖 값은 publication 전 거부한다. |
| 6 | 유효한 음수 `text-indent`가 모든 min-content fragment에 단순 더해지는가 | 통과. 음수는 scope 밖이며 계획은 복잡한 negative-advance 경계를 명시한다. |
| 7 | percentage가 definite px처럼 계산되는가 | 통과. percentage는 intrinsic basis 의존 입력으로 제외한다. |
| 8 | `hanging` 또는 `each-line`이 첫 줄만 indent하는 것으로 취급되는가 | 통과. 두 키워드를 allowlist 밖으로 두고 관측값도 분리했다. |
| 9 | computed value가 같은 authored expression의 출처까지 판정한다고 주장하는가 | 통과. author syntax origin은 이 profile의 계약이 아니다. |
| 10 | min-content에서 첫 단어와 다음 단어의 경쟁을 놓치는가 | 통과. 일반 식은 첫 fragment와 후속 unbreakable word 최대를 비교한다. |
| 11 | max-content에서 여러 단어의 separator advance를 누락하는가 | 통과. Ahem separator와 collapse 후 separator 수를 명시한다. |
| 12 | 반복 공백과 후행 공백이 별도 글리프로 남아 있는가 | 통과. normal collapse 후 세 단어와 두 separator를 기대한다. |
| 13 | separator 없는 한 단어가 두 word spacing 결과를 구별하지 못하는가 | 통과. `XX`를 min/max 양쪽 control로 포함한다. |
| 14 | font metrics/크기/DPR 변경으로 기대값이 달라지는가 | 통과. Ahem 10px 고정 및 font SHA-256을 oracle에 기록한다. |
| 15 | Chrome observed value를 독립 expected로 복사하는가 | 통과. font table과 규범식으로 기대값을 별도 산출한다. |
| 16 | Chrome observation을 Spinon runtime 적합성으로 확대하는가 | 통과. 사전 비교 한계와 브라우저 전용 결과를 분리한다. |
| 17 | Parley indent 설정이 line breaking 이후 적용되는가 | 통과. `set_text_indent`를 line breaking 전에 호출하는 순서를 계획했다. |
| 18 | 기존 text layout의 기본 입력이 비의도적으로 바뀌는가 | 통과. 새 field 기본값 0과 기존 default-profile 회귀를 요구한다. |
| 19 | default artifact에서 opt-in fixture 코드가 빠졌는지 확인하지 않는가 | 통과. 기본 archive negative와 fixture positive control을 별도 요구한다. |
| 20 | simulator trace/screenshot을 glyph pixels·실기기·GPU 증거라고 확대하는가 | 통과. AVD software, iOS compositor 전, 실제 paint 미검증을 구분한다. |

## 결과

20개 실패 경계를 별도로 검토했다. 계획에서 구현 시작 전 보완한 내용은 min-content formula의 한 단어 입력을 위해 후속 단어 집합의 빈 최댓값 기본값을 0으로 정의한 것이다. 남은 설계 경계는 계획에 반영되어 있다. 구현은 계획 PR 병합 이후에 시작한다.
