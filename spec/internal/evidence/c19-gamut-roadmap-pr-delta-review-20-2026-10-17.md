# C19 색역 판정·로드맵 갱신 PR 변경 적대 검토 20개 — 2026-10-17

검토 대상은 C19 상태 문구와 공개 로드맵 페이지의 diff다. 기존 Chromium source analysis의 근거를 다시 설명하는 데 그치지 않고, 이 변경이 스펙 판정·원본 범위·로드맵 상태를 잘못 바꾸는 실패 경로를 각각 확인했다.

| # | 실패 관점 | diff에서 확인한 방어와 결과 |
|---:|---|---|
| 1 | C15 계획 PR #236을 제품 구현 완료로 표시 | 로드맵은 C15를 계획만 병합했다고 설명하고 STATUS의 C15 checkbox는 계속 미완료다. 통과. |
| 2 | C15 사전 비교가 Spinon 출력으로 읽힘 | 문구는 이를 Chrome DOM oracle로 한정하고 Spinon runtime 미실행을 명시한다. 통과. |
| 3 | Android/iOS 확인을 완료로 암시 | 해당 Simulator 근거가 없다는 문구를 넣고 C15를 미완료로 남긴다. 통과. |
| 4 | 완료 수를 고정값으로 복사해 공식 상태와 분리 | 새 문구는 71/203 같은 숫자를 저장하지 않는다. Tailnet page의 집계는 main STATUS에서 갱신된다. 통과. |
| 5 | Local MINDE 결과를 CSS의 유일한 허용 RGB로 취급 | 2026-10-09 CRD가 허용한 알고리즘 세 가지를 밝히고 정확한 RGB 일치를 요구하지 않는다. 통과. |
| 6 | CSS Color 4가 색역 매핑을 선택 사항으로 허용한다고 오해 | SDR RGB display와 `unorm8` canvas의 out-of-gamut 경우 mapping 요구를 유지한다. 통과. |
| 7 | 구현체 선택 범위를 과장하거나 빠뜨림 | Local MINDE, EdgeSeeker, Ray Trace를 CRD §14.2의 선택지로 열거한다. 통과. |
| 8 | sample의 destination/colorType을 넓혀 일반화 | Chrome sample은 Canvas 2D `srgb`/`unorm8`로 한정한다. 통과. |
| 9 | Canvas sample을 화면 screenshot 값과 혼동 | 상태와 roadmap 모두 Canvas 2D 측정이라고 분명히 적는다. 통과. |
| 10 | Chromium feature 기본값을 실제 capture의 Finch state라고 단정 | `kBakedGamutMapping`의 default-disabled 소스 경로와 capture 당시 active Finch 미보존을 분리한다. 통과. |
| 11 | source 기반 추론을 모든 Chrome 경로의 확정 위반으로 확장 | 판단을 Chrome 155의 단일 Canvas sample로 한정하고 “어긋난 것으로 보임”으로 표현한다. 통과. |
| 12 | Stylo의 0.22 Local MINDE 값을 잘못된 oracle로 제거 | 해당 값은 CRD가 허용하는 구현체 결과로 보존하고 Chrome과 다른 구현값임을 설명한다. 통과. |
| 13 | Chrome mismatch를 Spinon 제품 버그 수정으로 오해 | change scope는 상태 설명과 roadmap text이며 product implementation result를 추가하지 않는다. 통과. |
| 14 | C19 parent를 완료로 변경 | C19 parent checkbox는 `[ ]` 유지이며 C19.3 통합 gate가 남는다. 통과. |
| 15 | C19.2 플랫폼 제한이 판정 변경으로 사라짐 | hardware GPU·실기기·iOS compositor·opacity group·text/hit-test 제한은 그대로 둔다. 통과. |
| 16 | C15 plan link가 Tailnet 배포 경로에서 깨짐 | 계획은 GitHub main의 tracked Markdown URL로 링크하고 사전 비교 자료도 main의 README로 연결한다. 통과. |
| 17 | Chromium 근거가 generic latest source로 바뀜 | existing source-analysis link는 Browser `0e66b94549a8bd85f03fbdf189b752d6e7b63c32`와 file blob을 pin한다. 통과. |
| 18 | W3C 문서 상태를 최종 Recommendation이라고 과장 | 문구는 날짜가 고정된 Candidate Recommendation Draft라고 유지한다. 통과. |
| 19 | STATUS와 공개 roadmap의 색상 해석이 다름 | 두 곳 모두 algorithm choice, default-disabled source path, Finch 한계와 표본 경계를 맞췄다. 통과. |
| 20 | road map latest note가 C19.2에서 멈춰 C15 작업 누락 | PR #236 계획 병합 note를 추가하고 C19.2는 제품 구현 evidence로 별도 유지했다. 통과. |

`git diff --check`와 변경 범위의 Markdown/HTML 링크 존재를 확인했다. 이 PR은 문서만 수정하며 C19/C15 지원 체크나 runtime 증거를 추가하지 않는다.
