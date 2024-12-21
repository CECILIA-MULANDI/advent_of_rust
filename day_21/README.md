### Day 21: It's Blitzen again

“Why is the sleigh autopilot slower than a reindeer in quicksand? JINGLESTACK is down, and the temp directory is 800 terabytes!”

Blitzen spins around in his chair, looking guilty. “It’s… fine! Just a minor bug in my Rust code.”

Bernard, the lead elf, cuts in, holding a clipboard. “A bug? Every file in the temp directory is creating three more when dropped. It’s a recursive explosion!”

Santa’s eyes narrow at Blitzen. “Recursive explosion? You’ve turned my servers into a snowball fight gone wrong! Fix it now, or I’ll make you clean every one of those files manually!”

Blitzen gulps, this is not his first time making Santa angry, cracking his knuckles. “On it! Uh, any chance we can blame the interns?”

Santa points a candy cane at him. “One more excuse, and you’re off sleigh duty for good!”

### Your Mission

The previous code Blitzen has written was supposed to create temporary files, but they were permanent.

You need to write a struct TempFile that is temporary and it will delete itself when out of scope.
