//!  Uma pequena biblioteca para validação de CPFs
use std;

/// Checa se a `string` de input é uma representação válida de um CPF.
/// O input deve conter exatamente 11 dígitos e nenhum caractere especial.
pub fn validate_cpf<S: AsRef<str>>(input: S) -> bool {
    let input = input.as_ref();

    if input.len() != 11 {
        return false;
    }

    let cpf_as_numbers: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();

    if cpf_as_numbers.len() != 11 {
        return false;
    }

    if !cpf_as_numbers.iter().any(|&n| n != cpf_as_numbers[0]) {
        return false;
    }

    validate_first_digit(&cpf_as_numbers) && validate_second_digit(&cpf_as_numbers)
}

fn validate_first_digit(numbers: &[u32]) -> bool {
    validate_group(&[10, 9, 8, 7, 6, 5, 4, 3, 2], numbers, 9)
}

fn validate_second_digit(numbers: &[u32]) -> bool {
    validate_group(&[11, 10, 9, 8, 7, 6, 5, 4, 3, 2], numbers, 10)
}

fn validate_group(weights: &[u32], numbers: &[u32], validator_index: usize) -> bool {
    assert_eq!(numbers.len(), 11);
    assert!(weights.len() < numbers.len());
    assert!(validator_index < numbers.len());

    let result = numbers
        .iter()
        .zip(weights.iter())
        .map(|(v, w)| v * w)
        .sum::<u32>() * 10 % 11;

    match (result, numbers[validator_index]) {
        (11...std::u32::MAX, 10...std::u32::MAX) => {
            unreachable!("Result greater than 10 or element in numbers greater than 9")
        }
        (10, 0) => (),
        (x, y) if x == y => (),
        _ => return false,
    }

    true
}