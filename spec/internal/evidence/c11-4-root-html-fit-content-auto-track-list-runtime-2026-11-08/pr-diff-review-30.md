# C11.4 fit-content implicit-row list `auto` member · PR diff 실패 관점 검토 30개

## 검토 범위

- 검토한 staged candidate diff SHA-256 (이 review 파일 제외): `af39447703022ade37017a53257187a53b01c5f6eecab79355f3e5cd1893c9fd`
- 코드, fixture, plan, 내부 계약, public roadmap와 Android/iOS 및 Tailnet evidence를 함께 검토했다.
- 각 항목은 별도의 실패 경로·불변 조건·계약 경계다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않았다.

## 실패 관점

1. **일반 profile 오염** — Auto 허용은 `RootHtmlFitContentAutoRowsV1`의 `validate_tracks` 인자에서만 켜진다. 새 enum/profile을 일반 CSS에 노출하지 않는다. 통과.
2. **placement 후처리 누락** — `normalize_sparse_grid_container`도 동일 profile에서만 `GridTrackSize::Auto` implicit row member를 받는다. 다른 placement mode는 기존 rejection을 유지한다. 통과.
3. **축 오염** — 변경은 `grid-auto-rows` typed list에만 적용된다. `grid-auto-columns`, explicit columns/rows와 template sizing allowlist를 열지 않는다. 통과.
4. **fit-content 선행 조건 삭제** — 새 Auto arm이 생겨도 list predicate는 non-empty와 FitContent 존재를 계속 요구한다. 통과.
5. **빈 목록 수용** — projection/Layout에서 빈 목록을 거부하고 error property를 `grid-auto-rows`로 보존한다. 통과.
6. **track quota 우회** — projection과 Layout 양쪽에서 `GRID_MAX_TRACKS` 초과를 차단한다. 10,001개 projection fixture가 거부되는 것을 확인했다. 통과.
7. **`min-content` 우발 admission** — mixed list의 지원 타입 집합에 없고 projection/Layout negative control이 거부한다. 통과.
8. **`max-content` 우발 admission** — C11.4 fit-content profile의 intrinsic sizing allowlist로 전파되지 않는다. projection negative matrix와 validator match를 대조했다. 통과.
9. **`fr` 우발 admission** — 값이 유한해도 이 profile에서 허용되지 않는다. projection/Layout에서 fail-closed한다. 통과.
10. **`minmax()` 우발 admission** — finite min/max 내부 값을 이유로 whole track을 통과시키지 않는다. projection negative matrix에 typed `MinMax`가 있다. 통과.
11. **계산식 grammar 확대** — direct typed Calc는 track list predicate에서 제외된다. Runtime dark-media negative control은 percentage-bearing fit-content calc를 `grid-auto-rows`로 거부한다. 통과.
12. **px 숫자 검증 누락** — 음수/NaN 길이는 projection에서 거부되고 Layout direct input도 finite non-negative 검사를 거친다. 통과.
13. **percentage 숫자 검증 누락** — 음수 및 infinity percentage는 거부한다. zero percentage는 의도된 positive boundary다. 통과.
14. **fit-content cap clamp/NaN** — 음수·비유한 cap을 fail-closed하고 120% 유한 cap을 100%로 clamp하지 않는 기존 계약을 유지한다. 통과.
15. **indefinite basis 대체** — Grid 높이 100px의 definite content-box basis를 고정한다. viewport나 화면 크기를 percent 계산에 대신 넣지 않는다. 통과.
16. **computed identity 손실** — projection은 FitContent, Auto, px/percentage typed variant와 author 순서를 보존하고 computed serialization도 전체 pattern을 출력한다. 통과.
17. **explicit row가 cycle index를 소비** — explicit 10px row 뒤 첫 implicit row에서 첫 pattern member를 사용한다. 세 scene frame과 기대값이 이를 확인한다. 통과.
18. **cycle wrap 오류** — pattern 끝에서 첫 member로 돌아와 네 번째 implicit row를 계산한다. 세 member list의 row vector가 확인된다. 통과.
19. **Auto intrinsic contribution** — `min-height:0` item의 고정 nested payload가 해당 Auto row의 max-content contribution이다. Auto track을 고정 px나 0으로 바꾸지 않는다. 통과.
20. **남은 공간 stretch 혼입** — fixture의 `align-content:start`는 free-space 분배를 제외한다. 이 PR이 `normal|stretch` 또는 general alignment 결과를 암묵적으로 약속하지 않는다. 통과.
21. **zero fit cap 결과** — `fit-content(0%)`은 implicit row 0px로 남고 다음 Auto row의 30px 기여를 밀어내지 않는다. 통과.
22. **overflow와 track frame 혼동** — 0px row에서도 nested payload의 34px frame과 visible overflow를 유지한다. 자손을 row 높이로 잘라내지 않는다. 통과.
23. **중복/누락 frame identity** — RuntimeSession scene마다 width 40px인 10 frame과 unique NodeId를 대조한다. box count만으로 통과시키지 않는다. 통과.
24. **Grid owner 넓히기** — bounded owner/40×100 shape/fixed explicit tracks/sparse row flow/zero gap 조건은 바꾸지 않았다. 다른 style/placement는 이전 unsupported diagnostics 경계를 유지한다. 통과.
25. **child topology 확대** — direct item/payload topology와 3 또는 5 visible child count 경계를 유지하며 nested Grid·arbitrary descendants 지원을 주장하지 않는다. 통과.
26. **진단 대상 손실** — style/layout failure는 `grid-auto-rows` property와 Grid owner NodeId를 전달한다. Runtime negative assert가 owner identity를 확인한다. 통과.
27. **partial scene publication** — invalid revision에서 completed layout과 render snapshot을 내보내지 않는다. 통과.
28. **session poison** — dark unsupported revision 뒤 같은 RuntimeSession instance가 light revision에서 `Ready`로 돌아오고 recovered payload frame을 확인한다. 통과.
29. **Simulator artifact misbinding** — Android APK와 iOS app bundle의 packaged HTML hash를 fixture hash에 대조하고, 설치 실행 log는 각 simulator와 같은 candidate executable에 결합했다. Screenshot은 visual smoke로만 기록하고 pixel readback을 주장하지 않는다. 시험 뒤 앱/Bootstrap 산출물을 보관 hash로 복원했다. 통과.
30. **문서 완료 상태 오염** — `spec/STATUS.md` 체크를 변경하지 않았고 official `78/214` 및 contract `0.1.0`을 유지했다. plan/contract/public Tailnet roadmap은 후보·남은 제한을 명시하며 C11.4/C11 완료로 표시하지 않는다. 통과.

