use wasm::{validate, Error, RuntimeInstance, ValidationInfo};
use wasm::{value, ValType};

#[test_log::test]
fn memory_basic() {
    let w = r#"
(module (memory 0))
(module (memory 1))
(module (memory 0 0))
(module (memory 0 1))
(module (memory 1 256))
(module (memory 0 65536))
"#
    .split("\n")
    .map(|el| el.trim())
    .filter(|el| el.len() > 0)
    .collect::<Vec<&str>>();

    w.iter().for_each(|wat| {
        let wasm_bytes = wat::parse_str(wat).unwrap();
        let validation_info = validate(&wasm_bytes).expect("validation failed");
        RuntimeInstance::new(&validation_info).expect("instantiation failed");
    });
}

#[test_log::test]
fn memory_min_greater_than_max() {
    let w = r#"
(module (memory 1 0))
    "#
    .split("\n")
    .map(|el| el.trim())
    .filter(|el| el.len() > 0)
    .collect::<Vec<&str>>();

    w.iter().for_each(|wat| {
        let wasm_bytes = wat::parse_str(wat).unwrap();
        let validation_info = validate(&wasm_bytes);
        assert_eq!(
            validation_info.err().unwrap(),
            Error::SizeMinIsGreaterThanMax
        );
    });
}

#[test_log::test]
fn memory_size_must_be_at_most_4gib() {
    let w = r#"
    (module (memory 65537))
    (module (memory 2147483648))
    (module (memory 4294967295))
    (module (memory 0 65537))
    (module (memory 0 2147483648))
    (module (memory 0 4294967295))
        "#
    .split("\n")
    .map(|el| el.trim())
    .filter(|el| el.len() > 0)
    .collect::<Vec<&str>>();

    w.iter().for_each(|wat| {
        let wasm_bytes = wat::parse_str(wat).unwrap();
        let validation_info = validate(&wasm_bytes);
        assert_eq!(validation_info.err().unwrap(), Error::MemSizeTooBig);
    });
}

#[test_log::test]
fn memory_test_data_section() {
    let wat = r#"(module
  (memory 1)
  (data (i32.const 0) "ABC\a7D")
  (data (i32.const 20) "WASM")
  (data (memory 0) (i32.const 1) "WASM")

  ;; Data section
  (func $c)

)"#;

/**
 *   (func (export "data") (result i32)
    (i32.and
      (i32.and
        (i32.and
          (i32.eq (i32.load8_u (i32.const 0)) (i32.const 65))
          (i32.eq (i32.load8_u (i32.const 3)) (i32.const 167))
        )
        (i32.and
          (i32.eq (i32.load8_u (i32.const 6)) (i32.const 0))
          (i32.eq (i32.load8_u (i32.const 19)) (i32.const 0))
        )
      )
      (i32.and
        (i32.and
          (i32.eq (i32.load8_u (i32.const 20)) (i32.const 87))
          (i32.eq (i32.load8_u (i32.const 23)) (i32.const 77))
        )
        (i32.and
          (i32.eq (i32.load8_u (i32.const 24)) (i32.const 0))
          (i32.eq (i32.load8_u (i32.const 1023)) (i32.const 0))
        )
      )
    )
  )
 */

    let wasm_bytes = wat::parse_str(wat).unwrap();
    let validation_info = validate(&wasm_bytes).expect("validation failed");
    let instance = RuntimeInstance::new(&validation_info).expect("instantiation failed");
}
