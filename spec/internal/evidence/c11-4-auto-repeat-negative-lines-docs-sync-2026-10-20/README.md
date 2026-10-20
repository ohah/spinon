# C11.4 PR #385 문서·로드맵 동기화 근거

2026-10-20에 병합된 구현 PR #385의 상태와 근거를 계획, 상태 대장, 내부 계약, 내부 인덱스, 공개 로드맵에 반영한다. 이 문서 변경은 제품 코드나 C11.4 완료 판정을 바꾸지 않는다.

## 원본 구현 근거

- PR: https://github.com/ohah/spinon/pull/385
- Rebase and merge SHA: 078f5708a5c13d956150b311d392490cc4faa311
- 구현 실행 자료: ../c11-4-negative-implicit-track-sizing-2026-10-20/README.md
- Android API 37 ARM64 AVD: 124/124 scene, 676 trace chunks, draw/present 124/124
- iPhone 17 Pro / iOS 26.2 Simulator: 124/124 scene, 1,284 trace chunks, draw/present 124/124
- 세 overflow scene은 각 플랫폼에서 PixelCopy/Metal drawable 표본을 각각 5/5 exact로 비교했다.

이번 sync가 기록하는 지원 범위는 축별 제한 auto-repeat 하나, expanded repeat count 기준 numeric positive/negative line placement, 제한된 finite non-negative px grid-auto sizing, leading implicit track의 역방향 list cycle, auto-fit 빈 반복 track과 인접 gutter collapse다. Full repeat grammar, WPT, 실기기, hardware GPU, iOS compositor 이후 표면, 전체 C11.4/C11 완료는 포함하지 않는다.

## 변경 파일

- plan/c11-4-auto-repeat-negative-lines.md
- plan/c11-4-grid-track-sizing.md
- plan/c11-4-negative-implicit-track-sizing.md
- spec/STATUS.md
- spec/internal/0059-c11-grid-layout.md
- spec/internal/README.md
- spec/public/roadmap.html

C11.4와 C11 상태는 unchecked이며 공식 완료 수는 바뀌지 않는다. 새 공개 API는 없고 내부 계약 숫자 버전은 0.1.0으로 유지한다. GitHub required status check이 설정되지 않았으므로 CI green을 주장하지 않는다.

## 문서 빌드 메모

마지막으로 실행한 mise exec -- bun run docs:build는 exit 1이었다. 이 실행은 plan 경로를 GitHub 원문 링크로 바꾸기 전이어서 그 이전 링크 오류도 포함한다. 전체 저장소의 기존 dead-link 진단도 다수 있었으며, 마지막 공개 로드맵 경로 수정 뒤 빌드는 다시 실행하지 않았다. 문서 빌드 통과로 보고하지 않는다.

## 이 sync 검토

- [문서·계약·상태 20개 실패 관점](./docs-sync-review-20.md)
- [최종 PR diff 30개 관점](./pr-diff-review-30.md)
