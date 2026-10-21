# C11.4 nested Grid 행 축 후속 문서 PR diff 검토 30개

검토 범위는 PR #445 병합 정보 반영, 실행 README의 Android/iOS 이미지 상대 경로, 공식 상태와 공개 로드맵의 연결이다. 각 항목은 변경 diff와 이미 고정된 실행 근거를 대조했다.

| # | 독립 실패 관점 | 결과 |
| --- | --- | --- |
| 1 | 상위 C11.4 상태가 구현 slice 하나만으로 완료 처리되지 않는가 | 통과. 상위 checkbox는 미완료로 유지한다. |
| 2 | 공식 구현 완료 집계가 잘못 증가하지 않는가 | 통과. `72/213`을 유지한다. |
| 3 | PR 번호가 실제 변경을 병합한 PR과 일치하는가 | 통과. #445는 nested Grid 행 축 런타임 변경 PR이다. |
| 4 | merge SHA가 PR의 main merge commit과 일치하는가 | 통과. `30d181e0ee80ef6e36b312fc9f34163ecd4366ed`를 기록한다. |
| 5 | 병합 시간이 UTC로 잘못 변환되지 않았는가 | 통과. GitHub 병합 시각 `2026-10-21 12:29:24 UTC`를 쓴다. |
| 6 | 비어 있던 check rollup을 통과로 오인하지 않는가 | 통과. rollup이 비었다고 명시한다. |
| 7 | required check 미설정을 CI 성공 주장으로 바꾸지 않는가 | 통과. `main` required status checks 미설정을 별도 기록한다. |
| 8 | PR 링크가 올바른 저장소와 번호를 가리키는가 | 통과. `ohah/spinon/pull/445`를 가리킨다. |
| 9 | 계획 링크가 행 축 slice 계획으로 연결되는가 | 통과. `c11-4-nested-grid-row-contribution.md`에 연결된다. |
| 10 | 실행 근거 링크가 해당 런타임 evidence README로 연결되는가 | 통과. nested Grid rows runtime README를 유지한다. |
| 11 | 상태 대장과 evidence README의 SHA·시각이 서로 다른가 | 통과. 두 문서가 동일한 merge SHA와 UTC 시각을 쓴다. |
| 12 | 선행 계획 PR #444의 병합 정보가 덮어써지는가 | 통과. 계획 SHA `1359f37024ad711df4b5253745d6dbf9b96af77f`를 그대로 둔다. |
| 13 | span-two 기대 track 배열이 변조되는가 | 통과. `[18,18]` 기대값을 유지한다. |
| 14 | one-track 장면의 0px 뒤 track이 누락되는가 | 통과. `[40,0]`을 유지한다. |
| 15 | definite-flex control의 배열이 바뀌는가 | 통과. `[28,28]`을 유지한다. |
| 16 | 세 root 높이에 outer row gap이 누락되는가 | 통과. `40/44/60px`을 유지한다. |
| 17 | Android 환경이 실기기로 과장되는가 | 통과. API 37 ARM64 AVD로 표기한다. |
| 18 | iOS 환경이 기기 검증으로 과장되는가 | 통과. iPhone 17 Pro의 iOS 26.2 Simulator로 표기한다. |
| 19 | 사용자가 선택한 시뮬레이터 범위와 충돌하는가 | 통과. 이번 slice에 Android 실기기 실행을 주장하지 않는다. |
| 20 | 첫 Android 실패가 성공 기록에서 숨겨지는가 | 통과. allowlist 누락 로그의 기존 링크를 보존한다. |
| 21 | Markdown 이미지가 package/module 경로로 해석되는가 | 수정. 이미지 경로에 `./`를 붙여 같은 디렉터리 asset임을 명시한다. |
| 22 | 지시한 Android 이미지 파일이 실제 경로에 존재하는가 | 통과. `android-row-span-two.png`가 evidence 디렉터리에 있다. |
| 23 | 지시한 iOS 이미지 파일이 실제 경로에 존재하는가 | 통과. `ios-row-span-two.png`가 evidence 디렉터리에 있다. |
| 24 | 이미지 alt가 Android 화면을 iOS 화면으로 혼동시키는가 | 통과. Android alt에 API/AVD와 scene을 명시한다. |
| 25 | 이미지 alt가 iOS 화면을 Android 화면으로 혼동시키는가 | 통과. iOS alt에 iPhone/iOS Simulator와 scene을 명시한다. |
| 26 | 프레임 캡처를 glyph paint 검증으로 확대하는가 | 통과. glyph paint는 검증하지 않았다고 유지한다. |
| 27 | 자연 PNG frame을 texel equality로 확대하는가 | 통과. PNG source texel 일치는 범위 밖이라고 유지한다. |
| 28 | draw/present를 surface pixel 검증으로 확대하는가 | 통과. PixelCopy/Metal 표본 검증을 주장하지 않는다. |
| 29 | 고정 fixture를 WPT 또는 전체 CSS Grid 적합성으로 일반화하는가 | 통과. WPT와 전체 C11.4 미완료를 유지한다. |
| 30 | 공개 roadmap의 HTML 문단·계획·상태 링크가 끊기는가 | 통과. 기존 `<p>` 구조와 계획·evidence·STATUS 링크를 유지한다. |

발견한 문서 결함은 로컬 Markdown 이미지 두 경로의 `./` 누락이었다. RSPress가 이를 모듈 요청으로 해석하는 것을 선택 빌드에서 재현했고, 위 diff에서 고쳤다. PR 병합 정보가 빠진 공식 상태·evidence·공개 roadmap에도 동일한 병합 사실을 추가했다.
