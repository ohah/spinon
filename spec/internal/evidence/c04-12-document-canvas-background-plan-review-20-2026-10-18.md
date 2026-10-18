# C04.12 문서 Canvas 배경 계획 적대 검토

**대상:** [`plan/c04-12-document-canvas-background.md`](../../../../plan/c04-12-document-canvas-background.md)

**기록:** 초기 계획 20회 + Chrome oracle 반영 뒤 독립 재검토 20회 · 2026-10-18

**판정:** oracle 결과와 분리된 범위를 반영한 뒤 계획 통과. 구현 전에 oracle 값과 해시를 고정했다.

## 초기 계획 검토 20회

| # | 독립 실패 관점 | 공격 질문과 발견 | 반영 / 판정 |
|---:|---|---|---|
| 1 | CSS canvas precedence | `html`과 `body`가 서로 다른 불투명 색이면 어느 색인가? | html 우선 선택을 계획했다. oracle에서 빈 body로 확인한다. |
| 2 | body propagation | root가 투명할 때 body 색이 viewport canvas 전체로 전파되는가? | body propagation을 screenshot pixel 표본으로 고정한다. |
| 3 | transparent fallback | 투명 값이 흰색으로 정규화되어 fallback을 가리지 않는가? | typed transparent/opaque 분류와 light/dark 행을 둔다. |
| 4 | scheme fallback | OS가 dark로 바뀌면 author background 위에 native dark가 덮이지 않는가? | opaque author 색 유지, 투명 시에만 scheme fallback으로 판정한다. |
| 5 | selector identity | `body`가 아니라 첫 자식이나 head의 색을 잘못 선택하지 않는가? | HostDocument의 HTML namespace node ID를 구분하고 head negative를 둔다. |
| 6 | parser repair | 생략된 태그·대문자 태그가 body 식별을 깨뜨리지 않는가? | HTML5 tree construction의 연결된 local name을 사용한다. |
| 7 | missing body | body가 없을 때 panic이나 임의 색 선택이 생기지 않는가? | body 부재는 scheme fallback으로 정의한다. |
| 8 | duplicate body | malformed 입력에서 복구된 body 색을 임의로 합치지 않는가? | parser가 만든 연결 body 하나만 사용하고 recovery 결과를 기록한다. |
| 9 | stylesheet source order | inline style과 `<style>`이 DOM/cascade 순서를 무시하지 않는가? | raw CSS를 재해석하지 않고 Stylo 결과만 소비한다. |
| 10 | custom property | `var()` computed color가 raw declaration로 오인되지 않는가? | typed computed snapshot과 C05.1 경계를 유지한다. |
| 11 | media invalidation | viewport/scheme 변경 뒤 이전 색이 남지 않는가? | 현재 revision key equality와 media fixture를 요구한다. |
| 12 | partial alpha | 부분 alpha body를 불투명으로 취급하거나 잘못 합성하지 않는가? | Chrome 관찰은 남기되 구현 지원 성공으로 반환하지 않는다. |
| 13 | wide gamut | P3/OKLCH를 임의로 sRGB byte 변환하지 않는가? | 이번 범위의 변환 결과를 고정 Chrome fixture로 제한하고 unsupported를 실패시킨다. |
| 14 | image interaction | `background-image`를 무시하면서 단색만 표시하지 않는가? | profile 밖 property는 기존 방식대로 전체 cascade를 실패시킨다. |
| 15 | failure atomicity | cascade 오류 뒤 새 fallback 색이 성공으로 게시되는가? | 실패/stale은 native 이전 색을 유지한다. |
| 16 | asynchronous stale result | 이전 document/style revision의 색이 새 화면에 적용되는가? | 세 revision과 cascade key를 대조한다. |
| 17 | platform surface | safe area 때문에 canvas color가 잘못된 view에 적용되는가? | content root만 바꾸며 system UI·inset 동등성은 주장하지 않는다. |
| 18 | C ABI layout | 새 field 추가로 기존 caller의 struct 해석이 깨지는가? | ABI 구조체 모양은 유지하고 RGBA 의미만 명세한다. |
| 19 | legacy session | JavaScript-only session이 root HTML 전용 색 처리 영향을 받는가? | 기존 `RuntimeSession::new()` control을 유지한다. |
| 20 | evidence overclaim | native 배경 하나를 HTML paint 완료로 오인하는가? | C04.12는 unchecked이며 body box/layout/text 근거와 분리한다. |

