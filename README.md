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
  - [Additional Info about Syntax Highlighting](#additional-info-about-syntax-highlighting)
    - [List supported terminals](#list-supported-terminals)
    - [Directory path of custom config](#directory-path-of-custom-config)
    - [Example of custom config](#example-of-custom-config)

## Overview

**Cedrh** (от англ **C**onsole **ED**ito**R** with syntax **H**ighlighting) - консольный текстовый редактор с базовой подсветкой синтаксиса.

<https://github.com/SergoGansta777/Cedrh/assets/98104790/a68d4e14-f323-4674-a80b-d0a3621808e4>

Редактор был написан в качестве курсовой работе по теме _"Разработка программного комплекса текстовый редактор с подсветкой синтаксиса"_. В качестве языка программирования был выбран [Rust](https://www.rust-lang.org/), как современный язык системного программирования, еще этот язык активно используется при написании современных консольных утилит, таких как [ripgrep](https://github.com/BurntSushi/ripgrep) или [bat](https://github.com/sharkdp/bat). Одной из целей работы было познакомиться с Rust и с общими принципами разработки текстовых редакторов с консольным интерфейсом, в процессе работы и изучения материала активно вдохновлялся  [Build Your Own Text Editor](https://viewsourcecode.org/snaptoken/kilo/) и [Kiro](https://github.com/rhysd/kiro-editor/blob/master/README.md)

Более детальную информацию о проекте можно получить в [wiki](https://github.com/SergoGansta777/Cedrh/wiki)

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

```zsh
git clone https://github.com/SergoGansta777/Cedrh.git
```

2. Установите [rust](https://www.rust-lang.org/tools/install)
3. Запустите следующую команду в корне репозитория:

```zsh
cargo build --release
```

4. Затем вы можете запустить собранное приложение:

```zsh
cargo run target/release/cedrh
```

5. Затем вы можете добавить символическую ссылку для запуска приложения

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

| Mapping                   | Description            |
| ------------------------- | ---------------------- |
| `Ctrl-q`                  | Выйти                  |
| `Ctrl-s`                  | Сохранить текущий файл |
| `Ctrl-f`                  | Инкрементный поиск     |
| `↑` or `←` after `Ctrl-f` | Предыдущее совпадение  |
| `↓` or `→` after `Ctrl-f` | Следущее совпадение    |

#### Navigation

| Mapping | Description               |
| ------- | ------------------------- |
| `↑`     | Передвинуть курсор вверх  |
| `↓`     | Передвинуть курсор вниз   |
| `→`     | Передвинуть курсор вправо |
| `←`     | Передвинуть курсов влево  |
| `Home`  | Начало документа          |

## Additional Info about Syntax Highlighting

**Cedrh** - это текстовый редактор с поддержкой подсветки синтаксиса. Для подсветки cedrh использует несколько вариантов определения цветов подсветки:

1. Использование поддерживаемого эмулятора терминала - В таком случае, согласно документации терминала cedrh спарсит конфиг, в котором перечислены цвета терминала. Таким образом **cedrh** поддерживает цветовой стиль среды и вкусовые предпочтения пользователя, с которыми работает. Если пользователь пожелает изменить цвета редактора, ему придется изменить настройки своего терминала
2. Если терминал не поддерживаемый, тогда используется заранее предустановленные стандартная цветовая палитра **cedrh**.
3. Возможно добавления кастомного файла, с конфигурацией цветов, если таковой указан, предпочтение будет отдаваться ему.
4. Запуск программы с флагом `-d` всегда запустит со стандартной цветовой палитрой (подробнее --help)

### List supported terminals

- _Kitty_

### Directory path of custom config

- `Linux/MacOs`: "~/.config/cedrh/cedrh.conf"
- Windows: **work in progress**

### Example of custom config

```
foreground              #CDD6F4
background              #09081B
active_border_color     #B4BEFE

# The 16 terminal colors
color0 #45475A
color8 #585B70

color1 #F38BA8
color9 #F38BA8

color2  #A6E3A1
color10 #A6E3A1

color3  #F9E2AF
color11 #F9E2AF

color4  #89B4FA
color12 #89B4FA

color5  #F5C2E7
color13 #F5C2E7

color6  #94E2D5
color14 #94E2D5

color7  #BAC2DE
color15 #A6ADC8
```
