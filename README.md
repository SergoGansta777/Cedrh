# Cedrh - text editor written in Rust

**WARNING**: Work is still in the progress

## Table of contents

- [Cedrh - text editor written in Rust](#cedrh---text-editor-written-in-rust)
  - [Table of contents](#table-of-contents)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Cargo install](#cargo-install)
    - [Binary](#binary)
  - [Usages](#usages)
  - [Mappings](#mappings)
      - [Operations](#operations)
      - [Navigation](#navigation)

## Overview

**Cedrh** (от англ **C**onsole **ED**ito**R** with syntax **H**ighlighting) - консольный текстовый редактор с базовой подсветкой синтаксиса.

![[Home/Projects/Rust_Course_Work_Sedrh/a![Alt text]Example.gif]]

Редактор был написан в качестве курсовой работе по теме _"Разработка программного комплекса текстовый редактор с подсветкой синтаксиса"_. В качестве языка программирования был выбран [Rust](https://www.rust-lang.org/), как современный язык системного программирования, еще этот язык активно используется при написании современных консольных утилит, таких как [ripgrep](https://github.com/BurntSushi/ripgrep) или [bat](https://github.com/sharkdp/bat). Одной из целей работы было познакомиться с Rust и с общими принципами разработки текстовых редакторов с консольным интерфейсом, в процессе работы и изучения материала активно вдохновлялся  [Build Your Own Text Editor](https://viewsourcecode.org/snaptoken/kilo/) и [Kiro](https://github.com/rhysd/kiro-editor/blob/master/README.md)

Cedrh предоставляет следующие базовые функции:

- Открытие существующего файла или создание нового
- Редактирование файла (вставка, удаление, добавление новой и строки…)
- Сохранение изменений
- Базовая подсветка синтаксиса (цвета подсветки берутся из терминала)
- Инкрементный поиск
- Нативная поддержка UTF 8
- Кроссплатформенность
- Поддержка различных терминалов и их функций (изменение размера окна, полный экран…)

Cedrh является _кроссплатформенной_ утилитой, которая поддерживает следующие операционные системы:

- MacOs
- Unix/Linux
- Windows
- BSD

## Installation

Пожалуйста, используйте новейшую версию Rust

### Cargo install

Текстовый редактор можно установить с помощью `cargo`. Для этого введите команду:

```zsh
cargo install cedrh
```

После этого Вы можете запустить редактор с помощью:

```zsh
cedrh
```

### Binary

1. Клонируйте этот репозиторий
2. Установите [rust](https://www.rust-lang.org/tools/install)
3. Запустите следующую команду в корне репозитория:

```zsh
cargo build --release
```

1. Затем вы можете запустить собранное приложение:

```zsh
cargo run target/release/cedrh   
```

1. Затем вы можете добавить символическую ссылку для запуска приложения

## Usages

Для получения вспомогательной информации используйте флаг `--help`

```zsh
cedrh --help
```

Вы можете открыть на редактирование существующий файл с помощью:

```zsh
cedrh file
```

Если вы хотите создать новый файл, запускайте редактор без аргументов

## Mappings

Управление редактором оптимизировано для работы с клавиатурой, следует знакомым клавишным комбинациям по `nano` или `micro`

#### Operations

| Mapping| Description|
|--|--|
| `Ctrl-q`| Выйти |
| `Ctrl-s`| Сохранить текущий файл|
| `Ctrl-f`| Инкрементный поиск|
| `↑` or ← after `Ctrl-f`| Предыдущее совпадение |
| ↓ or → after `Ctrl-f`| Следущее совпадение |

#### Navigation

| Mapping | Description|
| -- | -- |
| `↑`                    | Передвинуть курсор вверх                    |
| `↓`                    | Передвинуть курсор вниз                |
| `→`                    | Передвинуть курсор вправо               |
| `←`                    | Передвинуть курсов влево               |
| Home | Начало документа |
