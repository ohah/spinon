# C11.4 cyclic `minmax(calc())` minimum span implementation PR diff review · 30 points

구현 PR에 포함할 변경 전체(FFI/runtime/style, Android/iOS harness, simulator runner, 계획·내부 계약·로드맵과 실행 근거)를 독립된 failure angle로 재검토했다. 로드맵/문서 렌더링 및 GitHub PR 화면은 게시 전 확인 항목으로 남기고, merge 전에 Agent Browser 결과를 보충한다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | FFI fixture admission이 exact four IDs인가 | 통과. 새 모듈의 `SCENES`는 계획의 네 ID만 담는다. |
| 2 | FFI topology가 한 root·한 span item·한 text만 만드는가 | 통과. DOM source는 fixed root/item/style/text와 source order만 구성한다. |
| 3 | Columns/Rows calc expression이 축별로 정확한가 | 통과. 열은 `50% + 4px`, 행은 `50% + 2px`; 두 scene 각각 해당 축에서만 두 track을 쓴다. |
| 4 | cyclic scene과 definite control이 input basis를 달리하는가 | 통과. cyclic 축만 `max-content`; definite control은 동일한 50px/20px basis를 쓴다. |
| 5 | gap의 axis 및 count가 fixture·runtime에서 같은가 | 통과. 열 또는 행의 한 gap 4px을 typed scene과 computed spacing validator가 함께 검사한다. |
| 6 | Border/content-box 산술이 계획과 일치하는가 | 통과. 2px border에서 content basis와 border frame `54×14` / `24×24px`를 직접 단언한다. |
| 7 | Ahem fixture font의 버전/경로가 deterministic한가 | 통과. 기존 pinned test resource를 embed하고 system-font lookup 없는 engine을 쓴다. |
| 8 | text intrinsic metrics를 한 값으로 뭉개는가 | 통과. min/max inline/block values를 각각 검사한다. |
| 9 | Column span contribution에서 gap 한 번을 빼는가 | 통과. `[0,23,4,23,0]` intrinsic track/gutter sequence를 검증한다. |
| 10 | Row span contribution/wrapping이 맞는가 | 통과. 20px inline width의 2-line text와 `[0,8,4,8,0]` sequence를 검증한다. |
| 11 | 최종 minimum floor가 contribution 전에 적용되는가 | 통과. intrinsic values와 final `[29,29]` / `[12,12]`를 별도 assertion으로 둔다. |
| 12 | computed typed placement가 raw CSS string만으로 승인되는가 | 통과. runtime이 `Line(1)`과 `Span(2)`, 반대 축 Auto를 검사한다. |
| 13 | 잘못된 second track/track grammar가 accepted되는가 | 통과. runtime matcher는 정확히 두 동일 MinMax(Calc, MaxContent) track을 요구한다. |
| 14 | 예기치 않은 gap·margin·padding·border가 되는가 | 통과. root style gate가 expected spacing과 border를 확인한다. |
| 15 | unsupported runtime ID가 fallback scene으로 실행되는가 | 통과. exact matrix 외 ID는 runtime fixture adapter에서 거부한다. 누락 registry의 최초 실패도 보존되어 있다. |
| 16 | Android intent/scene route에 ID가 누락되는가 | 통과. 네 ID가 C114 matrix, geometry, intrinsic input, node frame 및 pixel allowlist에 연결되어 네 AVD 장면이 모두 통과했다. |
| 17 | iOS native scene route/expectation이 Android와 drift하는가 | 통과. 네 ID·geometry·intrinsic inputs·node frames와 sample expectation을 iOS harness에 각각 연결했다. |
| 18 | border 밖 item의 overflow frame이 사라지는가 | 통과. item frame은 column `62×10`, row `20×28px`; root보다 넓거나 높다. |
| 19 | surface sample이 border 안쪽 오차/경계에 걸리는가 | 통과. root-relative red sample은 border 바깥 item 내부, past-item black sample은 item 외부로 지정했다. |
| 20 | RGB/RGBA 기대 색이 결과와 다르게 문서화되는가 | 통과. Android/iOS scene logs에서 `(229,72,77,255)` 및 `(0,0,0,255)`가 exact다. |
| 21 | 앱 clear 검정을 Chrome page white와 혼동하는가 | 통과. plan/evidence/roadmap에서 browser oracle 배경과 native surface readback을 명시적으로 분리했다. |
| 22 | capture runner가 원래 대규모 C11.4 matrix까지 실행하는가 | 통과. 지정된 네 scene allowlist로 capture를 호출하고 각 플랫폼 4/4만 보고했다. |
| 23 | capture가 layout frame만 성공으로 보고 draw/present를 생략하는가 | 통과. 네 장면 모두 `drawPresented=true`, ready status, frame과 trace 확인을 요구한다. |
| 24 | Android ADB success가 stale APK에서 나온 것인가 | 통과. Gradle APK rebuild 후 installed/local SHA-256이 일치하는 최신 AVD run을 사용했다. |
| 25 | iOS 실행이 다른/실기기 destination으로 섞이는가 | 통과. 선택한 iPhone 17 Pro/iOS 26.2 Simulator UDID만 설치·실행 대상으로 썼다. |
| 26 | fixture가 기본 Rust archive에 들어가는가 | 통과. no-feature archive에서 네 ID와 marker가 없고 opt-in positive control에서 확인됐다. |
| 27 | runtime registry regression이 simulator 성공만 의존하는가 | 통과. runtime test 214 passed 및 별도 integration/node-lifecycle suite, FFI registry unit test를 보존했다. |
| 28 | format, JS harness, module boundary가 확인됐는가 | 통과. rustfmt, Node syntax, fixture-module boundary 6/6, `git diff --check`가 통과했다. |
| 29 | Clippy 결과나 build feature를 제품 전체로 과장하는가 | 통과. 변경 crate의 feature-enabled all-target Clippy와 opt-in Simulator builds만 근거로 적는다. |
| 30 | 문서가 전체 C11.4·공식 완료 count·미검증 범위를 과장하는가 | 통과. `spec/STATUS.md`와 현재 공식 77-of-213 집계는 변경하지 않고 일반 CSS admission, WPT, Android physical/hardware GPU, iOS compositor-after pixels를 제외했다. |

### PR 화면 및 병합 전 상태 보충

2026-10-22에 Agent Browser로 [PR #497](https://github.com/ohah/spinon/pull/497)을 열어 본문에 올린 Android AVD와 iPhone Simulator 이미지를 확인했다. 둘 다 complete=true, 자연 크기 각각 1080×2400과 1206×2622로 로드됐다. 응답 JSON은 pr-497-agent-browser-verification.json에 저장한다. gh pr view는 mergeStateStatus=CLEAN, 빈 statusCheckRollup을 반환했고 gh pr checks --required는 이 branch에 보고된 required checks가 없다고 반환했다. Tailnet 페이지 확인과 브라우저 종료 상태는 별도 동기화 근거에 기록한다.
