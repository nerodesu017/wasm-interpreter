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
    Error, RuntimeError, RuntimeInstance,
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

#[test_log::test]
fn table_init_3_test() {
    let w = r#"
(module
  (type (func (result i32)))  ;; type #0
  (func (export "ef0") (result i32) (i32.const 0))    ;; index 0
  (func (export "ef1") (result i32) (i32.const 1))
  (func (export "ef2") (result i32) (i32.const 2))
  (func (export "ef3") (result i32) (i32.const 3))
  (func (export "ef4") (result i32) (i32.const 4))    ;; index 4
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
    (table.init $t0 1 (i32.const 7) (i32.const 0) (i32.const 4))
         (elem.drop 1)
         (table.init $t0 3 (i32.const 15) (i32.const 1) (i32.const 3))
         (elem.drop 3)
         (table.copy $t0 0 (i32.const 20) (i32.const 15) (i32.const 5))
         (table.copy $t0 0 (i32.const 21) (i32.const 29) (i32.const 1))
         (table.copy $t0 0 (i32.const 24) (i32.const 10) (i32.const 1))
         (table.copy $t0 0 (i32.const 13) (i32.const 11) (i32.const 4))
         (table.copy $t0 0 (i32.const 19) (i32.const 20) (i32.const 5)))
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
    assert!(i.invoke::<i32, i32>(check, 13).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 14).unwrap());
    assert_eq!(5, i.invoke(check, 15).unwrap());
    assert_eq!(2, i.invoke(check, 16).unwrap());
    assert_eq!(7, i.invoke(check, 17).unwrap());
    assert!(i.invoke::<i32, i32>(check, 18).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(9, i.invoke(check, 19).unwrap());
    assert!(i.invoke::<i32, i32>(check, 20).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 21).unwrap());
    assert!(i.invoke::<i32, i32>(check, 22).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(8, i.invoke(check, 23).unwrap());
    assert_eq!(8, i.invoke(check, 24).unwrap());
    assert!(i.invoke::<i32, i32>(check, 25).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 26).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 27).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 28).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 29).err().unwrap() == RuntimeError::UninitializedElement);
}

#[test_log::test]
fn table_init_4_test() {
    let w = r#"
(module
  (type (func (result i32)))  ;; type #0
  (func (export "ef0") (result i32) (i32.const 0))    ;; index 0
  (func (export "ef1") (result i32) (i32.const 1))
  (func (export "ef2") (result i32) (i32.const 2))
  (func (export "ef3") (result i32) (i32.const 3))
  (func (export "ef4") (result i32) (i32.const 4))    ;; index 4
  (table $t0 30 30 funcref)
  (table $t1 30 30 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 5))  ;; index 5
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))  ;; index 9
  (func (export "test")
    (table.init $t1 3 (i32.const 15) (i32.const 1) (i32.const 3)))
  (func (export "check") (param i32) (result i32)
    (call_indirect $t1 (type 0) (local.get 0)))
)
    "#;

    let wasm_bytes = wat::parse_str(w).unwrap();
    let validation_info = validate(&wasm_bytes).unwrap();
    let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let test = get_func!(i, "test");
    let check = get_func!(i, "check");

    i.invoke::<(), ()>(test, ()).unwrap();

    println!("{:#?}", i.store.tables[1]);

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

