# C04.12 · 앱 루트 HTML과 문서 환경 metadata

- **문서 유형:** 구현 계획 · 공식 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [CSS 호환성](../spec/0008-css-compatibility.md), C01 HTML/CSS 기준 inventory, C04 런타임 스타일·환경
- **현재 상태:** 신규 계획. 구현 전이며 지원 기능으로 취급하지 않음
- **계획 적대 검토:** [20개 실패 관점 검토](../spec/internal/evidence/c04-root-html-document-environment-plan-review-2026-10-17.md)

## 배경과 목표

현재 앱 런타임은 JavaScript가 HostDocument 요소·속성·텍스트를 생성하는 경로다. 별도의 앱 루트 HTML 파일을 HTML 문서로 파싱해 `<html>`, `<head>`, `<body>`와 문서 metadata를 소유하는 경로는 없다. C04.11도 이미 만들어진 HostDocument에서 연결된 `<style>`을 수집할 뿐, HTML 파일을 읽거나 `<meta>`를 해석하지 않는다.

이 계획은 앱의 루트 HTML을 문서 입력의 기준으로 삼고, Chromium 모바일 동작을 oracle로 삼아 문서 구조와 환경 metadata를 런타임 cascade·layout 입력에 연결한다. 첫 구현 범위는 `<meta name="viewport">`, `<meta name="color-scheme">`, 루트 CSS의 `color-scheme`, 그리고 이 환경을 소비하는 viewport 및 `prefers-color-scheme` media 조건이다. 일반 HTML 전체 또는 CSS 전체 지원으로 확대 해석하지 않는다.

## 환경 소유권과 의미 경계

- 네이티브 호스트가 실제 surface 크기, display scale, OS 색상 선호와 pointer capability를 제공한다. HTML metadata는 이 물리 입력을 덮어쓰지 않는다.
- `meta viewport`는 Chrome 모바일 oracle에서 관찰한 규칙에 따라 CSS layout viewport와 scale 관련 입력을 만든다. CSS viewport와 device scale을 구별하고 native drawable 크기를 바꾸지 않는다. pinch zoom과 `visualViewport`가 이 slice에 들어가는지는 사전 비교에서 사용자 관찰 가능 범위를 고정한 뒤 명세에 쓴다.
- `meta name="color-scheme"`와 root의 CSS `color-scheme`은 문서가 지원하는 색상 모드 선언으로 보존한다. OS의 light/dark 선호와 선언된 지원 모드를 하나의 boolean으로 합치지 않는다. `prefers-color-scheme`의 결과, 계산된 `color-scheme`, UA canvas 기본색은 각각 Chromium에서 관찰해 연결 규칙을 고정한다.
- 환경 입력, 문서 metadata와 cascade snapshot은 같은 revision tuple (`DocumentRevision`, `EnvironmentRevision`)에 묶는다. surface resize, OS 색상 선호 변경, source 문서 교체 뒤 이전 계산 결과가 새 화면에 게시되지 않아야 한다. 동적 DOM metadata 변경은 이 계획의 범위에 없다.
- `<head>`의 metadata는 화면 layout node나 hit-test 대상이 되지 않는다. 기존 C04.11 `<style>` 경로는 파서가 만든 동일 HostDocument에서 재사용한다.

## 구현 범위

1. 앱 진입점에서 UTF-8 루트 HTML source를 받아 표준 HTML tree construction을 제공하는 parser 후보를 확인하고 선택한다. 수작업 tag tokenizer를 새로 만들지 않는다. 파서 선택, source size/depth/attribute budget, malformed markup recovery, fragment/document mode 차이는 구현 계약 전에 고정한다.
2. HTML document tree의 `<html>`, `<head>`, `<body>` 및 원본 순서를 HostDocument/runtime 입력으로 보존한다. metadata와 stylesheet 수집은 head/body 배치가 정상인 경우와 브라우저 recovery가 필요한 경우를 포함한다.
3. Chromium 비교로 정한 첫 viewport metadata profile을 구현한다. 최소한 metadata 부재, `width=device-width`, 고정 width, `initial-scale`, 중복 선언, 공백·대소문자·잘못된 값, orientation/resize를 다룬다. min/max scale, user-scalable, viewport-fit, pinch zoom은 관찰 결과와 제품 입력 경계를 기록한 뒤 이 slice에 포함할지 확정한다.
4. `<meta name="color-scheme">`와 root CSS `color-scheme`을 cascade와 UA 색상 선택에 연결한다. `light`, `dark`, `only` 및 잘못되거나 중복된 선언의 동작을 Chrome reference로 고정한다. `prefers-color-scheme`는 native OS/user preference 변화에 따라 재계산한다.
5. viewport width/height와 `prefers-color-scheme`를 대상으로 하는 `@media`를 기존 C04.11 author stylesheet 입력에서 동작시킨다. 지원할 media 문법·feature를 명시적으로 allowlist하고, profile 밖 feature나 parse diagnostic은 부분 결과 없이 실패시킨다. 일반 media query 전체 지원은 별도 범위다.
6. `<head>`의 비표시 요소가 layout, paint, hit-test, text 입력에 누출되지 않는지 보장한다. `<style>`은 기존 C04.11 범위에서 처리한다. 외부 `<link>` stylesheet, 원격 fetch, script 실행으로 metadata를 동적으로 만드는 동작은 C02/resource loader와 별도 계약이 준비되기 전까지 범위 밖이다.
7. 버전 `0.1.0` 내부 계약을 구현 전에 작성한다. source encoding·parse 오류, metadata 기본값과 우선순위, parser recovery, 환경 revision, unsupported 입력과 원자적 실패 규칙을 정의한다.

