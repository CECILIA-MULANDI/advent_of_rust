### Day 5: Parsing naughty-list.csv

The elves stared at their screens. They had just written the Kid struct and were testing it with Santa’s data.

But something was wrong with the data, Prancer leaned back, smirking. "We forgot something obvious, didn’t we? The data’s raw strings—we need to parse it first."

    Alice,10,2
    Bob,5,5
    Charlie,1,9

"We need to create another function," Prancer continued. "to parse the CSV rows into Kid structs."

Blitzen slammed his mug down. "And since Santa put me in charge of this project, I’m naming the function. It’s going to be called parse_row."

An elf from the back muttered just loud enough to hear, "Ugh, he thinks he’s better than us because Santa made him lead."

Blitzen shot them a look. "I heard that. If you’ve got a better name, I’m listening."

Silence.

"Exactly. parse_row it is."

### The Frustration

Blitzen paced. "We need a function that takes a CSV row, splits it, and converts it into a Kid. Name is easy—it stays a String. The good and bad deeds, though, need to be parsed to u32."

"But what if the row has garbage data?" asked an elf, holding up a note with Charlie,,9 scribbled on it.

Prancer rolled his eyes. "Obviously, we handle errors. No .unwrap() shortcuts."

### The Task

Blitzen wants you to create an associated function for the Kid struct and name it parse_row. It should take a CSV row as a &str and return a Result<Kid, &'static str>. The function should:

Split the CSV row into parts.
Extract the name as a String.
Parse the second and third fields as u32 for good and bad deeds.
Finally create a Kid struct using the new() associated function we created earlier.