## Chrome oracle 반영 후 최종 계획 재검토 20회

| # | 독립 실패 관점 | 검증 결과 | 최종 계획 반영 / 판정 |
|---:|---|---|---|
| 1 | 실제 browser build | Agent Browser UA 전체 버전이 `155.0.8059.85`로 확인됐다. | 계획의 pinned oracle과 일치한다. |
| 2 | CSS viewport/device scale | viewport `390×844`, DPR 3, screenshot `1170×2532`였다. | CSS 값과 pixel sample 좌표를 별도 기록한다. |
| 3 | computed style와 canvas 분리 | root/body computed background만으로 propagated canvas pixel을 알 수 없다. | computed 값과 screenshot pixel을 다른 observation으로 기록한다. |
| 4 | 빈 body isolation | 최초 `min-height:100vh` body가 viewport를 채워 html/body 색을 섞었다. | min-height를 제거하고 fixture를 빈 body로 고쳤다. |
| 5 | root opaque sample | html `#123456`, body `#abcdef`, 빈 body에서 canvas sample은 `#123456`이었다. | canvas 선택값으로 반영했다. |
| 6 | body propagation sample | html transparent, body `#1e8c46`에서 canvas sample은 `#1e8c46`이었다. | body color propagation 행을 통과시켰다. |
| 7 | 둘 다 opaque sample | 빈 body에서는 html `#123456`이 canvas에 남았다. | 둘 다 opaque인 canvas 행의 root 선택을 고정했다. |
| 8 | light fallback | 양쪽 transparent + light preference의 sample은 `#ffffff`였다. | 기존 light fallback을 유지한다. |
| 9 | dark fallback | 양쪽 transparent + dark preference의 sample은 `#121212`였다. | 기존 dark fallback을 유지한다. |
| 10 | media light | body media style computed RGB와 pixel 모두 `(210,220,230)`이었다. | 환경 cascade가 선택한 computed 값을 그대로 읽는다. |
| 11 | media dark | body media style computed RGB와 pixel 모두 `(12,34,56)`이었다. | OS preference에 따라 재계산하되 root/body 선택을 유지한다. |
| 12 | alpha compositing | 반투명 body의 Chrome pixel은 `(227,177,152)`였다. | 부분 alpha는 관찰만 기록하고 범위에서 제외한다. |
| 13 | color-scheme scope | transparent fallback만 OS preference에 따라 바뀌고 명시 색이 우선했다. | scheme을 root/body author 색과 합치지 않는다. |
| 14 | page content overlay | 실제 body box는 canvas 배경과 별도로 칠해질 수 있다. | 전체 page screenshot 동등성이 아니라 빈 body Canvas만 목표로 쓴다. |
| 15 | screenshot color space | 저장된 표본은 PNG RGBA8이며 두 위치에서 같은 byte였다. | 원본 PNG와 SHA-256을 보존한다. |
| 16 | property allowlist | background image/gradient는 단색 선택에 표현할 수 없다. | profile 밖 property는 fail-closed, 조용한 생략 금지다. |
| 17 | app screenshot scope | native host 배경은 HTML body box paint를 증명하지 않는다. | simulator capture를 native content root pixel로만 기술한다. |
| 18 | resize/environment freshness | Chrome matrix는 고정 viewport만 다뤘다. | 앱 resize/mutation/conformance 주장을 추가하지 않는다. |
| 19 | JS-only regression | Chrome 결과만으로 JavaScript-only session 호환을 보이지 못한다. | Rust/C API의 기존 session negative control을 계획에 남겼다. |
| 20 | browser cleanup | 해당 Agent Browser session을 닫고 프로세스 scan이 비어 있음을 확인했다. | browser 증거 수집 뒤 종료 절차를 충족했다. |

## 오라클 요약

