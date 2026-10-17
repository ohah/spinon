# Tailnet 선택 문서 렌더 결과

## 빌드 입력과 실행

- 구현 원본은 병합된 main `18a38c8a87135a2c7cb1df92ce1d4b7ec50afcc7`이다.
- Rspress는 저장소 lockfile의 `2.0.22`를 사용한다.
- 선택 root: `/private/tmp/spinon-c15-3-wgpu-tailnet-input-20261017/spec`.
- 선택 output: `/private/tmp/spinon-c15-3-wgpu-tailnet-selected-20261017`.
- 명령은 `packages/docs`에서 `SPINON_DOC_ROOT`, `SPINON_DOC_OUT_DIR`, `RSPRESS_PERSISTENT_CACHE=false`를 지정해 실행했다. 임시 `rspress.tailnet.config.ts`는 `markdown.link.checkDeadLinks=false`, `markdown.image.checkDeadImages=false`를 설정했다.
- 첫 scoped 실행에서 기본 dead-link 검사가 기존 페이지로 향하는 링크를 이 선택 root 안에서 찾지 못해 실패했다. 그 원본은 [selected-build-deadlinks.log](selected-build-deadlinks.log)에 보존했다. 저장소 전체 link/image 유효성을 통과했다고 주장하지 않는다.
- link/image 검사를 scoped build에서 끈 뒤 선택 Markdown route와 static output이 Rspress 2.0.22로 생성됐다. [최종 선택 build log](selected-build.log).

## 선택 범위

`STATUS.html`, `roadmap.html`, C15 계획, 내부 색인, 계약 0071·0072, C15.3 glyph raster/paint 실행 자료, Chromium 8059 source analysis, Tailnet sync README·30관점 검토·Agent Browser 캡처 6개 및 Rspress CSS/JS/search assets를 렌더했다. C19.1은 관측값 차이와 명세 오류 판정을 분리하는 보정 문구로 갱신했다. Roadmap이 runtime에서 가져오는 `STATUS.html`도 같은 selection에서 갱신된다. 공개 custom roadmap은 `spec/public/roadmap.html` 원본을 선택 산출물과 별도로 두 Tailnet route에 복사한다.

Markdown에서 직접 연결하는 PNG/log/fixture/font와 전체 C15.3 WGPU 실행 근거는 Rspress output과 별도로 선택된 경로에 복사하고, Tailnet 캡처 PNG도 별도 evidence asset으로 포함한다. core route와 그 asset의 경로 closure는 manifest 및 served verification으로 검사한다. 이 빌드는 저장소 전체 473개 Markdown 문서의 strict check 또는 전체 docs build 성공이 아니다.

## 제공 확인

복사 직전 target inventory, copy-only 실행, 덮어쓰기 backup, 선택 경로 복사 후 hash와 local listener HTTP 응답을 이 evidence의 JSON 파일에 기록한다. Agent Browser에서 실제 Tailnet 로드맵의 C15.3 진행 내용, 공식 완료 수와 미완료 상태, paint evidence route를 확인한다. Agent Browser 종료 뒤에도 listener `127.0.0.1:8768`이 유지되는지 확인한다.
