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
    ($instance:expr, $func_name:expr, $arg:expr, $result:expr) => {
        assert_eq!($result, $instance.invoke($func_name, $arg).unwrap());
    };
}


#[test_log::test]
fn memory_test_exporting_rand_globals_doesnt_change_a_memory_s_semantics() {
    let w = r#"
    (module
        (memory (export "memory0") 1 1)
        (data (i32.const 2) "\03\01\04\01")
        (data (i32.const 12) "\07\05\02\03\06")
        (func (export "test")
            (nop))
        (func (export "load8_u") (param i32) (result i32)
            (i32.load8_u (local.get 0))
        )
    )
  "#;
    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let test = get_func!(i, "test");
    i.invoke::<(), ()>(test, ()).unwrap();

    let load8_u = get_func!(i, "load8_u");
    let results = Vec::from([0,0,3,1,4,1,
            0,0,0,0,0,0,
            7,5,2,3,6,0,
            0,0,0,0,0,0,
            0,0,0,0,0,0]);
    for j in 0..30 {
        assert_result!(i, load8_u, j as i32, results[j]);
    }
    


}
