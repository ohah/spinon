# C11.4 direct-text aspect-ratio min/max 계획 PR diff 검토 30개

**검토 대상:** 계획·계획 검토·Chrome precomparison fixture/evidence의 최종 staged diff · 2026-10-30

각 관점은 별도 파일 경계·규범 조건·검증 실패를 확인했다. blocker는 없었다. Taffy 버전과 replaced/non-replaced 용어를 다시 소스와 대조하고 문서·fixture를 수정한 뒤 최종 hash와 browser capture를 갱신했다.

1. **변경 경계:** 계획, 두 검토 기록, 해당 precomparison의 14개 파일만 PR 범위다. 그 외 미추적 스냅샷과 evidence는 포함되지 않았다.
2. **상태 회계:** `spec/STATUS.md`는 diff에 없으며 공식 `72/213`과 C11 checkbox를 유지한다.
3. **작업 순서:** 계획 상태가 명시돼 있고 계획 PR 병합 전 구현에 들어가지 않도록 적었다.
4. **제품 API:** 계획은 CSS author admission이나 완료 상태를 선언하지 않는다.
5. **상위 연결:** C11.4 parent plan, 내부 계약 0059, STATUS 링크가 저장소 내 실제 경로로 연결된다.
6. **규범 URL:** Grid/Sizing Level 4의 primary CSSWG 초안 링크와 섹션이 쓰였다. 초안이라는 상태도 명시한다.
7. **Grid 용어:** non-replaced item의 content-size suggestion과 replaced item의 transferred-size suggestion을 구분한다.
8. **Intrinsic 단계:** track frame만으로 내부 기여 단계를 추정하지 않으며 runtime trace를 acceptance에 요구한다.
9. **Vendored Taffy 고정:** 문서가 `vendor/taffy/Cargo.toml`과 layout crate의 정확한 `0.15.0`을 가리킨다.
10. **현재 후보 track:** canonical fixture의 `auto` row와 `max-content` columns가 현재 direct-text collector에서 허용되는 조합이다.
11. **Root geometry:** 100px width, auto height, gap 0, normal alignment와 root edges 0의 subset이 분명하다.
12. **Item cardinality:** 두 direct-text child `AAAA`/`A`만 허용하며 nested, extra, replaced item이 범위에 섞이지 않는다.
13. **Font 재현성:** local Ahem과 embedded Ahem 모두 준비 완료를 검사하며 fixture hash가 font asset과 연결된다.
14. **Ratio owner:** ratio owner는 `AAAA` 하나다. Wrong owner와 duplicate ownership rejection을 검토했다.
15. **기존 ratio 회귀:** min/max 초기값에서는 기존 finite-positive ratio subset을 유지하고, 새 min/max positive는 2:1로 제한한다.
16. **No-ratio control:** computed `auto`를 명시한 두 control이 ratio 전이 누락·축 독립성을 검출한다.
17. **Opposite-axis minimum:** min-width/min-height가 바뀌는 방향의 row/column 결과가 둘 다 있다.
18. **Opposite-axis maximum:** max-width/max-height 전이의 양 축 결과가 고정돼 있다.
19. **Automatic minimum:** `min-width:auto`와 `min-width:0` 장면을 분리해 40px min-content floor와 30px transferred cap을 혼동하지 않는다.
20. **Constraint precedence:** source min이 source max보다 큰 가로·세로 장면을 따로 기록한다.
21. **Cross clamp:** opposite max가 transferred minimum을 제한하는 case와 opposite min이 transferred maximum을 제한하는 case를 함께 둔다.
22. **Box-sizing 숫자 ratio:** padding/border가 있는 numeric ratio control은 border-box basis다.
23. **`auto <ratio>` basis:** border-box authored value에도 auto ratio가 content-box를 사용하며 edge를 정확히 한 번 복원한다.
24. **Allowed CSS domain:** width/height auto, margin zero, min/max 0..60px로 제한하고 percentage/calc/keyword/negative/out-of-range 입력을 닫는다.
25. **Root and item rejection:** root min/max, unsupported rows, explicit placement, stale revisions, wrong profile rejection을 수용 기준에 연결했다.
26. **Expected row/column values:** 17개 JSON ID/순서·track list·frame·computed ratio가 관측 JSON과 exact로 대조된다.
27. **Raw provenance:** canonical 및 embedded Agent Browser 원본 stdout과 normalized JSON을 별도 보존한다.
28. **Font-source equality:** 두 원본 문서의 17개 browser result와 UA가 같고 `fontReady=true`인지 verifier가 검사한다.
29. **Verifier negative control:** item height 1px 변형이 기대값 검증에서 실패하는지 확인하고, 모든 hash를 로컬 파일과 대조한다.
30. **화면·세션·브라우저 경계:** full-page screenshot hash가 저장되고 Agent Browser session이 닫혀 `active=false`, `pid=null`이며 terminal-browser 사용 기록이 없다.

**실행 결과:** `git diff --cached --check` 성공, local Markdown link targets 전부 존재, `verify-precomparison.mjs`가 17 exact cases와 negative control을 통과했다. 이는 문서/evidence PR 점검이며 product build, runtime matrix, CI 통과를 뜻하지 않는다.
