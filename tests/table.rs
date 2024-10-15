// (module
//     (table $fns 2 funcref)

//     (elem (i32.const 0) $add) ;; add function reference at index 0
//     (func $add (param $x i32) (param $y i32) (result i32)
//       local.get $x
//       local.get $y
//       i32.add
//     )
//   )
use wasm::{validate, Limits, RefType, RuntimeInstance};

#[test_log::test]
fn table_basic() {
    let wat = r#"
(module
    (table $fns 2 funcref)
    
    ;; (elem (i32.const 0) $add) ;; add function reference at index 0
    
    ;;(func $add (param $x i32) (param $y i32) (result i32)
    ;;    local.get $x
    ;;    local.get $y
    ;;    i32.add
    ;;)
  )
"#;

    let wasm_bytes = wat::parse_str(wat).unwrap();
    let validation_info = validate(&wasm_bytes).expect("validation failed");
    let instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");

    let tables = instance.store.tables;
    assert_eq!(tables.len(), 1);
    let table1 = tables.get(0).unwrap();
    assert_eq!(table1.ty.lim, Limits {min: 2, max: None});
    assert_eq!(table1.ty.et, RefType::FuncRef);
}