#[test_log::test]
fn table_init_5_test() {
    let w = r#"
(module
  (type (func (result i32)))  ;; type #0
  (func (export "ef0") (result i32) (i32.const 0))    ;; index 0
  (func (export "ef1") (result i32) (i32.const 1))
  (func (export "ef2") (result i32) (i32.const 2))
  (func (export "ef3") (result i32) (i32.const 3))
  (func (export "ef4") (result i32) (i32.const 4))    ;; index 4
  (table $t0 30 30 funcref)
  (table $t1 30 30 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 5))  ;; index 5
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))  ;; index 9
  (func (export "test")
    (table.init $t1 1 (i32.const 7) (i32.const 0) (i32.const 4))
         (elem.drop 1)
         (table.init $t1 3 (i32.const 15) (i32.const 1) (i32.const 3))
         (elem.drop 3)
         (table.copy $t1 1 (i32.const 20) (i32.const 15) (i32.const 5))
         (table.copy $t1 1 (i32.const 21) (i32.const 29) (i32.const 1))
         (table.copy $t1 1 (i32.const 24) (i32.const 10) (i32.const 1))
         (table.copy $t1 1 (i32.const 13) (i32.const 11) (i32.const 4))
         (table.copy $t1 1 (i32.const 19) (i32.const 20) (i32.const 5)))
  (func (export "check") (param i32) (result i32)
    (call_indirect $t1 (type 0) (local.get 0)))
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
    assert!(i.invoke::<i32, i32>(check, 13).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 14).unwrap());
    assert_eq!(5, i.invoke(check, 15).unwrap());
    assert_eq!(2, i.invoke(check, 16).unwrap());
    assert_eq!(7, i.invoke(check, 17).unwrap());
    assert!(i.invoke::<i32, i32>(check, 18).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(9, i.invoke(check, 19).unwrap());
    assert!(i.invoke::<i32, i32>(check, 20).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(7, i.invoke(check, 21).unwrap());
    assert!(i.invoke::<i32, i32>(check, 22).err().unwrap() == RuntimeError::UninitializedElement);
    assert_eq!(8, i.invoke(check, 23).unwrap());
    assert_eq!(8, i.invoke(check, 24).unwrap());
    assert!(i.invoke::<i32, i32>(check, 25).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 26).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 27).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 28).err().unwrap() == RuntimeError::UninitializedElement);
    assert!(i.invoke::<i32, i32>(check, 29).err().unwrap() == RuntimeError::UninitializedElement);
}

#[test_log::test]
fn table_init_6_test() {
    let w = r#"
(module
    (func (export "test")
(elem.drop 0)))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        assert!(validate(&wasm_bytes).err().unwrap() == Error::ElementIsNotDefined(0));
}

#[test_log::test]
fn table_init_7_test() {
    let w = r#"
  (module
    (func (export "test")
      (table.init 0 (i32.const 12) (i32.const 1) (i32.const 1))))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        assert!(validate(&wasm_bytes).err().unwrap() == Error::TableIsNotDefined(0));
}

#[test_log::test]
fn table_init_8_test() {
    let w = r#"
  (module
    (elem funcref (ref.func 0))
    (func (result i32) (i32.const 0))
    (func (export "test")
      (elem.drop 4)))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        assert!(validate(&wasm_bytes).err().unwrap() == Error::ElementIsNotDefined(4));
}

#[test_log::test]
fn table_init_9_test() {
    let w = r#"
  (module
    (elem funcref (ref.func 0))
    (func (result i32) (i32.const 0))
    (func (export "test")
      (table.init 4 (i32.const 12) (i32.const 1) (i32.const 1))))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        assert!(validate(&wasm_bytes).err().unwrap() == Error::TableIsNotDefined(0));
}


#[test_log::test]
fn table_init_10_test() {
    let w = r#"
(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (elem.drop 2)
    ))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        let validation_info = validate(&wasm_bytes).unwrap();
        let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");
    
        let test = get_func!(i, "test");
    
        i.invoke::<(), ()>(test, ()).unwrap();
}


#[test_log::test]
fn table_init_11_test() {
    let w = r#"
(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init 2 (i32.const 12) (i32.const 1) (i32.const 1))
    ))
        "#;
    
        let wasm_bytes = wat::parse_str(w).unwrap();
        let validation_info = validate(&wasm_bytes).unwrap();
        let mut i = RuntimeInstance::new(&validation_info).expect("instantiation failed");
    
        let test = get_func!(i, "test");
    
    // let res = i.invoke::<(), ()>(test, ());
    // println!("{:#?}", res);
        assert!(i.invoke::<(), ()>(test, ()).err().unwrap() == RuntimeError::TableAccessOutOfBounds);
}