## 사전 비교 모델

- **기준:** 고정 Chromium 모바일 및 데스크톱 빌드. 웹 관찰은 Agent Browser에서 수행하고 terminal-browser는 사용하지 않는다.
- **고정 입력:** browser revision, OS image, CSS viewport, device scale factor, locale, native scheme/pointer emulation, HTML source 및 inline stylesheet hash를 각 실행 기록에 남긴다.
- **관찰값:** `document.documentElement/head/body` 구조, viewport metrics, `matchMedia()` 결과, root `color-scheme` 계산값, probe computed style/frame, `document.elementFromPoint()` 결과, 앱의 HostDocument/environment/style revision, CSS frame, simulator surface capture를 구분해 보존한다.
- **비교 행렬:** meta viewport 없음·device width·고정 width·scale·중복/잘못된 값, portrait/landscape 및 resize, OS light/dark, color-scheme metadata/CSS 선언 조합, width/scheme media query 조합, 잘못된 markup과 metadata 순서.
- **사전 판정:** Chromium reference capture 전 허용 오차와 성공/실패 기준을 고정한다. 계산 viewport·media 결과는 정확 비교하고, CSS frame 허용치는 fixture별 좌표 축 최대 `0.5 CSS px` 이하로 정한다. 픽셀 비교는 전체 화면 screenshot만으로 surface readback 결과를 대체하지 않는다.
- **검증 환경:** Rust parser/metadata/cascade fixture, Android API 37 AVD와 iOS Simulator의 실제 app runtime을 사용한다. 이 계획은 실기기나 전체 Chromium HTML/CSS conformance 증거를 요구하지 않는다.
- **프로세스 정리:** Agent Browser 실행 뒤 브라우저 세션을 닫고 해당 작업에서 시작한 Agent Browser 프로세스가 남지 않았는지 확인한다. 기존 사용자 브라우저나 tmux 세션은 종료하지 않는다.

## 완료 기준

- 고정 Chromium reference와 metadata parsing·CSS viewport·scheme/media 결과가 명시한 기준에 맞는다.
- Android AVD와 iOS Simulator 양쪽에서 같은 root HTML source가 동일한 HostDocument metadata, CSS viewport와 지원된 media 결과를 만든다. platform surface size와 device scale은 별도 값으로 유지된다.
- resize와 OS light/dark 변경 때 대응 revision이 바뀌고 오래된 cascade/layout/paint 결과가 게시되지 않는다. source HTML 교체는 새 document revision으로 반영한다.
- malformed markup의 recovery, malformed/중복 metadata, unsupported media 입력이 계획된 Chrome 관찰 또는 명시된 fail-closed 규칙에 따라 처리된다.
- head metadata가 visible node/hit target이 되지 않으며 기존 C04.11 `<style>` cascade와 C04.10 runtime path가 회귀하지 않는다.
- 공개/내부 지원 범위, parser 및 metadata contract, precomparison, Android/iOS Simulator 실행 근거와 별도 20개 구현 실패 관점 검토를 남긴다. 이 item의 완료 표시는 이 근거가 모두 있을 때만 한다.

## 명시적 제외

완전한 WHATWG HTML parser conformance, DOMParser/`document.write`, HTML script 실행·동적 `meta` mutation, CSSOM, 모든 viewport descriptor와 pinch zoom, 일반 CSS media feature 전체, 외부 stylesheet/resource fetch, `<base>` URL resolution, 전체 UA stylesheet/system color/form control 테마, forced colors·contrast·reduced motion, webview navigation/history는 이 계획만으로 지원하지 않는다. 사전 비교에서 반드시 필요하다고 드러난 항목은 별도 하위 계획과 roadmap ID로 추가한다.
