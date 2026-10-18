# C04.12 direct text Tailnet 로드맵 동기화

## 갱신 내용

PR [#317](https://github.com/ohah/spinon/pull/317)과 보완 PR [#318](https://github.com/ohah/spinon/pull/318)이 Rebase and merge로 병합된 뒤 Tailnet 로드맵을 갱신했다. 공식 상태 원본은 [`spec/STATUS.md`](/spinon/STATUS.html)이며, 상위 C04.12/C15 상태는 계속 미완료다. 계획 문서에는 두 PR의 병합 상태와 범위를 반영했다.

로드맵의 일반 HTML text 미지원 문장은 기본 앱 `RuntimeSession` 경로 기준으로 고쳤다. opt-in fixture 전용 direct text 구현은 명시하고 일반 앱 텍스트, font loading, nested inline, 전체 C04.12/C15 지원과 구분했다. 당시 74/207이었던 Canvas 이력은 역사값으로 남기고 현재 수를 하드코딩하지 않는다.

## Agent Browser 확인

기존 Agent Browser 세션 `spinon-c04-12` / `spinon-pr317-138060237e7c`에서 Tailnet URL을 확인했다. Chrome user agent는 HeadlessChrome 155였다.

- Agent Browser가 공식 상태 동기화를 마친 뒤 표시한 값: **209개 표시 · 공식 완료 76/209**.
- 페이지의 동기화 메모는 상위 78개, 세부 ID 129개, ID 없는 하위 체크 2개, 합계 209개로 표시한다.
- C04.12 상위 항목은 미완료로 남아 있다. 현행 완료 수는 GitHub `main`의 공식 상태 대장에서 동적으로 읽는다.

공식 집계와 text paint 항목의 Agent Browser 캡처는 이 PR 본문에 첨부한다. 저장소 evidence directory에도 같은 PNG와 해시를 보존한다.

[30개 PR diff 실패 관점 검토](/spinon/internal/evidence/c04-12-root-html-text-paint-tailnet-sync-2026-10-18/pr-delta-review-30.html).

기존 로드맵 항목에서 Simulator PNG를 직접 가리키던 두 링크가 404인 것을 Agent Browser와 HTTP 확인에서 발견했다. 구현 근거 페이지 안의 Android 이미지(840×630)와 iOS 이미지(1206×2622)는 정상 로드됐다. 로드맵의 깨진 직접 링크를 제거하고 해당 근거 페이지 링크를 남긴 뒤 다시 빌드·동기화했다.

## 선택 문서 빌드·Tailnet 동기화

RSPress `2.0.22` 선택 빌드는 C04.12 계획·계약·구현 근거, 이 동기화 기록과 PR 검토를 포함해 생성했다. 전체 문서 빌드와 전체 링크 적합성은 수행하지 않았고, 선택 config의 dead-link 검사는 껐다.

미리보기 root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`다. PID 1191의 기존 listener `127.0.0.1:8768`은 재시작하지 않았다. 첫 반영은 기존 파일을 백업·대조하고 새 경로만 추가했다. 깨진 직접 이미지 링크를 수정한 뒤 Tailnet 검증 페이지를 포함하도록 선택 빌드를 다시 만들었다. 각 반영에서 manifest에 있는 경로만 원자적으로 교체했고 파일 삭제는 0개다. 최종 경로 수와 기존 파일 대조 수는 preflight/copy 기록에 보존한다.

최종 선택 빌드 산출물은 로컬 HTTP와 Tailnet HTTPS에서 각각 HTTP 200과 SHA-256를 대조했다. 빌드·백업·복사·HTTP 기록은 저장소의 같은 evidence directory에 JSON으로 보존한다. 마지막 Agent Browser 세션 종료 결과도 같은 곳에 기록한다.

- [공식 상태 대장](/spinon/STATUS.html) · [계획](/spinon/plan/c04-12-root-html-text-paint.html) · [내부 계약 0080](/spinon/internal/0080-c04-12-root-html-direct-text-paint.html)
- [구현·Simulator 근거와 화면](/spinon/internal/evidence/c04-12-root-html-text-paint-2026-10-18/README.html)

## 범위와 한계

Simulator 근거는 고정 `XX`/Ahem opt-in fixture, Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 결과다. 일반 앱 HTML text 렌더링, font loading, nested inline layout, 전체 C04.12/C15, 실기기, Android hardware GPU, iOS compositor 이후 픽셀을 완료로 주장하지 않는다. 전체 문서 사이트 배포나 GitHub Pages 변경도 수행하지 않았다.
