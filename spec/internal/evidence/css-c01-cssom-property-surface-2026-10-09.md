# C01.3 · Chromium CSSOM 속성 이름 표면

**상태:** 내부 기준 산출 완료 · **API 명세:** 해당 없음 · **제품 CSS 지원 판정:** 아님

## 관찰 범위

2026-10-09에 로컬에 설치된 Chrome `154.0.8037.98` / Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`을 headless mode로 실행했다. author stylesheet와 외부 자원이 없는 고정 HTML fixture에서 HTML namespace `div#probe` 한 개의 `getComputedStyle(element).item(index)` 이름을 읽고 원문 이름을 보존해 JavaScript 기본 문자열 순서로 정렬했다.

| 항목 | 관찰값 |
| --- | --- |
| CSSOM 이름 수 | 478 |
| 일반 이름 (`unprefixed`) | 442 |
| vendor prefix 형태 이름 | 36 |
| custom property | 0 |
| CSS viewport / device scale | `800×600` / `1` |
| locale / accept-language / time zone | `en-US` / `en-US` / `UTC` |
| 미디어 설정 | light / no-preference / forced-colors none |
| 실행체계 | macOS `26.5.1` (`25F80`), arm64 · Node.js `v24.20.0` |

`unprefixed`는 표준 CSS 속성이라는 분류가 아니다. 이 자료는 단일 HTML 요소의 CSSOM computed-style 이름 표면만 나타낸다. 전체 CSS property registry, 속성 값 문법·초기값, 선언 파싱 또는 적용 지원, selector·pseudo selector·at-rule·cascade, HTML/SVG 전체 요소 적용성, UA stylesheet 규칙, layout·text shaping·GPU 출력, Android/iOS 호환성을 확인하지 않았다. 따라서 478개를 구현해야 할 속성 수나 Spinon 지원 개수로 해석하면 안 된다.

## 원본과 재현

- [고정 Chromium JSON](../../../tests/fixtures/css/references/cssom-property-surface-v1-macos-arm64-macos-26.5.1-25f80-154.0.8037.98-node-v24.20.0-revision-b859317bf11f-binary-ccffd5c5fe77-fixture-b4ea33587c76-tools-a6651dbb324d/cssom-property-surface.json)
- Fixture SHA-256: `b4ea33587c76c7538a15218bcabcbfd3486bf19704a4e94d35ab0a8c8d7e4513`
- Chromium 실행 파일 SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- CSSOM 이름 목록 SHA-256: `7a2d87e6df1d13d0bfb335ff86b38e684e19d192d4a243f52d1dfa3772b1fbcb`
- 캡처 도구 SHA-256: `1b47c66a148998085b117cc3c322bbe51781594fa315e805453560bbcf6dd21f`
- `property-surface.mjs` SHA-256: `815c84d9cc6ec9f331df6fc310168555567a011ef158eadfe2e466d987b786c7`
- `chromium-session.mjs` SHA-256: `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1`
- reference ID: `cssom-property-surface-v1-macos-arm64-macos-26.5.1-25f80-154.0.8037.98-node-v24.20.0-revision-b859317bf11f-binary-ccffd5c5fe77-fixture-b4ea33587c76-tools-a6651dbb324d`

실행 명령:

```sh
mise exec -- bun run css:reference:c01-property-surface
mise exec -- bun run test:css-reference
```

2026-10-09 `03:06:04.709Z`에 수집한 JSON에서 CLI/CDP version·revision, 실제 viewport·locale·time zone·media, fixture author stylesheet/resource count, 이름 정렬·분류·digest를 검사했다. CSS reference 검증 suite는 **14개 통과, 0개 실패**했다. 추가로 같은 reference를 다시 캡처했을 때 기존 디렉터리 충돌(`EEXIST`)로 거부되고 기존 파일 SHA-256이 유지되는 것을 확인했다. 성공·충돌 뒤 `/tmp/spinon-css-property-surface-*` 임시 profile과 그 profile을 사용하는 Chromium process는 남지 않았다.

## 검토에서 반영한 사항

- CDP WebSocket 연결 열림에도 timeout을 적용하고 연결 오류·조기 종료와 구분했다.
- schema에 정의되지 않은 최상위·inventory·property 필드를 거부해 이 부분 자료에 지원 상태를 덧붙이지 못하게 했다.
- fixture·도구 경로와 Chromium 실행 flags를 고정했다. reference ID에 들어가는 외부 환경 문자열에서 경로 구분자와 상위 경로 조각을 차단했다.
- Chromium 종료 신호 전 해당 process group의 모든 프로세스가 이 실행의 임시 profile을 사용하는지 확인한다. 소유자·종료 확인에 실패하면 profile을 삭제하지 않고 성공으로 보고하지 않는다.
- JSON은 임시 디렉터리에서 검증한 뒤 새 reference 경로에만 원자적으로 저장한다. 이미 존재하는 snapshot은 덮어쓰지 않는다.

## 남은 범위

전체 C01은 미완료다. 다음 비교 기준에는 지원 HTML·SVG node set, UA stylesheet 및 CSS 값·선택자·at-rule inventory, 기능별 계산값·geometry·paint 기대치가 필요하다. C01.3 JSON 하나로는 제품 Stylo/Taffy/GPU 결과, Android·iOS 동작, CSS 표준 적합성 또는 CSS 기능 지원을 판정할 수 없다.
