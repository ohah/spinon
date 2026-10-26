# PR #653 · 변경 실패 관점 검토 30개

기본 30관점 검토는 [PR #653](https://github.com/ohah/spinon/pull/653)의 base `baa55101a5557edbc547ccd4208db7f1906f5760`와 당시 head `bc04740f892690df2e544e6e7e80597d3d33660d` 사이 diff를 대상으로 했다. 이후 세 커밋 제목만 한글화됐다. 2026-10-26 revision metadata 재확인 시 원격 PR head와 로컬 `HEAD`는 새 head `27edda1314eef77a0e6263c37e37624fe7f29d1a`에서 일치했고, base는 그대로였다. 이전·새 commit tree와 base-to-head binary diff SHA-256이 동일해 substantive diff가 바뀌지 않았음을 확인했다. 따라서 아래 30개는 앞선 substantive review 결과이며, 이번 revision metadata recheck는 새 30회 검토로 세지 않는다.

| 이전 commit | 새 commit 제목 | 새 commit SHA | 양쪽 commit tree SHA |
| --- | --- | --- | --- |
| `2f7d5f0410155344ba9a957df13a7ee2a0df6f5d` | `docs(c11-4): 자동 트랙 늘림 계획` | `cabc7e6d670fd3c8aaee1c2b91eb9ea92b6e952b` | `8da4703ed04baf81a9feb628daf00b762ae36fb9` |
| `977fc5d831dc3cd3462646cc99c7562c31541ff4` | `test(c11-4): 자동 트랙 늘림 단계 검증` | `143ee56ca9f6fdba0f20b019903eece51495b8f6` | `659e163a13c0b7b6dce8d6cb372ee506719cb91e` |
| `bc04740f892690df2e544e6e7e80597d3d33660d` | `docs(c11-4): 늘림 근거 한국어화` | `27edda1314eef77a0e6263c37e37624fe7f29d1a` | `fa452fb712c25133393ee793f28e00a6e2a90e1a` |

Base-to-head `git diff --binary` SHA-256은 이전·새 head 모두 `2545542267a7746df2740ab3e72ad1cbe122c923cdc0415b36d546b889e4d047`다. 새 시점에도 PR은 OPEN, mergeable이며 `chore`, `area: layout`, `area: testing` 라벨을 갖는다. review decision과 status check rollup은 비어 있고, PR 본문은 CI 결과가 없다고 밝힌다. PR 본문은 현재도 PR diff 검토가 진행 중이라고 표시한다. 이번 metadata recheck에서는 테스트나 브라우저를 실행하지 않았다.

1. **대상 revision 고정** — substantive review 당시의 원격 base/head를 기록했고, metadata recheck에서 GitHub base `baa55101a5557edbc547ccd4208db7f1906f5760`, 새 head `27edda1314eef77a0e6263c37e37624fe7f29d1a`와 로컬 `HEAD` 일치를 확인했다. **PASS**
2. **PR diff 경계** — base부터 head까지 Rust 테스트 모듈, 테스트 등록, 계획, 검토 자료와 실행 근거만 변경됐다. 앱·플랫폼 코드나 `spec/STATUS.md`는 diff에 없다. **PASS**
3. **작업 유형·영역 라벨** — `chore`와 layout/testing 영역 라벨이 있어 작업 유형과 주요 영역을 표시한다. **PASS**
4. **제목 형식** — PR 제목은 `test(c11-4): auto track stretch 단계 검증`으로 갱신됐다. 영어 접두어 뒤의 설명은 CSS 동작명만 원어로 두고 한국어로 작성했다. **PASS**
5. **문서 언어** — 추가된 계획·리뷰·실행 근거의 서술은 한국어다. 초기에 영어였던 evidence README는 `bc04740f`에서 한국어로 수정되어 프로젝트 언어 규칙을 충족한다. **PASS**
6. **PR 본문 형식과 범위 설명** — C11.4 테스트 전용 변경, API 없음, 미지원 범위 및 화면 캡처가 없는 이유가 본문에서 구분된다. **PASS**
7. **근거 링크 연결성** — PR이 링크한 계획과 evidence README가 diff 안에 있으며, README에서 focused test·fmt·Clippy 로그와 별도 계획/구현 검토로 연결한다. **PASS**
8. **검증 주장과 기록 범위** — 본문과 README가 focused test 1/1, fmt, targeted Clippy와 계획/구현 20관점 검토를 주장하고, diff에 해당 로그·리뷰가 있다. 이 PR diff 검토는 별도의 30관점 검토 산출물이며 여기서는 실행 검증을 다시 주장하지 않는다. **PASS**
9. **CI 상태 공개** — PR status check rollup은 비어 있고 본문도 GitHub CI 결과가 없다고 밝힌다. 비어 있는 상태를 CI 통과로 표시하지 않는다. **PASS**
10. **공식 완료 상태** — `spec/STATUS.md`는 바뀌지 않았고 C11.4 미완료 및 공식 `72/213` 유지 문구가 PR 본문·계획·근거에서 일치한다. **PASS**
11. **제품 코드와 테스트 경계** — 새 모듈은 `lib.rs`의 `#[cfg(test)]` 아래에서만 등록된다. 레이아웃 알고리즘이나 trace 생산 코드는 변경하지 않는다. **PASS**
12. **API·ABI·fixture 표면** — 공개/내부 versioned interface, FFI, fixture feature, 앱 adapter 변경이 없다. **PASS**
13. **테스트 입력 식별자** — root와 세 child에 고정된 고유 NodeId를 사용하며 trace owner와 frame lookup도 동일 ID를 쓴다. **PASS**
14. **revision 입력** — `LayoutInput`의 tree/style/environment revision을 유효한 고정 기본값으로 구성한다. **PASS**
15. **root constraint 일관성** — root와 viewport 모두 `20×100px`, `RootSizingPolicy::Match`여서 definite root block size가 일치한다. **PASS**
16. **box geometry 기본값** — 기본 border-box, zero border/padding/margin을 사용해 border box와 content box 차이가 free-space 산술에 들어오지 않는다. **PASS**
17. **축별 style 투영** — 10px row gap과 zero column gap을 각각 지정하고 row pass를 대상으로 한다. 기존 변환은 row gap을 Taffy의 block-axis gap으로 보낸다. **PASS**
18. **정렬 조건** — positive 입력은 `align-content:stretch`, 대조군은 `Position(Start)`다. 기존 style 변환상 첫 입력만 Taffy `STRETCH`로 매핑되어 stretch stage 조건을 분리한다. **PASS**
19. **track sizing 함수 선택** — rows `[Auto, Auto, LengthPx(5)]`는 auto 최대 sizing function 두 개와 stretch 대상이 아닌 fixed 5px track 하나를 만든다. **PASS**
20. **자동 배치 topology** — 단일 열, row auto-flow, 세 direct child와 세 explicit rows를 사용해 child가 각 행 하나씩 차례로 배치된다. **PASS**
21. **intrinsic contribution 안정성** — child 높이 `[10,20,5]`와 너비 20px가 고정되어 text wrapping, font, replaced content 측정이 기대값을 흔들지 않는다. **PASS**
22. **독립 free-space 산술** — 선행 row bases 35px과 두 내부 gap 20px을 빼 `100−35−20=45px`을 얻고 이를 세 auto track이 아닌 두 auto track에만 배분한다. **PASS**
23. **trace 활성화와 owner** — `GridPlacementMode::TrackSizing`으로 trace를 활성화하고, trace 하나의 node가 root ID인지 확인한다. **PASS**
24. **pass·axis 선택** — 단일 trace 안에 Rows pass가 정확히 하나인지 확인하므로 columns pass나 다른 grid의 snapshot을 잘못 읽지 않는다. **PASS**
25. **단계 정체성과 순서** — 선행 단계와 `StretchAutoTracks`가 각각 한 번 존재하고 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서를 요구한다. **PASS**
26. **선행 snapshot 불변식** — stretch 이전 세 stage의 CSS row bases가 각각 `[10,20,5]`인지 검사해 기대 변화가 앞 단계에서 발생한 경우를 잡는다. **PASS**
27. **stretch 결과와 fixed track 보호** — positive stage에서 `{applied:true}`와 `[32.5,42.5,5]`를 검사하고 fixed 5px row를 보존한다. 전체 stage 공간 합이 container 높이를 채우는지도 확인한다. **PASS**
28. **gutter 구조와 크기** — interleaved vector 길이와 track/gutter 순서, collapsed boundary gutter flags, 네 gutter의 `[0,10,10,0]` 크기를 검사한다. gap 누락·중복을 free-space 산술과 별도로 잡는다. **PASS**
29. **수치 및 positive frame 경계** — 비교 helper가 모든 기대값의 유한성과 `1e-4` 허용 오차를 확인한다. stretch 입력에서 root와 세 child의 x/y/width/height를 직접 단언한다. **PASS**
30. **negative control 결과와 frame** — `start` 입력에서 `{applied:false}`, 원래 row bases, 45px unused space, root frame과 child y `[0,20,50]`를 검사한다. 이 slice가 다루지 않는 indefinite space·minimum fallback·WPT·플랫폼 parity·전체 C11.4는 근거에 미지원 범위로 적혀 있다. **PASS**

## 판정

**PASS — substantive PR diff 30/30, 차단 항목 0개.** 후속 metadata-only recheck는 세 commit 제목만 한글화됐으며 파일 diff가 byte-for-byte 동일함을 확인했다. 본문·계획의 PR diff 검토 상태 문구는 아직 pending이다. PR은 recheck 시점에 OPEN이며 CI status checks는 비어 있으므로 CI 통과를 의미하지 않는다. 이 기록은 merge 승인이나 merge 수행 기록이 아니다.
