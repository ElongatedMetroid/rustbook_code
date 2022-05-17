fn main() {
    let num_list = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "nineth", 
        "tenth", 
        "eleventh", 
        "twelfth"
    ];

    let present_list = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];

    // loop through day 0 to 12, add one to day until it is 12 
    for day in 0..num_list.len() {
        println!("On the {} day of Christmas, my true love sent to me", num_list[day]);

        // loop through 0 to day + 1 in reverse (example (5 + 1)..0)
        // print present_list[6] .. present_list[0]
        for present in (0..(day+1)).rev() {
            println!("{}", present_list[present]);
        }

        print!("\n\n");
    }
}