## 결과

30개의 분리된 실패 관점을 확인했고 수정이 필요한 결함을 찾지 않았다. PR 생성 후에도 CI/check rollup, merge state, merge SHA를 이 검토와 별개로 확인해야 한다. C11.4/C11 및 official `78/214`는 미완료 상태다.

## 후속 문서·로드맵 확인 · 2026-11-08

- 후속 변경을 포함한 staged candidate diff SHA-256 (이 review 파일 제외): `9d5f3c5fbd695f3c963200f64144b4d0471e63b97ada43da2fa2d8fa4c13dd7b`.
- 원래 30개 항목 이후 Rust/runtime 동작 코드는 변경하지 않았다. 새 변경은 PR 상태 문구, 칸반 진행 카드의 짧은 `auto member` 제목, Tailnet sync/process evidence다. 항목 30을 다시 확인해 공식 `78/214`, 미완료 C11.4/C11, contract `0.1.0` 상태를 유지한다.
- Agent Browser 원격 화면은 desktop 1280px 및 mobile 390px에서 `auto member`와 `C11.4 · ACTIVE`를 표시했고 mobile document/body 폭은 390px였다. Tailnet source와 served HTML SHA-256은 `1d858acee89e244073e50e2d40bf12ca1146cca2563fd7b280a07382910c0d59`로 일치한다.
- README, 계획, 내부 계약과 roadmap은 PR diff review 30개가 완료됐으며 PR 생성·CI/check 확인·merge가 남았다고 일치시켰다. `git diff --cached --check`를 다시 통과했다. PR/CI/merge 상태는 이 문서 검토와 별도로 확인한다.
