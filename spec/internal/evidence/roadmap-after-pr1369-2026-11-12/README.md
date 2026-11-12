# PR #1369 병합 후 C11.4 상태·Tailnet 로드맵 동기화

구현 PR [#1369](https://github.com/ohah/spinon/pull/1369)의 실제 merge SHA `79cc6328fb61e0babbb6d55c164485145f0058f5`와 실행 범위를 상태 대장, 내부 계약 0059, mixed-minimum 계획, track-sizing 순차 계획, 공개 roadmap에 반영했다. C11.4/C11은 미완료이며 공식 완료는 `78/214`, 내부 계약은 `0.1.0`으로 유지한다.

## 선택 문서 빌드

- Rspress `2.0.22`로 12개 선택 경로를 빌드했다. 전체 docs site build나 전체 dead-link 검사는 실행·주장하지 않는다.
- `selected-root.json`은 source commit `65894cc451cf71069b6dac19da2114b7cfcb3003`, 129개 입력 파일, 6,742,859 bytes를 기록한다.
- `selected-site-verification.json`은 source commit `ef7f239a59d19595698071666a71b37b2b5dfb45`의 공개 roadmap marker, C11.4 NEXT card, 공식 `78/214` 및 빌드된 12개 문서 경로를 기록한다.
- [선택 빌드 로그](./selected-build.log) · [선택 경로 검증](./selected-site-verification.json).

## Tailnet preview

사용자가 지정한 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 선택 동기화했다. preview server PID `90711` / port `8768`은 기존 서버로 확인했고 재시작·종료하지 않았다.

- Pass 01은 168개 경로 중 148개를 변경하고 14개 기존 파일을 전용 backup에 남겼다. Agent Browser가 실제 페이지를 확인하는 중 dynamic C11.4 NEXT card에 낡은 `#1365` stage가 남았음을 찾아 원본 HTML에서 `#1369`로 수정했다. Pass 01 보고서는 중간 상태로 이름을 구분해 보존했다.
- Pass 02는 수정한 roadmap HTML 한 개를 교체했고 나머지 167개 경로는 동일했다. Pass 02 전용 backup 1개를 추가했으며 삭제는 0개다. Pass 01·02의 15개 backup은 각각의 원본 SHA-256과 일치한다.
- 최종 168/168 경로에서 source, preview file, local HTTP, Tailnet HTTPS의 byte count와 SHA-256이 일치한다. 두 HTTP origin 모두 168/168 status 200이며 전체 docs build/dead-link 검사는 포함하지 않는다.
- [최종 copy manifest](./tailnet-copy-manifest-pass02.json) · [최종 served verification](./served-verification-pass02.json) · [Pass 01 intermediate record](./tailnet-copy-manifest-pass01.json) · [selected sync script](./sync-preview.mjs) · [verification script](./verify-served-files.mjs) · [Tailnet sync PR diff review 30](./tailnet-sync-pr-diff-review-30.md).

## Agent Browser 확인

전용 Agent Browser namespace/session으로 공개 roadmap, mixed plan, 내부 계약 0059, `STATUS.html`을 열었다. 최종 페이지는 PR #1369와 merge SHA, 공식 `78/214`, C11.4/C11 미완료, `C11.4 · NEXT · #1369 merged · next candidate not selected`를 표시한다. 1440×1000 desktop 및 390×844 mobile 화면에서 page-level document width는 viewport와 같다. Mobile 칸반은 페이지에 정의된 가로 스크롤 영역을 유지한다.

- [Desktop Tailnet 화면](./tailnet-roadmap-desktop-agent-browser.png), SHA-256 `fff1f11a531bab7dd2bcfdb31c6b3b716a8bad2473b1d51cae6436e47521c68a`
- [Mobile Tailnet 화면](./tailnet-roadmap-mobile-agent-browser.png), SHA-256 `2ad7825f49b1ad8eeeca6f6e68b033efa91bcf90980c0fd64bc229f23bda1f8d`
- [Agent Browser page verification](./tailnet-agent-browser-verification.json) · [process cleanup record](./resource-cleanup.json).

완료 뒤 Agent Browser 세션을 닫고, 이 검증에서 시작된 ADB server가 남지 않게 종료했다. 새로 만든 Rspress build output 16,226,641 bytes / 168 files도 검증 뒤 제거했다. 사용자 Tailnet preview server와 공유 simulator profiles는 유지했다.

이번 문서 동기화는 제품 런타임 코드를 수정하지 않았다. PR #1369의 GitHub Actions workflow runs와 commit status checks는 각각 0개라 CI green으로 기록하지 않는다.
