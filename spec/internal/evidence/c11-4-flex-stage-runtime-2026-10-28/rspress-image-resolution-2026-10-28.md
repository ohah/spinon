# Rspress 이미지 상대 경로 검증 · 2026-10-28

## 결과

RSPress 2.0.22 선택 빌드에서 flexible-track README의 Android·iOS screenshot 두 개와 이 보고서 링크가 정상 resolve됐다. Markdown image URL을 문서 디렉터리에 대한 명시적 상대 경로 `./runs/...`로 바꿨고, Rspress는 `/spinon/static/image/` 아래 fingerprint된 PNG 두 개를 생성했다.

검증 source commit은 `5d2ec5a5f2fbf4afd5d6bbec06353ad17d2e105b`다. selected root의 tracked source 78개와 raw route 156개를 빌드하고, 선택 publication 227개에서 두 README alias(`/spinon/internal/.../README.html`, `/spinon/spec/internal/.../README.html`)와 report/JSON 링크를 확인했다. 양쪽 페이지의 Android·iOS asset byte hash는 원본 simulator screenshot과 각각 일치한다. 세부 경로와 hash는 [검증 JSON](./rspress-image-resolution-2026-10-28.json), [Rspress build log](./rspress-image-resolution-build.log), [selected-root manifest](./rspress-image-resolution-source.json)에 있다.

검증은 이 두 screenshot asset과 두 README alias의 report link만 다룬다. 전체 문서 링크, Tailnet 게시 상태 또는 screenshot pixel equality를 증명하지 않는다.
