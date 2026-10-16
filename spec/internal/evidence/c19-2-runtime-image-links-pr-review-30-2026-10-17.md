# C19.2 실행 페이지 이미지 링크 PR 변경 검토 30개 관점 · 2026-10-17

검토 범위는 C19.2 simulator evidence의 네 상대 링크, 내부 명세 인덱스와 이 링크 수정 PR이다. 기능 코드·표본 데이터·공식 완료 상태는 변경하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | MDX가 PNG 경로를 package import로 해석하는가 | 이미지 경로 앞에 `./`를 붙여 문서 상대경로로 고쳤고 격리 Rspress build가 통과한다. 통과. |
| 2 | Android 화면 파일명이 바뀌어 링크가 끊기는가 | 기존 `android-screen.png` 이름과 대상 파일을 유지한다. 통과. |
| 3 | iPhone 화면 파일명이 바뀌어 링크가 끊기는가 | 기존 `iphone17pro-screen.png` 이름과 대상 파일을 유지한다. 통과. |
| 4 | iPad 화면 파일명이 바뀌어 링크가 끊기는가 | 기존 `ipad-pro-screen.png` 이름과 대상 파일을 유지한다. 통과. |
| 5 | 상대경로가 문서 폴더가 아닌 사이트 root 기준으로 바뀌는가 | 모든 이미지가 README와 같은 evidence 하위 폴더를 기준으로 한다. 통과. |
| 6 | rerun 폴더 링크만 고쳐 이미지 링크가 남는가 | 실행 폴더와 세 PNG 링크 네 곳 모두 `./` prefix를 갖는다. 통과. |
| 7 | 새 경로에서 실제 파일이 존재하지 않는가 | 세 이미지와 폴더가 저장소에 존재한다. 통과. |
| 8 | 경로 대소문자가 macOS에만 우연히 맞는가 | 문서 표기와 파일명의 대소문자가 정확히 일치한다. 통과. |
| 9 | 경로에 플랫폼별 separator를 넣는가 | POSIX `/` separator만 사용한다. 통과. |
| 10 | 화면 이미지의 캡처 내용이나 해시가 바뀌는가 | 변경은 Markdown target 문자열뿐이며 PNG 바이트는 건드리지 않는다. 통과. |
| 11 | alt text가 제거돼 이미지의 목적이 사라지는가 | Android/iPhone/iPad 각각의 alt text를 보존한다. 통과. |
| 12 | 화면 순서가 바뀌어 증거 대응이 흐려지는가 | 기존 Android→iPhone→iPad 순서를 유지한다. 통과. |
| 13 | README의 기존 run ID·pixel 결과를 수정하는가 | 실행 수치와 run ID는 바뀌지 않는다. 통과. |
| 14 | Android AVD를 실기기로 설명하는가 | 본문은 Android API 37 AVD와 PixelCopy라고 유지한다. 통과. |
| 15 | iOS drawable을 OS compositor 이후로 과장하는가 | Metal drawable 제출 전 readback이라는 경계를 유지한다. 통과. |
| 16 | Chrome 15/16을 제품 정답 완료로 해석하는가 | 제품 `16/16`과 Chrome `15/16`의 기존 구분을 유지한다. 통과. |
| 17 | wide-gamut 차이의 실제 채널 값이 바뀌는가 | blue 196/200 표기는 변경되지 않는다. 통과. |
| 18 | C19.2 완료 체크를 추가하는가 | `spec/STATUS.md`를 변경하지 않고 미완료로 둔다. 통과. |
| 19 | C08 회귀 수정 설명이 빠지는가 | 기존 C08 partial-alpha 경계 문장을 보존한다. 통과. |
| 20 | C19.2 제한 profile이 전체 CSS 지원으로 읽히는가 | 전용 opt-in profile이라는 기술 범위를 유지한다. 통과. |
| 21 | Rspress global config를 바꿔 모든 경로 해석을 느슨하게 하는가 | 저장소 config와 package 설정을 수정하지 않는다. 통과. |
| 22 | dead-image 검사 비활성화만으로 이미지 경로를 검증했다고 오해하는가 | 격리 config의 dead-image 경고는 비활성화됐지만 bundler가 세 PNG를 실제 import해 hashed `static/image` 파일을 출력했다. 경로 검증은 이 결과로 분리한다. 통과. |
| 23 | Rspress 버전을 올려 문제를 우회하는가 | 고정 `2.0.22`를 유지한다. 통과. |
| 24 | 사이트 base path를 변경하는가 | `/spinon/` base와 모든 기존 route를 유지한다. 통과. |
| 25 | raw Markdown 경로도 같은 디렉터리에서 열리는가 | `./`는 Markdown 상대 URL에서도 같은 파일을 가리킨다. 통과. |
| 26 | 내부 README에서 새 PR 근거를 찾을 수 없는가 | C19 인덱스 문단에 별도의 30관점 검토 링크를 추가한다. 통과. |
| 27 | 새 검토 문서 링크의 파일명이 틀리는가 | 인덱스 링크가 추가한 review 파일과 일치한다. 통과. |
| 28 | 변경에 무관한 CSS/JavaScript source를 수정하는가 | 앱 코드와 사이트 동작 파일은 바꾸지 않는다. 통과. |
| 29 | 종속성·lockfile·빌드 script 변경을 포함하는가 | 변경은 문서와 내부 문서 링크뿐이다. 통과. |
| 30 | PR 캡처가 별도 복사본이나 이전 화면만 보여 주는가 | PR 본문은 이 branch의 후보 roadmap 화면과 원본 C19.2 simulator PNG 3개를 직접 연결하고 Agent Browser에서 image load를 확인한다. 통과. |

초기 격리 build는 세 PNG import에서 경로 오류를 냈다. `./` 수정 뒤 Rspress `2.0.22` build에서 모든 페이지와 hashed image assets가 생성됐다. 전체 저장소 dead-link 검사는 이번 좁은 build의 판정에 포함하지 않는다.
