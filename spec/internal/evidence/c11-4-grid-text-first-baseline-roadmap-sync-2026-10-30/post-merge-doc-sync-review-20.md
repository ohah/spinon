# C11.4 first-baseline 병합 후 상태·roadmap sync 적대 검토 20개

**범위:** 구현 PR #827 병합 사실을 공식 상태·계획·public roadmap에 반영하는 문서 변경과 Tailnet pass03 게시 증거를 검토한다. 이 검토는 C11.4 구현의 재검증이 아니다.

| # | 잘못된 문서 변경 또는 게시 상태 | 확인 및 판정 |
|---:|---|---|
| 1 | PR #827 상태가 열림/검토 중으로 남아 병합 사실과 어긋난다. | GitHub PR query의 `state=MERGED`, `mergedAt=2026-10-30T00:02:17Z`를 확인했다. 상태·계획·roadmap에 Rebase and merge를 기록한다. **PASS** |
| 2 | 병합 SHA가 PR merge commit과 다르다. | PR merge commit은 `f35ab0f15b7c65e2a7cb16fa3fb2769b9946f7ca`다. 계획·STATUS·roadmap은 같은 SHA를 사용한다. **PASS** |
| 3 | Rebase 병합 기록이 저장소 linear-history 규칙과 충돌한다. | `main`은 linear history를 요구하고 merge 방식은 Rebase and merge였다. **PASS** |
| 4 | PR checks가 비어 있는데 CI 성공으로 기록한다. | 병합 후 PR check rollup은 비어 있었고 required status checks도 설정되지 않았다. 문서는 check run 부재만 기록하고 CI 통과를 주장하지 않는다. **PASS** |
| 5 | 제한 fixture 병합이 C11.4 완료 checkbox를 켠다. | `C11.4 intrinsic track sizing`은 unchecked 상태를 유지한다. **PASS** |
| 6 | C11 상위 항목도 완료로 승격된다. | C11 CSS Grid parent checkbox는 unchecked 상태를 유지한다. **PASS** |
| 7 | 좁은 구현 slice 때문에 repository-wide 완료 수가 잘못 증가한다. | 공식 완료 수는 `72/213` 그대로다. **PASS** |
| 8 | 네 고정 scene의 결과가 다른 장면 수나 지원 수로 부풀려진다. | 문서는 고정 direct-text 4 scene, 양 플랫폼 각 4/4, root/item/gap 표본 12/12를 그대로 기록한다. **PASS** |
| 9 | Android AVD 결과가 실기기/hardware GPU 검증으로 오인된다. | 구현 근거의 Android 실행은 API 37 ARM64 QEMU AVD이며 실기기/hardware GPU를 주장하지 않는다. **PASS** |
| 10 | iOS drawable 표본이 compositor 이후 pixel 증거로 표현된다. | iPhone 17 Pro/iOS 26.2 Simulator Metal readback은 compositor 제출 전이라고 유지한다. **PASS** |
| 11 | root/item/gap sample이 glyph paint를 검증했다고 해석된다. | 표면 표본은 fill/gap만 검사하고 WGPU glyph paint는 미검증으로 남긴다. **PASS** |
| 12 | fixture-only profile이 일반 author CSS admission으로 확장된다. | `c11-4-grid-track-sizing-fixture-test`와 `RuntimeBlockFlexGridTrackSizingV1` 제한을 유지한다. **PASS** |
| 13 | direct-text first baseline이 전체 baseline 정렬 지원으로 확장된다. | 다중 line, 복수 baseline 및 일반 CSS 지원은 범위 밖으로 유지한다. **PASS** |
| 14 | baseline 구현이 WPT 적합성으로 표현된다. | 이 slice가 WPT/전체 CSS Grid 적합성을 증명하지 않는다는 제한을 유지한다. **PASS** |
| 15 | 문서 동기화가 내부 계약 version을 바꾼다. | 내부 계약 0059는 `0.1.0`을 유지하며 sync 변경은 상태 기록과 링크에 한정한다. **PASS** |
| 16 | 계획, 공식 상태, public roadmap의 merge SHA나 범위가 서로 다르다. | 세 문서의 PR 상태·SHA·완료 수·범위 문구를 교차 대조했다. **PASS** |
| 17 | roadmap review 링크가 Tailnet 선택 build에서 빠져 실제로 열리지 않는다. | pass03 selected-build manifest는 이 검토 문서의 HTML route를 필수 경로로 검증한다. **PASS** |
| 18 | Tailnet 갱신이 preview 외부를 변경하거나 파일을 삭제한다. | pass03 sync script의 대상은 `SpinonPreview`이고 `no_deletes=true`다. 서버 종료/재시작은 실행하지 않았다. **PASS** |
| 19 | Tailnet에 복사한 상태가 원격 페이지와 일치하지 않는다. | pass03 verification은 local path와 원격 HTTP 200/SHA-256을 선택된 각 route에서 대조하고, Agent Browser가 최신 roadmap/STATUS를 렌더링해 확인한다. **PASS** |
| 20 | browser process 또는 unrelated working files가 후속 변경에 섞인다. | Agent Browser를 닫고 프로세스 부재를 확인한다. 문서 PR에는 명시적으로 선택한 상태·roadmap·sync evidence 경로만 stage한다. **PASS** |

**결과:** 20개 항목 PASS. 이 기록은 PR #827의 bounded implementation을 병합한 사실과 문서 동기화 정확성만 판정한다. C11.4와 C11은 미완료이며 공식 완료 수는 `72/213`이다.
