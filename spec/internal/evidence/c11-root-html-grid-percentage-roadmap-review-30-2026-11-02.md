# C11.1 percentage-track roadmap 변경 검토 30개 관점

`spec/public/roadmap.html`에 추가한 PR #1019 진행 카드를 독립된 실패 관점 30개로 순차 대조했다. 이 기록은 로컬 문서 검토이며 외부 reviewer나 CI 결과가 아니다.

| # | 실패 가설 | 판정 |
|---:|---|---|
| 1 | PR #1019가 아직 열려 있거나 닫힌 PR로 잘못 설명된다. | 기각. GitHub에서 merged 상태를 확인했다. |
| 2 | 병합 시각이 현지 시간과 UTC 사이에서 뒤바뀐다. | 기각. `2026-11-02T01:42:07Z`를 기록했다. |
| 3 | 표시된 merge SHA가 다른 PR 커밋이다. | 기각. `8e549ed55ac6f7e9611a4f7e1e571fca3a1e5684`로 확인했다. |
| 4 | percentage columns와 rows가 아무 축에나 적용된다고 읽힌다. | 기각. definite CSS-pixel width/height 축으로 각각 제한했다. |
| 5 | negative/non-finite percentage가 허용된다고 읽힌다. | 기각. finite non-negative direct values로 범위를 적었다. |
| 6 | `% + px` 조합이 누락된다. | 기각. 양성 범위를 카드에 표시했다. |
| 7 | `% + fr` 조합이 누락된다. | 기각. 양성 범위를 카드에 표시했다. |
| 8 | percentage gap도 구현됐다고 읽힌다. | 기각. percentage gap을 계속 거부한다고 적었다. |
| 9 | fixed-px gap이 percentage basis에서 빠진다고 읽힌다. | 기각. fixed-px gap은 지원하고 unsupported gap을 명시했다. |
| 10 | 0%, 100%, zero basis 경계가 빠진다. | 기각. 세 경계를 카드에 적었다. |
| 11 | 100% 초과 track이 clamp되거나 숨겨진다고 읽힌다. | 기각. visible overflow로 설명했다. |
| 12 | intrinsic/cyclic percentage sizing이 포함된다. | 기각. 두 경로를 지원 밖으로 명시했다. |
| 13 | named-area + percentage 결합이 지원된다. | 기각. 결합을 계속 거부한다고 적었다. |
| 14 | `calc()`, `minmax()`, repeat 또는 implicit tracks가 지원된다. | 기각. 네 범위를 모두 제외했다. |
| 15 | 테스트 수가 원 evidence에서 바뀐다. | 기각. 4/4, 1/1, 16/16을 유지했다. |
| 16 | simulator 실행이 기본 route가 아닌 fixture-only entry라고 오해된다. | 기각. 기본 `index.html` route임을 표시했다. |
| 17 | simulator geometry 결과가 draw/present와 구별되지 않는다. | 기각. layout-ready 15 boxes와 draw/present를 별도로 적었다. |
| 18 | Android AVD가 physical device로 표현된다. | 기각. API 37 ARM64 AVD라고 적었다. |
| 19 | iOS Simulator가 iPhone 실기기로 표현된다. | 기각. iPhone 17 Pro/iOS 26.2 Simulator라고 적었다. |
| 20 | screenshot이 PixelCopy/Metal readback 또는 pixel parity 증거로 과장된다. | 기각. 화면 배치용이며 readback이 아니라고 적었다. |
| 21 | PR checks가 없는데 CI 성공이라고 읽힌다. | 기각. 빈 check rollup과 미설정된 required check를 표시했다. |
| 22 | C11.1 또는 C11 상위가 완료됐다고 읽힌다. | 기각. 둘 다 미완료라고 적었다. |
| 23 | 공식 완료 수가 72/213에서 변한다. | 기각. 카드의 완료 수를 72/213으로 유지했다. |
| 24 | 내부 계약 숫자 버전이 증가했다고 읽힌다. | 기각. 버전을 카드에서 변경하지 않았고 계약은 0.1.0이다. |
| 25 | 카드를 #1015보다 앞에 두어 병합 순서가 잘못된다. | 기각. #1015 바로 뒤에 삽입했다. |
| 26 | 계획 PR #1018과 구현 PR #1019가 혼동된다. | 기각. 카드는 구현 PR #1019를, 관련 링크는 최종 계획을 가리킨다. |
| 27 | 계획·evidence·contract 링크가 후보 branch를 가리킨다. | 기각. GitHub main 경로를 사용한다. |
| 28 | Android 또는 iOS 화면 경로가 소스 evidence에 없다. | 기각. 두 PNG 파일과 기존 PR 화면 크기를 확인했다. |
| 29 | 카드 추가가 공식 체크박스 원본을 수정한다. | 기각. `spec/STATUS.md`는 변경하지 않는다. |
| 30 | HTML 변경에 공백 오류 또는 다른 파일 변경이 섞인다. | 기각. `git diff --check`를 실행하고 변경 파일을 확인했다. |

## 결과

새 카드는 PR #1019의 제한 Root HTML percentage-track 병합 상태와 미완료 범위를 공개 roadmap에 표시한다. 공식 완료 수와 C11 완료 판정은 바꾸지 않는다.
