# C04.11 · 런타임 문서 `<style>` stylesheet cascade

- **문서 유형:** 구현 계획 · 공식 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C04 stylesheet·selector·cascade](../spec/STATUS.md#css-구현-체크리스트)
- **선행 구현:** C04.8 runtime cascade, C04.9 runtime Taffy layout, C04.10 runtime GPU scene, C05.1 사용자 지정 속성
- **내부 계약 버전:** `0.1.0-draft` 고정 · 출시 또는 호환성 정책을 정하기 전 숫자 버전은 올리지 않음
- **계획 적대 검토:** [별도 검토 기록](../spec/internal/evidence/c04-runtime-author-stylesheets-plan-review-2026-10-10.md)

## 목표

현재 runtime은 HostDocument의 inline `style` 속성과 내장 UA stylesheet만 계산한다. C04.11은 같은 불변 HostDocument snapshot에서 현재 연결된 HTML `<style>` 요소를 문서 순서대로 수집해 Stylo author origin에 전달한다. 기존 C05.1 `var()` 계산을 재사용해 stylesheet의 사용자 지정 속성도 기존 제한 Flex layout·paint 속성에 연결한다.

입력은 앱의 DOM 코드가 만든 요소·텍스트 노드다. CSSOM, 번들러 stylesheet 전달, 정적 `<link>` 자원, 외부 네트워크 loader를 추가하지 않는다. C04·C02의 나머지 항목과 C05 부모 항목은 완료 처리하지 않는다.

## 입력·cascade 계약

- 수집 시작점은 `HostDocumentSnapshot::root_children()`이며 연결된 노드만 순회한다. 전위 순회 순서가 stylesheet 등록 순서다. 각 `<style>`의 ID는 현재 문서 generation과 node ID에서 만든다.
- HTML namespace의 ASCII 대소문자 무관 `style` 요소만 CSS 입력 후보로 삼는다. `type` 누락·공백·`text/css`는 적용하고 다른 MIME essence는 HTML 동작처럼 무시한다. `media` 누락·공백·`all`·`screen`만 허용한다. 다른 media 조건, HTML이 아닌 namespace의 `style`, `rel` 토큰에 `stylesheet`가 있는 연결 `<link>`는 해당 계산 전체를 실패시킨다. 이 제한을 부분 성공으로 감추지 않는다.
- CSS 본문은 해당 노드 snapshot의 자손 텍스트를 문서 순서로 연결한다. 유효하지 않은 UTF-16은 대체 문자로 조용히 바꾸지 않고 계산 오류로 반환한다. 기존 HostDocument 문자열 한도 안에서만 입력을 받는다.
- 각 HostRoot fragment의 계산에 문서 전체에서 수집한 author stylesheet 순서를 전달한다. Stylo가 UA·author·inline 출처와 specificity·importance를 계산한다. 별도 문자열 cascade나 `var()` parser를 만들지 않는다.
- 이 단계의 명시 비교 selector는 HTML type, ID, class, descendant selector다. Stylo matcher를 사용하며 이 fixture를 전체 selector 표면 지원으로 확대 해석하지 않는다.
- `@import`는 현재 registry처럼 차단하며 원격 fetch를 시작하지 않는다. C04.11 author stylesheet의 Stylo parse diagnostic은 layout-only와 GPU 양 runtime 계산을 모두 실패시킨다. inline `style` diagnostic은 기존 C04.8 동작을 유지한다. 지원 profile 밖 at-rule·선언 기능은 성공으로 표시하지 않는다. 해당 runtime allowlist만 허용하고 C05.1 profile에서만 stylesheet custom property를 허용한다. GPU profile에서는 기존 단색 `background-color`도 허용한다.
- `DocumentRevision`이 stylesheet 본문·위치·연결 상태의 유일한 입력 revision이다. 매 변경마다 연결 fragment 전체를 재계산하며 worker의 capacity-one latest-wins 및 전체 revision-key 게시 검사를 유지한다. stylesheet registry·computed style cache는 요청 사이에 보존하지 않는다. 별도 stylesheet owner가 없으므로 `StyleRevision` 계약을 임의로 증가시키지 않는다.
- 기본 UA stylesheet에 HTML `style { display: none }`을 추가한다. style 요소의 숨겨진 자손 텍스트는 layout 입력·runtime frame/render traversal에서 제외한다. 보이는 text node는 기존과 같이 미지원 오류다. Author CSS가 style 요소를 보이게 만들면 text layout 오류로 실패한다.
- 외부 CSS·링크·`url()` 자원은 요청하지 않는다. CSS parser 진단은 source/node ID와 위치를 보존하고 해당 요청을 실패시킨다. 기존 inline style diagnostic 동작은 바꾸지 않는다.

## 비교 모델과 합격 기준

기준은 저장소가 고정한 Chromium `154.0.8037.98` 실행 파일과 같은 HTML/CSS 입력이다. 고정 viewport·media 환경에서 CSS computed string은 정확히 비교한다. root·style 요소·대상 tile의 `x`, `y`, `width`, `height` 최대 절대 오차는 각각 `0.5 CSS px` 이하여야 한다. GPU fixture의 화면은 Android/iOS Simulator에서 확인하되 GPU·OS 색상 차이를 CSS cascade 일치로 대신 주장하지 않는다.

Chromium reference에는 두 stylesheet의 등록 순서, type·class·ID 선택자, specificity, `!important`, inline override, stylesheet `--*`/`var()`, gap geometry, style 요소의 `display:none`과 0 geometry를 기록한다. runtime 검증은 동일 계산 결과와 다음 동적 변화를 확인한다.

- 연결된 style 텍스트 변경·stylesheet 위치 이동·제거 후 새 `DocumentRevision` 값이 반영되고 이전 source 규칙이 남지 않는다.
- detached style 요소의 규칙은 적용되지 않고 연결 후에만 적용된다. 서로 다른 stylesheet의 source order가 트리 순서와 일치한다.
- 보이는 text는 계속 실패하고, UA `display:none`인 style의 CSS 텍스트만 layout 대상에서 빠진다. Author CSS로 style을 보이게 하면 실패한다.
- stylesheet MIME 무시, 지원 밖 `media`, HTML이 아닌 style, `<link rel=stylesheet>`, `@import`, 파싱 진단, 일반 미지원 선언은 각기 정해진 실패 또는 무시 결과를 낸다. 어떤 경로도 네트워크를 요청하지 않는다.
- 구형 C04.10 inline CSS 및 C05.1 사용자 지정 속성 fixture는 동일 reference와 장면 결과를 유지한다.

## 구현 순서

1. 본 계획의 Chromium 입력·환경·관찰값·오차 기준을 고정하고 `C04.11` 전용 oracle fixture/reference를 만든다. 기존 C01/C04 reference 파일은 덮어쓰지 않는다.
2. `spinon-runtime`에 connected HTML style collection과 명시적 입력 오류를 둔다. 모든 root 계산에 동일 immutable source 순서를 전달한다.
3. `spinon-style`의 C05.1 runtime profile에 stylesheet custom property와 paint `background-color` 허용을 연결한다. 이전 runtime API 호출 경로와 다른 제한 profile은 유지한다.
4. 내장 UA `style` display 규칙과 hidden subtree의 layout/frame/render text 처리 규칙을 추가한다. 보이는 text node 오류를 보존한다.
5. fixture·runtime unit test에서 source 순서·specificity·inline override·변경/이동/detach/reinsert·오래된 요청 폐기·미지원 입력 실패를 검증한다. Chromium oracle와 computed value·geometry를 비교한다.
6. 기존 V8 fixture를 통해 Android API 37 emulator 및 iPhone 17 Pro / iOS 26.2 Simulator에서 DOM style node→Rust HostDocument→Stylo→Taffy→wgpu 화면을 확인한다. 두 앱과 화면 근거를 동일 계획 범위로 기록한다.
7. 내부 계약·상태 대장·구현 및 검토 근거·PR 본문을 동기화한다. `0.1.0` 숫자 계약 버전은 변경하지 않는다. 외부 네트워크 자원 loader, CSSOM, stylesheet `media` 일반 문법, SVG style, text layout, 증분 재계산은 미완료로 남긴다.

## 계획 검토에서 고정할 쟁점

| 관점 | 실패 가능성 | 계획의 방지 기준 |
|---:|---|---|
| 1 | C04.11을 C04 전체 또는 CSS 지원 선언으로 과장 | C04.11만 추가하고 C04·C05 부모는 미완료 유지 |
| 2 | DOM상 순서와 stylesheet source order가 달라짐 | 연결 snapshot 전위 순회 순서로 등록 |
| 3 | 분리 노드가 stylesheet로 계속 적용됨 | root children에서 도달 가능한 노드만 수집 |
| 4 | 다른 HostRoot의 CSS가 누락됨 | 문서 전체 stylesheet 목록을 각 root cascade에 전달 |
| 5 | 형제 selector가 서로 분리된 root를 건넘 | 별도 fragment root 제한을 상태·근거에 표시하고 이 단계에서 보장하지 않음 |
| 6 | SVG namespace `style`이 묵살됨 | 비 HTML style은 명시적 unsupported 오류 |
| 7 | 지원하지 않는 type을 CSS로 잘못 계산 | MIME essence 기준으로 HTML식 비 CSS type은 무시 |
| 8 | 지원하지 않는 `media`가 무조건 적용됨 | 누락/공백/all/screen만 적용하고 다른 값은 전체 요청 오류 |
| 9 | `<link>`가 원격 fetch를 암묵적으로 시작 | rel token을 감지해 요청 없이 명시 오류 |
| 10 | `@import`가 layout-only runtime에서 진단 뒤 부분 성공으로 남음 | C04.11 stylesheet parse diagnostic은 두 runtime 경로 모두 요청 전체를 실패시킴 |
| 11 | stylesheet 규칙이 inline precedence를 깨뜨림 | Chromium에서 source order·specificity·important·inline 우선순위 비교 |
| 12 | CSS custom property를 C04 일반 profile에도 허용 | custom property 허용을 C05.1 runtime profile에만 한정 |
| 13 | paint profile author `background-color`가 inline 검사와 다름 | C05.1 paint allowlist만 background-color 추가, typed color 변환 유지 |
| 14 | parser 진단이 있어도 조용히 부분 layout/scene을 게시 | author stylesheet parser 진단은 두 경로에서 source ID·위치와 함께 요청 전체 실패 |
| 15 | 16M UTF-16 문서 한도보다 큰 별도 CSS 복사 | 추가 별도 상한은 만들지 않고 document 한도 내 입력, 과도한 복사 여부 검토 |
| 16 | UTF-16을 lossy 변환해 CSS 토큰을 바꿈 | strict UTF-16 decode, 실패 경로 테스트 |
| 17 | style 본문 텍스트가 layout text 오류를 일으킴 | display:none 조상 밑 text만 제외하고 visible text 실패는 유지 |
| 18 | author가 style을 보이게 했을 때 text를 빈 상자로 성공 처리 | 보이는 style text는 미지원 오류로 fail closed |
| 19 | style 요소가 화면/geometry에 나타남 | Chromium style display/rect 대조, UA rule과 renderer 필터를 확인 |
| 20 | stale worker result나 이전 sheet 목록이 새 revision을 덮음 | 요청마다 무캐시 재수집, full key latest-wins 유지·동적 제거 테스트 |
