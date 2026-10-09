# C01.3 Chromium CSSOM 속성 이름 기준 목록

**상태:** 계획 검토 완료 · 구현 및 근거 완료 · **상태 대장 ID:** C01.3 · **대상 범위:** 내부 비교 기준 자료

## 목적

C01.1의 19개 CSS 값 seed와 C01.2의 layout fixture를 넓히기 전에, 고정 Chromium이 HTML 요소의 계산 스타일 선언에서 노출하는 CSS 속성 이름 집합을 별도 기준 파일로 남긴다. 이 작업은 속성 이름 표면을 기록할 뿐 CSS 값 전체, 선택자, at-rule 또는 CSS 기능 지원을 정하지 않는다.

Chromium DevTools Protocol의 비공개·실험 메서드를 조회하는 대신 페이지 표준 CSSOM의 `getComputedStyle(element).item(index)`를 관찰한다. 설치된 Chrome `154.0.8037.98` / Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`과 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`를 사용한다. 이 값은 현재 개발 환경에서 확인한 실행 기준이며 전역 최신 Stable 선언이 아니다. 새 Chrome 빌드마다 별도 불변 스냅샷을 만들고 기존 자료를 덮어쓰지 않는다.

## 비교 모델

- **입력:** 저장소 fixture의 `#probe` HTML `div` 한 개. author stylesheet와 외부 자원은 두지 않는다.
- **관찰:** HTML namespace인지 확인한 뒤 계산 스타일 선언의 `item(index)` 이름을 수집한다. 이름은 원문 그대로 보존하고 JavaScript 기본 문자열 정렬로 정규화한다. 중복 이름은 실패다.
- **분류:** 일반 이름, `-`로 시작하는 prefixed 이름, `--` custom property를 분리한다. unprefixed라는 사실은 CSS 표준 속성이라는 뜻이 아니다. custom property는 결과에 보존할 수 있지만 unprefixed/prefixed 속성 개수에 합산하지 않는다. 각 ID는 `cssom.property.`와 UTF-8 이름의 소문자 hex 인코딩을 이어 붙여 브라우저 버전과 무관하게 결정적으로 만든다.
- **비교:** 속성 이름의 정렬된 집합을 정확 비교한다. 숫자 허용 오차, 평균 점수 또는 픽셀 fuzziness는 적용하지 않는다.
- **조건:** CSS viewport `800×600`, device scale factor `1`, `navigator.language`·`Intl` locale·accept-language `en-US`, time zone `UTC`, light color scheme, reduced-motion `no-preference`, forced-colors `none`, viewport·미디어 관측값을 고정하고 출력한다. locale 설정은 CDP emulation과 user-agent accept-language 모두 적용하고 페이지에서 다시 읽는다.
- **식별:** 브라우저 product/version/revision·실행 파일 SHA-256, OS/build·architecture, Node 버전, Chrome flags, fixture·캡처 스크립트·지원 모듈 SHA-256을 저장한다. reference ID는 브라우저 빌드·Node 버전·fixture hash·모든 캡처 코드 hash로 만든다. `SPINON_CHROMIUM_BIN`은 절대 경로만 허용한다. 같은 출력 경로가 있으면 덮어쓰지 않고, 임시 디렉터리에서 완성한 결과만 최종 경로로 원자적으로 이동한다.
- **프로세스 수명:** 이 실행에서 만든 detached process group만 종료한다. 각 신호 전 process group 멤버의 명령과 이 실행의 고유 임시 profile 경로가 맞는지 확인한다. 기존 Chrome 앱이나 다른 프로세스를 이름 검색으로 종료하지 않는다. bounded wait와 SIGTERM/SIGKILL 단계 상승 뒤 group 소멸을 확인하고, 그 뒤 profile을 지운다. 소유자나 종료 여부를 확인할 수 없으면 캡처 성공·정리를 주장하지 않는다.
- **한계:** 이 결과는 한 HTML `div`의 CSSOM 속성 이름 표면이다. 속성의 값 문법·초기값·selector/cascade·상속·HTML/SVG 전체 요소·UA stylesheet·layout·텍스트·paint·GPU·Android/iOS 호환성은 확인하지 않는다. C01 전체 또는 제품 CSS 지원 완료로 해석할 수 없다.

## 산출물과 통과 기준

