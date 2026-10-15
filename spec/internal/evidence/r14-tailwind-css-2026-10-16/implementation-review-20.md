# R14 구현·실행 적대 검토 · 2026-10-16

계획 표와 다른 20개 관점으로 구현·실행 결과를 검토했다. 판정 대상은 pinned generator, analyzer, 기존 mobile converter와 Browser reference이며 제품 CSS 지원은 아니다.

| # | 실패 관점 | 확인한 근거 | 결과 |
|---:|---|---|---|
| 1 | Tailwind CLI와 core 버전이 어긋남 | package manifest·lockfile 모두 4.3.3, 실행 banner도 4.3.3 | 통과 |
| 2 | source 탐지가 저장소의 unrelated 문서에서 class를 추가 | `source(none)`·단일 `@source`; utility selector 집합이 예상 10개와 정확히 일치 | 통과 |
| 3 | lockfile 밖 floating dependency로 재현이 흔들림 | 두 spike frozen lock install 및 생성물 SHA-256 고정 | 통과 |
| 4 | analyzer가 generated CSS를 고쳐 converter 결과를 바꿈 | 원본 CSS bytes를 파싱하고 `rule.toString()` 격리 case만 별도 기록 | 통과 |
| 5 | 예상 class 누락을 생성 성공으로 묵인 | 10개 selector 각각 요구, 누락 시 verify 실패 | 통과 |
| 6 | fixture에 없는 utility가 더 생성돼도 눈치채지 못함 | utility layer의 전체 selector 집합을 예상 집합과 비교 | 통과 |
| 7 | custom token은 있지만 utility가 연결되지 않음 | theme layer 두 선언과 `bg-brand-*`의 var 참조를 함께 검사 | 통과 |
| 8 | `@layer` 순서 statement가 없어도 analyzer 통과 | `properties` 및 theme/base/components/utilities statement exact 검사 | 통과 |
| 9 | 필요한 layer block 하나가 없어도 통과 | theme, base, utilities block 존재 검사 | 통과 |
| 10 | Preflight global reset의 한 선언이 빠짐 | box-sizing, margin, padding, border sentinel을 각각 검사 | 통과 |
| 11 | heading/list reset이 빠짐 | h1–h6와 ol/ul/menu의 값 검사 | 통과 |
| 12 | image 또는 hidden reset의 영향 범위를 누락 | replaced display, image sizing, `[hidden] !important` 검사 | 통과 |
| 13 | Preflight를 내장 UA 규칙으로 잘못 합침 | output은 base author layer이며 README에서 UA origin과 분리 | 통과 |
| 14 | full converter가 첫 실패 뒤 부분 JSON을 남김 | 원본 generated CSS 실패 때 output file 부재를 강제 | 통과 |
| 15 | 변환기가 모든 입력을 거부해도 정상처럼 보임 | 실제 `.flex` positive control이 `display:flex` JSON을 생성 | 통과 |
| 16 | 변수 기반 spacing/color가 px/static 값인 것처럼 변환 | `p-4`·`bg-brand-600` isolated negative control에서 `unparsed` 거부 | 통과 |
| 17 | hover selector를 plain class로 평탄화 | selector 단독과 `(hover: hover)` media case를 따로 거부 | 통과 |
| 18 | responsive variant 조건을 버림 | `(width >= 40rem)` wrapper가 있는 변환 case를 거부; Chromium 390/1680 폭 비교 | 통과 |
| 19 | screenshot의 스타일을 Spinon runtime 증거로 오해 | screenshot 안쪽·alt·README에 standard Chromium reference 표기, 앱/runtime 주장 없음 | 통과 |
| 20 | 도구 설치 경고를 숨기거나 임의 script 실행 | Bun이 `@parcel/watcher` install script를 차단했으며 허용하지 않고 one-shot build를 재검증 | 통과 |

계산 결과: 20/20 관점 검토 통과. converter의 거부는 미지원 동작이 실제로 안전하게 drop됐다는 뜻이 아니며 해당 fixture 전체가 변환 불가라는 판정이다.
