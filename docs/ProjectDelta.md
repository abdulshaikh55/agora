# Project Delta

## What's new:
There have been two significant changes in the Delta phase. They are not much because I had to learn File I/O in Rust from scratch.
1. A new struct called Priority has been added to help users prioritize and sequentialize their tasks. The output depending on priority will be added in the next phase.
   
2. File I/O: Now we can save the tasks locally and access them whenever we want. When you add a task, it automatically updates the task in the file you specified. Updating a task in the terminal will also update the task in the file. Deleting task(s) will automatically delete tasks in the file. The [tasks.json](..\tasks.json) file is created usin File I/O.


Expected updates:
* Enhanced data structures
* Integration with a database
* Implementation of advanced features like task prioritization and task dependencies.
* Integration with a calendar system for task scheduling and reminders.
* Continuous improvement of the user interface and user experience based on user feedback.


## Today I learned
Today I learned about a Text-based User Interface called [ratatui](https://ratatui.rs/index.html) (I know). I even wrote a hello world program on it, as is tradition. I will be learning about ratatui and TUI and see which one is better to implement into this code. Hopefully, our program gets completed before the Greek letters run out.
