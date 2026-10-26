# PR #669 이후 로드맵과 Tailnet 미리보기 동기화

PR [#669](https://github.com/ohah/spinon/pull/669)이 병합된 뒤 공개 로드맵과 집중 계획의 구현 상태를 맞췄다. 이 변경은 C11.4 전체 지원이나 공식 완료 상태를 승격하지 않는다. `spec/STATUS.md`의 C11.4 체크박스는 미완료이고 공식 완료 수는 `72/213`이다.

## 변경 내용

- `spec/public/roadmap.html`에 #669의 제한 fixture 범위, Android/iOS Simulator 결과, 검증하지 않은 영역, 병합 SHA를 추가했다.
- `plan/c11-4-align-content-normal-runtime.md`의 구현 PR 상태를 병합 완료로 바꾸고 빈 GitHub check rollup을 CI 통과로 표현하지 않았다.
- PR #669의 Android/iOS 화면 링크와 구현 검토 기록을 유지했다. PR 본문의 raw 화면은 구현 커밋 SHA에 고정돼 있다.

## Tailnet 미리보기

선택 route build는 `rspress-selected.config.ts`와 `prepare-selected-root.mjs`를 사용한다. 전체 `docs:build`나 GitHub Pages 배포를 주장하지 않는다. 생성 route, raw evidence asset, byte 수와 SHA-256은 `selected-build.json`에 기록한다.

미리보기 사전 상태는 `preflight.json`, 선택 대상 파일에 열린 handle이 있었는지는 `target-open-handles.json`, 교체 전 백업과 쓰기 결과는 `tailnet-copy.json`, 제공 URL에서 받은 응답 본문 hash는 `tailnet-verify.json`에 기록한다. 기존 파일은 교체 전에 별도 backup root에 byte-exact로 보존하고 파일 삭제는 하지 않는다. 이미 같은 hash인 route는 다시 쓰지 않는다.

## Agent Browser 확인

터미널 브라우저를 사용하지 않고 Agent Browser로 Tailnet 로드맵 화면과 #669 계획·근거 링크를 확인한다. 관찰 JSON, 화면 캡처, 종료 확인은 각각 `tailnet-agent-browser-observation.json`, `tailnet-roadmap-agent-browser-pass01.png`, `agent-browser-lifecycle.json`에 둔다.

## 범위

이 작업은 문서 route와 Tailnet 미리보기 동기화만 다룬다. Android/iOS 앱이나 Simulator를 다시 실행하지 않으며 CSS 지원 범위, WPT 적합성, 실기기 결과, `spec/STATUS.md` 공식 상태를 바꾸지 않는다. C11.4 전체는 미완료이고 `72/213`을 유지한다.
