# Tailnet R07 동기화 변경 검토

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 동기화 원본이 아직 열린 PR branch를 가리키는가 | PR #204는 merge SHA `a328a69d8b5904f659b53666c944ae4e76cc8e53`로 main에 병합됐고 manifest도 그 commit을 가리킨다. |
| 2 | 로드맵 HTML이 `spec/public/roadmap.html`과 다른 임시 파일에서 생성되는가 | root와 `public/` 두 사본을 동일한 main 원본에서 복사해 SHA가 같다. |
| 3 | 두 roadmap URL 중 한 쪽만 예전 내용을 제공하는가 | Agent Browser에서 `roadmap.html`과 `public/roadmap.html` 모두 200 및 동일 SHA를 받았다. |
| 4 | 카운터가 문서에 수동으로 고정되어 완료 숫자만 올리는가 | HTML은 GitHub main `STATUS.md`를 요청해 계산한다. `62/197` 문자열을 임의 주입하지 않았다. |
| 5 | 상위 항목 수와 세부 작업 수를 섞어 총합이 달라지는가 | 화면은 완료 62/197, 상위 5/78, 세부 57개로 표시한다. |
| 6 | R07 체크 외 다른 완료 항목이 새로 체크되는가 | 상태 원본의 R07만 새 상위 체크이며 다른 체크박스는 바꾸지 않았다. |
| 7 | R07의 완료가 R03 또는 제품 지원 완료로 오인되는가 | R03은 unchecked, 제품 지원 완료 0개다. R07 설명은 가능성 조사 범위로 제한한다. |
| 8 | 카운터는 62지만 카드에 이전 요약이 남는가 | Agent Browser에서 Vue/Svelte 버전과 mount·click·unmount 조사 설명을 확인했다. |
| 9 | 제품 지원/API 완료로 보이는 문구가 생기는가 | 카드와 STATUS는 FFI·제품 adapter·CSS/GPU 화면 미완료를 명시한다. |
| 10 | 상태 Markdown이 PR 병합 전 snapshot인가 | preview의 `spec/STATUS.md`는 `origin/main`의 현재 파일이며 SHA가 manifest에 기록됐다. |
| 11 | 상태 HTML이 다른 STATUS 원본을 렌더링하는가 | Rspress 2.0.22 격리 build 입력은 main의 `spec/STATUS.md` 한 파일이다. |
| 12 | 빌드 경로가 문서 앱에서 작동하지 않는가 | root `/spinon/STATUS.html`을 Agent Browser에서 열어 R07 내용과 정상 제목을 확인했다. |
| 13 | nested `/spec/STATUS.html`도 표시되는 것으로 잘못 주장하는가 | 직접 navigation은 client 404를 보였다. README와 served 기록에서 제한을 명시했고, 로드맵의 공식 링크는 GitHub main이다. |
| 14 | 격리 build가 요구하는 CSS/JS/SVG 자산을 빠뜨리는가 | 생성 HTML의 참조를 수집해 asset 경로를 manifest에 포함했다. |
| 15 | 같은 이름의 기존 asset을 다른 내용으로 덮어 다른 문서를 깨뜨리는가 | 기존 경로의 SHA를 비교했다. 같은 해시 6개는 유지하고 새 해시 파일은 새 이름으로 추가했다. |
| 16 | 제공 페이지 또는 자산이 실제로 404인가 | 15개 대상 각각 Agent Browser fetch에서 HTTP 200을 받았다. |
| 17 | HTTP 200이 stale body를 숨기는가 | 15개 응답의 SHA-256 모두 manifest 기대값과 일치했다. |
| 18 | HTTP cache 때문에 이전 roadmap만 캡처되는가 | query nonce를 붙여 page를 열고 `no-store` fetch로 파일 해시를 계산했다. |
| 19 | 교체 전 파일이 복구 불가능하게 사라지는가 | 교체된 5개 파일을 고유 backup 경로에 복사한 뒤 atomic rename으로 반영했다. |
| 20 | 신규 자산이나 status page를 부분 기록하다 서버가 절반 파일을 읽는가 | 임시 파일을 쓴 뒤 rename했고 제공 중 파일 크기와 hash를 다시 확인했다. |
| 21 | 범위 밖 Tailnet 파일이 삭제되는가 | 삭제 명령은 없고 기존 hash-versioned asset도 보존했다. |
| 22 | 서버 재시작으로 사용자의 preview session을 끊는가 | 파일 sync만 했고 서비스 프로세스를 재시작하지 않았다. |
| 23 | 문서 sync를 GitHub Pages 배포라고 오인하는가 | workflow는 실행하지 않았다. 이 변경은 Tailnet preview 갱신뿐이다. |
| 24 | 전체 Rspress 성공을 잘못 보고하는가 | 전체 빌드 실패를 기록하고, dead-link/image 검사를 끈 임시 1-page build 성공만 분리했다. |
| 25 | dead-link 검사를 저장소 설정에서 영구 해제하는가 | temporary config만 썼고 제거했다. 추적되는 Rspress 설정에는 diff가 없다. |
| 26 | Agent Browser 캡처가 다른 시점의 화면인가 | 2026-10-16 05:22 KST에 counter 62/197과 R07 checked를 읽고 같은 상태를 캡처했다. |
| 27 | 이미지가 PR에 첨부된 뒤 내용을 검증할 수 없는가 | PNG 1680×1200 SHA가 served 검증 JSON에 저장되고 PR 본문에서 해당 파일을 표시한다. |
| 28 | 원래 dirty workspace 변경을 branch에 섞는가 | `origin/main` 기반 별도 worktree에서만 작업했고 사용자의 340개 dirty path는 수정하지 않았다. |
| 29 | 정적 문서 갱신을 simulator/device 검증으로 부풀리는가 | 이 sync에는 app runtime·simulator·device 검증이 없으며 README에서 제외 범위를 기록했다. |
| 30 | 병합 방식이 저장소 linear history 요구를 깨뜨리는가 | 저장소 규칙과 branch protection을 다시 확인하고 Rebase and merge를 사용한다. |

## 판정

로드맵의 공식 카운터와 R07 표시, root 상태 HTML, 대상 파일 hash를 확인했다. 전체 문서 build와 nested status URL의 한계는 성공 범위에서 제외했고 README에 남겼다.
