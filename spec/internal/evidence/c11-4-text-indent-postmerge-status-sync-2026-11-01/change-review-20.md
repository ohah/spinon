# C11.4 `text-indent` 병합 상태 동기화 변경 검토 20개

## 검토 대상

- `plan/c11-4-anonymous-grid-text-computed-text-indent.md`의 상태 문구
- `plan/c11-4-grid-track-sizing.md`의 최신 C11.4 이력
- `spec/STATUS.md`의 C11.4 `text-indent` 기록
- 원본 병합 PR [#930](https://github.com/ohah/spinon/pull/930), 병합 SHA `b1b20e30cc8d36e90d87e902668eb95339698c25`

이번 변경은 이미 병합된 구현 slice의 문서 상태를 정정한다. 제품 코드·fixture·API 계약 동작은 변경하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #930이 open/closed 상태인데 병합으로 잘못 표기될 수 있다 | GitHub 상태가 `MERGED`임을 확인했다. 통과. |
| 2 | 병합 시간이 다른 PR의 시간을 복사했을 수 있다 | GitHub의 `2026-10-31T16:37:17Z`와 대조했다. 통과. |
| 3 | merge SHA가 PR head SHA 또는 base SHA일 수 있다 | GitHub가 반환한 merge commit `b1b20e30cc8d36e90d87e902668eb95339698c25`와 대조했다. 통과. |
| 4 | 문서가 #930의 다른 제목·기능을 가리킬 수 있다 | PR 제목은 `feat(c11-4): anonymous Grid text-indent 제한 구현`이다. 통과. |
| 5 | 계획 상태에 “검토 중” 문구가 남아 완료된 PR과 충돌할 수 있다 | 계획 상단 상태를 병합 사실과 제한 범위로 바꿨다. 통과. |
| 6 | `STATUS.md`에서 “구현 후보”가 미병합 후보라는 인상을 줄 수 있다 | 항목에 PR #930 병합 시각과 SHA를 명시했다. 통과. |
| 7 | C11.4 이력의 날짜와 병합일이 서로 모순될 수 있다 | 상태 관찰일 `2026-11-01`과 실제 병합일 `2026-10-31`을 각각 유지했다. 통과. |
| 8 | 18개 scene을 전체 anonymous-text 지원으로 과장할 수 있다 | Ahem·LTR·normal white-space의 고정 profile이라고 범위를 한정했다. 통과. |
| 9 | 값 0/5/10px가 음수·percentage·keyword도 지원한다고 읽힐 수 있다 | 계획과 상태에서 음수·percentage·`hanging`·`each-line`을 미지원으로 유지했다. 통과. |
| 10 | Android AVD 결과가 실기기 결과로 오인될 수 있다 | `Android API 37 ARM64 AVD`로 명시했다. 통과. |
| 11 | iOS Simulator 결과가 iOS 실기기 결과로 오인될 수 있다 | `iPhone 17 Pro/iOS 26.2 Simulator`로 명시했다. 통과. |
| 12 | 검은 renderer 캡처가 glyph paint 증거라고 읽힐 수 있다 | STATUS의 캡처 한계를 유지했다. 통과. |
| 13 | simulator frame/track 검증을 glyph pixel equality로 확대할 수 있다 | glyph pixel을 증명하지 않는다고 명시했다. 통과. |
| 14 | 구현 merge가 전체 WPT 통과를 뜻한다고 오해할 수 있다 | WPT 미검증 범위를 유지했다. 통과. |
| 15 | opt-in fixture merge가 일반 author CSS admission이라고 읽힐 수 있다 | 일반 CSS admission이 범위 밖임을 유지했다. 통과. |
| 16 | slice merge로 C11.4 checkbox가 닫힐 수 있다 | C11.4 checkbox를 미완료로 유지했다. 통과. |
| 17 | C11.4 slice merge로 C11 상위 checkbox가 닫힐 수 있다 | C11 상위 checkbox를 미완료로 유지했다. 통과. |
| 18 | slice merge만으로 전체 완료 수를 올릴 수 있다 | 공식 `72/213`을 그대로 유지했다. 통과. |
| 19 | 기존 0059 계약 번호 또는 버전을 새로 확정한 것처럼 보일 수 있다 | 기존 내부 계약 0059 링크만 유지하고 계약 내용·버전은 바꾸지 않았다. 통과. |
| 20 | 내부 evidence 링크가 없는 파일을 가리킬 수 있다 | precomparison, runtime README, 구현 review, PR diff review 파일 경로의 존재를 확인했다. 통과. |

## 판정

20개 관점에서 병합 상태·증거 경계·공식 완료 집계가 일치한다. 결함은 발견되지 않았다. 이는 문서 상태 대조이며 코드를 다시 빌드하거나 simulator를 재실행한 결과가 아니다.
