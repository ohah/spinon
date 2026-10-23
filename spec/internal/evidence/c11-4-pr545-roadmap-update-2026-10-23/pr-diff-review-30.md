# PR #545 병합 후 C11.4 로드맵 변경사항 검토 30개

`spec/public/roadmap.html`, 집중 계획, 공식 STATUS를 변경 대상으로 검토했다. 30개 항목은 각각 다른 회귀 경로·상태 경계·링크 계약을 확인한다. 이 검토는 별도 reviewer 수가 아니라 개별 관점의 근거 기록이다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | 로드맵이 잘못된 PR을 새 결과의 근거로 표시하는가 | PR #545를 연결하고 다른 구현 PR #539 뒤에 후속 slice로 배치한다. |
| 2 | merge SHA가 실제 Rebase and merge commit과 다른가 | `d7a6b35966f7ba9ad53fc5c450bfd1e6ec9ead34`를 `gh pr view 545` 결과와 대조했다. |
| 3 | 원래 feature branch SHA를 main merge SHA로 표시하는가 | branch head `341cf711`가 아니라 main merge commit을 기록한다. |
| 4 | C11.4 전체 구현 완료로 과장하는가 | 제목과 문단에서 제한 runtime slice로 한정하고 C11.4 미완료를 명시한다. |
| 5 | 공식 완료 수가 바뀌는가 | `77/213`을 그대로 유지한다. |
| 6 | 공식 STATUS의 C11.4 checkbox가 잘못 완료 처리되는가 | 해당 상위 checkbox는 미완료 상태로 둔다. |
| 7 | 34 scene을 신규 34개로 오해하게 하는가 | 기존 양축 12 + 단일축 area 13 + 신규 양축 area 9의 구성을 쓴다. |
| 8 | Android 검증 환경을 실기기로 과장하는가 | Android API 37 ARM64 AVD로 명시한다. |
| 9 | iOS 검증 환경을 실기기로 과장하는가 | iPhone 17 Pro / iOS 26.2 Simulator로 명시한다. |
| 10 | 양쪽 simulator 결과가 서로 다른 scene count를 가졌는데 동일시하는가 | 각 target에서 34/34를 별도로 표기한다. |
| 11 | root 안쪽의 일반 geometry만 검사한 것으로 적는가 | `area-added-both`의 root 40×40과 item `(44,40,8,8)` overflow를 함께 적는다. |
| 12 | columns/rows track size가 뒤바뀌거나 생략되는가 | columns `[20,16,16]`, rows `[20,12,12]` 순서를 확인했다. |
| 13 | PixelCopy 표본을 Android hardware GPU 증거로 오해하게 하는가 | PixelCopy는 AVD 결과이며 hardware GPU 미검증을 유지한다. |
| 14 | Metal drawable 표본을 iOS compositor 이후 화면으로 오해하게 하는가 | 제출 전 drawable이라고 표시하고 compositor 이후 pixels를 미검증으로 둔다. |
| 15 | surface 표본 4/4를 전체 renderer 적합성으로 넓히는가 | 고정 overflow 장면 표본으로 한정한다. |
| 16 | Chrome 비교의 환경과 범위가 모호한가 | Chrome 156 oracle 9/9를 simulator 행렬과 분리한다. |
| 17 | Taffy 역할을 Spinon 자체 구현으로 오해하는가 | 확장 grid/area resolution은 pinned Taffy 0.15.0 경로에 전달한다고 쓴다. |
| 18 | computed-style profile gate가 넓어졌다고 읽히는가 | `RuntimeBlockFlexGridTrackSizingV1` 전용 제한을 집중 계획과 실행 근거에 둔다. |
| 19 | auto-repeat grammar 전체 지원으로 읽히는가 | 복수 repeat, 혼합 repeat 등 현재 거부 범위를 제외 항목으로 유지한다. |
| 20 | product artifact 분리를 검증했다고 잘못 선언하는가 | current-source default/product archive negative control 미실행을 명시한다. |
| 21 | Android APK/IPA에서 fixture 데이터가 빠졌다고 오해하게 하는가 | 최종 APK/IPA exclusion 미검증을 명시한다. |
| 22 | CSS border stroke 검증을 pixel sample로 대신했다고 읽히는가 | border stroke 미검증을 명시한다. |
| 23 | WPT 또는 일반 CSS admission까지 확인한 것처럼 보이는가 | 두 범위를 미검증으로 적는다. |
| 24 | plan 링크가 최신 PR #545 변경 계획을 가리키는가 | `plan/c11-4-two-axis-template-areas.md`를 merge commit에서 확인했다. |
| 25 | internal contract 링크가 다른 버전을 가리키는가 | `spec/internal/0059-c11-grid-layout.md`와 `0.1.0` 유지 문구를 연결한다. |
| 26 | Chrome 사전 비교 링크가 다른 작업의 evidence를 가리키는가 | `c11-4-two-axis-template-areas-precomparison-2026-10-23` 경로를 확인했다. |
| 27 | runtime README 링크가 캡처 요약과 다른 evidence를 가리키는가 | `c11-4-two-axis-template-areas-runtime-2026-10-23/README.md`를 연결한다. |
| 28 | 구현/runtime 20개 검토 링크가 빠지거나 계획 검토와 혼동되는가 | `implementation-runtime-review-20.md`를 별도 연결한다. |
| 29 | Tailnet STATUS anchor가 잘못된 fragment로 바뀌는가 | 기존 `/spinon/STATUS.html#css-구현-체크리스트` 링크를 유지한다. |
| 30 | 무관한 roadmap 카드·완료 수·기능 선언까지 바뀌는가 | 새 단락 하나만 PR #539 runtime 단락 뒤에 삽입하고 그 외 카드는 수정하지 않는다. |

## 판정

로드맵과 공식 문서는 PR #545의 제한 범위 및 미완료 상태와 일치한다. Tailnet preview 파일의 hash·실제 화면·링크 렌더링은 별도 Tailnet 동기화 단계에서 검증한다.
