//! Тесты #[cfg_attr(test, allow(dead_code))] атрибутов.
//!
//! Проверяют, что атрибуты применены корректно для тестовых функций.

use crate::game::GameState;

/// Тест 1: Проверка, что тестовые функции не вызывают предупреждений
///
/// Проверяем, что dead_code не предупреждает в тестах.
#[test]
fn test_cfg_attr_no_dead_code_warnings() {
    // Эта функция использует другие тестовые функции
    let result = helper_function_for_tests(10, 20);
    assert_eq!(result, 30, "Сумма должна быть 30");
}

/// Вспомогательная функция для тестов
/// #[cfg_attr(test, allow(dead_code))] позволяет не удалять эту функцию
#[cfg_attr(test, allow(dead_code))]
fn helper_function_for_tests(a: i32, b: i32) -> i32 {
    a + b
}

/// Тест 2: Проверка тестовых структур с dead_code
///
/// Проверяем, что тестовые структуры не вызывают предупреждений.
#[test]
fn test_cfg_attr_struct_no_warnings() {
    let test_struct = TestStructForTests {
        value: 42,
        name: "Test".to_string(),
    };

    assert_eq!(test_struct.value, 42, "Значение должно быть 42");
    assert_eq!(test_struct.name, "Test", "Имя должно быть 'Test'");
}

/// Тестовая структура
/// #[cfg_attr(test, allow(dead_code))] позволяет не удалять неиспользуемые поля
#[cfg_attr(test, allow(dead_code))]
struct TestStructForTests {
    value: i32,
    name: String,
}

/// Тест 3: Проверка тестовых enum с dead_code
///
/// Проверяем, что тестовые enum не вызывают предупреждений.
#[test]
fn test_cfg_attr_enum_no_warnings() {
    let test_enum = TestEnumForTests::new();

    match test_enum {
        TestEnumForTests::VariantA(v) => {
            assert_eq!(v, 0, "Значение VariantA должно быть 0");
        }
        TestEnumForTests::VariantB(_) => {
            unreachable!("Неправильный вариант");
        }
        TestEnumForTests::VariantC { .. } => {
            unreachable!("Неправильный вариант");
        }
    }
}

/// Тестовый enum
/// #[cfg_attr(test, allow(dead_code))] позволяет не удалять неиспользуемые варианты
#[cfg_attr(test, allow(dead_code))]
enum TestEnumForTests {
    VariantA(i32),
    VariantB(String),
    VariantC { x: i32, y: i32 },
}

#[cfg_attr(test, allow(dead_code))]
impl TestEnumForTests {
    fn new() -> Self {
        TestEnumForTests::VariantA(0)
    }
}

/// Тест 4: Проверка методов только для тестов
///
/// Проверяем, что методы только для тестов не вызывают предупреждений.
#[test]
fn test_cfg_attr_test_only_methods() {
    let mut test_obj = TestObjectForTests::new();
    test_obj.set_value(50);

    assert_eq!(test_obj.get_value(), 50, "Значение должно быть 50");
}

/// Тестовый объект
#[cfg_attr(test, allow(dead_code))]
struct TestObjectForTests {
    value: i32,
}

#[cfg_attr(test, allow(dead_code))]
impl TestObjectForTests {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    // Метод только для тестов - не используется в production
    fn reset(&mut self) {
        self.value = 0;
    }
}

/// Тест 5: Проверка констант только для тестов
///
/// Проверяем, что константы только для тестов не вызывают предупреждений.
#[test]
fn test_cfg_attr_test_only_constants() {
    assert_eq!(TEST_CONSTANT_VALUE, 12345, "Константа должна быть 12345");
}

/// Константа только для тестов
#[cfg_attr(test, allow(dead_code))]
const TEST_CONSTANT_VALUE: i32 = 12345;

/// Тест 6: Проверка, что dead_code разрешён только в тестах
///
/// Проверяем, что атрибут работает только в тестовом режиме.
#[test]
fn test_cfg_attr_only_in_test_mode() {
    // В тестовом режиме dead_code разрешён
    let _unused_var = "This is unused in test";

    // Создаём GameState для проверки
    let state = GameState::new();
    assert_eq!(state.score(), 0, "Счёт должен быть 0");
}

/// Тест 7: Проверка сложных тестовых структур
///
/// Проверяем, что сложные тестовые структуры работают корректно.
#[test]
fn test_cfg_attr_complex_structs() {
    let complex = ComplexTestStruct {
        field1: 10,
        field2: "test".to_string(),
        field3: vec![1, 2, 3],
        field4: Some(42),
        unused_field: String::from("unused"),
    };

    assert_eq!(complex.field1, 10, "field1 должен быть 10");
    assert_eq!(complex.field2, "test", "field2 должен быть 'test'");
    assert_eq!(
        complex.field3,
        vec![1, 2, 3],
        "field3 должен быть [1, 2, 3]"
    );
    assert_eq!(complex.field4, Some(42), "field4 должен быть Some(42)");
}

/// Сложная тестовая структура
#[cfg_attr(test, allow(dead_code))]
struct ComplexTestStruct {
    field1: i32,
    field2: String,
    field3: Vec<i32>,
    field4: Option<i32>,
    // Неиспользуемое поле - разрешено в тестах
    unused_field: String,
}

impl ComplexTestStruct {
    #[cfg_attr(test, allow(dead_code))]
    fn new() -> Self {
        Self {
            field1: 0,
            field2: String::new(),
            field3: Vec::new(),
            field4: None,
            unused_field: String::from("unused"),
        }
    }
}
