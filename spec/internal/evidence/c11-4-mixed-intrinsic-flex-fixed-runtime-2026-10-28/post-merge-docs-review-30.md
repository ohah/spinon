# C11.4 mixed intrinsic/flexible/fixed 병합 후 문서 변경 검토 30개

검토 대상은 PR #729 병합 뒤 `spec/STATUS.md`, 내부 계약 0059, 계획, 공개 roadmap, runtime evidence의 아티팩트 경계 기록이다. 구현/runtime 20개와 PR #729 변경분 30개 검토를 반복하지 않고, 상태 동기화·근거 출처·링크·지원 범위의 서로 다른 실패 조건을 확인했다.

| # | 별도 실패 가설 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11.4 checkbox가 제한 fixture만으로 완료 처리된다 | checkbox는 `[ ]`로 유지했다. |
| 2 | C11 parent가 하위 slice 때문에 완료 처리된다 | C11 parent 상태를 바꾸지 않았다. |
| 3 | 공식 완료 집계가 72/213에서 임의로 증가한다 | 이 변경은 상태 체크를 추가하지 않고 기존 `72/213`을 유지한다. |
| 4 | 내부 계약 숫자 버전이 올라간다 | `0.1.0`을 유지한다. |
| 5 | fixture 전용 구현이 공개 CSS 지원으로 게시된다 | 계획·계약·로드맵 모두 opt-in 제한 fixture로 한정한다. |
| 6 | PR URL이 다른 작업을 가리킨다 | 세 문서의 링크 대상은 `ohah/spinon#729`다. |
| 7 | 병합 시각이 PR 기록과 다르다 | GitHub PR metadata의 `2026-10-28T01:38:11Z`와 일치한다. |
| 8 | Rebase and merge 후 SHA를 원래 branch commit으로 잘못 적는다 | merge SHA `3e1add7414d534c0b5ca830a9c772ba65a3a3ace`를 기록했다. |
| 9 | Android 실행을 실기기 검증으로 표시한다 | API 37 ARM64 AVD라고 명시한다. |
| 10 | iOS Simulator 결과를 실제 iPhone 결과로 표시한다 | iPhone 17 Pro / iOS 26.2 Simulator라고 명시한다. |
| 11 | 8 scene을 각 플랫폼 합계가 아닌 전체 8회로 축소해 쓴다 | Android와 iOS 각 8/8로 기록한다. |
| 12 | Columns와 Rows 중 한 축의 결과가 빠진다 | 두 target axis를 모두 명시한다. |
| 13 | freeze track 결과가 잘못 기록된다 | `100px` base freeze의 최종 base `[100, 40, 140]px`를 evidence와 대조했다. |
| 14 | equality 경계를 freeze로 설명한다 | 정확한 80px equality는 no-freeze `[80, 40, 160]px`로 기록한다. |
| 15 | no-freeze 대조군에 다른 expected tracks를 쓴다 | 40px min-content 대조도 `[80, 40, 160]px`임을 확인했다. |
| 16 | gutter를 제외한 track 합계를 definite size와 같다고 오해한다 | 두 10px gutter까지 합쳐 300px 축을 채운다고 설명한다. |
| 17 | reverse source order scene 결과가 누락된다 | 실행 근거에 고정 scene 목록과 source-order reversal이 있다. |
| 18 | fixed 40px neighbor가 제외되거나 flex track으로 오인된다 | 계획·계약·로드맵의 track grammar가 같은 순서를 유지한다. |
| 19 | image natural size가 CSS 고정 크기로 만들어진 것처럼 게시된다 | evidence는 natural size `24×16 CSS px`와 auto dimension을 구분한다. |
| 20 | screenshot 경로가 Rspress에서 모듈 경로로 해석되거나 화면을 pixel equality로 과장한다 | README 상대 이미지에 `./`를 붙여 선택 페이지 build에서 resolve하고, pixel equality 주장은 하지 않는다. |
| 21 | 미등록 scene 또는 conflicting profile이 성공했다고 기록된다 | 두 플랫폼에서 각각 fail-closed controls였다고 명시한다. |
| 22 | 전체 negative-control matrix가 실행된 것으로 과장된다 | 계획한 이미지 변조 등 일부 개별 control 미실행 한계를 유지한다. |
| 23 | 기본 product archive에 새 fixture marker가 포함된다 | default archive의 hash와 `nm`/`strings` 부재 결과를 기록했다. |
| 24 | fixture positive control이 실제 opt-in 구성인지 불명확하다 | positive build feature 목록에 `c04-runtime-gpu-fixture-test`가 명시돼 있다. |
| 25 | positive control에서 scene marker가 없는데 포함으로 주장한다 | positive archive에서 marker와 Rust/FFI symbol을 확인했다. |
| 26 | 결과가 Rust archive 전체 또는 앱 bundle 전체를 대표한다고 오해된다 | macOS ARM64 release static FFI archive에 한정한다고 명시한다. |
| 27 | Android APK/iOS IPA fixture 제외를 잘못 주장한다 | 두 app bundle은 이 검사에서 미검증으로 남긴다. |
| 28 | CI check rollup이 비었는데 CI pass라고 쓴다 | check rollup이 비었다고 적고 CI 통과 주장은 하지 않는다. |
| 29 | targeted Clippy의 기존 실패를 숨기거나 제품 성공으로 둔갑시킨다 | `type_complexity`·`map_identity` 기존 lint와 검증 한계를 기록한다. |
| 30 | public roadmap의 구현 요약과 공식 상태 문서가 달라진다 | merge SHA, 8/8 플랫폼 결과, 제한 범위, 미완료 및 `72/213`을 서로 대조했다. |

## 문서 빌드 경계

`mise exec -- bun run docs:build` 전체 빌드는 기존 evidence Markdown의 dead link와 Shiki `rust,ignore` 표기 때문에 실패했다. 이번 변경 문서와 연결된 계획·계약·상태·evidence를 선택한 Rspress 2.0.22 build에서 처음에 README 상대 이미지 링크의 `./` 누락을 검출했다. 경로를 고친 뒤 선택 build가 통과했다. 선택 build는 이번 경로의 MDX/이미지 번들 검증이며 전체 문서 링크 검증으로 확대하지 않는다.

병합 뒤 Tailnet 실제 제공 URL과 Agent Browser에서 roadmap·evidence 표시를 별도 확인한다. Pages/`gh-pages`는 이 변경에서 갱신하지 않는다.
