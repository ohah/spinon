# 선택 문서 Tailnet 렌더 결과

- 기준 commit: `5cff677cb6dce5a96bdbeb083a8ae566c04da30a` (PR #335 Rebase and merge)
- RSPress: 프로젝트 고정 버전 `2.0.22`
- 최종 명령: `mise exec -- bun run docs:build -- --config rspress.tailnet-implementation-pass-4-20261019.config.ts`
- 최종 재렌더(pass 4): exit code `0`; Markdown route 13개와 정적 roadmap 1개, 404 문서 및 assets를 포함한 출력 파일 42개다. 원본 log는 [`render-build-pass-4.log`](render-build-pass-4.log)에 있다.
- 범위: C11 상태 대장, conformance, Grid 계획·계약, C11.4 계획·oracle, text intrinsic 실행·검토 근거와 정적 roadmap. 관련 raw fixture·JSON·log도 preview 경로에 복사한다.
- Tailnet 페이지 검토에서 상태 대장의 C11.4 계획 링크가 `/spinon/` 범위 밖으로 이동하는 것을 발견했다. [`spec/STATUS.md`](https://github.com/ohah/spinon/blob/main/spec/STATUS.md)의 해당 링크를 canonical GitHub blob URL로 수정했고, 최종 브라우저 DOM에서 목적지를 확인했다. C11.4 체크는 미완료로 유지했다.
- 이전 pass 1–3에서 evidence/review의 fixture·font·image·plan 상대 링크 문제를 발견해 canonical GitHub blob URL로 수정했다. 각 pass 결과를 별도 JSON·log로 보존했다.
- 이 선택 렌더 성공은 전체 문서 dead-link/image 검사나 전체 문서 build 성공을 뜻하지 않는다. GitHub Pages 배포는 하지 않는다.
