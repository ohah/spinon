# Tailnet 동기화 문서 렌더링 결과

## 빌드 입력

- Rspress: `2.0.22` (`packages/docs/package.json`과 `bun.lock` 고정 버전)
- 선택 root: `/private/tmp/spinon-c15-3-glyph-raster-tailnet-input-20261017`
- 선택 출력: `/private/tmp/spinon-c15-3-glyph-raster-tailnet-selected-final5-20261017`
- 설정: 이 임시 입력과 출력 경로를 지정하는 `packages/docs/rspress.tailnet.config.ts`. 저장소 기본 `rspress.config.ts`나 제품 문서는 수정하지 않았다.
- 선택 입력: `STATUS.md`, C15 계획, 내부 색인, 계약 0071, glyph raster 실행·리뷰 근거, 이번 Tailnet sync README·리뷰, 세 Agent Browser 캡처와 Rspress icon/logo.

## 전체 문서 빌드 결과

저장소 전체 문서 빌드는 기존 문서에서 dead link를 보고해 실패했다. dead-link 검사를 끈 전체 경로 렌더링도 legacy Shiki fence 이름 `rust,ignore`와 `./` 접두사가 없는 기존 이미지 경로를 처리하지 못해 실패했다. 두 실패는 이번 C15.3 변경 파일의 문법 오류가 아니며, 광범위한 기존 링크·fence를 바꾸거나 검사를 끄는 방식으로 감추지 않았다. 원본 실행 로그는 작업 환경의 `/tmp/spinon-c15-3-glyph-raster-docs-build-20261017.log`와 `/tmp/spinon-c15-3-glyph-raster-docs-tailnet-build-20261017.log`에 남아 있다.

## 선택 빌드

```sh
cd /private/tmp/spinon-c15-3-glyph-raster-tailnet-20261017/packages/docs
SPINON_DOC_ROOT=/private/tmp/spinon-c15-3-glyph-raster-tailnet-input-20261017 \
SPINON_DOC_OUT_DIR=/private/tmp/spinon-c15-3-glyph-raster-tailnet-selected-final5-20261017 \
  bunx rspress build --config rspress.tailnet.config.ts
```

선택한 10개 문서 경로를 같은 Rspress 2.0.22로 생성했고 404 fallback은 별도로 출력됐다. HTML 산출물과 정적 asset의 전체 크기·빌드 시간·경로 목록은 [selected-build.log](selected-build.log)에 보존한다. 이는 선택 입력의 MDX/SSR 렌더링만 검증하며 전체 문서 링크 완전성이나 제품 동작을 검증하지 않는다.

## Agent Browser 및 동기화 경계

선택 출력과 Tailnet 정적 미리보기에서 roadmap, 공식 상태 대장, 내부 계약 0071의 링크와 문구를 Agent Browser로 확인했다. 세 캡처는 같은 디렉터리의 `tailnet-*-agent-browser.png`다. 주 브라우저와 빈 보조 세션을 닫았고, 확인 직후 Agent Browser 프로세스는 남아 있지 않았다. 기존 Tailnet listener를 제공하는 PID `1191`은 살아 있었다.

이 렌더링과 복사는 C15 구현을 추가하지 않고 완료 체크 수를 바꾸지 않는다. WGPU text draw, runtime text admission, glyph ink/clip/hit-test, color glyph, Android/iOS Simulator 결과는 포함하지 않는다.
