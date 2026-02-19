// Дебаг для вывода структуры
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Реализ. методов
impl Rectangle {
    // Метод для вычисления площади
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Метод для проверки, может ли текущий прямоугольник вместить другой
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Ассоциированная функция "конструктор" для квадрата
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //Создание экземпляра
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //Попытка вывода (до #[derive(Debug)] не работало)
    //Работает благодаря атрибуту
    println!("rect1 is {rect1:#?}"); // {:#?} просто для красивого вывода

    // Вызов метода ареа
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //Тестирование метода can_hold
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //Использование ассоциированной функции square
    let sq = Rectangle::square(25);
    println!("Square: {sq:#?}");

    // Демонстрация dbg! макроса
    dbg!(&rect1);
}