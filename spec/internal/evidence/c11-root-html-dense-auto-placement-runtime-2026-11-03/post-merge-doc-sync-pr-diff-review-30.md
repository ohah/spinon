# C11.1 dense placement 병합 후 문서 PR diff 검토 · 30개 관점

이 검토는 PR #1117 병합 후 내부 계약·상태 대장·계획·evidence index를 맞춘 문서 전용 diff를 검사한다. PR #1117의 구현 검토를 반복하지 않고, 문서 사이에 생길 수 있는 서로 다른 불일치를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 PR 번호나 경로를 병합 증거로 연결 | PR #1117 URL이 계약, 상태 대장, 계획 및 evidence에 일치한다. PASS |
| 2 | PR 번호와 다른 merge SHA를 기록 | 다섯 동기화 문서의 SHA가 `f1043bda5ed69845b89992ba5f7cd28a03f42f5c`와 일치한다. PASS |
| 3 | 잘못된 날짜·시간대 또는 merge 방식을 기록 | `2026-11-03T12:57:55Z`, Rebase and merge를 GitHub 값과 대조했다. PASS |
| 4 | 빈 check rollup을 CI 통과로 오해 | 모든 새 상태 문구가 check rollup이 비어 있음을 기록하고 CI 성공을 주장하지 않는다. PASS |
| 5 | 내부 계약 버전을 구현 추가만으로 올림 | 계약 버전 `0.1.0`을 유지했다. PASS |
| 6 | 전체 공식 완료 개수를 후보 결과만으로 증가 | `72/213`이 각 문서에서 유지된다. PASS |
| 7 | 제한 구현을 이유로 상위 C11 checkbox를 완료 처리 | C11 부모 항목은 미완료 상태다. PASS |
| 8 | 제한 구현을 이유로 C11.1을 완료 처리 | 상태 문구는 C11.1 미완료를 명시한다. PASS |
| 9 | fixture profile을 일반 CSS 지원으로 확대 해석 | 내부 계약과 상태 설명 모두 public CSS admission이 아님을 밝힌다. PASS |
| 10 | 전용 계획의 상대 링크가 잘못된 디렉터리를 가리킴 | plan 및 evidence에서 링크 경로를 확인했다. PASS |
| 11 | `spec/STATUS.md`의 계획 링크가 잘못됨 | 저장소 기준 `../plan/c11-root-html-dense-auto-placement.md` 경로가 유효하다. PASS |
| 12 | run evidence 링크가 존재하지 않거나 다른 작업 ID로 연결 | `c11-root-html-dense-auto-placement-runtime-2026-11-03/README.md`를 확인했다. PASS |
| 13 | 계획 상태가 여전히 PR 미생성이라고 보고 | 계획은 PR #1117의 병합 시각과 SHA를 기록한다. PASS |
| 14 | 공식 상태의 scene 개수가 계획/evidence와 다름 | 세 문서가 14-scene 제품 행렬로 일치한다. PASS |
| 15 | 성공과 기대 거부 장면 수를 서로 바꿈 | 12 성공·2 기대 거부로 대조했다. PASS |
| 16 | authored node 수를 잘못 요약 | 최종 행렬의 60 node ID와 문서의 60 authored nodes가 일치한다. PASS |
| 17 | geometry 오차를 허용 오차와 혼동 | 최대 관측 오차 0 CSS px, 판정 한계 0.5 CSS px를 분리한다. PASS |
| 18 | Android PixelCopy 표본 수를 과장 | Android 52/52 exact를 final run comparison과 대조했다. PASS |
| 19 | iOS Metal 표본 수를 Android 값으로 복사 | iOS 52/52 exact를 final run comparison과 대조했다. PASS |
| 20 | Android 검증 장치를 실기기로 잘못 표시 | API 37 ARM64 AVD라고 명시한다. PASS |
| 21 | Android software renderer 결과를 hardware GPU로 주장 | ANGLE/SwiftShader 소프트웨어 경계를 기록한다. PASS |
| 22 | iOS Simulator 모델 또는 OS 버전을 혼동 | iPhone 17 Pro / iOS 26.2 Simulator로 기록했다. PASS |
| 23 | iOS drawable 결과를 compositor 이후 출력으로 해석 | OS compositor 제출 전 readback이라고 명시한다. PASS |
| 24 | 양 플랫폼 시뮬레이터 결과를 실기기 증거로 확대 | 실기기 및 hardware GPU를 입증하지 않는다고 기록한다. PASS |
| 25 | Chrome oracle 비교를 Spinon WPT 적합성으로 확대 | 전체 WPT 적합성이 미검증임을 유지한다. PASS |
| 26 | nested mixed-flow 장면을 양쪽 nesting 방향으로 확대 | sparse parent 안의 dense child만 검사했고 reverse nesting은 별도 장면이 아니라고 기록한다. PASS |
| 27 | dense computed style을 문자열 재파싱 또는 generic route admission으로 설명 | typed computed value와 전용 `RootHtmlDenseAutoPlacementV1` 경계를 기록한다. PASS |
| 28 | explicit/implicit track 지원 범위를 모호하게 확장 | finite non-negative fixed-px explicit tracks 및 축별 implicit rule을 계약에 적었다. PASS |
| 29 | negative/named placement와 visibility 경계를 누락 | positive placement만 허용하고 `display:none`/`visibility:hidden` 처리를 구분한다. PASS |
| 30 | fixture artifact 증거를 최종 앱 산출물 보장으로 확대하거나 partial 결과 처리를 누락 | 정적 archive 검증의 범위와 fail-closed publication 경계를 분리한다. PASS |

모든 30개 문서 diff 관점에서 병합 결과·상태 집계·검증 한계가 서로 일치했다. C11.1/C11은 미완료이며 official completion `72/213`을 유지한다.
