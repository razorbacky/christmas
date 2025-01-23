// Christmas Song . 25.01.23
// 크리스마스 캐롤 반복

fn main() {
    println!("TWLVE DAYS OF CHRISTMAS : ");

    for day in 1..13 { // for 문을 통해 day 에 1~13까지 Argument를 전달한다.
        intro(day); // 전달 받은 인자를 intro 함수에 대입한다.

        for gift_day in (1..(day + 1)).rev() { // .rev는 출력 순서를 뒤집는다.
            gift(
                gift_day,
                if gift_day == 1 && day != 1 { // prefix 접두사, 1절에서는 and를 표시하지 않음, 2절부터 and를 표시함
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

fn intro(n: u32) { // intro 함수에서 전달받은 인자를 n 에 할당한다.
    let day = match n { // n 에 할당된 것과 동일한 것을 매칭하여 찾는다. 매칭된 것은 day 변수에 할당한다.
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };
    println!("\nOn the {day} day of Christmas\nmy true love sent to me :"); // n이 매칭된 day 변수를 포함하여 출력한다.
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };

    println!("{prefix}{gift_text}");
}