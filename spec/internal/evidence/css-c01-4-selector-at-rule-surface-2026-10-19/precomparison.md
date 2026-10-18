# C01.4 사전 비교 · Agent Browser 관찰

**대상:** `selector-at-rule-surface.html` · 구현 도구 작성 전 관찰
**시각:** 2026-10-19 05:31 KST · **browser:** Google Chrome / HeadlessChrome `155.0.8059.85`
**용도:** 비교 입력과 판정 형태 확인. 아래 관찰은 fixed-environment JSON이 아니며 최종 reference로 사용하지 않는다.

## 환경과 원본

| 항목 | 관찰값 |
|---|---|
| Agent Browser | `0.31.1`, session `spinon-c01-4-oracle-20261019` |
| Chromium version | `155.0.8059.85` (`navigator.userAgentData.fullVersionList`) |
| viewport / DPR | `800×600` / `1` |
| host locale / Intl locale | `ko-KR` / `ko` |
| host time zone | `Asia/Seoul` |
| initial color scheme | `dark`; screenshot 전에 `light`으로 전환 |
| reduced motion / forced colors | `no-preference` / `none` |
| fixture SHA-256 | `4783ff01e66382874c74027543dcd659eb2af23bc85a04d928330f190b03b435` |
| inventory SHA-256 (initial probe input, before expected fields) | `a1ee3528349cccabf79f41c9543c135e6d5e64f15a6863c48a862aef205c2801` |
| inventory SHA-256 (fixed expected observations) | `970e9f0aab8134b603e6e4f4c790ad2c4b59709beb7eee927b276fa728ece7f8` |
| screenshot SHA-256 | `1bdd0a93ae2bb5b265bfb5568577a731ef427ffe054cd55bbc7bc2f715a4e9cc` |
| resource requests | 없음 |

![Agent Browser의 CSS oracle fixture 화면](./agent-browser-fixture.png)

## 사전 관찰

19개 일반 selector query가 실행되었고 고정 `data-c01-node` ID를 반환했다. 세 개의 문법 오류 후보(`[data-state=`, `main > > section`, `:spinon-invalid`)는 모두 `SyntaxError`였다. forgiving list인 `:is(.target, :spinon-invalid)`는 유효 branch를 유지해 `target`, `nested-target`, `sibling`을 반환했다. `section::before`는 `querySelectorAll` 결과가 아니라 `CSSStyleRule`의 `selectorText`로 관찰했다.

| selector ID | 반환 node ID 순서 |
|---|---|
| `selector.type.section` | `target` |
| `selector.class.target` | `target`, `nested-target`, `sibling` |
| `selector.id.target` | `target` |
| `selector.universal` | `document-root`, `head`, `meta-charset`, `meta-viewport`, `title`, `body`, `main`, `heading`, `target`, `nested-target`, `nested-fallback`, `sibling`, `disabled`, `checked-label`, `checked-input` |
| `selector.attribute.presence` | `target`, `sibling`, `disabled` |
| `selector.attribute.exact` | `target` |
| `selector.combinator.descendant` | `target`, `nested-target`, `sibling` |
| `selector.combinator.child` | `target`, `sibling` |
| `selector.combinator.adjacent` | `target` |
| `selector.combinator.general-sibling` | `target`, `sibling` |
| `selector.pseudo.root` | `document-root` |
| `selector.pseudo.nth-child` | `target` |
| `selector.pseudo.not` | `target` |
| `selector.pseudo.is` | `target`, `nested-fallback` |
| `selector.pseudo.where` | `target`, `nested-target`, `nested-fallback`, `sibling` |
| `selector.pseudo.has` | `main` |
| `selector.pseudo.lang` | `nested-target` |
| `selector.pseudo.checked` | `checked-input` |
| `selector.pseudo.is-forgiving-list` | `target`, `nested-target`, `sibling` |

11개 정상 at-rule 후보는 각각 한 개의 CSSOM rule을 유지했다. constructor/type 표본은 `CSSMediaRule/4`, `CSSSupportsRule/12`, `CSSLayerBlockRule/0`, `CSSContainerRule/0`, `CSSScopeRule/0`, `CSSStartingStyleRule/0`, `CSSKeyframesRule/7`, `CSSPropertyRule/0`, `CSSFontFaceRule/5`, `CSSPageRule/6`, `CSSCounterStyleRule/11`이었다. 새 rule 계열의 numeric type `0`은 이 Chromium snapshot 관찰값이며 고정 schema가 이를 공통 의미값으로 해석하지 않는다.

`@c01-unknown`은 예외 없이 0개 rule로 반환됐다. `@media (width > )`는 예외 없이 `CSSMediaRule` 한 개를 유지했다. 이번 관찰은 media condition의 평가나 rule 적용 여부를 확인하지 않았다.

## 고정 기준과 분리한 이유

Agent Browser는 지정된 Chrome version을 사용했지만 host locale, time zone, 초기 color scheme이 계획의 `en-US`/`UTC`/light와 달랐다. 따라서 이 실행은 fixture rendering과 selector/rule 관찰 형태를 확인하는 사전 비교에 한정한다. 다음 단계의 capture harness가 동일 Chrome revision에서 viewport, locale, time zone, media를 고정하고 최종 immutable reference를 생성한다. 이 screenshot은 Spinon app, simulator, 제품 CSS 동작의 증거가 아니다.

fixture node ID를 모든 요소로 확장한 뒤 Agent Browser에서 selector 결과와 screenshot을 다시 확인했다. 화면 파일의 SHA-256은 `1bdd0a93ae2bb5b265bfb5568577a731ef427ffe054cd55bbc7bc2f715a4e9cc`로 동일했다. 브라우저 종료 뒤 session 목록은 `No active sessions`였고 브라우저 PID는 존재하지 않았다. 최초 실행 전 Agent Browser session 목록은 비어 있었다.
