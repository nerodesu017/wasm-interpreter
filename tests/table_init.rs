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

use wasm::{
    validate,
    value::{FuncAddr, FuncRefForInteropValue, Ref},
    Error, RuntimeError, RuntimeInstance, ValType,
};

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
fn table_init_1_test() {
    let w = r#"
    (module
        (type (func (result i32)))
        (func (export "ef0") (result i32) (i32.const 0))
        (func (export "ef1") (result i32) (i32.const 1))
        (func (export "ef2") (result i32) (i32.const 2))
        (func (export "ef3") (result i32) (i32.const 3))
        (func (export "ef4") (result i32) (i32.const 4))
        (table $t0 30 30 funcref)
        (table $t1 30 30 funcref)
        (elem (table $t0) (i32.const 2) func 3 1 4 1)
        (elem funcref
            (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
        (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
        (elem funcref
            (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
        (func (result i32) (i32.const 5))  ;; index 5
        (func (result i32) (i32.const 6))
        (func (result i32) (i32.const 7))
        (func (result i32) (i32.const 8))
        (func (result i32) (i32.const 9))  ;; index 9
        (func (export "test")
            (table.init $t0 1 (i32.const 7) (i32.const 0) (i32.const 4)))
        (func (export "check") (param i32) (result i32)
            (call_indirect $t0 (type 0) (local.get 0)))
    )
    "#;

    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let test = get_func!(i, "test");
    let check = get_func!(i, "check");

    i.invoke::<(), ()>(test, ()).unwrap();

    assert!(i.invoke::<i32, i32>(check, 0).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 1).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(3, i.invoke(check, 2).unwrap());
    assert_eq!(1, i.invoke(check, 3).unwrap());
    assert_eq!(4, i.invoke(check, 4).unwrap());
    assert_eq!(1, i.invoke(check, 5).unwrap());
    assert!(i.invoke::<i32, i32>(check, 6).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(2, i.invoke(check, 7).unwrap());
    assert_eq!(7, i.invoke(check, 8).unwrap());
    assert_eq!(1, i.invoke(check, 9).unwrap());
    assert_eq!(8, i.invoke(check, 10).unwrap());
    assert!(i.invoke::<i32, i32>(check, 11).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 12).unwrap());
    assert_eq!(5, i.invoke(check, 13).unwrap());
    assert_eq!(2, i.invoke(check, 14).unwrap());
    assert_eq!(3, i.invoke(check, 15).unwrap());
    assert_eq!(6, i.invoke(check, 16).unwrap());
    assert!(i.invoke::<i32, i32>(check, 17).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 18).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 19).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 20).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 21).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 22).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 23).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 24).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 25).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 26).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 27).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 28).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 29).err().unwrap() == RuntimeError::UninitializedElement);
}


#[test_log::test]
fn table_init_2_test() {
    let w = r#"
(module
    (type (func (result i32)))  ;; type #0
    (func (export "ef0") (result i32) (i32.const 0))
    (func (export "ef1") (result i32) (i32.const 1))
    (func (export "ef2") (result i32) (i32.const 2))
    (func (export "ef3") (result i32) (i32.const 3))
    (func (export "ef4") (result i32) (i32.const 4))
    (table $t0 30 30 funcref)
    (table $t1 30 30 funcref)
    (elem (table $t0) (i32.const 2) func 3 1 4 1)
    (elem funcref
        (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8)
    )
    (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
    (elem funcref
        (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6)
    )
    (func (result i32) (i32.const 5))  ;; index 5
    (func (result i32) (i32.const 6))
    (func (result i32) (i32.const 7))
    (func (result i32) (i32.const 8))
    (func (result i32) (i32.const 9))  ;; index 9
    (func (export "test")
        (table.init $t0 3 (i32.const 15) (i32.const 1) (i32.const 3))
    )
    (func (export "check") (param i32) (result i32)
        (call_indirect $t0 (type 0) (local.get 0))
    )
)
    "#;

    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let test = get_func!(i, "test");
    let check = get_func!(i, "check");

    i.invoke::<(), ()>(test, ()).unwrap();

    assert!(i.invoke::<i32, i32>(check, 0).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 1).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(3, i.invoke(check, 2).unwrap());
    assert_eq!(1, i.invoke(check, 3).unwrap());
    assert_eq!(4, i.invoke(check, 4).unwrap());
    assert_eq!(1, i.invoke(check, 5).unwrap());
    assert!(i.invoke::<i32, i32>(check, 6).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 7).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 8).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 9).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 10).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 11).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 12).unwrap());
    assert_eq!(5, i.invoke(check, 13).unwrap());
    assert_eq!(2, i.invoke(check, 14).unwrap());
    assert_eq!(9, i.invoke(check, 15).unwrap());
    assert_eq!(2, i.invoke(check, 16).unwrap());
    assert_eq!(7, i.invoke(check, 17).unwrap());
    assert!(i.invoke::<i32, i32>(check, 18).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 19).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 20).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 21).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 22).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 23).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 24).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 25).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 26).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 27).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 28).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 29).err().unwrap() == RuntimeError::UninitializedElement);
}
