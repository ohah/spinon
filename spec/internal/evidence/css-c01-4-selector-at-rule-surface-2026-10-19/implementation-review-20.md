# C01.4 구현 실패 관점 검토 20개

검토 대상은 inventory 검증기, Chromium 캡처기, CSSOM snapshot validator와 실행 근거다. 계획 검토 표를 재사용하지 않고 실제 실패 입력과 고정 실행 경계를 따로 확인했다.

| # | 독립 실패 경로 | 확인 및 판정 |
|---:|---|---|
| 1 | 알 수 없는 inventory 필드가 조용히 통과해 schema 의미를 바꿈 | 정확한 key set을 검사한다. `알 수 없는 최상위 필드를 거부한다` 테스트 통과. |
| 2 | 부분 표본이 `complete`로 바뀌어 제품 지원으로 오인됨 | schema, identity, `partial` 상태와 uncovered 목록을 고정한다. 승격 변조 테스트 통과. |
| 3 | expected 결과를 수정해도 기준 입력으로 채택됨 | v1 inventory의 SHA-256을 validator와 테스트 양쪽에 고정한다. byte 변경 테스트 통과. |
| 4 | 중복되거나 잘못된 selector/at-rule ID가 결과를 덮음 | stable ID 문법과 행 내 고유성을 검사한다. 중복 selector negative test 통과. |
| 5 | fixture node ID가 누락·중복되거나 순서가 바뀜 | 15개 node ID 배열을 inventory와 exact 비교하고 각 ID를 고유하게 제한한다. 캡처기에서도 DOM 실제 순서를 검사한다. |
| 6 | selector가 fixture 밖 또는 중복 node를 반환 | 관찰 배열을 expected node ID 배열과 정확히 비교하고 node ownership·중복·document order를 검사한다. 변조 테스트 통과. |
| 7 | 잘못된 selector를 빈 match로 처리해 syntax failure를 숨김 | query 결과와 exception 결과를 별도 schema로 두고 `SyntaxError`를 exact 비교한다. accepted 변조 테스트 통과. |
| 8 | pseudo-element를 DOM element selector로 검사 | `CSSStyleSheet.replaceSync`의 `CSSStyleRule.selectorText`로만 수집한다. `querySelectorAll` 결과와 혼동하지 않는다. negative test 통과. |
| 9 | fixture가 네트워크 CSS/URL을 끌어와 oracle을 오염 | inventory에서 `@import`와 `url()`을 거부하고 실제 fixture stylesheet/resource count가 모두 0인지 검사한다. |
| 10 | at-rule drop, retained rule, parse exception을 한 결과로 합침 | `dropped`, `retained`, `exception`을 별도 결과로 보존한다. unknown drop과 malformed media retention 변조 테스트 통과. |
| 11 | at-rule parser가 규칙을 추가/삭제했는데 결과 수는 통과 | 고정 행 순서, 결과 종류 및 exact rule count를 검사한다. retained count 변조 테스트 통과. |
| 12 | constructor/type 또는 nested child 수가 달라짐 | top-level constructor/type/child count를 exact 비교한다. 각 nested CSSRule의 정규화된 full tree는 observation digest와 snapshot 재비교에 포함된다. |
| 13 | 깊거나 큰 CSSRule tree가 검증기 자원을 소모 | 깊이 16, 전체 rule 256개 제한을 둔다. 재귀 검증에서 누적 상태를 공유한다. |
| 14 | 새 CSSRule 필드가 schema에 묵시적으로 들어오거나 필수 값이 깨짐 | 각 rule의 key set과 constructor/type/cssText/nullable fields/children 타입을 검증한다. 변조 test 통과. |
| 15 | 선언한 emulation과 실제 Chromium 환경이 다름 | viewport, DPR, locale, Intl locale, timezone, media, UA 실제 관찰을 fixed profile과 대조한다. 환경 변조 테스트 통과. |
| 16 | 다른 Chrome revision 또는 실행 파일이 같은 oracle로 표시됨 | CLI/CDP version과 CDP revision을 고정하고 executable SHA-256을 기록한다. validator와 reference ID가 oracle provenance를 확인한다. |
| 17 | browser가 fixture 변경이나 잘못된 문서를 관찰 | fixture byte hash, 15개 실제 node ID와 문서 순서, 외부 resource 및 author stylesheet 부재를 확인한다. |
| 18 | JSON 일부 관찰값이 바뀌어도 전체 digest가 유지 | selector와 at-rule observation digest를 다시 계산해 비교한다. 내용 변조 테스트 통과. |
| 19 | 캡처 재실행이 기존 결과를 덮거나 시각만 다른 값을 새 결과로 오판 | reference ID는 browser·OS·Node·binary·fixture·inventory·도구 hash에서 계산한다. 기존 경로는 생성하지 않으며 재현 비교는 timestamp만 제외한다. |
| 20 | 캡처 실패 뒤 Chromium/profile이 남거나 동시 실행이 reference를 덮음 | 기존 guarded `chromium-session.mjs`가 process-group 종료 확인 뒤 임시 profile을 정리한다. 새 파일은 pending directory에서 validate한 뒤 `link()`로 게시한다. 임시 경로 negative control에서 기존 target에 `EEXIST`가 나고 원래 내용이 보존됨을 확인했다. |

**판정:** 이 20개 경로에서 C01.4 구현의 미해결 결함은 발견하지 못했다. 고정 capture JSON과 Agent Browser screenshot은 oracle 자료이며 앱 검증으로 계산하지 않았다. 전체 CSS reference suite의 16개 실패는 [실행 근거](./README.md#검증-결과와-한계)에 별도 기록했다.
