/*
# This file incorporates code from the WebAssembly testsuite, originally
# available at https://github.com/WebAssembly/testsuite.
#
# The original code is licensed under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance
# with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
*/
use wasm::{validate, RuntimeInstance};

macro_rules! get_func {
    ($instance:ident, $func_name:expr) => {
        &$instance.get_function_by_name("", $func_name).unwrap()
    };
}

macro_rules! assert_result {
    ($instance:expr, $func:expr, $arg:expr, $result:expr) => {
        assert_eq!($result, $instance.invoke($func, $arg).unwrap());
    };
}

#[test_log::test]
fn memory_size_1() {
    let w = r#"
(module
  (memory 0)
  (func (export "size") (result i32) (memory.size))
  (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
)
  "#;
    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let size = get_func!(i, "size");
    let grow = get_func!(i, "grow");

    assert_result!(i, size, (), 0);
    assert_result!(i, grow, 1, ());
    assert_result!(i, size, (), 1);
    assert_result!(i, grow, 4, ());
    assert_result!(i, size, (), 5);
    assert_result!(i, grow, 0, ());
    assert_result!(i, size, (), 5);
}

#[test_log::test]
fn memory_size_2() {
    let w = r#"
(module
  (memory 1)
  (func (export "size") (result i32) (memory.size))
  (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
)
  "#;
    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let size = get_func!(i, "size");
    let grow = get_func!(i, "grow");

    assert_result!(i, size, (), 1);
    assert_result!(i, grow, 1, ());
    assert_result!(i, size, (), 2);
    assert_result!(i, grow, 4, ());
    assert_result!(i, size, (), 6);
    assert_result!(i, grow, 0, ());
    assert_result!(i, size, (), 6);
}
