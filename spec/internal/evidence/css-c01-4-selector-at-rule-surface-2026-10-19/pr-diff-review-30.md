# C01.4 PR diff 적대 검토 30개

검토 대상은 이 PR의 계획·inventory·fixture·캡처기·validator·reference·문서·실행 근거 전체다. 계획 검토 20개와 구현 실패 관점 검토 20개는 별도로 유지하며 이 표의 회차에 재사용하지 않는다.

| # | 독립 검토 관점 | 결과 |
|---:|---|---|
| 1 | 새 slice가 상태 대장의 다음 미완료 P0인 C01.4에 한정되는가 | 통과. C01 전체 지원 구현으로 범위를 확장하지 않았다. |
| 2 | 계획의 완료 기준과 실제 산출물이 일치하는가 | 통과. 고정 fixture, versioned inventory, Chromium capture와 provenance JSON이 있다. |
| 3 | 계획이 API 작업이 아닌 oracle 자료 작업임을 분명히 하는가 | 통과. Rust/C API 및 제품 CSS 지원을 산출물에서 제외한다. |
| 4 | 상위 C01 상태를 하위 artifact와 혼동하지 않는가 | 통과. C01 부모 `[ ]`를 유지하고 하위 C01.4만 부분 artifact 완료로 표시했다. |
| 5 | 고정 Chrome의 version, CDP revision, executable hash가 서로 연결되는가 | 통과. JSON, evidence 및 reference ID에 기록한다. |
| 6 | host 환경과 CDP emulation 값이 구분되는가 | 통과. macOS/build/architecture 기록과 viewport/locale/timezone/media emulation을 분리한다. |
| 7 | 실제 관찰값이 선언한 emulation과 다른 경우 실패하는가 | 통과. 캡처기와 snapshot validator에서 관찰값을 fixed profile과 대조한다. |
| 8 | fixture의 문서 순서와 node ID가 고정되는가 | 통과. 15개 요소 모두 안정 ID가 있으며 배열을 exact 비교한다. |
| 9 | universal selector가 head/meta/title를 빠뜨리는 fixture 결함이 재발하지 않는가 | 통과. document element부터 모든 고정 요소에 ID가 있다. |
| 10 | selector 결과가 단순 match/non-match가 아닌 exact ordered IDs인가 | 통과. expected ID 전체와 document order를 비교한다. |
| 11 | pseudo-class 표본이 HTML 구조에 실제 조건을 갖는가 | 통과. `:root`, `:has`, `:lang`, `:checked`, child index와 nested targets가 fixture에 있다. |
| 12 | invalid selector를 빈 결과로 숨기지 않는가 | 통과. 오류 selector 3개를 `SyntaxError` 결과로 분리한다. |
| 13 | forgiving selector list의 오류 회복이 독립 표본인가 | 통과. `:is(.target, :spinon-invalid)` 행을 별도로 고정했다. |
| 14 | pseudo-element를 element match로 잘못 계산하지 않는가 | 통과. `CSSStyleSheet`의 `CSSStyleRule.selectorText` 관찰로 분리한다. |
| 15 | author stylesheet나 외부 요청이 selector oracle에 섞이는가 | 통과. fixture는 CSS/link/image/script resource가 없고 stylesheet/resource count를 검사한다. |
| 16 | at-rule 샘플이 구현이 주장하는 전체 CSS grammar처럼 보이지 않는가 | 통과. inventory와 문서에서 고정 표본·partial 범위라고 명시한다. |
| 17 | unknown at-rule drop과 malformed media retention을 구분하는가 | 통과. 각각 dropped와 retained CSSMediaRule로 기록한다. |
| 18 | CSSRule type 숫자 `0`을 의미상 표준화하는가 | 통과. 관찰값 그대로 저장하고 의미값으로 해석하지 않는다. |
| 19 | nested CSSRule 순서를 정렬해 원본 tree를 바꾸는가 | 통과. 브라우저 CSSOM 배열 순서와 canonical `cssText`를 그대로 직렬화한다. |
| 20 | inventory가 수정돼도 오래된 expected 결과를 재사용할 수 있는가 | 통과. v1 inventory의 고정 SHA-256을 validator와 focused test에 둔다. |
| 21 | unknown key, duplicate ID, unknown node, ordering drift가 검증을 우회하는가 | 통과. exact schema와 stable ID/node ownership 검증 및 negative tests가 있다. |
| 22 | at-rule count/type/constructor/child tree의 차이가 숨겨지는가 | 통과. top-level 기준, recursive schema, observation digest를 검사한다. |
| 23 | 과도한 rule depth/count가 validator에 무제한 재귀를 일으키는가 | 통과. depth 16 및 총 256 rule을 제한한다. |
| 24 | snapshot hash가 일부 관찰 변조를 감지하는가 | 통과. inventory input과 observation 별도 digest를 확인한다. |
| 25 | reference ID가 tool/fixture/inventory/browser provenance를 빠뜨리는가 | 통과. 각 SHA와 환경 요소를 ID 구성에 포함한다. |
| 26 | 같은 reference 경로 재실행이나 경쟁 생성이 기존 JSON을 덮는가 | 통과. 임시 경로 검증 후 no-clobber `link()`로 publish하며 충돌 negative control에서 기존 내용을 보존했다. |
| 27 | 캡처 실패 때 기존 Chromium까지 종료하거나 임시 profile을 남기는가 | 통과. repository guarded session helper의 process-group 소유·종료 확인과 profile cleanup을 사용한다. |
| 28 | 재현 비교가 timestamp 말고 다른 차이를 무시하는가 | 통과. `capturedAtUtc`만 제외하며 binary, tools, environment, observation의 차이는 실패한다. |
| 29 | 코드와 실행 기록이 성공·실패를 과장하는가 | 통과. 집중 검사 24개 통과, 전체 reference suite 16개 C04/C11/C13.3 assertion 실패를 함께 기록하고 clean-main baseline으로 오인하지 않는다. |
| 30 | screenshot, status, docs 링크가 제품/모바일 검증을 과장하거나 stale reference를 가리키는가 | 통과. Agent Browser fixture 화면만 표시한다고 명시하고 앱·simulator 증거로 주장하지 않는다. C01 부모는 미완료이며 C01.4 문서 링크는 최종 reference로 연결된다. |

**판정:** diff에서 추가 수정이 필요한 항목은 발견하지 못했다. 실제 통합 CI 및 GitHub merge gate 결과는 PR 생성 뒤 따로 확인한다.
