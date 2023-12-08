# Project Epsilon

## What's new:

1. Readibility: The program was one big main.rs file before. Now it has been divided into multiple modules so that it can be more readible and accessible. The project has 3 files now: [task.rs](..\src\task.rs), [task_manager.rs](..\src\task_manager.rs), and [main.rs](..\src\main.rs). The task mod contains the data structures used by the application, the task_manager mod contains all the methods used to interact with the data. This way the user and/or developer will be able to read the code to understand what's going on behind the scenes.

2. Better interface: Using the colored crate, I added some colors to the output of the terminal so that it looks attractive.

3. Removing description: The task should be summed up in the title.

4. Cleaner code: I am reading clean code by Robert C. Martin, and trying to implement some of the practices and teachings from it.

This Command Line based application is complete. Now I will take some time to study the [Ratatui](https://www.ratatui.rs/) crate and learn how to implement it in this project.

Thank you!
<br><br><br>

<a href="https://www.linkedin.com/in/abdulshaikh55"><img src ="./images/linkedin.png" alt = "LinkedIn" height="30px" width="30px"></a>