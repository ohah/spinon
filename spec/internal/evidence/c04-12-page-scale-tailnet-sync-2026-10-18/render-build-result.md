# 선택 문서 Rspress build 결과

- 실행 버전: `@rspress/core 2.0.22` (lockfile 고정)
- 실행 명령: `SPINON_DOC_ROOT=/tmp/spinon-c04-12-tailnet-selected-20261018/spec SPINON_DOC_OUT_DIR=/tmp/spinon-c04-12-tailnet-selected-20261018/out-final2 RSPRESS_PERSISTENT_CACHE=false mise exec -- ./node_modules/.bin/rspress build --config rspress.selected.config.ts`
- 대상: `STATUS`, C04.12 관련 계획 4개, 내부 계약 0077, 내부 인덱스, page-scale 구현 근거, Tailnet 동기화 근거, public 자산
- 결과: 성공. 선택 산출물은 16개 content HTML route와 `404.html`을 포함한다. 최종 stdout/stderr는 [selected-build.log](./selected-build.log)에 보존한다.
- 전체 저장소 docs build와 전체 링크 검증은 수행하지 않았다. 선택 config에서 dead-link 검사를 껐다.
