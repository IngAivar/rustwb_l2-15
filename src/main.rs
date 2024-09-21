fn main() {
    // Создаем неизменяемую строку с литералом
    let s1 = "hello";

    // Создаем изменяемую строку с помощью метода `String::from`
    let s2 = String::from("hello");

    // Преобразуем `String` в `&str` (срез строки) для совместимости
    let s3 = s2.as_str();

    // Вычисляем размер в байтах, занимаемый переменной `s1` в стеке
    let size_of_s1 = std::mem::size_of_val(s1);

    // Вычисляем размер в байтах, занимаемый указателем на `s2` в стеке
    let size_of_s2 = std::mem::size_of_val(&s2);

    // Вычисляем размер в байтах, занимаемый указателем на `s3` в стеке
    let size_of_s3 = std::mem::size_of_val(&s3);

    // Выводим размеры в консоль
    println!("{:?}", size_of_s1);
    println!("{:?}", size_of_s2);
    println!("{:?}", size_of_s3);
}