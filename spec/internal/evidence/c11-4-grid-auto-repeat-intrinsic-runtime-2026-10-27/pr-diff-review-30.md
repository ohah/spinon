# C11.4 intrinsic auto-repeat 구현 PR 변경 검토 30개

검토 대상은 현재 구현 변경분 전체다. 계획 검토와 구현/runtime 20개 관점 검토는 재사용하지 않고, 변경 파일 간 계약·상태·artifact·증거 연결을 중심으로 서로 다른 실패 경로 30개를 확인했다. 한 검토자가 코드·문서·저장된 실행 결과를 대조한 기록이며 외부 독립 reviewer를 주장하지 않는다.

| # | 별도 실패 관점 | 판정 및 확인 근거 |
|---:|---|---|
| 1 | 작업 도중 unrelated 파일이나 사용자 산출물이 PR에 섞임 | 통과. staged 목록은 구현·계약·계획·상태·이번 최종 Android/iOS evidence로 한정했다. 기존 재계산/auto-track 로그와 `wpt-grid-change-intrinsic-initial.png`는 미추적 상태로 보존됐다. |
| 2 | 실행 근거가 다른 base revision이나 새 worktree를 가리킴 | 통과. 실행 manifest의 base commit은 당시 `origin/main`과 같은 `5196e78c`; 새 checkout은 만들지 않고 이미 사용 중인 task checkout에서 이어갔다. |
| 3 | 사전 비교 계획 PR과 구현 PR의 순서가 뒤섞임 | 통과. 계획 PR #690은 병합된 것으로 계획과 실행 README가 기록하고, 현재 diff는 별도 구현 slice만 담는다. |
| 4 | 계획 문서가 WPT 전체 통과나 C11.4 완료를 과장함 | 통과. 계획 첫머리와 범위 절이 8-scene app fixture만 완료했다고 한정하며 WPT harness·전체 C11.4 완료를 명시적으로 부인한다. |
| 5 | oracle matrix의 scene 순서가 앱 fixture와 다름 | 통과. pinned `expected.json`, FFI scene table, Android/iOS allowlist, runner 선택 순서가 동일한 여덟 ID를 사용한다. |
| 6 | `fill`/`fit`, `fixed`/`auto`, `min`/`max` 조합 하나가 누락되거나 중복됨 | 통과. 변경 fixture·platform allowlist는 8개 고유 조합을 포함하고 capture catalog도 8개를 선택한다. |
| 7 | 기존 508-scene catalog 수가 갱신되지 않아 이전 runner 계약이 깨짐 | 통과. runner는 516으로 갱신됐고 boundary test의 기존 C11.4 선택 경로도 새 전체 수와 자기 선택 수를 함께 확인한다. |
| 8 | 이전 fixture의 scene count만 고쳐 놓고 새 matrix가 실제 catalog에 붙지 않음 | 통과. runner가 oracle JSON에서 8개 scene을 구성하고 전체 ID 유일성과 별도 WPT scene 수를 검사한다. |
| 9 | 같은 expected 값이 oracle과 구현 상수에서 서로 다르게 진화함 | 통과. oracle 기대값은 고정 `expected.json`에 있고, runner와 플랫폼 matcher는 각자 고정 track/frame을 검사한다. boundary test가 oracle ID와 used columns를 fixture catalog 계약에 연결한다. |
| 10 | `GridContainerInput` 초기화 지점 일부가 새 field를 누락함 | 통과. 관련 Rust 테스트·projection initializer가 `justify_items`를 명시하고 이전 auto-repeat fixture initializer도 갱신됐다. 해당 타입을 쓰는 layout/style suites는 evidence README의 실행 결과대로 통과했다. |
| 11 | 내부 `GridContainerInput` 변화가 기본 layout 정렬을 바꿈 | 통과. 기본은 `LayoutAlignItems::Normal`; Taffy 변환도 container 값이 없으면 `AlignItems::NORMAL`로 간다. |
| 12 | `justify-items:start` 허용이 다른 profile이나 CSS 속성까지 넓어짐 | 통과. parser는 `RuntimeBlockFlexGridTrackSizingV1`에서만 값을 읽고 unsupported-property gate도 그 profile의 Grid container에만 `start`를 추가 허용한다. |
| 13 | computed alignment가 typed layout 단계에서 손실되거나 item 정렬로 잘못 들어감 | 통과. projected Grid container field가 Taffy `justify_items`에 연결되고 item `justify_self` 경로와 분리돼 있다. |
| 14 | 새 minmax 이웃 허용이 모든 앞·뒤 track으로 일반화됨 | 통과. projection gate는 auto-repeat 직후의 한 trailing column에만 새 두 track 형태를 허용하며 prefix, 여러 suffix, rows는 negative test로 닫는다. |
| 15 | auto-fill만 허용해 `auto-fit`인 원본 WPT case가 빠짐 | 통과. 두 intrinsic neighbor 형태 모두 auto-fill과 auto-fit에서 projection 및 layout 경로를 확인한다. |
| 16 | NaN·무한대·음수 최소/최대가 projection 또는 Taffy 경계에 도달함 | 통과. projection/layout의 기존 finite·nonnegative guard가 유지되고 새 테스트가 음수·무한·NaN을 각각 거부한다. |
| 17 | 새 matcher가 이전 absolute auto-repeat fixture 허용 범위를 넓힘 | 통과. 새 WPT matrix matcher는 독립 feature-gated module이며 기존 matcher의 호출 계약은 그대로 유지한다. |
| 18 | scene route가 opt-in fixture module 밖에서 기본 runtime에 포함됨 | 통과. FFI route의 `#[path]`와 `runtime_source`는 fixture 모듈 아래 있고 Cargo feature 경계 문서는 default/positive-control `.rlib` 결과를 구분한다. |
| 19 | 새 route가 unknown ID를 비슷한 scene으로 넘겨 성공 처리함 | 통과. 고정 ID 검색 이외에는 source가 없고 fixture boundary test가 route·scene ID wiring을 확인한다. |
| 20 | dedicated Rust matcher 파일이 여러 책임을 섞거나 500줄 초과를 방치함 | 범위 한정. 파일은 556줄이며 production exact-shape matcher와 그 matcher의 전용 unit tests 한 책임으로 구성된다. 별도 파일로 쪼개 책임을 분산할 근거는 없고 `docs/project-rules.md`의 응집 모듈 원칙에 맞는다. |
| 21 | Android와 iOS가 다른 root/item/probe geometry를 성공으로 판정함 | 통과. 두 adapter의 8개 상태 표와 전체 node frame이 같은 oracle 기대값을 사용하며 최종 결과 JSON은 모든 scene의 frame·track을 보존한다. |
| 22 | 0-width 마지막 probe가 box count나 paint sample에 포함됨 | 통과. runner의 box count는 양수 폭 probe만 세고, 표면 sample은 overflow 두 scene만 가지며 node frame에는 0-width probe가 남는다. |
| 23 | overflow sample이 border·item 바깥 control과 관계없는 점을 읽음 | 통과. oracle `surfaceChecks`에서 x/y를 가져오고 Android PixelCopy와 iOS Metal 기대 sample이 border, overflow item, past-item 순서로 3개씩 일치한다. |
| 24 | PR screenshots가 다른 run의 화면이거나 artifact 경로가 깨짐 | 통과. README 링크는 최종 Android run `T101029...`와 iOS run `T101156...`의 `fill-auto-min` PNG를 가리킨다. Agent Browser에서 PR #691 원격 이미지를 읽어 Android 1080×2400, iOS 1206×2622의 `complete=true`와 유효한 intrinsic size를 확인했다. |
| 25 | 앱 실행 결과가 로컬 binary만 가리키고 설치된 앱은 다른 빌드임 | 통과. 두 `simulator.json`에서 로컬·설치 APK/app hash가 각각 일치하고 동일 source snapshot hash와 dirty input 경로를 기록한다. |
| 26 | 결과 JSON이 일부 scene만 담거나 실패 retry를 성공에 섞음 | 통과. 두 최종 `result.json`은 8 scene 각각의 frame/track/trace chunk, `drawPresented=true`, retry 0을 보존한다. |
| 27 | 앱 trace와 capture verifier의 최종 track 단계가 서로 다름 | 통과. 양 플랫폼 matcher와 runner verifier 모두 `StretchAutoTracks` 마지막 stage의 track 목록을 확인한다. |
| 28 | `GridContainerInput` 필드 추가를 외부 API 호환 보장으로 오해함 | 범위 한정. Rust 타입은 re-export되지만 `spinon-layout`은 `publish = false`; 내부 계약 0059에만 field를 추가하고 숫자 버전 `0.1.0`을 유지한다. PR 본문은 이를 내부 workspace interface 변경으로 표현하고 외부 semver 호환을 주장하지 않는다. |
| 29 | static archive 근거를 APK/IPA 전체 fixture exclusion으로 확대함 | 통과. 계획·README·feature-boundary·내부 계약 모두 검사 범위를 macOS ARM64 `.rlib`로 한정하고 앱 bundle 전체 exclusion은 확인하지 않았다고 쓴다. |
| 30 | 로드맵이 제한 slice를 완료 항목으로 세거나 공식 수치를 조용히 변경함 | 통과. C11.4 checkbox는 `[ ]`, 공식 `72/213`은 그대로며 새 STATUS 항목도 전체 WPT·제품 지원을 주장하지 않는다. |

검토에서 PR을 막는 미해결 코드·계약 결함은 찾지 못했다. 로컬 staged diff에는 Android가 캡처한 두 원본 runtime log 행 끝의 공백만 `git diff --cached --check` 경고로 나타난다. 로그를 보존하기 위해 원문을 편집하지 않았고 나머지 소스·문서 diff는 whitespace 검사에서 깨끗하다. PR #691은 열려 있으며 screenshot 렌더링은 Agent Browser로 확인했다. GitHub check rollup은 비어 있고 main required status check 및 approval은 설정되지 않아 CI 통과나 독립 승인을 주장하지 않는다.
