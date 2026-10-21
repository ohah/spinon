# C11.4 실기기 근거 링크 수정 적대 검토

검토 범위는 실기기 근거 README의 코드·oracle·로그·캡처 URL과 공개 로드맵의 Android 화면 URL이다. 기존 PR #415의 구현 검토 20개와 diff 검토 20개를 재사용하지 않는다.

| # | 실패 관점 | 판정과 처리 |
| ---: | --- | --- |
| 1 | 문서 URL과 정적 파일 URL이 같은 `/spec/...` 경로를 쓰면 자산이 404가 되는가 | **수정.** 문서 페이지는 `/spinon/spec/internal/.../README.html`, 공개 정적 근거는 `/spinon/internal/evidence/...`로 분리했다. |
| 2 | GitHub 저장소의 README 상대 경로가 Tailnet 기준으로 잘못 해석되는가 | **수정.** Chrome oracle과 사전 비교 문서는 GitHub `blob/main` 절대 URL로 연결했다. |
| 3 | 코드 링크의 `../../../../crates`가 Tailnet 사이트 안에서 존재하는 척하는가 | **수정.** 세 source link 모두 GitHub 저장소의 실제 파일 경로를 가리킨다. |
| 4 | 사이트 base path `/spinon/` 누락으로 호스트 루트에 요청하는가 | **수정.** 내부 자산은 `/spinon/internal/evidence/`를 사용한다. |
| 5 | base path를 두 번 붙여 `/spinon/spinon/`을 만드는가 | **통과.** 변경 URL은 `/spinon/`을 한 번만 포함한다. |
| 6 | 경로 깊이에 따라 `../`가 다른 정적 위치로 풀리는가 | **수정.** 근거 자산 링크를 문서 위치에 의존하지 않는 절대 경로로 바꿨다. |
| 7 | `README.html` 문서 route를 정적 자산 route로 잘못 바꾸는가 | **통과.** 로드맵의 근거 문서는 기존 RSPress 문서 URL을 유지한다. |
| 8 | `.md` 검토 문서 링크가 `.html` 재작성이나 페이지 생성에 의존하는가 | **수정.** Tailnet의 공개 자산 복사본 URL을 직접 가리킨다. |
| 9 | Chrome JSON이 selected site root에 없어 preview에서 빠지는가 | **수정.** Tailnet 밖의 oracle은 GitHub의 저장소 파일로 연결한다. |
| 10 | GitHub source URL이 PR 브랜치 삭제 뒤 사라지는가 | **통과.** source/oracle 링크는 `main`의 기존 파일을 가리키며, 검증 재실행 산출물은 Tailnet evidence route를 쓴다. |
| 11 | 한국어 anchor가 URL fragment에서 깨지는가 | **통과.** 비교 문서 heading anchor를 포함했고 주소에 path 인코딩 공백은 없다. |
| 12 | PNG 파일명이 대소문자 차이로 macOS에서만 우연히 통과하는가 | **통과.** URL은 실제 저장된 소문자 파일명과 일치한다. |
| 13 | 이전 실패 로그와 성공 로그가 서로 바뀌는가 | **통과.** 실패 APK hash `b9c803…`와 통과 APK hash `d67d89…`의 run directory를 따로 유지한다. |
| 14 | 성공 결과 JSON이 다른 scene의 요약을 보여주는가 | **통과.** 새 결과는 `spanning-auto-min-flex-tracks` 단일 scene, `sceneCount=1`이다. |
| 15 | 새 Android run의 설치 hash 파일이 문서에서 빠지는가 | **통과.** 로드맵 문서에는 결과와 화면 URL을, 상세 근거에는 설치 hash URL을 연결한다. |
| 16 | 새 Android screenshot URL이 새 run ID와 맞지 않는가 | **통과.** 이미지·로그·결과 링크가 동일한 `c114-phys-recheck-…` 디렉터리를 사용한다. |
| 17 | 이전 Android screenshot 링크가 실제 화면 대신 HTML fallback을 보여주는가 | **통과.** pass34 preview에서 잘못된 `/spec/internal/...png`는 HTTP 404였고 수정 대상 `/internal/evidence/...png`는 1080×2340 PNG로 표시됐다. 새 재실행 화면은 pass35에서 다시 확인한다. |
| 18 | iOS 결과를 Android 실기기 결과로 오인하게 되는가 | **통과.** iOS 링크는 별도 `ios-simulator-…` 디렉터리에 있고 본문도 Simulator라고 표기한다. |
| 19 | iOS 이미지가 Android 이미지 URL을 재사용하는가 | **통과.** iOS screenshot은 iOS run directory를 직접 가리킨다. |
| 20 | screenshot 파일이 링크 문자열만 있고 selected root에 복사되지 않는가 | **통과.** selected-root 준비기는 evidence 디렉터리를 `internal/evidence`, `spec/internal/evidence`, `public/internal/evidence`로 각각 복사한다. pass35에서는 세 트리 hash와 실제 route를 다시 확인한다. |
| 21 | 실기기 결과가 emulator에서 나온 것으로 보일 수 있는가 | **통과.** 재실행 metadata는 serial `R5KYB06SMPE`, model `SM-S731N`, `qemu=0`을 기록한다. |
| 22 | 실패한 장치 선택이 다른 연결 emulator로 조용히 넘어가는가 | **통과.** 재실행 도구에 실기기 serial을 명시했고 해당 serial의 qemu 값을 검사했다. |
| 23 | 새 결과가 기존 APK와 다른 APK를 설치했는가 | **통과.** local/install SHA-256 모두 `d67d89fb…bee8922`로 일치한다. |
| 24 | Android 결과가 layout 준비만 하고 draw/present 전에 종료됐는가 | **통과.** 결과에 WGPU draw/present true와 superseded retry 0이 기록됐다. |
| 25 | 48/48 track 값과 frame 값이 서로 다른 run에서 조합되는가 | **통과.** result JSON의 단일 observation에 tracks와 root/item frames가 같이 저장된다. |
| 26 | 과도하게 긴 run-kind가 앱의 80자 run ID 검증에서 실패하는가 | **수정.** 시험 호출 중 `ARGUMENT_ERROR`를 재현했고 run-kind를 `phys-recheck`로 줄여 같은 장면을 통과시켰다. |
| 27 | item box 캡처를 200px 텍스트 overflow의 픽셀 증거로 과장하는가 | **통과.** 문서와 로드맵은 글리프·overflow pixel 검증을 명시적으로 제외한다. |
| 28 | 이번 링크 보강을 C11.4 전체 지원 완료로 보이게 하는가 | **통과.** C11.4 check와 완료 집계를 미완료로 유지한다. |
| 29 | 링크 수정과 함께 공식 상태·로드맵 완료 수가 바뀌는가 | **통과.** `spec/STATUS.md`와 체크 수에는 변경이 없다. |
| 30 | 새 preview를 확인하지 않고 원격 페이지 검증을 주장하는가 | **통과.** PR은 pass34 이후 Tailnet이 아직 갱신되지 않았음을 구분한다. pass35 sync와 Agent Browser 검증은 병합 후 delivery gate로 따로 수행하며, 이 PR에서 원격 갱신을 완료했다고 주장하지 않는다. |

PR #416은 source link와 로컬 자산 존재를 검토한다. Tailnet pass35는 병합 뒤 별도 동기화·Agent Browser 확인으로 검증한다.
