# C04.12 Tailnet 로드맵 설명 정정·동기화 근거

## 목적과 정정 내용

PR [#274](https://github.com/ohah/spinon/pull/274)은 기본 앱의 `index.html`을 pinned HTML5 parser로 V8 초기 document에 연결하고, `<meta name="viewport">` 기반 CSS viewport와 OS light/dark 선호에 따른 `prefers-color-scheme` 입력을 root-only cascade에 연결했다. Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 viewport, scheme 및 environment revision 전환을 확인했다.

로드맵에는 이전 기본 앱 DOM 검사만 설명되어 있어 viewport·scheme 효과가 아직 적용되지 않는다고 잘못 적혀 있었다. [로드맵 원본](../../../public/roadmap.html)을 구현 및 실행 근거와 일치하도록 고쳤다. `<meta name="color-scheme">` 해석값과 `:root { color-scheme }`은 typed/computed snapshot에만 남는다. UA Canvas 색상, native 배경·form control 테마, page-scale renderer transform, HTML layout·paint는 지원하지 않는다. 앱 화면은 빈 native host이며 C04.12는 계속 미완료다.

## 빌드와 동기화

- 선택 문서 집합을 Rspress `2.0.22`와 Bun `1.4.2`로 빌드했다. [원본 빌드 로그(gzip)](selected-docs-build.log.gz)
- 저장소 전체 문서 빌드가 아니라 Tailnet에 올릴 선택 문서 빌드다. 임시 Rspress 설정에서 기존 문서 전체의 dead-link 검사만 끄고 경로를 렌더링했다. 따라서 전체 문서 링크 적합성 통과로 해석하지 않는다.
- Preview root: `/Users/yoonhb/Library/Application Support/SpinonPreview`
- Tailnet URL: <https://macstudio.tailed42f2.ts.net/spinon/>
- 동기화 payload는 137개, 5,312,053 bytes다. 기존 페이지 `roadmap.html`과 원본 복사본 `spec/public/roadmap.html` 두 파일을 별도 백업한 뒤 교체했다. 나머지 135개는 사전 대조와 복사 후 해시가 일치했다. 삭제는 0건이다.
- [사전 대조](preflight.json), [복사 후 검증](copy-verification.json), [두 교체 파일의 백업 검증](backup-verification.json), [최종 제공 응답 검증](served-verification.json)에 경로별 byte 수와 SHA-256을 보존했다. 백업 위치는 각 JSON에 기록했다.
- 로컬 Preview와 Tailnet에서 payload 137개씩, 총 274개 응답을 확인했다. 모두 HTTP 200이며 각 응답 body의 크기와 SHA-256이 빌드 원본과 일치한다.

## Agent Browser 확인

[로드맵 화면 캡처](roadmap-tailnet-preview.png)는 Agent Browser `0.31.1`로 실제 Tailnet URL을 연 결과다. 페이지에 viewport·scheme 환경 cascade의 실행 범위와 남은 제한, 내부 계약·기본 앱·환경 실행 근거 링크가 표시된다. 공식 상태 동기화 문구는 상위 78개, 세부 ID 125개, ID 없는 하위 체크 2개, 완료 `71/205`를 보고했다. Tailnet `STATUS.html`에서도 C04.12 체크가 해제 상태임을 확인했다. 상세 기록은 [browser-verification.json](browser-verification.json)에 있다.

이 이미지는 앱 동작 화면이 아니라 문서 Preview 화면이다. HTML paint가 아직 없으므로 Simulator의 빈 native host를 앱 화면 캡처로 제시하지 않았다. 검증에 사용한 격리 Agent Browser 세션을 닫았고, browser launch 비활성·page count 0 및 해당 daemon PID 종료를 확인했다. [세션 종료 기록](browser-session-close.txt)

## parser 타입 소유권

`ParsedRootHtml`과 `ParsedNodeKind`는 Spinon이 소유하는 Rust 타입이다. HTML5 tokenizer/tree builder는 pinned `html5ever 0.40.1`을 사용하며 Spinon 제품 DOM adapter가 `TreeSink`를 구현한다. `markup5ever_rcdom`의 test-only DOM 타입을 제품 표현으로 사용하지 않는다. 현재 범위와 parser 경계는 [내부 계약 0077](../../0077-c04-12-root-html-document.md)에 있다.

## 포함 자료

- `preflight.json`: 최종 137개 payload의 source 및 이전 대상 상태
- `copy-verification.json`: preview root 복사 후 137개 source hash 대조
- `backup-verification.json`: 교체 전 파일 2개의 백업 hash 대조
- `served-verification.json`: 로컬·Tailnet 각각 137개 HTTP 응답 hash 대조
- `browser-verification.json`: 화면·공식 상태·체크·링크·세션 종료 확인
- `roadmap-tailnet-preview.png`: Agent Browser Tailnet 문서 화면
- `selected-docs-build.log`: 선택 문서 빌드 원본 로그
- `pr-delta-review-30.md`: 이번 문서 변경의 30개 실패 관점 검토
