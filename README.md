# FRenamer

Утилита командной строки для переименования файлов по заданным правилам.

## Правила переименования

- Все цифры переносятся в конец имени файла (перед расширением)
- Специальные символы (точки, тире) заменяются на нижнее подчеркивание
- Расширение файла сохраняется

### Пример
```
35.search-insert-position.rs -> search_insert_position_35.rs
test-file.123.txt -> test_file_123.txt
```

## Установка

```bash
cargo install frenamer
```

## Использование

```bash
# Переименовать один файл
frenamer path/to/file.txt

# Переименовать все файлы в директории (рекурсивно)
frenamer path/to/directory

# Показать подробную информацию о процессе
frenamer --verbose path/to/directory

# Показать справку
frenamer --help
```

## Опции

- `-v, --verbose` - Показать подробную информацию о процессе
- `-h, --help` - Показать справку
- `-V, --version` - Показать версию

## Разработка

### Требования

- Rust 1.70+
- Cargo

### Сборка

```bash
git clone https://github.com/your-username/frenamer
cd frenamer
cargo build --release
```

### Тестирование

```bash
cargo test
```

## Лицензия

MIT - [LICENSE](LICENSE)
