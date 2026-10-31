# C11.3.3 fixed-only `auto` guard 후속 검증

fixed-only `TemplateGridImplicitTracks`가 template area 없는 `grid-auto-* : auto`를 받아들이지 않도록 기존 validation 조건을 복원한 뒤 최종 소스를 다시 검사했다. 새 positive named-line candidate는 별도 `TemplateGridPositiveNamedLineAutoImplicitTracks`에서만 `auto`를 허용한다.

- [전체 Grid regression](./layout-grid-tests.log): 143/143 통과.
- [C11.3.3 style projection](./style-projection-tests.log): 20/20 통과. 이 중 positive named-end-only implicit 배치가 positive named-start candidate보다 먼저 오는 source-order negative case를 포함한다.
- [CSS reference / fixture boundary Node tests](./css-reference-node-tests.log): 51/51 통과.
- [workspace Clippy](./workspace-clippy.log), [fixture feature Clippy](./fixture-clippy.log), `cargo fmt --all -- --check`: 통과.
- [기본 Release build](./default-build.log): [archive hash](./default-sha256.txt), fixture symbol 0개와 신규 scene marker 0개.
- [fixture-test Release build](./fixture-test-build.log): [archive hash](./fixture-test-sha256.txt), [fixture symbol 2개](./fixture-test-symbols.txt), [scene payload marker 3개](./fixture-test-payload-markers.txt).
- 검사 뒤 기본 archive를 복원했으며 [복원 build](./default-restore-build.log) hash는 기본 archive와 같다.

macOS ARM64 static archive 경계만 확인했다. Android APK/iOS IPA 전체의 fixture 제외는 이 검사에 포함하지 않는다.
