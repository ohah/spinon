# Rspress 이미지 상대 경로 검증 · 2026-10-28

## 결과

RSPress 2.0.22 선택 빌드에서 flexible-track README의 Android·iOS screenshot 두 개가 정상적으로 asset으로 번들링됐다. Markdown image URL을 문서 디렉터리에 대한 명시적 상대 경로 `./runs/...`로 바꾼 뒤 Rspress가 `/spinon/static/image/` 아래에 fingerprint된 두 PNG asset을 생성했다.

빌드한 source commit은 `7ccdc231715b37f1800c2edbaa81ed291351693e`다. selected root에는 73개 tracked source와 146개 raw route를 넣었다. 두 README alias(`/spinon/internal/.../README.html`, `/spinon/spec/internal/.../README.html`)에서 각 asset이 모두 resolve됐고, 생성 asset의 SHA-256이 원본 screenshot과 일치했다. 세부 경로·hash·source tree는 [검증 JSON](./rspress-image-resolution-2026-10-28.json), 원본 출력은 [Rspress build log](./rspress-image-resolution-build.log), 선택 source inventory는 [selected-root manifest](./rspress-image-resolution-source.json)에 있다.

검증은 이 두 screenshot asset의 Rspress 해석과 바이트 동일성만 다룬다. 전체 문서 링크, 모든 route, Tailnet 게시 상태 또는 screenshot pixel equality를 증명하지 않는다.
