# C11.4 anonymous Grid text `font-size: 0` 계획 PR diff 적대 검토 30개

**검토 범위:** `plan/c11-4-grid-track-sizing.md`, `plan/c11-4-anonymous-grid-text-font-size-zero.md`, 계획 검토 20개, 고정 Chrome fixture, 사전 비교 README·JSON·스크린샷·SHA-256 manifest.

**기준:** `origin/main` `ba3e19f4ad168a4ed16248564cbb3eb1e1366df4`. 구현 전 계획 PR이므로 코드 빌드·테스트나 플랫폼 실행 결과로 판정하지 않는다. 계획 실패 관점 20개와 다른 diff 관점을 사용한다.

| # | 적대 검토 관점 | 판정 |
|---:|---|---|
| 1 | 기준 main이 작업 시작 시 remote main과 다른가 | **통과.** 체크아웃 `HEAD`와 `origin/main`이 모두 `ba3e19f4ad168a4ed16248564cbb3eb1e1366df4`다. |
| 2 | diff가 계획 slice 밖의 구현 파일을 바꾸는가 | **통과.** 의도한 diff는 C11.4 계획·계획 검토·브라우저 fixture/evidence로 제한된다. |
| 3 | parent plan 문장이 계획 후보를 구현 완료로 올리는가 | **통과.** parent note는 계획 단계라고 쓰고 implementation/status/count 반영이 없다고 구분한다. |
| 4 | `spec/STATUS.md`나 공개 roadmap checkbox를 간접 수정하는가 | **통과.** 상태 원본과 roadmap은 변경 대상이 아니며 계획 문서도 변경하지 말라고 명시한다. |
| 5 | `72/213` 집계를 후보 하나로 증가시키는가 | **통과.** 상위/집중 계획에 공식 `72/213`을 유지한다고 명시한다. |
| 6 | 상위 C11/C11.4 미완료 상태를 감추는가 | **통과.** 전체 C11.4/C11 완료는 지원·완료로 주장하지 않는다. |
| 7 | 기존 computed font-size 계획을 링크 없이 다시 설명하는가 | **통과.** 이전 positive-only slice에 연결하고 5px 이하 금지 범위를 선행 문서에서 확인한다. |
| 8 | 내부 계약 0059와 parent Grid 계획 링크가 깨지는가 | **통과.** 대상 문서가 저장소에 존재하고 상대 경로가 저장소 구조에 맞는다. |
| 9 | `font-size` 값 범위를 최신 Recommendation으로 오표기하는가 | **통과.** 2026-09-06 CSS Fonts Level 4 Working Draft라고 날짜·상태를 밝힌다. |
| 10 | `<length-percentage [0,∞]>`에서 0 허용을 빠뜨리는가 | **통과.** zero를 포함한 범위를 직접 인용하며 draft임을 표시한다. |
| 11 | 음수 CSS 선언을 zero 입력과 혼동하는가 | **통과.** `-1px`를 invalid declaration 및 inherited 10px negative control로 분리한다. |
| 12 | CSS signed zero와 Rust `-0.0` 표현을 동일시하는가 | **통과.** `-0px` computed serialization을 `0px`로 고정하고 typed finite zero와 비교한다. |
| 13 | `line-height:10px`의 근거 문서 상태를 과장하는가 | **통과.** CSS Inline Layout Level 3 §5.1을 2024-12-18 Working Draft로 표기한다. |
| 14 | zero font size가 absolute `line-height`를 0으로 바꾼다고 가정하는가 | **통과.** 절대 10px line-height를 별도 명세 기대값과 Chrome 관측으로 검사한다. |
| 15 | Grid anonymous item/whitespace 규칙을 다른 명세 버전에서 복사하는가 | **통과.** CSS Grid Layout Level 2의 2025-03-26 Candidate Recommendation Draft를 밝힌다. |
| 16 | whitespace-only scene이 anonymous item을 만든다고 기대하는가 | **통과.** no-item control과 Grid §6 기대를 명시한다. |
| 17 | zero-size direct text item이 auto-placement 순서를 건너뛴다고 보는가 | **통과.** Grid §8.5와 second-row fixed box 관측을 연결한다. |
| 18 | Ahem 독립 산식을 Chrome track 결과에서 역산하는가 | **통과.** pinned `unitsPerEm`·`hmtx` metric을 별도 입력으로 기록한다. |
| 19 | min-content에 대한 zero contribution만 확인하고 max-content를 추정하는가 | **통과.** min/max scene을 따로 고정하고 각 기대값을 명시한다. |
| 20 | max-content에서 space advance가 누락되는가 | **통과.** Ahem space advance도 1em이고 zero size에서 0px임을 포함한다. |
| 21 | explicit 100px Grid width가 intrinsic column 결과를 가리는가 | **통과.** fixed root width와 intrinsic column track을 별도 기대값으로 둔다. |
| 22 | auto-height root가 track row 크기와 불일치하는가 | **통과.** root frame 높이와 explicit auto row를 각각 관측한다. |
| 23 | Range 0×0을 anonymous item의 frame으로 오해하는가 | **통과.** Range는 text fragment 관측에 한정하고 block frame/row로 placement를 본다. |
| 24 | `XX` 뒤 5×3 block의 위치를 text item frame으로 잘못 보고하는가 | **통과.** `(0,10)`은 다음 box의 root-relative frame이고 text box 증거가 아니라고 적는다. |
| 25 | fixture label이 target Grid width나 track sizing에 끼어드는가 | **통과.** label은 Grid 바깥 sibling이며 CSS 입력을 바꾸지 않는다고 기록한다. |
| 26 | Chrome/Ahem 실행 환경이 재현에 부족한가 | **통과.** Chrome 156 build, UA, viewport, DPR, font-loaded 확인을 기록한다. |
| 27 | screenshot을 pixel parity나 실제 제품 glyph paint로 과장하는가 | **통과.** 시각 장면 자료로 제한하고 glyph paint·surface parity 근거가 아니라고 명시한다. |
| 28 | 구현 시 Root HTML renderer가 zero size를 rasterizer에 넘겨 화면 publish를 실패시키는 경계를 놓치는가 | **수정 후 통과.** 소스 확인에서 `rasterize_glyphs`의 `<=0` 거부를 발견했다. 계획에 all-zero paragraph no-paint 경로를 넣었다. |
| 29 | no-paint 처리로 C15 rasterizer나 mixed run까지 넓게 허용하는가 | **통과.** `InlineRunStyle`·rasterizer의 positive-only 계약과 mixed zero/nonzero 거부를 유지한다. |
| 30 | AVD/iOS, default artifact, CI 또는 full C11 완료를 이미 검증했다고 읽히는가 | **통과.** Android/iOS runtime, release artifact 경계, focused tests는 구현 관문으로 남고 CI 통과나 전체 완료 주장은 없다. |

**결론:** 계획 PR diff는 구현·공식 상태와 분리되어 있다. source inspection으로 발견한 zero-size raster handoff 문제를 계획에 반영했다. 아직 계획 검토이므로 runtime no-paint 동작은 구현 후 별도 20관점 review, 플랫폼 실행과 구현 PR diff 30관점 검토가 필요하다.

## 병합 후 구현 route 재확인

구현 branch에서 profile dispatch를 끝까지 추적한 결과, 위 28번의 no-paint 요구는 C11.4 fixture 경로에 적용되지 않는 것으로 정정했다. `BlockFlexGridTrackSizing`은 anonymous-text intrinsic projection을 선택하고 `root_html_default_render`는 별도 `RootHtmlDefaultRender` profile에서만 prepare/attach 된다. 이에 따라 구현 계획에서 no-paint renderer 변경을 제거하고, 이 fixture는 C11.4 layout/track evidence만 생성하도록 바로잡았다. 초기 계획 PR 검토 기록은 보존하며 이 정정이 zero-size text의 runtime 동작을 증명하지는 않는다.
