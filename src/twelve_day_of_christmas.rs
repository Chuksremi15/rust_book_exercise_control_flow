pub fn print_lyrics() {
    let lyrics_array = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "And a partridge in a pear tree",
    ];

    let array_of_day_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let last_index = lyrics_array.len() - 1;
    let mut track = last_index - 1;

    for (index, _) in lyrics_array.iter().enumerate() {
        let day = array_of_day_numbers[index];
        println!("On the {day} day of Christmas");
        println!("My true love sent to me");

        if index == 0 {
            println!("A partridge in a pear tree");
            println!("");
            continue;
        }

        for line in &lyrics_array[track..=last_index] {
            println!("{line}");
        }

        if last_index == index {
            println!("And a partridge in a pear tree");
        }

        println!("");

        if track > 0 {
            track -= 1;
        }
    }
}
