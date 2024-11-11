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
use wasm::{validate, RuntimeError, RuntimeInstance};
use wasm::Error as GeneralError;
use wasm::value::{FuncRefForInteropValue, Ref};

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

macro_rules! assert_error {
    ($instance:expr, $func:expr, $arg:expr, $ret_type:ty, $invoke_param_type:ty, $invoke_return_type:ty, $err_type:expr) => {
        let val: $ret_type =
            $instance.invoke::<$invoke_param_type, $invoke_return_type>($func, $arg);
        assert!(val.is_err());
        assert!(val.unwrap_err() == $err_type);
    };
}


#[test_log::test]
fn table_fill_test() {
    let w = r#"
    (module
      (table $t 10 funcref)
    
      (func (export "fill") (param $i i32) (param $r funcref) (param $n i32)
        (table.fill $t (local.get $i) (local.get $r) (local.get $n))
      )
    
      (func (export "fill-abbrev") (param $i i32) (param $r funcref) (param $n i32)
        (table.fill $t (local.get $i) (local.get $r) (local.get $n))
      )
    
      (func (export "get") (param $i i32) (result funcref)
        (table.get $t (local.get $i))
      )
    )
    "#;


    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let get = get_func!(i, "get");
    let fill = get_func!(i, "fill");
    let fill_abbrev = get_func!(i, "fill-abbrev");

}