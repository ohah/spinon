# C18.3 Tailnet 동기화 PR 변경 검토

검토 대상은 구현 실행 근거의 두 screenshot 상대 경로 수정과 `c18-3-object-fit-position-tailnet-sync-2026-10-16/`의 매니페스트·브라우저 확인·스크린샷·빌드 로그다. 각 항목은 다른 실패 경로나 경계를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 이미지 URL을 문서 위치가 아닌 사이트 root 기준으로 해석 | `./android/...`, `./ios/...`로 문서 디렉터리 기준을 고정했다. |
| 2 | 대소문자 차이로 플랫폼 screenshot 파일을 찾지 못함 | README 경로의 대소문자와 체크인 파일 이름이 일치한다. |
| 3 | 수정이 Android 이미지 하나에만 적용됨 | Android와 iOS 두 참조를 모두 수정했고 DOM에서 둘 다 로드됐다. |
| 4 | 수정 과정에서 원본 screenshot을 재인코딩·교체 | image path만 바뀌었고 실행 근거 PNG는 변경하지 않았다. |
| 5 | Android screenshot이 깨졌지만 페이지 상태만 통과 | `complete=true`, natural size `1080×2400`을 브라우저 DOM에서 확인했다. |
| 6 | iOS screenshot이 깨졌지만 Android만 확인 | `complete=true`, natural size `1206×2622`를 확인했다. |
| 7 | Rspress 이미지 번들링은 됐지만 브라우저 페이지는 이전 HTML 제공 | Tailnet 근거 route를 열고 두 hashed `static/image` URL의 실제 이미지를 확인했다. |
| 8 | 제목만 맞고 계획 본문이 이전 상태 | 계획 페이지 본문에서 구현 PR #224 연결과 C18.3 범위를 확인했다. |
| 9 | 내부 계약 route가 누락되거나 다른 번호를 제공 | `/spec/internal/0066-c18-3-object-fit-position.html`의 제목과 본문을 확인했다. |
| 10 | 상태표가 C18.3 이전 버전 | 상태 페이지에서 C18.3과 PR #224 연결을 확인했다. |
| 11 | 로드맵 HTML은 갱신됐지만 GitHub main 데이터를 못 가져옴 | Agent Browser 화면에서 완료 카운터 `70/201`을 확인했다. |
| 12 | 완료 수는 보이지만 C18.3 checkbox가 미완료 | 해당 checkbox의 DOM `checked=true`, `disabled=true`를 확인했다. |
| 13 | 초기 비동기 로딩 중 카운터가 비어 있는 순간을 캡처 | fetch가 끝난 뒤 카운터·201개 checkbox를 읽고 화면을 캡처했다. |
| 14 | 이전 roadmap count가 하드코딩돼 새 값을 가림 | 공개 HTML이 `main`의 STATUS를 읽는 구현을 유지하며 직접 값을 입력하지 않았다. |
| 15 | 카운터가 이후에도 고정값이라고 오해 | README와 JSON에 확인 시각과 live count라는 한계를 적었다. |
| 16 | 매니페스트의 source commit과 실제 파일 내용 불일치 | 소스 tree `d4348c61…`와 branch source commit `1acfc692…`을 기록했다. |
| 17 | implementation merge와 Tailnet source revision 혼동 | PR #224 merge commit과 동기화 source commit을 별도 필드로 기록했다. |
| 18 | scoped build를 전체 사이트 build로 과장 | Rspress log와 README에 C18.3 범위 build라고 명시했다. |
| 19 | 저장소 Rspress config를 몰래 변경 | 변경 파일 목록에서 config·lockfile 변경이 없음을 확인했다. |
| 20 | Rspress 버전을 떠 있는 최신 버전으로 사용 | CLI가 `2.0.22`를 출력했고 그 버전을 기록했다. |
| 21 | staged fixture JS가 문서 route로 실행되어 SSG 실패 | 브라우저 전용 JS를 route 입력에서 제외하고 관련 정적 입력만 빌드했다. |
| 22 | scoped build에서 dead-link 검사 비활성화를 전체 검사 통과로 오해 | 비활성화 설정은 임시 config에만 두고 명시된 라우트·이미지를 브라우저에서 직접 확인했다. |
| 23 | 매니페스트 경로 수와 created/replaced/unchanged 합계 불일치 | `119 + 16 + 31 = 166`을 확인했다. |
| 24 | 동일 파일을 백업 없이 덮어씀 | 교체 대상 16개의 이전 SHA를 기록하고 고유 backup root에 원본을 보존했다. |
| 25 | 교체 파일 백업이 원본과 다름 | apply 과정에서 각 backup SHA를 이전 destination hash와 대조했다. |
| 26 | copy-only 동기화가 기존 파일을 지움 | 계획과 적용에서 삭제 경로는 비어 있고 삭제 수는 0이다. |
| 27 | 이전 C18.3 계획 전용 매니페스트를 덮어씀 | 별도 이름을 사용하고 기존 계획 manifest의 경로와 hash를 유지했다. |
| 28 | 복사 후 브라우저에서 오래된 파일이나 잘못된 MIME/body가 제공 | 166 URL을 same-origin fetch로 읽고 HTTP 200 및 byte hash를 확인했다. |
| 29 | manifest 비교가 로컬 파일만 확인하고 실제 preview를 확인하지 않음 | 실제 Tailnet origin의 응답 byte를 Web Crypto SHA-256으로 비교했다. |
| 30 | preview 반영을 Pages 배포·서버 재시작·제품 검증으로 과장 | 서버 재시작과 Pages 배포는 없었으며 본 근거는 문서 preview만 다룬다고 분리했다. |

발견한 경로 문제는 README의 상대 이미지 경로로 수정했다. 이후 scoped Rspress build와 Agent Browser 이미지·라우트·hash 검증에서 같은 문제가 재현되지 않았다. 남은 제한은 Tailnet 표시가 확인 시각의 GitHub `main` 내용을 사용한다는 점과 구현 근거가 지정 AVD/Simulator fixture 범위라는 점이다.