/*

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init 2 (i32.const 12) (i32.const 1) (i32.const 1))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init 1 (i32.const 12) (i32.const 1) (i32.const 1))
    (table.init 1 (i32.const 21) (i32.const 1) (i32.const 1))))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (elem.drop 1)
    (elem.drop 1)))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (elem.drop 1)
    (table.init 1 (i32.const 12) (i32.const 1) (i32.const 1))))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init 1 (i32.const 12) (i32.const 0) (i32.const 5))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init 1 (i32.const 12) (i32.const 2) (i32.const 3))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 28) (i32.const 1) (i32.const 3))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 12) (i32.const 4) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 12) (i32.const 5) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 30) (i32.const 2) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 31) (i32.const 2) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 30) (i32.const 4) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t0 1 (i32.const 31) (i32.const 5) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 26) (i32.const 1) (i32.const 3))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 12) (i32.const 4) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 12) (i32.const 5) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 28) (i32.const 2) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 29) (i32.const 2) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 28) (i32.const 4) (i32.const 0))
    ))
(invoke "test")

(module
  (table $t0 30 30 funcref)
  (table $t1 28 28 funcref)
  (elem (table $t1) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t1) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 0))
  (func (result i32) (i32.const 1))
  (func (result i32) (i32.const 2))
  (func (result i32) (i32.const 3))
  (func (result i32) (i32.const 4))
  (func (result i32) (i32.const 5))
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))
  (func (export "test")
    (table.init $t1 1 (i32.const 29) (i32.const 5) (i32.const 0))
    ))
(assert_trap (invoke "test") "out of bounds table access")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (i64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i32.const 1) (f64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (i64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f32.const 1) (f64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (i64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (i64.const 1) (f64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f32.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f32.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f32.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f32.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (i64.const 1) (f64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f64.const 1) (i32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f64.const 1) (f32.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f64.const 1) (i64.const 1))))
  "type mismatch")

(assert_invalid
  (module
    (table 10 funcref)
    (elem funcref (ref.func $f0) (ref.func $f0) (ref.func $f0))
    (func $f0)
    (func (export "test")
      (table.init 0 (f64.const 1) (f64.const 1) (f64.const 1))))
  "type mismatch")

(module
  (type (func (result i32)))
  (table 32 64 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 0) (local.get $len))))
(assert_trap (invoke "run" (i32.const 24) (i32.const 16)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 16)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 17)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 18)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 19)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 20)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 21)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 22)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 23)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 24)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 25)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 26)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 27)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 28)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 29)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 30)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 31)) "uninitialized element")

(module
  (type (func (result i32)))
  (table 32 64 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 0) (local.get $len))))
(assert_trap (invoke "run" (i32.const 25) (i32.const 16)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 16)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 17)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 18)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 19)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 20)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 21)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 22)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 23)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 24)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 25)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 26)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 27)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 28)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 29)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 30)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 31)) "uninitialized element")

(module
  (type (func (result i32)))
  (table 160 320 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 0) (local.get $len))))
(assert_trap (invoke "run" (i32.const 96) (i32.const 32)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 16)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 17)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 18)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 19)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 20)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 21)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 22)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 23)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 24)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 25)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 26)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 27)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 28)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 29)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 30)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 31)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 32)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 33)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 34)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 35)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 36)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 37)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 38)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 39)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 40)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 41)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 42)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 43)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 44)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 45)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 46)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 47)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 48)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 49)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 50)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 51)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 52)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 53)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 54)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 55)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 56)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 57)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 58)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 59)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 60)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 61)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 62)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 63)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 64)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 65)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 66)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 67)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 68)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 69)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 70)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 71)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 72)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 73)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 74)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 75)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 76)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 77)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 78)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 79)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 80)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 81)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 82)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 83)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 84)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 85)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 86)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 87)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 88)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 89)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 90)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 91)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 92)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 93)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 94)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 95)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 96)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 97)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 98)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 99)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 100)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 101)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 102)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 103)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 104)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 105)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 106)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 107)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 108)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 109)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 110)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 111)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 112)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 113)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 114)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 115)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 116)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 117)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 118)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 119)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 120)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 121)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 122)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 123)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 124)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 125)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 126)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 127)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 128)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 129)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 130)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 131)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 132)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 133)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 134)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 135)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 136)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 137)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 138)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 139)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 140)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 141)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 142)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 143)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 144)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 145)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 146)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 147)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 148)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 149)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 150)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 151)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 152)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 153)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 154)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 155)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 156)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 157)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 158)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 159)) "uninitialized element")

(module
  (type (func (result i32)))
  (table 160 320 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 0) (local.get $len))))
(assert_trap (invoke "run" (i32.const 97) (i32.const 31)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 16)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 17)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 18)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 19)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 20)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 21)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 22)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 23)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 24)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 25)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 26)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 27)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 28)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 29)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 30)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 31)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 32)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 33)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 34)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 35)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 36)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 37)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 38)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 39)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 40)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 41)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 42)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 43)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 44)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 45)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 46)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 47)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 48)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 49)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 50)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 51)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 52)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 53)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 54)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 55)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 56)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 57)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 58)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 59)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 60)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 61)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 62)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 63)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 64)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 65)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 66)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 67)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 68)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 69)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 70)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 71)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 72)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 73)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 74)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 75)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 76)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 77)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 78)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 79)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 80)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 81)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 82)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 83)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 84)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 85)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 86)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 87)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 88)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 89)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 90)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 91)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 92)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 93)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 94)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 95)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 96)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 97)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 98)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 99)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 100)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 101)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 102)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 103)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 104)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 105)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 106)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 107)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 108)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 109)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 110)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 111)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 112)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 113)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 114)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 115)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 116)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 117)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 118)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 119)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 120)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 121)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 122)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 123)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 124)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 125)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 126)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 127)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 128)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 129)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 130)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 131)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 132)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 133)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 134)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 135)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 136)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 137)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 138)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 139)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 140)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 141)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 142)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 143)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 144)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 145)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 146)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 147)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 148)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 149)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 150)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 151)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 152)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 153)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 154)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 155)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 156)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 157)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 158)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 159)) "uninitialized element")

(module
  (type (func (result i32)))
  (table 64 64 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 0) (local.get $len))))
(assert_trap (invoke "run" (i32.const 48) (i32.const 4294967280)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 16)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 17)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 18)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 19)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 20)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 21)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 22)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 23)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 24)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 25)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 26)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 27)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 28)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 29)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 30)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 31)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 32)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 33)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 34)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 35)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 36)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 37)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 38)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 39)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 40)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 41)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 42)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 43)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 44)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 45)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 46)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 47)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 48)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 49)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 50)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 51)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 52)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 53)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 54)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 55)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 56)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 57)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 58)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 59)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 60)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 61)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 62)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 63)) "uninitialized element")

(module
  (type (func (result i32)))
  (table 16 16 funcref)
  (elem funcref
    (ref.func $f0) (ref.func $f1) (ref.func $f2) (ref.func $f3)
    (ref.func $f4) (ref.func $f5) (ref.func $f6) (ref.func $f7)
    (ref.func $f8) (ref.func $f9) (ref.func $f10) (ref.func $f11)
    (ref.func $f12) (ref.func $f13) (ref.func $f14) (ref.func $f15))
  (func $f0 (export "f0") (result i32) (i32.const 0))
  (func $f1 (export "f1") (result i32) (i32.const 1))
  (func $f2 (export "f2") (result i32) (i32.const 2))
  (func $f3 (export "f3") (result i32) (i32.const 3))
  (func $f4 (export "f4") (result i32) (i32.const 4))
  (func $f5 (export "f5") (result i32) (i32.const 5))
  (func $f6 (export "f6") (result i32) (i32.const 6))
  (func $f7 (export "f7") (result i32) (i32.const 7))
  (func $f8 (export "f8") (result i32) (i32.const 8))
  (func $f9 (export "f9") (result i32) (i32.const 9))
  (func $f10 (export "f10") (result i32) (i32.const 10))
  (func $f11 (export "f11") (result i32) (i32.const 11))
  (func $f12 (export "f12") (result i32) (i32.const 12))
  (func $f13 (export "f13") (result i32) (i32.const 13))
  (func $f14 (export "f14") (result i32) (i32.const 14))
  (func $f15 (export "f15") (result i32) (i32.const 15))
  (func (export "test") (param $n i32) (result i32)
    (call_indirect (type 0) (local.get $n)))
  (func (export "run") (param $offs i32) (param $len i32)
    (table.init 0 (local.get $offs) (i32.const 8) (local.get $len))))
(assert_trap (invoke "run" (i32.const 0) (i32.const 4294967292)) "out of bounds table access")
(assert_trap (invoke "test" (i32.const 0)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 1)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 2)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 3)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 4)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 5)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 6)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 7)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 8)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 9)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 10)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 11)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 12)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 13)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 14)) "uninitialized element")
(assert_trap (invoke "test" (i32.const 15)) "uninitialized element")

(module
  (table 1 funcref)
  ;; 65 elem segments. 64 is the smallest positive number that is encoded
  ;; differently as a signed LEB.
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref) (elem funcref) (elem funcref) (elem funcref)
  (elem funcref)
  (func (table.init 64 (i32.const 0) (i32.const 0) (i32.const 0))))



*/
