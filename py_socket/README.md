# py-socket

### Включение и выключение умной розетки с помощью Python

1. Создать виртуальное окружение
```
python -m venv .venv
```

2. Активировать виртуальное окружение, например, для Linux, Mac
```
source .venv/bin/activate
```

3. Установить требуемые пакеты
```
pip install -r requirements.txt
```

4. Собрать пакет языка программирования Python и установить его в созданное виртуальное окружение.

```
maturin develop
```

5. После чего возможно использовать класс SmartClient пакета pysocket в интерактивном и пакетном режимах. Например при запуске

```
python ./examples/smartsocket.py
```
