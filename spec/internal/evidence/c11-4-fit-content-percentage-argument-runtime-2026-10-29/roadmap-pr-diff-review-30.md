# C11.4 percentage fit-content 로드맵 변경 검토 30개

## 검토 범위

공개 Tailnet 미리보기의 정적 로드맵에 PR #771의 제한 구현 결과를 추가한 한 문단을 서로 다른 관점 30개로 검토했다. 공식 체크 상태는 바꾸지 않았다.

| # | 실패 관점 | 확인 결과 |
|---|---|---|
| 1 | 새 기록이 기존 PR #768 앞에 잘못 삽입될 수 있음 | PR #771 행을 #768 기록 바로 다음에 추가했다. |
| 2 | 실제 병합되지 않은 PR을 완료 기록으로 표시할 수 있음 | GitHub에서 PR #771의 MERGED 상태를 확인했다. |
| 3 | 병합 시각이 지역 시간으로 잘못 변환될 수 있음 | UTC 시각 2026-10-28T20:48:31Z를 그대로 기록했다. |
| 4 | 구현 head SHA를 merge SHA로 쓸 수 있음 | merge SHA f89dfc2be9ff732167986bd4e2e453f3819343d1을 기록했다. |
| 5 | Rebase and merge 정책과 다른 방식을 쓸 수 있음 | 실제 병합 방식을 Rebase and merge로 표기했다. |
| 6 | WPT 전체 실행을 구현 검증으로 오해할 수 있음 | 대상은 percentage-argument assertion 하나라고 명시했다. |
| 7 | 지원하지 않는 선언까지 적용 범위가 넓어질 수 있음 | 제한 opt-in fixture로 범위를 고정했다. |
| 8 | percentage basis가 indefinite인 것으로 읽힐 수 있음 | definite 300px block basis를 기록했다. |
| 9 | fit-content argument 값이 잘못 표시될 수 있음 | fit-content(50%)를 명시했다. |
| 10 | 최종 row가 contribution 크기로 혼동될 수 있음 | 사용된 row와 item 크기 150px을 분리해 적었다. |
| 11 | child 크기가 잘못 표시될 수 있음 | child 높이 400px을 기록했다. |
| 12 | root 바깥 paint가 clipping 지원으로 확대될 수 있음 | root 밖 overflow 관찰만 기록한다. |
| 13 | Android AVD를 실기기로 오해할 수 있음 | Android API 37 ARM64 AVD라고 한정했다. |
| 14 | Android 표본을 hardware GPU 결과로 오해할 수 있음 | software AVD 한계를 명시했다. |
| 15 | PixelCopy 근거가 누락될 수 있음 | Android PixelCopy 표본 결과 4/4 exact를 기록했다. |
| 16 | iOS Simulator를 iOS 실기기로 오해할 수 있음 | iPhone 17 Pro / iOS 26.2 Simulator로 명시했다. |
| 17 | iOS drawable 표본을 compositor 이후 픽셀로 오해할 수 있음 | compositor 제출 전 drawable이라고 기록했다. |
| 18 | iOS 표본 수가 과장될 수 있음 | Metal 표본 4/4 exact로 정확히 적었다. |
| 19 | 일반 CSS admission을 추가한 것으로 오해할 수 있음 | 일반 CSS admission을 지원 완료하지 않는다고 적었다. |
| 20 | arbitrary Grid tree를 지원하는 것으로 오해할 수 있음 | arbitrary Grid tree 범위를 명시적으로 제외했다. |
| 21 | 전체 WPT 통과로 오해할 수 있음 | 전체 WPT는 완료하지 않았다고 적었다. |
| 22 | 전체 C11.4 완료로 오해할 수 있음 | C11.4 미완료 상태를 유지했다. |
| 23 | 공식 완료 수가 바뀔 수 있음 | 72/213을 유지했다. |
| 24 | PR에 check가 없는데 CI 통과로 오해할 수 있음 | required-check 결과가 비었다고 명시하고 CI 통과라 표현하지 않는다. |
| 25 | 계획 링크가 존재하지 않는 route를 가리킬 수 있음 | C11.4 계획의 generated HTML route를 대조했다. |
| 26 | 내부 계약 링크가 잘못될 수 있음 | 내부 계약 0059의 generated HTML route를 대조했다. |
| 27 | 실행 README 링크가 잘못될 수 있음 | runtime evidence README source와 route가 존재한다. |
| 28 | Android screenshot 링크가 잘못될 수 있음 | 연결한 AVD screenshot source가 존재한다. |
| 29 | iOS screenshot 링크가 잘못될 수 있음 | 연결한 Simulator screenshot source가 존재한다. |
| 30 | HTML 손상이나 Tailnet base path 오류가 생길 수 있음 | 문단 태그와 /spinon/ route 형식을 기존 페이지 구조에 맞췄다. |

## 판정

변경은 최신 C11.4 제한 slice의 병합 기록과 실행 범위만 추가한다. C11/C11.4 체크 상태, 공식 완료 수, 공개 지원 선언은 바뀌지 않는다. 발견한 수정 필요 항목은 없다.