관찰 원본과 hash는 [Chrome precomparison](c04-12-document-canvas-background-2026-10-18/README.md)에 있다. Canvas 색 선택은 재현 가능한 빈 body fixture에서 측정했다. body box paint, partial-alpha compositing 및 전체 HTML rendering은 다음 별도 작업 없이는 지원하지 않는다.

## sRGB 8-bit 경계 범위 재고정 뒤 최종 검토 20회

| # | 독립 실패 관점 | 검증 결과 | 최종 계획 반영 / 판정 |
|---:|---|---|---|
| 1 | CSSOM vs pixel precision | `color(srgb 0.2 0.2 0.9)` CSSOM과 screenshot channel이 달랐다. | CSSOM만으로 pixel byte를 확정하지 않는다. |
| 2 | exact byte conversion | fraction channel을 nearest rounding하면 Chrome의 229와 230이 달라진다. | 정확한 byte 경계에 없는 channel은 fail-closed한다. |
| 3 | color-space source | hwb sample의 computed color와 final channel이 한 단계 차이를 보였다. | hwb source를 지원한다고 표시하지 않는다. |
| 4 | HSL boundary | HSL 표본은 이 fixture에서 computed RGB와 pixel이 일치했다. | 한 표본만으로 HSL 전체 동작을 확장하지 않고 제외한다. |
| 5 | sRGB source boundary | hex와 정수 RGB fixture는 exact byte로 확인됐다. | CSS computed source가 sRGB여야 한다. |
| 6 | display-p3 gamut | 이번 matrix는 Display-P3 output을 검증하지 않았다. | wide-gamut 색은 지원 계약에 포함하지 않는다. |
| 7 | system color resolution | Canvas system color 입력을 별도 oracle로 고정하지 않았다. | system color는 profile 밖으로 유지한다. |
| 8 | alpha-zero handling | transparent root/body fallback은 light/dark exact로 확인됐다. | alpha 0은 투명으로 다룬다. |
| 9 | partial alpha | Chrome은 alpha composite pixel을 내지만 계산 channel 반올림 경로가 다르다. | 부분 alpha는 -15로 오류 처리한다. |
| 10 | root vs body choice | 빈 body에서 root opaque 값이 canvas pixel을 정했다. | root opaque 선택 순서를 고정한다. |
| 11 | body propagation | transparent root의 opaque body 색이 canvas 전체로 전파됐다. | body fallback을 root transparent일 때만 사용한다. |
| 12 | author origin | inline 및 author stylesheet 양쪽 값을 browser에서 관찰했다. | raw CSS를 다시 파싱하지 않고 Stylo cascade를 사용한다. |
| 13 | custom properties | `var()`가 resolved body value와 matching pixel을 만들었다. | typed computed style 경로와 기존 C05.1 semantics를 유지한다. |
| 14 | media environment | light/dark media 값과 screenshot byte가 각각 일치했다. | scheme 변경은 revision key와 함께 재계산한다. |
| 15 | fallback color | transparent background에 light/dark 각각 흰색/#121212가 관찰됐다. | 기존 scheme fallback만 재사용한다. |
| 16 | body box isolation | body min-height가 root/body box paint를 섞어 canvas 판정을 숨겼다. | oracle fixture는 body를 빈 상태로 둔다. |
| 17 | host view scope | simulator root 배경은 native content root만 바꾼다. | system bar, body box 및 element layer는 판정 대상이 아니다. |
| 18 | parse/profile failure | background image는 단색 resolver가 표현할 수 없다. | unsupported declaration은 기존 cascade error로 전파한다. |
| 19 | public ABI shape | RGBA byte 배열 외 새 field가 필요하지 않다. | C ABI structure layout은 그대로 둔다. |
| 20 | claim boundary | Chrome oracle와 앱 실행 자료가 서로 다른 환경이다. | simulator 화면은 native content root evidence로만 기술한다. |

### 최종 계획 승인

앞의 60개 서로 다른 실패 관점에서 blocker를 찾지 못했다. root/body Canvas 배경의 정확한 8-bit sRGB 하위 범위에 한해 구현을 계속한다. 화면 증거와 코드가 범위 밖 값에 대해 성공을 반환하면 계획 실패로 판정한다.
