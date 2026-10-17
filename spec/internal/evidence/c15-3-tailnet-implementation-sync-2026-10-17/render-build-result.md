# 선택한 Tailnet 문서 렌더링 기록

- Source HEAD: `bbbe0d47e23c0c8214e38d48ed36ad8b2e8779e2` (병합된 C15.3 구현 PR #248).
- Rspress: `2.0.22`, 저장소 lockfile의 설치 버전을 사용했다.
- 입력: `/tmp/spinon-c15-3-doc-fullactual-20261017/spec`에 복사한 Markdown snapshot과 이번 동기화 근거 문서. TypeScript/JavaScript 문서 route는 기존 SSR의 `window` 의존 때문에 이 임시 snapshot에서 제외했다.
- 출력: `/tmp/spinon-c15-3-tailnet-implementation-out-final3-20261017`.
- 실행 (`packages/docs`에서): `SPINON_DOC_ROOT=/tmp/spinon-c15-3-doc-fullactual-20261017/spec SPINON_DOC_OUT_DIR=/tmp/spinon-c15-3-tailnet-implementation-out-final3-20261017 RSPRESS_PERSISTENT_CACHE=false ./node_modules/.bin/rspress build --config rspress.tailnet.config.ts`.
- 캡처를 페이지에 처음 삽입한 빌드는 이미지 경로에 `./`가 빠져 Rspack 모듈 해석에 실패했다. 실패 로그 `/tmp/spinon-c15-3-tailnet-implementation-build-final2-20261017.log`를 보존하고 경로를 고쳐 최종 빌드를 다시 했다.
- 최종 결과: exit code 0 (`/tmp/spinon-c15-3-tailnet-implementation-build-final3-20261017.log`). 로드맵, 상태 대장, 내부 색인, 0070 계약, C15.3 implementation evidence/review와 이번 Tailnet sync 페이지가 생성됐다.
- 외부 링크 검증과 원본 fixture·글꼴은 strict check 대상으로 쓰지 않았다. 선택 페이지 출력과 직접 연결된 static assets만 별도 manifest에 기록했다.

입력 snapshot에 있던 오래된 자료의 image path와 fenced-code label 보정은 `/tmp` 복사본에만 적용했다. Repository source나 생성된 Rspress output을 성공으로 가장하기 위해 원문 문서들을 일괄 수정하지 않았다. 저장소 strict build의 473개 Markdown dead-link 실패는 [`C15.3 strict docs build 결과`](../c15-3-unicode-shaping-impl-2026-10-17/docs-build-result.md)에 별도로 보존되어 있다.
