fn main() {
    let mut number = 0u64;

    for i in 0..65 {
        // Попробуем включить бит
        if let Err(e) = set_bit(&mut number, i) {
            println!("Не удалось включить бит с индексом {}: {}", i, e);
        } else {
            println!("Включили {}-й бит: {}", i, number);
        }

        // Попробуем выключить бит
        if let Err(e) = clear_bit(&mut number, i) {
            println!("Не удалось выключить бит с индексом {}: {}", i, e);
        } else {
            println!("Выключили {}-й бит: {}", i, number);
        }
    }
}

// Включение бита на позиции i
fn set_bit(value: &mut u64, index: u32) -> Result<(), String> {
    if index < 64 {
        *value |= 1 << index;
        Ok(())
    } else {
        Err(format!("Индекс {} выходит за пределы допустимого диапазона", index))
    }
}

// Выключение бита на позиции i
fn clear_bit(value: &mut u64, index: u32) -> Result<(), String> {
    if index < 64 {
        *value &= !(1 << index);
        Ok(())
    } else {
        Err(format!("Индекс {} выходит за пределы допустимого диапазона", index))
    }
}
