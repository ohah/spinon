# C05.1 계획 검토 근거

- **검토 대상:** [`plan/c05-runtime-custom-properties.md`](../../../plan/c05-runtime-custom-properties.md)
- **기준 코드:** `origin/main`의 C04.10 병합 상태, Stylo `0.22.0`, `spinon-runtime` latest-wins CSS worker
- **결론:** 구현 범위는 기존 `style` attribute 경로와 runtime 재계산에 한정한다. `@property`와 incremental dirty-subtree는 분리한다.

| 관점 | 공격 질문·실패 입력 | 판정과 계획 보정 |
|---:|---|---|
| 1 | C05 전체 체크를 하위 기능 하나로 잘못 완료 처리하는가? | C05 부모는 유지하고 C05.1만 분리 완료한다. |
| 2 | author stylesheet가 runtime 요청에 없는 상태에서 `@property`를 지원한다고 선언하는가? | runtime `WorkRequest`에 stylesheet 입력이 없음을 확인해 등록 at-rule을 이번 구현에서 제외한다. |
| 3 | JS CSSOM setter·`getComputedStyle()`가 구현된 것처럼 오해하는가? | 기존 `setAttribute("style", ...)` 경로만 명세한다. |
| 4 | 직접 `var()` 문자열을 재작성해 CSS token·comma·escape 의미를 깨뜨리는가? | Stylo의 pinned parser/cascade에 대체를 맡긴다. |
| 5 | `--Theme`와 `--theme`를 같은 변수로 취급하는가? | case-sensitive 구분값 fixture를 추가한다. |
| 6 | unregistered custom property의 기본 상속이 끊기는가? | 부모 선언을 자식의 `width`로 사용하는 fixture를 둔다. |
| 7 | 빈 문자열 custom value를 missing value로 간주해 fallback을 잘못 실행하는가? | empty token value와 미정의 name을 별도 case로 둔다. |
| 8 | fallback 내부의 쉼표나 함수 괄호를 잘못 분리하는가? | 중첩 `var()` fallback을 oracle case에 둔다. |
| 9 | `--x:var(--x)` self-cycle이 살아남는가? | self-cycle과 fallback을 확인한다. |
| 10 | `--a→--b→--a` cycle에서 한 변수만 무효화되는가? | 상호 cycle 의존성을 독립 case로 둔다. |
| 11 | `var(--missing)`을 선언 파싱 단계에서 잘못 버리거나 fallback으로 복구하는가? | 계산 시점 invalid 선언의 CSS property 기본값을 대조한다. |
| 12 | 대체 토큰이 property 문법에 맞지 않을 때 기존 값이 남는가? | non-inherited margin longhand의 invalid-at-computed-value 결과를 대조한다. |
| 13 | shorthand 안의 `var()`가 shorthand 확장 이전·이후에 잘못 처리되는가? | 기존 지원 shorthand의 variable input을 포함한다. |
| 14 | 같은 `--name` 중복 선언과 `!important`의 승자가 어긋나는가? | inline cascade 결과를 별도로 비교한다. |
| 15 | 허용 로직이 `foo: value` 같은 미지원 일반 속성까지 통과시키는가? | 사용자 지정 속성만 예외로 두고 기존 allowlist를 유지한다. |
| 16 | stylesheet `@property`가 parser diagnostics에서 조용히 누락되는가? | stylesheet 경로 부재를 계획상 미지원으로 명시하고 후속 조건으로 남긴다. |
| 17 | 수정된 상위 variable이 이전 computed child 값에 가려지는가? | HostDocument의 다음 revision을 새 요청으로 제출해 child 결과를 다시 검사한다. |
| 18 | parent 변경이나 detach 뒤 옛 상속 값·NodeId cache가 재사용되는가? | 새 snapshot·Stylo view의 연결 membership과 재부착 결과를 검사한다. |
| 19 | 모든 문서 변경을 dirty subtree 계산이라고 부풀리는가? | 현재 코드의 전체 연결 fragment 재계산을 유지하고 증분 계산 성능을 주장하지 않는다. |
| 20 | 오래된 worker 결과가 최신 request 결과를 덮는가? | 기존 generation/document/render/style/environment 전체 key의 publish gate를 그대로 사용한다. |

### 확인이 필요한 구현 전제

- C04.9/C04.10은 각 request에 `StyloDocumentView`와 `Stylist`를 새로 만들고 결과에 computed string과 NodeId만 보존한다. request 사이에 node/style cache가 보존되지 않는다는 점을 코드 및 제거·이동 테스트로 확인한다.
- Stylo `0.22.0` 소스에는 custom-property substitution과 `CssRule::Property` 처리가 있다. 이번 런타임 입력에는 stylesheet registry가 없어서, source-level 지원만으로 `@property` runtime 지원을 주장할 수 없다.
- runtime validator에서 새 custom declaration을 열더라도 허용 layout/paint output property는 현재 profile에 고정한다. computed snapshot의 일반 CSS 표면을 넓히지 않는다.
- Chromium fixture와 캡처 reference는 서로 다른 파일·도구 해시를 기록하고 새 경로에 생성한다. 기존 C01/C04 oracle 파일은 수정하지 않는다.

위 전제를 코드·고정 Chromium 비교·revision 전이 테스트로 확인한 다음에만 구현 상태를 갱신한다.
