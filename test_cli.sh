PRIVATES=11:i64  1:i64  2:i64  1:i64  2:i64  0:i64  0:i64  0:i64  0:i64  0:i64  0:i64  0:i64  0x7137da164bacaa9332b307e25c1abd906c5c240dcb27e520b84522a1674aab01:bytes-packed  0x33b51854d1cde428aa0379606752a341b85cf1d47803e22330a0c9d41ce59c2b:bytes-packed  0x13184ae5f66fba569653633e1af188615c916cb4121c48632509cd97942d8905:bytes-packed  0x4298271e26561a079d91d589233fe138a6963fcb62a45692d1ddf15009b8de07:bytes-packed  0x2c2ac8a44e2522be90e4e2a50c645b18359e30e32116a17338bdec8b11e74b05:bytes-packed

~/zkWasm/target/release/delphinus-cli --host standard -k 18 --function zkmain --param ./params --output ./output --wasm crates/core/pkg/gameplay_bg.wasm dry-run --private $PRIVATES