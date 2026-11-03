# Tailscale 로드맵 C11.1 동기화 PR diff 검토 · 30개 관점

검토 대상은 공개 `roadmap.html` C11.1 항목과 단일 파일 Tailnet 동기화 절차다. 이전 상태 동기화 PR 검토와 겹치지 않도록 HTML 정확성·반응형 표시·게시 경계 및 파일 교체 실패 경로를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 카드가 다른 기능이나 PR을 최신 작업으로 표시 | C11.1 parsed Root HTML dense placement와 PR #1117을 연결한다. PASS |
| 2 | merge 시각 또는 방식 불일치 | GitHub의 `2026-11-03T12:57:55Z` Rebase and merge와 맞는다. PASS |
| 3 | merge SHA 오기 | `f1043bda5ed69845b89992ba5f7cd28a03f42f5c`가 정확하다. PASS |
| 4 | follow-up docs PR 링크가 누락되거나 잘못됨 | PR #1118 링크를 추가했다. PASS |
| 5 | C11 전체를 완료로 오해 | C11.1/C11 미완료 문구를 유지한다. PASS |
| 6 | 공식 개수를 후보 결과에 맞춰 올림 | `<code>72/213</code>`를 유지한다. PASS |
| 7 | 14개 행렬을 지원 성공만으로 요약 | 성공 12·기대 거부 2를 각각 표시한다. PASS |
| 8 | authored node 수 혼동 | 60개 node로 실행 근거와 일치한다. PASS |
| 9 | geometry 결과를 허용 한계와 혼동 | 관측 최대 오차 0 CSS px를 기록한다. PASS |
| 10 | Android 표면 표본 수를 잘못 기록 | PixelCopy 표본 52/52 exact를 기록한다. PASS |
| 11 | iOS 표본을 compositor 이후 결과로 해석 | Metal drawable이 compositor 제출 전임을 표시한다. PASS |
| 12 | AVD 결과를 실기기나 hardware GPU 결과로 확대 | ANGLE/SwiftShader software renderer라고 밝힌다. PASS |
| 13 | 시뮬레이터 모델이나 OS 버전을 잘못 씀 | iPhone 17 Pro / iOS 26.2 Simulator로 일치한다. PASS |
| 14 | 빈 PR check을 CI 통과로 기록 | check rollup 없음과 CI 비주장을 명시한다. PASS |
| 15 | WPT 또는 전체 Grid 지원을 주장 | 그런 적합성을 주장하지 않는다. PASS |
| 16 | mixed-flow 증거를 양쪽 nesting 모두로 확대 | sparse parent 안의 dense child만 검사했다고 적는다. PASS |
| 17 | 페이지 내부 링크가 Tailnet에 없는 새 로컬 route로 이어짐 | 새 카드의 계획·계약·근거·상태는 main GitHub URL로 연결한다. PASS |
| 18 | 카드가 먼저 나와 최근 실행을 가림 | 신규 PR 카드를 가장 최근 C11.1 기록 앞에 삽입한다. PASS |
| 19 | HTML 삽입으로 section nesting이 깨짐 | 기존 `<p>` 구조 안에 닫힌 형제 문단으로 추가한다. PASS |
| 20 | 사용자 문자열이 markup이나 attribute를 탈출 | 고정 문자열과 알려진 URL만 쓰고 사용자 입력을 삽입하지 않는다. PASS |
| 21 | narrow viewport에서 긴 카드가 가로로 넘침 | 기존 `.hero-progress` wrapping CSS를 재사용하고 Agent Browser 390px viewport에서 문서 폭과 카드 bounds를 측정했다. PASS |
| 22 | 통계나 checkbox를 바꿔 C11 완료처럼 보이게 함 | hero count와 official checklist 값을 수정하지 않는다. PASS |
| 23 | preflight가 대상 파일을 덮어씀 | `preflight.json`은 `wx`로 한 번만 기록한다. PASS |
| 24 | 예상하지 않은 파일 여러 개를 교체 | route는 상수 `roadmap.html` 하나로 고정한다. PASS |
| 25 | 경로에 symlink 또는 비정규 파일이 들어옴 | 원본·대상·backup 읽기에서 regular file을 강제한다. PASS |
| 26 | 미병합 로컬 HTML이나 preflight 이후 바뀐 파일을 게시 | `origin/main`의 roadmap bytes와 비교하고 원본 및 이전 target 존재·크기·hash를 복사 직전에 재검사한다. PASS |
| 27 | 이전 파일이 backup 없이 손실 | 기존 파일을 고유 backup 경로에 `wx`로 먼저 보존한다. PASS |
| 28 | 중단된 복사가 target을 부분 기록 | 임시 파일을 같은 디렉터리에 쓴 뒤 atomic rename하고 결과 hash를 확인한다. PASS |
| 29 | delete 또는 기존 backup overwrite가 발생 | 삭제 명령이 없고 backup 경로가 이미 있으면 preflight가 중단한다. PASS |
| 30 | stale cache가 남아 다른 remote bytes를 확인 | cache-busting URL로 HTTPS를 요청하고 HTTP 200·byte length·SHA-256을 검증한다. PASS |

모든 30개 관점이 현재 diff에 통과했다. 화면 너비와 live Tailnet hash는 병합 뒤 Agent Browser 및 `verify` 실행으로 확인한다.
