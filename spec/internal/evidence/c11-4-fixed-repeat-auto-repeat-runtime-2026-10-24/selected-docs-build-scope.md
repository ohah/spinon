# 선택 문서 build와 Tailnet preview 범위

## 확인 결과

- 전체 `bun run docs:build`는 이번 변경 밖의 기존 `spec/internal/README.md`에서 RSPress `Dead link found` 오류로 실패했다. 같은 실패 로그에는 다수의 기존 evidence와 plan-review 문서가 이어진다. 이 오류는 새 C11.4 파일의 route를 지목하지 않았다.
- 실패 로그 원본은 로컬 ignored build 경로 `build/spinon/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/full-docs-build-pre-pr.log`에 보존하고, PR에는 요약만 둔다.
- 현재 변경 페이지와 링크된 선택 route는 동일 RSPress 설정으로 다시 렌더링했다. 선택 build는 기존 전체 dead-link 검사를 끄므로 저장소 전체 링크 검증으로 해석하지 않는다.
- Tailnet preview root는 저장소가 아니라 `~/Library/Application Support/SpinonPreview`다. 변경된 선택 route를 사전 manifest와 기존 파일 hash 백업으로 교체하고, 로컬·HTTP served hash를 대조한다. 파일 삭제는 하지 않는다.

## 72/213 집계

로드맵은 `spec/STATUS.md`에서 상위 작업, ID가 있는 하위 작업, ID 없는 표시 하위 체크를 읽는다. 현재 공식 표시 대상은 213개이며 그중 72개가 체크되어 있다. fixed-repeat + auto-repeat은 C11.4 intrinsic track sizing의 부분 진행 근거로만 기록했고 별도의 공식 완료 ID를 추가하지 않았다. C11.4 상위 체크는 미완료라 집계는 72/213이다. 이 값은 공개 지원 완료 API 수나 병합된 PR 수가 아니다.

## 한계

선택 build는 새 근거·상태 문서가 링크한 페이지와 정적 자산만 확인한다. 저장소 전체 dead link를 닫지 않으며, GitHub Pages를 배포하지 않는다.
