# C04.12 루트 HTML·문서 환경 계획 적대 검토

**대상:** [`plan/c04-root-html-document-environment.md`](../../../plan/c04-root-html-document-environment.md) 초안
**검토일:** 2026-10-17
**종류:** 계획 문서의 서로 다른 실패 관점 20개. 구현 검토나 실행 증거가 아니다.

| # | 실패 관점 | 계획에서 고정한 대응 |
|---:|---|---|
| 1 | `<head>`/`<body>`를 정규식이나 순서 가정으로 찾아 malformed HTML에서 잘못된 tree 구성 | 표준 HTML tree construction parser 후보를 고르고 recovery를 계약에 기록 |
| 2 | `<meta>`가 body에 있거나 head가 암시적으로 닫힌 경우를 무시 | head/body 정상 배치와 browser recovery가 필요한 순서를 비교 행렬에 포함 |
| 3 | viewport metadata의 대소문자, 공백, 속성 순서를 다르게 처리 | Chromium reference에 속성·공백·대소문자 변형을 포함 |
| 4 | 두 개 이상의 viewport 선언의 승자 규칙이 임의로 고정됨 | 중복 선언을 기준 비교와 contract 우선순위 규칙으로 고정 |
| 5 | 잘못된 width/scale 값 일부만 적용해 browser와 다른 결과 발생 | valid/invalid 조합을 oracle로 분류하고 전체 metadata 오류 정책을 명시 |
| 6 | device width, CSS viewport와 native pixel surface가 같은 단위로 혼동 | native surface·display scale과 CSS viewport를 별도 값으로 유지 |
| 7 | `initial-scale`을 layout viewport와 drawable scale에 이중 적용 | Chromium에서 관찰한 CSS viewport/scale 결과를 각각 비교 |
| 8 | min/max scale, zoom, `viewport-fit`을 지원한다고 과장하거나 조용히 무시 | 이 계획의 포함 여부를 oracle capture 뒤 고정하고 나머지는 명시적 제외 |
| 9 | orientation/resize 뒤 초기 viewport가 남아 이전 frame을 게시 | resize별 environment revision과 stale 결과 거부를 완료 기준에 포함 |
| 10 | `meta color-scheme`을 OS의 dark preference 자체로 취급 | native user preference와 document supported schemes를 분리해 입력·관찰 |
| 11 | meta 선언과 root CSS `color-scheme` 간 cascade 우선순위가 불분명 | 혼합 선언 조합을 Chrome reference에 저장하고 계약 전 우선순위 확정 |
| 12 | `prefers-color-scheme`가 meta/CSS 선언과 OS 변경에 대해 잘못 갱신 | media 결과, computed property와 OS theme 전환을 각각 비교 |
| 13 | 지원하지 않는 `@media` 조건이 부분 cascade 성공으로 숨겨짐 | media allowlist와 diagnostic fail-closed 동작을 명시 |
| 14 | stylesheet `media` attribute와 내부 `@media`의 제한이 서로 다르게 해석 | C04.11 기존 입력과 media feature subset의 경계를 계획에 연결 |
| 15 | head의 공백 text, title, meta가 화면 node/hit 대상이 됨 | head metadata 비표시 및 layout/paint/hit-test 누출 금지를 완료 기준으로 둠 |
| 16 | 파서가 생성한 document root와 기존 합성 Stylo root가 중복되어 `:root`/`rem` 오작동 | 새 HTML tree가 문서 루트를 소유하는 기준과 기존 C04.11 통합을 contract에서 정의 |
| 17 | UTF-8 BOM, 잘못된 byte sequence 또는 인코딩 sniffing 불일치 | 첫 구현 source encoding을 UTF-8로 제한하고 오류 처리를 내부 계약에 기록 |
| 18 | 과도한 input/depth/attribute 수로 parser 메모리 또는 stack 사용이 폭증 | parser/source/depth/attribute budget을 구현 전 고정 |
| 19 | metadata parse 실패 후 문서 일부만 새 revision으로 게시 | source/document/environment 입력을 atomic하게 묶고 unsupported 입력은 부분 결과 없이 실패 |
| 20 | root HTML 구현이 원격 link fetch, script, base URL이나 전체 HTML 지원까지 암묵 확장 | 외부 resource, script, base URL, 전체 conformance를 명시적으로 제외하고 별도 계획 요구 |

## 판정

계획은 물리 surface와 CSS viewport, OS 선호와 문서 색상 모드 선언, HTML document tree와 기존 HostDocument fragment의 경계를 구분한다. 구현 착수 전에는 실제 Chromium capture로 metadata 우선순위와 허용 입력을 확정하고 버전 있는 contract 및 실행 fixture를 추가해야 한다. 계획 검토만으로 런타임 동작이나 구현 완료를 주장하지 않는다.
