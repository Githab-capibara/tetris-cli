//! Модуль работы с линиями.
//!
//! # Ответственность
//! - Поиск заполненных линий
//! - Удаление заполненных линий и сдвиг верхних линий вниз
//!
//! # Зависимости
//! - [`crate::io::GRID_HEIGHT`](crate::io): высота игрового поля

use crate::io::GRID_HEIGHT;

/// Найти все заполненные линии.
///
/// # Возвращает
/// Битовую маску заполненных линий и количество заполненных линий
///
/// # Производительность
/// O(n) сложность где n = `GRID_HEIGHT` (20 итераций).
/// Используется `.all()` с ранним выходом при обнаружении пустой ячейки.
///
/// # Исправление #6
/// Исправлено: не требуется .take() так как `row` имеет тип `[i8; GRID_WIDTH]`
/// и итерация происходит по всем элементам массива фиксированного размера.
#[must_use]
pub fn find_full_rows(blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT]) -> (u32, u32) {
    let mut rows_mask: u32 = 0;
    let mut remove_count = 0;

    // Поиск заполненных линий
    for (y, row) in blocks.iter().enumerate() {
        // Оптимизация: .all() делает ранний выход при первом false
        // Исправление #2.4: убран .take(GRID_WIDTH) как избыточный
        let row_full = row.iter().all(|&cell| cell != -1);
        if row_full {
            rows_mask |= 1 << y;
            remove_count += 1;
        }
    }

    (rows_mask, remove_count)
}

/// Удалить заполненные линии и сдвинуть верхние линии вниз.
///
/// # Аргументы
/// * `blocks` - игровое поле (изменяемое)
/// * `rows_mask` - битовая маска заполненных линий
pub fn remove_rows(blocks: &mut [[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT], rows_mask: u32) {
    // Проверка валидности rows_mask
    if rows_mask >= (1u32 << GRID_HEIGHT) {
        eprintln!(
            "Предупреждение: rows_mask ({}) выходит за пределы поля (максимум {})",
            rows_mask,
            (1u32 << GRID_HEIGHT) - 1
        );
        return;
    }

    // Подсчитываем количество строк для удаления снизу вверх
    let mut rows_removed_below = 0;

    for y in (0..GRID_HEIGHT).rev() {
        if (rows_mask & (1 << y)) != 0 {
            rows_removed_below += 1;
        } else if rows_removed_below > 0 {
            // Перемещаем строку вниз на rows_removed_below позиций
            if y + rows_removed_below < GRID_HEIGHT {
                blocks[y + rows_removed_below] = blocks[y];
            }
        }
    }

    // Заполняем верхние строки пустыми значениями (-1)
    for row in blocks.iter_mut().take(rows_removed_below) {
        *row = [-1; crate::io::GRID_WIDTH];
    }
}

#[cfg(test)]
mod lines_tests {
    use super::*;

    #[test]
    fn test_find_full_rows_empty() {
        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let (mask, count) = find_full_rows(&blocks);
        assert_eq!(mask, 0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_remove_rows_single() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        // Заполняем последнюю строку
        blocks[GRID_HEIGHT - 1] = [1i8; crate::io::GRID_WIDTH];

        remove_rows(&mut blocks, 1u32 << (GRID_HEIGHT - 1));

        // Последняя строка должна быть пустой
        assert!(blocks[GRID_HEIGHT - 1].iter().all(|&c| c == -1));
    }
}
