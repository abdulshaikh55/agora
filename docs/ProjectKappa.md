# Completed
1. Replaced the data strutures to be used.
2. Remove introduction [ this is an open source project ]
3. Design the edit ui section
4. Change the input so that you can you can accomodate the other aspects of input
5. In ui.rs edit CurrentScreen::Task to show all the different aspects of the tasks.
6. In CurrentScreen::New, with the help of crossterm, take chars as input


# Ongoing
1. Add a CurrentScreen::New, where you add a new task into the list


# NotStarted
1. for task input, figure out how to accept the task string
2. input the task string into the vector
3. Implement the database
4.  Do something about the panics. The app should exit the alternate screen and then show all the errors.
5. Set up a different section/ module/ file for file input and output
6. implement task::Task {task, duedate...} into the project. i.e. instead of a Vec<string> of tasks, display a Vec<Task>.
7. render all the information about selected tasks on the Task tab.
8. Make the documents more readable.
9. Fix the image and video not rendering on markdowns.

> # Thoughts
> There is a problem with app.rs. It seems that I will have to create two instances of two different modules that control the screen enums and task_management enums.  
> How do I solve this  
> -[ ] Do I merge the task_management with app? 
> Not it better how it is now. Having two different modules for components that control entirely different aspects of the projects is goood.
>
> How do you intend to add a new task?
> How do you intend to delete an existing task?
> You have only been working under the assumption that the task already exists.