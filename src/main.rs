use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

const ALPHABET: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";

fn clean_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| ALPHABET.contains(*c) || *c == ' ')
        .collect()
}

fn decode_vizhener(text: &str, key: &str) -> String {
    let clean_key: Vec<char> = key
        .to_lowercase()
        .chars()
        .filter(|c| ALPHABET.contains(*c))
        .collect();
    if clean_key.is_empty() {
        return text.to_string();
    }

    let alphabet: Vec<char> = ALPHABET.chars().collect();
    let alphabet_len = alphabet.len();
    let mut result = String::with_capacity(text.len());
    let mut key_pos = 0;

    for ch in text.chars() {
        if !ALPHABET.contains(ch) {
            result.push(ch);
        } else {
            let key_char = clean_key[key_pos % clean_key.len()];
            key_pos += 1;
            let char_index = alphabet.iter().position(|&c| c == ch).unwrap();
            let key_index = alphabet.iter().position(|&c| c == key_char).unwrap();
            let new_index = if char_index >= key_index {
                char_index - key_index
            } else {
                alphabet_len + char_index - key_index
            };
            result.push(alphabet[new_index]);
        }
    }
    result
}

fn main() -> io::Result<()> {
    let content = fs::read_to_string("russian.txt")?;
    let words: Vec<String> = content
        .lines()
        .map(|line| clean_text(line.trim()))
        .filter(|w| !w.is_empty())
        .collect();

    let word_set: HashSet<String> = words.iter().cloned().collect();
    let candidate_keys: Vec<String> = words
        .into_iter()
        .filter(|word| word.chars().count() > 5)
        .collect();

    print!("Введите исходный текст: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let source = clean_text(input.trim());
    if source.is_empty() {
        println!("Исходный текст не может быть пустым");
        return Ok(());
    }

    let total_keys = candidate_keys.len();
    let start_time = Instant::now();
    let mut results: Vec<(String, String, usize)> = Vec::with_capacity(total_keys);

    for (i, key) in candidate_keys.iter().enumerate() {
        let decoded = decode_vizhener(&source, key);
        let score = decoded
            .split_whitespace()
            .filter(|w| word_set.contains(*w))
            .count();
        results.push((key.clone(), decoded, score));

        if i % 10_000 == 0 {
            let percent = (i as f64 / total_keys as f64) * 100.0;
            println!(
                "Обработано {} ключей из {} ({:.2}%)",
                i, total_keys, percent
            );
        }
    }

    results.sort_by(|a, b| b.2.cmp(&a.2));

    println!("\nТоп 10 вариантов:");
    for (key, decoded, score) in results.iter().take(10) {
        println!("Ключ: {}, Счёт: {}", key, score);
        println!("Расшифровка: {}", decoded);
        println!("Ключ для расшифровки: {}", key);
        println!("--------------------------");
    }

    let elapsed = start_time.elapsed();
    println!("Время выполнения: {} мс", elapsed.as_millis());
    Ok(())
}
