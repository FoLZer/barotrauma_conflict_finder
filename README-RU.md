# Barotrauma Conflict Finder

`barotrauma_conflict_finder` - это инструмент, созданный для выявления конфликтов между различными модами, которые установлены и активны в игре Barotrauma.
Этот инструмент также позволяет вам создать патч, который исправляет конфликтующие файлы.

## Features

- Обнаруживает конфликты между различными модами, установленными в игре Barotrauma.
- Создание патча для исправления конфликтующих файлов.

## Установка

### GitHub

1. Загрузите последний релиз для вашей операционной системы со страницы [Releases] (https://github.com/FoLZer/barotrauma_conflict_finder/releases).
2. Запустите исполняемый файл.

## Ручная установка

Чтобы установить `barotrauma_conflict_finder`, вам необходимо иметь установленный [Rust] (https://www.rust-lang.org/learn/get-started).

1. Клонируйте репозиторий:
    ``sh
    git clone https://github.com/FoLZer/barotrauma_conflict_finder.git
    cd barotrauma_conflict_finder
    ```

2. Соберите и запустите проект:
    ``sh
    cargo run --release
    ```

## Использование

### Запуск с GUI-редактором

Просто запустите исполняемый файл (либо из Cargo, либо загруженный исполняемый файл).

### Запуск с помощью аргументов

Вы также можете запустить инструмент с коммандной строки, задавая аргументы напрямую. Это не обходит GUI, но позволяет задать настройки для программы в качестве аргументов вместо того, чтобы задавать их позже в GUI.

```sh
barotrauma_conflict_finder.exe [GAME_PATH] [CONFIG_PLAYER_PATH] [PATCH_MOD_PATH]
```

- `GAME_PATH`: Путь к корневой папки игры Barotrauma. (по умолчанию: "C:\Program Files (x86)\Steam\steamapps\common\Barotrauma")
- `CONFIG_PLAYER_PATH`: Путь к файлу конфигурации игрока. (по умолчанию: "%GAME_PATH%\config_player.xml")
- `PATCH_MOD_PATH`: Путь к папке, в которую будет сохранен патч. (по умолчанию: "%GAME_PATH%\LocalMods\conflict_finder_patchmod")

## Contributing

Вклад приветствуется! Если у вас есть дополнение или пожелание к программе, откройте "Issue" (для пожеланий) или "Pull Request" (для дополнений с включенным кодом) в репозиторие GitHub.

## License

Этот проект использует лицензию GPL-2.0. Подробности смотрите в файле [LICENSE](LICENSE).

## Contacts

Если у вас возникли вопросы или предложения, не стесняйтесь открыть "Issue" в репозитории GitHub.
Также вы можете написать мне напрямую в Discord: @folzer
