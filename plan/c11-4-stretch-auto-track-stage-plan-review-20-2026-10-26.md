# C11.4 auto track stretch stage plan · 실패 관점 검토 20개

대상 계획: [auto track stretch stage oracle](./c11-4-stretch-auto-track-stage-oracle.md)

검토 기준은 `origin/main`의 vendored Taffy trace 경계, 기존 `spinon-layout` Rust test 구조와 CSS Grid 2025 CRD §11.8이다. 계획 수정 후 확인했으며 계획·코드 변경이나 테스트 실행은 하지 않았다.

1. **규범 단계 번호** — Auto track stretch를 §11.6 Maximize Tracks와 혼동할 수 있다. 계획은 §11.8 Stretch auto Tracks로 분리했다. **PASS**
2. **규범 조건** — 아무 content alignment에서나 늘어난다고 오해할 수 있다. `normal`/`stretch`만 positive stretch 입력으로 두고 `start`를 대조군으로 둔다. **PASS**
3. **양수·definite free space** — 음수 또는 indefinite 공간을 균등 분배할 수 있다. root block size와 viewport height를 100px으로 고정하고 positive free space 45px을 계산한다. **PASS**
4. **root sizing 일관성** — viewport와 root constraint가 달라 definite height가 보장되지 않을 수 있다. 둘 다 20×100px, `RootSizingPolicy::Match`로 고정했다. **PASS**
5. **trace 활성화** — 기본 placement mode에서 stage trace가 비어 oracle이 실행되지 않을 수 있다. 모든 입력에 `GridPlacementMode::TrackSizing`을 명시했다. **PASS**
6. **축 혼동** — 열 pass에 stretch 기대를 적용할 수 있다. 열은 20px 고정, row gap·row tracks·block size만 바꾸고 Rows pass를 검사한다. **PASS**
7. **자동 배치 순서** — child가 기대 row와 다르게 놓일 수 있다. 세 direct child와 DOM 순서대로 한 행씩 배치하는 topology를 고정했다. **PASS**
8. **gutter 개수** — 세 행의 gap을 세 번 계산할 수 있다. 내부 gutter 두 개만 10px씩 포함한다. **PASS**
9. **기초 크기 합산** — row bases `[10,20,5]`를 다르게 합산할 수 있다. 총 base size 35px로 고정했다. **PASS**
10. **남은 공간 산술** — gaps 누락 시 free space가 65px로 잘못 계산된다. `100 - 35 - 20 = 45px`을 명시했다. **PASS**
11. **분배 대상 수** — 세 번째 fixed track까지 eligible 수에 넣을 수 있다. `auto` max track은 두 개뿐이고 각 22.5px씩 성장한다. **PASS**
12. **fixed-track 불변식** — stretch가 fixed 5px track까지 바꿀 수 있다. 최종 세 번째 track을 5px로 명시했다. **PASS**
13. **최종 합계** — 분배 후 tracks가 container를 채우지 못할 수 있다. `[32.5,42.5,5] + 2×10 = 100px`으로 합계를 검산했다. **PASS**
14. **기여 입력 안정성** — text wrapping이나 replaced-size 규칙이 기초 크기를 바꿀 수 있다. child heights를 `[10,20,5]px`로 definite하게 두어 별도 intrinsic 콘텐츠 의존성을 피한다. **PASS**
15. **선행 stage 경계** — stretch 전 base size가 이미 달라져 잘못된 단계의 값을 비교할 수 있다. `IntrinsicSizingComplete`와 `MaximizeTracks`에서 원래 bases `[10,20,5]`를 검사한다. **PASS**
16. **stage 순서와 유연 트랙 단계** — `StretchAutoTracks`가 `ExpandFlexibleTracks`보다 앞서 실행될 수 있다. 두 stage와 선행 snapshot 순서를 요구한다. **PASS**
17. **stage 선택 모호성** — 다른 axis/pass의 stage를 사용해 false pass가 날 수 있다. Rows pass에서 고유 stage snapshot을 요구한다. fixed-size 입력은 expected row pass를 구분한다. **PASS**
18. **대조군의 의미** — `start`가 stretch를 허용하는 것처럼 구현될 수 있다. `StretchAutoTracks { applied:false }`, 원래 rows와 남은 공간 45px을 negative control로 명시했다. **PASS**
19. **테스트/제품 경계** — trace 확인을 위해 새 public API나 app fixture가 생길 수 있다. `#[cfg(test)]` module 한 개와 module declaration만 추가하고 production, FFI, Android/iOS를 수정하지 않는다. **PASS**
20. **frame 판정값 명시** — stage snapshot만으로 root frame이나 child placement가 틀린 경우를 놓칠 수 있다. 계획은 root `(0,0,20,100)`과 stretch/start child 좌표 `[0,42.5,95]` / `[0,20,50]`를 직접 명시하고 크기까지 검사한다. **PASS**

## 판정

**PASS — 20/20, 차단 항목 0개.** 기준 입력·규범 산술·stage 경계와 frame expectation이 구현 전 판정에 충분히 구체적이다. 계획 검토와 중복되지 않는 별도 구현/runtime 20관점 검토가 필요하다.

## 사후 색인 일치 확인

`plan/c11-4-grid-track-sizing.md`의 2026-10-26 색인 항목을 검토했다. 링크된 테스트 전용 계획과 §11.8 범위, `[auto, auto, 5px]` 행, 두 개의 `10px` row gap, definite `100px` block size, `align-content:start` 대조군이 일치한다. 20관점 검토 링크가 연결됐고 구현 미착수 및 C11.4 미완료·공식 `72/213` 유지도 정확하다. 색인 불일치 없음.
