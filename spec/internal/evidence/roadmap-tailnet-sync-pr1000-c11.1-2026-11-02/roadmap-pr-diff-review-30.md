# PR #1000 로드맵 카드 변경 적대 검토 30개

검토 대상은 `spec/public/roadmap.html`의 신규 C11.1 후속 progress card 1개다. 목적은 병합된 제한 구현의 최신 진행을 보이면서 공식 완료 상태와 미지원 범위를 유지하는 것이다. 각 항목은 서로 다른 표현·링크·상태 실패를 겨냥한다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 신규 기록이 기존 progress card를 덮어씀 | 기존 첫 카드 앞에 별도 문단을 삽입했고 뒤의 기존 항목을 보존했다. PASS |
| 2 | 오래된 활동이 최신 항목처럼 보임 | 2026-11-01T20:04:43Z 병합 기록을 기준으로 새 카드를 목록 맨 앞에 놓았다. PASS |
| 3 | UTC 병합 날짜와 한국 로컬 표시 날짜가 혼합됨 | 병합 시각은 `Z` 표기로 보존하고 headline 날짜는 한국 기준 2026-11-02로 적었다. PASS |
| 4 | 다른 PR을 링크하거나 번호를 잘못 표기 | GitHub PR #1000 제목·URL을 병합 PR로 직접 확인했다. PASS |
| 5 | 잘못된 merge SHA가 추적 근거로 남음 | PR #1000 merge SHA `1971edf0ad9e11ec32f08b83728220f2161b17f5`를 `gh` 결과와 대조했다. PASS |
| 6 | C11.1 제한 후속을 Grid 전체 구현처럼 소개 | 카드에서 “제한 기본-runtime 연동”으로 이름 붙이고 전체 CSS Grid는 완료됐다고 쓰지 않는다. PASS |
| 7 | 실험용 fixture constructor를 제품 기본 경로라고 부름 | 실제 `new_runtime_gpu_with_root_html` 기본 경로로 명시했다. PASS |
| 8 | auto-sized·fractional track을 지원하는 듯 보임 | 이름 없는 고정 non-negative px track만 기록했다. PASS |
| 9 | authored placement나 dense/column flow까지 확장된 듯 보임 | 기본 row auto-placement만 언급하고 나머지 범위는 확장하지 않는다. PASS |
| 10 | visible overflow가 clipping/scroll 지원으로 오인됨 | visible-overflow scene을 실행했다고만 쓰고 PixelCopy/Metal readback은 아니라고 한계를 붙였다. PASS |
| 11 | unsupported text/image/nested Grid가 성공하는 듯 보임 | 해당 입력과 그 밖의 지원 밖 입력은 scene을 발행하지 않는다고 적었다. PASS |
| 12 | regression 수가 focused test 수와 혼동됨 | Root HTML 17/17 전체를 쓰고 그 안에 Grid 2개가 포함됨을 구별했다. PASS |
| 13 | 전체 style-to-layout 276/277을 모두 green으로 소개 | 기존 C07.3 비교 실패 1건과 실제 276/277을 함께 표기했다. PASS |
| 14 | formatting/Clippy 결과를 CI 통과로 과장 | 로컬 formatting·scoped Clippy와 PR checks는 별도 문장으로 구분했다. PASS |
| 15 | GitHub checks가 없는 상태를 숨은 CI success로 표시 | 카드에 check run/required status check이 없다고 명시한다. PASS |
| 16 | Android emulator 결과를 실기기/GPU 결과로 일반화 | Android API 37 ARM64 AVD/SwiftShader로 범위를 적었다. PASS |
| 17 | iOS Simulator를 iPhone 실기기라고 부름 | iPhone 17 Pro / iOS 26.2 Simulator라고 표시했다. PASS |
| 18 | base 장면만 확인하고 overflow 장면을 빠뜨림 | 두 플랫폼에서 base와 visible-overflow 모두 실행했다고 기록했다. PASS |
| 19 | 화면 캡처를 pixel equality 증거로 오인 | 화면은 draw/present 보조 증거이며 readback은 아니라고 명시했다. PASS |
| 20 | Android PixelCopy 결과가 있는 것처럼 해석 | PixelCopy를 수행하지 않았음을 카드에서 분명히 했다. PASS |
| 21 | iOS Metal drawable readback 결과처럼 해석 | Metal drawable readback을 수행하지 않았음을 카드에서 분명히 했다. PASS |
| 22 | C11 parent가 완료됐다고 읽힘 | C11 미완료 상태를 문장으로 유지했다. PASS |
| 23 | C11.3 또는 C11.4가 이 변경으로 완료됐다고 읽힘 | 두 하위 항목도 미완료라고 표시했다. PASS |
| 24 | 공식 완료 수가 임의로 증가 | 공식 `72/213`을 그대로 표기했고 체크박스 데이터는 바꾸지 않았다. PASS |
| 25 | 계획 링크 route 오타·잘못된 prefix | 병합된 main의 `plan/c11-root-html-grid-runtime.md` GitHub URL로 연결해 preview에 아직 없는 route로 가지 않는다. PASS |
| 26 | 내부 계약 link가 공개 API를 가리킴 | 병합된 main의 `spec/internal/0059-c11-grid-layout.md` GitHub URL로 연결한다. PASS |
| 27 | 실행 근거 link가 다른 날짜·run을 가리킴 | PR #1000이 추가한 evidence README의 main GitHub URL로 연결한다. PASS |
| 28 | 구현 검토가 없거나 동일 검토를 중복 링크 | implementation/runtime 20개와 별도의 PR diff 30개 Markdown 문서를 main GitHub URL로 각각 링크한다. PASS |
| 29 | 이 로드맵 변경의 자체 검토 근거를 찾을 수 없음 | 이 30개 문서를 main GitHub URL의 `로드맵 변경 검토 30개` 링크로 연결한다. PASS |
| 30 | 화면의 공식 상태 link가 오래된 route/anchor로 이동 | 기존 roadmap 카드와 같은 `/spinon/STATUS.html#css-구현-체크리스트`를 사용한다. PASS |

## 판정

**PASS 30 / FAIL 0.** 변경은 공식 상태 대장과 C11 completion count를 수정하지 않는 progress record 한 건이다. Agent Browser에서 1440×1000 desktop과 390×844 mobile로 렌더링해 카드 문구·GitHub 링크를 확인했다. Mobile document/body 폭은 viewport와 같은 390px이고 카드 폭은 362px로 가로 overflow가 없다. Tailnet 배포 후 응답 hash와 실제 배포 화면 확인은 별도 sync 단계에서 한다.
