# C11.4 definite-axis ratio fixture 병합 후 문서 동기화 검토 30개

검토 기준은 GitHub PR #878의 merge state와 merge SHA, PR check rollup, 계획·runtime·Chrome 사전 비교 evidence 및 이 문서 변경 diff다. 구현/runtime 검토와 PR #878 코드 diff 검토를 재사용하지 않았다. 이 검토는 문서 정확성만 다룬다.

| # | 별도 실패 경로·불변 조건 | 결과 |
|---:|---|---|
| 1 | 계획이 병합된 구현을 계속 후보로 표시 | 상태를 제한 fixture 구현 완료로 갱신 |
| 2 | GitHub state를 병합이 아닌 open/closed로 오기 | PR #878 `MERGED` 확인 |
| 3 | 병합 시각을 잘못 표기 | `2026-10-30T21:47:44Z` 대조 |
| 4 | merge SHA를 source head SHA로 대체 | `b1b7356e2e2afe397c2d41ebf111863a7f25ccd9` 기록 |
| 5 | 올바르지 않은 PR을 연결 | #878 제목과 definite-axis fixture 확인 |
| 6 | 빈 PR check rollup을 CI 성공으로 표현 | CI 성공 주장을 추가하지 않음 |
| 7 | 계획 문서가 55-scene 결과를 과장/축소 | Android/iOS 각각 55/55 대조 |
| 8 | Android target/device 종류 누락 | API 37 ARM64 software-rendered AVD로 한정 |
| 9 | iOS Simulator를 실기기로 표기 | iPhone 17 Pro/iOS 26.2 Simulator 유지 |
| 10 | Android 전체 surface sample 수 오류 | PixelCopy 177/177 대조 |
| 11 | iOS 전체 surface sample 수 오류 | 제출 전 Metal drawable 177/177 대조 |
| 12 | 신규 scene sample count 누락/오류 | 양 플랫폼 신규 4 scene, 각 12/12 |
| 13 | Metal 표본을 compositor 뒤 픽셀로 과장 | 제출 전 readback 한계를 명시 |
| 14 | Android AVD 결과를 hardware GPU로 과장 | software renderer 한계를 명시 |
| 15 | iOS validator의 첫 실패를 감춤 | multispan index-order 실패와 수정 후 rerun을 기록 |
| 16 | 수정 후 일부 iOS scene만 다시 실행했다고 오기 | 전체 55-scene matrix rerun으로 기록 |
| 17 | 4 fixture scene의 input shape를 일반화 | 두 definite axis와 두 no-ratio controls로 한정 |
| 18 | 2:1 계산 결과를 runtime expectation과 다르게 표기 | width 60→height 30, height 30→width 60 대조 |
| 19 | control scene을 ratio scene과 혼합 | computed `auto`와 no typed ratio를 명시 |
| 20 | text Range를 item frame으로 혼동 | text Range 40×10px인 사실 유지 |
| 21 | 내부 contract가 다른 root/child shape를 선언 | width 100px, A/AAAA, Ahem 10px profile 대조 |
| 22 | runtime trace/owner provenance를 누락 | owner NodeId, revision, track/frame 확인 계약 기록 |
| 23 | fixture route가 generic CSS admission으로 확장 | opt-in `RuntimeBlockFlexGridTrackSizingV1` 경계를 유지 |
| 24 | unsupported dimensions/ratio constraints를 허용한다고 오기 | 다른 ratio, both-definite, min/max, percentage/calc 등을 fail-closed로 명시 |
| 25 | anonymous/nested/replaced text를 허용 범위로 오기 | 거부 범위를 계약에 명시 |
| 26 | public API/FFI가 생긴 것으로 기록 | 추가하지 않았음을 명시 |
| 27 | 내부 계약 버전을 부당하게 올림 | `0.1.0` 유지 |
| 28 | 상위 C11/C11.4 체크를 완료 처리 | 미완료 유지 |
| 29 | 공식 완료 집계를 임의로 증가 | `72/213` 유지 |
| 30 | roadmap HTML의 기존 병합 정보와 문서가 충돌 | `spec/public/roadmap.html`의 PR #878 시각·SHA·범위가 일치하며 이미 정확하므로 수정 불필요 확인 |

## 판정

`spec/STATUS.md`, 계획, 내부 계약 0059는 PR #878 병합 사실과 제한 지원 범위가 일치한다. `spec/public/roadmap.html`은 이미 같은 병합 정보를 표시하므로 중복 편집하지 않는다. 문서 변경은 테스트나 runtime evidence를 새로 만들지 않으며 C11.4/C11 완료 집계는 바꾸지 않는다.
