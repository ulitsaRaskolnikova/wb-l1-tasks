fn main() {
    let mut number = 0u64;

    for i in 0..65 {
        // Попробуем включить бит
        if let Err(e) = set_bit(&mut number, i) {
            println!("Failed to set bit at index {}: {}", i, e);
        } else {
            println!("Set {}-th bit: {}", i, number);
        }

        // Попробуем выключить бит
        if let Err(e) = clear_bit(&mut number, i) {
            println!("Failed to clear bit at index {}: {}", i, e);
        } else {
            println!("Cleared {}-th bit: {}", i, number);
        }
    }
}

// Включение бита на позиции i
fn set_bit(value: &mut u64, index: u32) -> Result<(), String> {
    if index < 64 {
        *value |= 1 << index; // Включаем бит
        Ok(())
    } else {
        Err(format!("Index {} is out of range", index)) // Возвращаем ошибку, если индекс выходит за пределы
    }
}

// Выключение бита на позиции i
fn clear_bit(value: &mut u64, index: u32) -> Result<(), String> {
    if index < 64 {
        *value &= !(1 << index); // Выключаем бит
        Ok(())
    } else {
        Err(format!("Index {} is out of range", index)) // Возвращаем ошибку, если индекс выходит за пределы
    }
}
