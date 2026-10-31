# PR #948 병합 후 상태·로드맵 동기화 검토 30개

기준은 PR #948의 GitHub 병합 사실과 고정 실행 evidence다. 검토 대상은 `spec/STATUS.md`, 내부 계약 0059, overflow-wrap 계획 및 `spec/public/roadmap.html`의 문서 변경이다. 구현 코드 검토 20개와 PR #948 구현 diff 검토 30개를 재사용하지 않았다.

| # | 별도 실패 경로·불변 조건 | 결과 |
|---:|---|---|
| 1 | 계획서가 병합된 PR을 여전히 open으로 표시 | 수정 확인 |
| 2 | 상태 대장의 PR 번호가 다른 작업으로 연결 | #948 링크 확인 |
| 3 | 병합 시각이 GitHub 기록과 다름 | 2026-10-31T22:23:22Z 확인 |
| 4 | merge SHA가 GitHub merge commit과 다름 | `499dd1c7ef92362b39ed228ad900ac7c7b464948` 확인 |
| 5 | 문서가 원래 head SHA를 merge SHA로 기록 | merge SHA만 결과로 표시 |
| 6 | 빈 check rollup을 CI 성공으로 과장 | 성공으로 기록하지 않음 |
| 7 | required checks 부재 사실을 누락 | main 설정 부재를 별도 기록 |
| 8 | `72/213` 완료 수가 잘못 증가 | 네 문서 모두 72/213 유지 |
| 9 | C11 또는 C11.4 상위 checkbox가 완료로 바뀜 | 미완료 유지 |
| 10 | fixture slice를 일반 CSS admission으로 확대 | opt-in fixture 한정 명시 |
| 11 | 테스트 입력 글꼴/크기/문자열이 runtime 기록과 다름 | Ahem 10px/400 `XXXX` 일치 |
| 12 | overflow-wrap allowlist에 다른 값이 추가 | 세 값만 기록 |
| 13 | min-content anywhere 기대값이 40px로 기록 | 10px로 기록 |
| 14 | min-content break-word가 anywhere처럼 처리 | normal과 같은 40px로 기록 |
| 15 | max-content scene의 결과가 변형 | 세 값 모두 40px/10px로 기록 |
| 16 | fixed 10px normal overflow가 wrap으로 기술 | 한 줄/40px advance overflow 유지 |
| 17 | fixed 10px anywhere의 줄 수/행 높이 누락 | 네 줄/40px 명시 |
| 18 | fixed 10px break-word의 definite wrapping 누락 | 네 줄/40px 명시 |
| 19 | 시뮬레이터 실행 수가 Android evidence와 다름 | Android 9/9 일치 |
| 20 | 시뮬레이터 실행 수가 iOS evidence와 다름 | iOS 9/9 일치 |
| 21 | Android AVD를 실기기로 부름 | software AVD로 한정 |
| 22 | iOS Simulator 결과를 실제 iPhone 결과로 부름 | Simulator로 한정 |
| 23 | 화면 캡처를 glyph pixel proof로 표시 | glyph 증거가 아님을 명시 |
| 24 | Metal 결과를 compositor 이후 화면으로 과장 | compositor 이후 pixel 미검증 명시 |
| 25 | static archive 검사로 APK/IPA 전체 제외를 주장 | archive 경계에 한정 |
| 26 | style-to-layout 전체 suite의 실패를 숨김 | 270 pass/1 unrelated failure 명시 |
| 27 | C07 aspect-ratio 실패를 새 overflow-wrap 회귀로 귀속 | 변경 범위 밖 기존 assertion으로 분리 |
| 28 | 내부 계약 수정이 버전을 부당하게 올림 | `0.1.0` 유지 |
| 29 | overflow-wrap roadmap row가 계획 후보 문구를 계속 노출 | PR #948 병합 note로 교체 |
| 30 | runtime/사전 비교 evidence URL이 저장소 경로와 불일치 | 링크 경로 대조 완료 |

## 판정

문서 간 PR 병합 정보, 지원 경계와 공식 집계가 일치한다. 상태 동기화 변경은 문서만 수정하며 구현/runtime 증거를 새로 만들거나 기존 실패 로그를 덮지 않는다. 공개 roadmap 빌드·Tailnet HTTP/Agent Browser 확인은 별도 배포 검증 단계에서 수행한다.
