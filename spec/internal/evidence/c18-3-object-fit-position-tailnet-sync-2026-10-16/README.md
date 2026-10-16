# C18.3 Tailnet 미리보기 동기화 근거

## 대상과 입력

C18.3 구현 PR [#224](https://github.com/ohah/spinon/pull/224)은 `ad243765aad2b2dc45bb461d23f2c0061672037a`로 2026-10-16 11:36 UTC에 rebase merge됐다. 이번 동기화는 그 구현 계획·상태·0066 계약·실행 근거와 관련 fixture 자료를 Tailnet 문서 미리보기에 반영했다. 빌드 소스 tree는 이미지 경로 수정 커밋 `1acfc692184e05371a05a097b5b605327be6c97c`의 `d4348c61a0ab748450b8373b2619a16af3a75512`다. 이 tree는 PR에 추가된 동기화 근거 파일을 포함하지 않는다. 해당 파일은 Tailnet 소스 페이지를 바꾸지 않는다. 동기화된 근거 README의 Android/iOS 이미지 경로 두 곳은 Rspress가 상대 이미지로 해석하도록 `./`를 명시했다.

미리보기는 `https://macstudio.tailed42f2.ts.net/spinon/`이다. 상태 대장에 수동으로 완료 수를 입력하지 않았다. [로드맵 HTML](https://github.com/ohah/spinon/blob/main/spec/public/roadmap.html)은 GitHub `main`의 `spec/STATUS.md`를 가져와 표시한다.

## 빌드와 파일 반영

- 고정 버전 Rspress `2.0.22`로 C18.3 관련 페이지와 연결된 내부 자료만 빌드했다. [공백을 정리한 읽기용 빌드 로그](./rspress-scoped-build-output.txt)와 정확한 원본 바이트 gzip·SHA-256 기록을 보존했다.
- 저장소 Rspress 설정과 `bun.lock`을 수정하지 않았다. 전체 문서 사이트 배포나 전체 사이트 빌드가 아니다.
- 166개 경로, 7,421,423바이트를 계산해 복사했다: 신규 119개, 교체 16개, 동일 31개. 삭제는 0개다. 교체된 16개 원본은 `backup/c18-3-object-fit-position-implementation-tailnet-core-2026-10-16/`에 경로별로 보존했다.
- 이전 계획 전용 기록 [`sync-manifest-c18-3-object-fit-position-tailnet-2026-10-16.json`](https://macstudio.tailed42f2.ts.net/spinon/sync-manifest-c18-3-object-fit-position-tailnet-2026-10-16.json)은 보존했다. 이번 파일 목록·이전 hash·백업 경로는 [동기화 매니페스트](./manifest.json)에 있다. 같은 매니페스트의 [Tailnet 응답](https://macstudio.tailed42f2.ts.net/spinon/sync-manifest-c18-3-object-fit-position-implementation-tailnet-2026-10-16.json)은 HTTP 200, 75,449바이트, SHA-256 `b3b0c292e121e0ab00dcb30b6346ee58ca6e16195b831c7f99703bb89456a51d`였다.
- 공유용 매니페스트에서 서버 로컬 절대 경로를 지우며 앞서 복사한 사본은 같은 백업 root에 별도로 보존했다. 그 백업 URL도 HTTP 200과 기존 hash 일치를 확인했다.
- preview 서버를 재시작하지 않았고 GitHub Pages를 배포하지 않았다. Tailnet의 기존 preview service에 파일만 동기화했다.

이번 문서 변경과 동기화 기록을 대상으로 한 [PR 변경 검토](../c18-3-object-fit-position-tailnet-sync-pr-delta-review-30-2026-10-16.md)도 저장소에 연결했다.

## Agent Browser 확인

2026-10-16 20:44–20:46 KST에 Agent Browser 0.31.1로 실제 Tailnet 페이지를 열었다. [로드맵 첫 화면](./roadmap-summary-agent-browser.png)과 [상태 카운터·C18.3 행](./roadmap-c183-agent-browser.png)을 캡처했다. 후자에서 `201개 표시 · 공식 완료 70/201`과 C18.3 checked 상태를 확인했다. 이 수치는 확인 시각의 GitHub `main` 상태이며 이후 병합으로 달라질 수 있다.

계획, 내부 계약 0066, 상태 대장, 구현 실행 근거 라우트는 모두 HTTP 200으로 열렸고 제목과 C18.3 내용을 확인했다. [실행 근거 페이지 화면](./implementation-readme-agent-browser.png)에서 Android와 iOS 이미지도 깨짐 없이 나타났다. Agent Browser DOM에서 Android 이미지 `1080×2400`, iOS 이미지 `1206×2622`를 확인했다.

Agent Browser의 same-origin `fetch`와 Web Crypto SHA-256으로 동기화 대상 166개 경로를 대조했다. HTTP 200 및 기대 hash 일치 166/166, 실패 0개다. 매니페스트 응답도 별도로 200과 hash 일치를 확인했다. URL별 상태와 실제 hash는 [서빙 검증 기록](./served-verification.json)에 있다.

## 범위

이 근거는 Tailnet 정적 preview의 지정된 문서·이미지와 시각 검증을 다룬다. 전체 사이트 릴리스, GitHub Pages, 구현 변경, 전체 CSS reference suite 통과를 뜻하지 않는다. C18.3 실행의 플랫폼·fixture 한계와 기존 C11/C13 source-boundary 실패는 [구현 실행 근거](../c18-3-object-fit-position-implementation-2026-10-16/README.md)에 구분해 기록했다.
