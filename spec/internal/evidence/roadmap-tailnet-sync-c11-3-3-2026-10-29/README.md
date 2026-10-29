# Tailnet 로드맵 동기화 근거 (2026-10-29)

## 게시 원본

- 저장소 `main`의 기준 commit은 `900404c95c57b60c3cc3e3306f2c488c2abc3586`, tree는 `2522d04bb844705e5b2741490b4c7c49edecf9bb`다. PR #793 병합 뒤의 문서 원본을 사용했다.
- RSPress 2.0.22 산출물 중 이번 C11.3.3 제한 slice와 연결된 경로를 선택해 게시했다. 생성 publication은 990 route, 18,439,539 bytes다. 전체 문서 링크 검증은 실행하지 않았다(`fullDocsLinkValidation: false`).
- 선택된 원본에는 로드맵·상태 대장·계획·내부 계약 0059·C11.3.3 실행/리뷰 자료와 Android/iOS 캡처가 포함됐다. `selected-publication-pass03.json`의 route/hash 목록이 게시 집합을 정의한다.

## 복사와 무결성

- Preview 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview` 아래의 `/spinon/` 경로다. 실행 중이던 미리보기 서버 PID 28040을 그대로 사용했고 재시작하지 않았다.
- 사전 비교는 990개 경로에서 신규 961, 교체 13, 동일 16으로 판정했다. 기존 13개 교체 파일(총 4,552,085 bytes)은 별도 백업 경로에 먼저 보존했다. 대상 파일 삭제는 없었다.
- 복사 직후와 별도 로컬 재검증에서 990/990 대상 SHA-256 및 13/13 백업 SHA-256이 원본과 일치했다. 열려 있는 대상 파일 핸들은 관찰되지 않았다.
- HTTPS 제공 검증은 `https://macstudio.tailed42f2.ts.net/spinon/` 아래 990/990 경로가 HTTP 200이고, 응답 SHA-256도 게시 원본과 990/990 일치했다. 실패 경로는 0개다.

## Agent Browser 확인

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html) 첫 카드에서 PR #790/#791과 merge SHA, Android/iOS 장면 결과 및 한계, 공식 `72/213`을 확인했다. 네 개 연결 경로(상태 대장, 계획, 실행 근거, 로드맵 리뷰)가 모두 HTTP 200이었다.
- 카드의 Android와 iOS 이미지 링크가 각각 1080×2400, 1206×2622로 로드됐다. [로드맵 화면 캡처](tailnet-roadmap-agent-browser-pass03.png) SHA-256: `ebdae9cbd894527a0173ffa645a77675a10c142efb171c19cb60dd6b01f6f1a4`.
- [상태 대장](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)의 체크박스는 C11, C11.3, C11.3.3 candidate, C11.4가 미체크이고 C11.2.2와 C11.2.3은 체크 상태였다. 따라서 C11의 제한 slice 병합을 상위 완료로 승격하지 않았고 공식 수 `72/213`도 유지했다. 원본 DOM 판독은 `agent-browser-status-checkboxes-pass03.json`에 보관했다.
- Agent Browser 세션 `spinon-c1133-roadmap-20261029`를 닫았다. 종료 뒤 세션 목록은 비어 있었고 `agent-browser` 프로세스도 남지 않았다.

## 관련 실행 한계

- 이 동기화는 문서 미리보기 게시 확인이며 제품 적합성 검사가 아니다. Android 결과는 API 37 ARM64 AVD의 SwiftShader software renderer이고, iOS 결과는 Simulator의 compositor 제출 전 Metal drawable 표본이다. 실기기 GPU나 iOS compositor 이후 화면을 입증하지 않는다.
- 전체 `spinon-style` suite는 247개 중 246개 통과했고 percentage auto-repeat track sizing 기존 실패 1개가 남아 있다. PR #790 시점 GitHub check rollup은 비어 있었다. 이 동기화 근거는 이를 성공으로 표현하지 않는다.

## 원본 기록

- 계획·변경 파일과 해시: `selected-publication-pass03.json`
- 사전 비교: `preflight-pass03.json`
- 복사·백업 결과: `copy-results-pass03.json`
- 로컬 재검증: `local-verification-pass03.json`
- HTTPS 응답·해시: `served-verification-pass03.json`, `served-verification-run-pass03.log`
- Agent Browser 로드맵과 체크박스: `agent-browser-roadmap-pass03.json`, `agent-browser-status-checkboxes-pass03.json`
- 빌드 설정·선택·복사·검증 도구와 원본 로그는 이 디렉터리에 보존했다. RSPress 표 형식의 공백이 있는 재시도 빌드 로그는 바이트 그대로 gzip 보관했고, 압축 해제 결과가 원본과 같은 SHA-256인지 확인했다. 실패한 초기 실행 기록도 삭제하지 않았다.
