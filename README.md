# rpg-game

# Версии
### **0.1.4**
- добавлена механика здоровья
- добавлены полоски здоровья. у всех персонажей кроме игрока - красные, у игрока - зеленая
- добавлена механика смена анимации в зависимости от направления движения игрока. Самой анимации пока нет)
### **0.1.5**
- добавлен флаг dev-tools для запуска с различными плагинами разработки
- рефакторинг
### **0.1.6**
- добавлена реализация отношения существа к игроку
### **0.1.7**
- враждебные существа стремятся догнать игрока, пугающиеся - убегают
- улучшена система распознавания препятствий на пути
### **0.1.8**
- определены компоненты видимости и дружественности для зомби
### **0.1.9**
- работает убийство монстров с деспавном спрайтов
- пока что урон наносится всем существам в целевой области
### **0.1.10**
- улучшена атака по монстрам, теперь выбирается один из определенной области в направлении курсора относительно игрока 
- рефакторинг
- подготовка к дальней атаке
### **0.1.11**
- добавлена дальняя атака
### **0.1.12**
- добавлена анимация полета стрелы для дальней атаки
### **0.2.0**
- теперь камера следует за игроком
### **0.2.1 - 0.2.4**
- бесшовный переход по уровням
### **0.2.5** 
- добавлен прототип ui главного меню
- переход между состояниями игры
- скорее состояние главного меню сейчас можно назвать паузой


# Планы

### 0.2
- ~~существа делятся на дружественных и враждебных~~
- ~~враждебные существа стараются подойти к персонажу~~
- ~~реализация мили (ближней атаки) персонажа~~
- ~~реализация мили (ближней атаки) враждебного существа~~
- ~~реализация дальней атаки персонажа~~
- ~~смерть персонажа, смерть существа~~
- ~~добавить спрайт стрелы к дальней атаке~~

### 0.3
- ~~динамическая подгрузка соседних уровней~~
- состояния игры: игра, пауза
- возрождение при смерти
- сохранение игры

### 0.4
- сокрытие файлов ассетов и прочего под сериализованными и кодированными файлами data (как в старые добрые)

### 0.5
- простейшая реализация NPC
- состояния игры: диалог/торговля
- простейшая реализация диалогов
- простейшая реализация квестов (а-ля принеси 6 хвостов козы)

### 0.6
- прокачка уровня и статов
- реализация инвентаря
- лут с убитых существ
- торговля

### 0.7
- респавн мобов
- респавн мобов только по истечение времени респавна

### 0.8
- дальняя атака враждебных существ

### 0.9
- механика классов персонажа
- межаника навыков

### 0.10
- смена брони и оружия в визуальном отображении
- кастомизация персонажа

### 0.11
- случайная генерация нестандартных локаций

### 0.12
- случайная генерация истории предметов, существ и может быть локаций

### 0.13
- запретить существам стакаться (ограничить их физический размер полностью по вертикали и частично по горизонтали)
- добавить предметы окружения