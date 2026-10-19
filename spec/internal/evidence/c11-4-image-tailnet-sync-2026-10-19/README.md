# C11.4 PR #351 뒤 Tailnet 로드맵 미리보기 갱신

**기준 원본:** PR #351 rebase 병합 commit `f339f77c5bf1dae7b0a9b8502479136f6647c83c`.<br>
**범위:** public roadmap의 C11.4 image intrinsic 9-scene 요약, 내부 근거 색인, 선택 문서 묶음과 해당 실행 근거의 Tailnet 정적 미리보기 갱신. GitHub Pages를 배포하지 않고 대상 파일을 삭제하지 않는다.

## 갱신 내용

- public roadmap에 PR #351의 9-scene image intrinsic 결과와 24×16 natural size, fixed-track overflow 표본, 미완료 범위를 추가했다. 기존 Ahem text intrinsic 결과와 공식 C11.4 상태는 유지한다.
- 내부 색인에서 새 동기화 근거를 연결한다. 앱 검증은 [PR #351](https://github.com/ohah/spinon/pull/351)의 별도 evidence가 기준이다.
- [최종 선택 빌드](./selected-build-pass5-final.log)는 RSPress `2.0.22`로 roadmap, 상태 대장, C11 계획·계약, 내부 색인과 C11.4 image evidence를 구성한다. 전체 문서 사이트 빌드는 하지 않는다.

## 동기화 근거

- 선택 root 구성과 호스트 경로 제거: [준비 스크립트](./prepare-selected-root.mjs), [RSPress 설정](./rspress.tailnet-sync.config.ts).
- pass-4 화면·근거 묶음 151개 파일은 [사전 비교](./preflight-pass4.json), [복사 hash](./copy-verification-pass4.json), [기존 파일 backup](./backup-verification-pass4.json), [Tailnet HTTP 응답 hash](./served-verification-pass4.json)에서 모두 대조했다. 24개 추가, 17개 교체, 110개 동일이며 삭제는 없다. 교체 전 파일은 pass별 backup 경로에 보존한다. 이 증거와 화면은 [preview inputs SHA-256 목록](./SHA256SUMS-preview-inputs.txt)에도 포함한다.
- Agent Browser가 실제 Tailnet URL에서 확인한 [로드맵](./tailnet-roadmap-agent-browser.png), [상태 대장](./tailnet-status-agent-browser.png), [C11.4 계획](./tailnet-plan-agent-browser.png) 캡처를 포함한다. 사용한 Agent Browser 세션과 그 프로세스는 캡처·확인 후 종료했다. 최종 pass-5는 다음 문서 검토 페이지와 preview-input manifest를 추가해 다시 동기화한다. pass-5 최종 hash 결과는 PR evidence에 기록한다.

## 검토와 범위

- [동기화 실패 관점 20개](./sync-review-20.md)
- [PR diff 실패 관점 30개](./pr-delta-review-30.md)
- [미리보기 입력 SHA-256 목록](./SHA256SUMS-preview-inputs.txt)은 [생성 스크립트](./write-preview-inputs-manifest.mjs)로 만든다. 선택된 문서·근거 원본의 저장소 루트 기준 경로와 hash를 기록하며, 저장소 루트에서 `shasum -a 256 -c spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19/SHA256SUMS-preview-inputs.txt`로 확인할 수 있다. manifest 자체와 pass-5 준비 이후 생성되는 결과 로그·동기화 보고서는 제외하며, 해당 pass-5 결과는 별도 동기화 검증 및 최종 evidence checksum으로 확인한다.

이 작업은 정적 Tailnet 미리보기만 갱신한다. Android/iOS 앱을 여기서 실행하지 않았고, image paint나 C11.4 완료를 뜻하지 않는다.