1. `tests/fixtures/css/c01/cssom-property-surface.html`에 외부 참조가 없는 고정 fixture를 둔다.
2. `tools/css-reference/capture-property-surface.mjs`가 저장소 Node.js 내장 `WebSocket`으로 Chromium을 실행하고, CDP로 고정 환경의 fixture를 열어 CSSOM 속성 이름을 수집한다. CLI 버전과 CDP product/version/revision이 다르면 실패한다.
3. `tools/css-reference/property-surface.mjs`가 schema·환경·이름 정렬·중복·분류·ID·부분 범위 설명을 검증한다. 빈 결과, 비정상 이름, 불일치 메타데이터, 잘못된 ID는 거부한다.
4. `tools/css-reference/property-surface.test.mjs`에 유효 입력과 빈 목록·중복·순서 변경·ID 충돌/변조·잘못된 분류·누락된 미포함 범위에 대한 음성 검증을 둔다.
5. 기준 JSON은 `tests/fixtures/css/references/` 아래 버전·revision·OS·fixture·도구 hash를 포함하는 새 디렉터리에 기록한다. 캡처 전 같은 경로가 있으면 실패하고, 임시 Chromium profile/process를 제한 시간 안에 종료한 뒤에만 성공을 보고한다.
6. `package.json`에 `css:reference:c01-property-surface` 명령을 연결하고 `test:css-reference`에 단위 검증이 포함되게 한다.
7. `spec/STATUS.md`의 C01.3 하위 항목, 이 계획, `docs/plans/css-rendering.md`를 함께 동기화한다. C01 부모 항목은 계속 미완료로 둔다.

### 실행과 판정

```sh
mise exec -- node tools/css-reference/capture-property-surface.mjs
mise exec -- bun run test:css-reference
```

fixture, validator, Chromium 버전 또는 실행 조건이 바뀌면 새 reference ID로 저장한다. 새 snapshot은 같은 schema의 이전 결과를 수정하지 않는다. 공개 CSS API 변경은 없으므로 공개 API 명세를 새로 만들지 않는다. 이 단계는 기준 목록 생성 도구와 기준 자료만 다룬다.

## 계획 검토에서 보강한 부분

- 현재 Chrome에 없는 CDP 실험 메서드 대신 표준 CSSOM 관찰로 기준 출처를 바꿨다. 실측에서 CDP locale만으로 `navigator.language`가 바뀌지 않아 accept-language override를 함께 고정했다.
- 속성 이름 관찰을 전체 CSS 지원·표준 속성 분류로 오해하지 않도록 custom/prefixed/unprefixed 이름을 분리하고, 값·selector·at-rule·SVG·플랫폼 결과를 미포함으로 고정했다.
- 브라우저·fixture·모든 캡처 코드 hash와 Node 환경을 식별하고, browser process group과 출력 디렉터리의 충돌·부분 기록을 제한하는 절차를 추가했다.

## 구현 검토에서 반영한 보완

- WebSocket 응답 대기에는 제한 시간이 있었지만 연결 열림에는 없었다. 열림·오류·조기 종료·시간 초과를 유한하게 처리하고 단위 검증을 추가했다.
- JSON schema의 알 수 없는 필드는 거부한다. 상위 snapshot이나 property에 `supported: true` 같은 선언을 덧붙여 부분 관찰 결과를 기능 지원으로 오해하게 만들 수 없게 했다.
- fixture·수집기·보조 모듈 경로와 Chromium flags를 고정 계약으로 검증한다. 외부 환경 문자열은 reference 디렉터리 ID의 각 경로 조각에서 구분자와 상위 경로 표기가 되지 않도록 정규화한다.
- 수집 전 CLI/CDP 버전을 대조하고, 수집 후 실제 fixture·바이너리·도구 해시와 관측 환경을 고정 JSON으로 검증한다. 이미 존재하는 reference는 다시 캡처해도 덮어쓰지 않는다.
- 실패·성공 뒤 Chromium 임시 profile이 남지 않는지 실행 환경에서 확인했다. 종료 소유자를 증명할 수 없을 때 profile을 지우거나 성공으로 보고하지 않는 경계를 유지한다.

고정 환경에서 실제로 관찰한 478개 이름은 [C01.3 실행 근거](../spec/internal/evidence/css-c01-cssom-property-surface-2026-10-09.md)와 [불변 JSON snapshot](../tests/fixtures/css/references/cssom-property-surface-v1-macos-arm64-macos-26.5.1-25f80-154.0.8037.98-node-v24.20.0-revision-b859317bf11f-binary-ccffd5c5fe77-fixture-b4ea33587c76-tools-a6651dbb324d/cssom-property-surface.json)에 있다. 이 slice만 완료했으며 C01의 전체 Chromium 기능 inventory는 계속 미완료다.
