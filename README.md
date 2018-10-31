# Urgent TODOs

* Add whether the task needs outside travel.
* Add priorities and weighting.
* Add internet/no internet boolean.

## Maybe TODOs

* Add tracking of last use of review mode. Prevent use in review mode
  except once per day.
* Install for root.

## Description

**`models`**

* `Task`, `TaskEffort`, and `TaskStatus` are pretty self-explanatory.
* Diesel's DSL is pretty nice.

**`Reviewer`**

* The `Reviewer` class is sort of the top level object.
  * It basically holds the other objects.
  * It runs the run loop.
  * It uses `TaskResultsWindow` to redraw the interface.
  * People press a key, and this is dispatched to a
    `Commands#handle_key`.

**`Commands`**

* Commands has an enum of commands the user can press keys for.
* A big switch determines what action to perform for each command.
* Many commands will CRUD actions. The `Task` API is used directly.
* The `Reviewer`'s `PgConnection` object is used by the commands.
* CRUD actions typically need to know the currently selected `Task` (to
  update or destroy). Thus the `Scroller` is also used by the commands.
* Basically: the commands need everything the reviewer has. Each command
  may use different parts of the entire reviewer's components.
  * It is as if commands are methods of the `Reviewer`, in a sense.
  * But cleaner.
* After execution, a command returns a `CommandResult`. These tell the
  `Reviewer` what kind of command was performed.
  * E.g., there is a `CommandResult` that tells the `Reviewer` to exit
    the run loop.
  * There is another which says: "I updated a record, please pull down
    all data gain."
  * The `CommandResult` is handled in the `Reviewer#run` method

**`UI::Window`**

* This is a wrapper around ncurses.
* It does UI like read a character or a line.

**`TaskResultsWindow`**

* Sadly, this does a lot of ncurses stuff, and doesn't use `UI::Window`
  much.
* It holds a reference to the `UI::Window`. More important, it holds a
  reference to the `Scroller`, which actually holds the data.
* This basically just iterates the records and displays them.

**`DataSource`**

* This is upstream of everything (lol, except the `Reviewer`'s
  `PgConnection`).
* The `Reviewer` calls the `#refresh` method to fetch more data. Refresh
  does an N+1 query to fetch both `Task`s and their "age" since last
  worked.
* It pairs this info up as a `TaskRecord` object.
* The `DataSource` "pumps" data down because you give it a callback.
* My callback captures the `Scroller` (via an `Rc`).

**`Scroller`**

* The `DataSource` callback clones the records. A little wasteful of
  course.
* The `Scroller` is the only "stateful" object.
* However, because the `Scroller` is captured by `Rc`, you can't have
  mutable references to the `Scroller`.
* Thus, I use `Cell` and `RefCell`.
  * `Cell` is good for the `current_result_idx`, since I never need a
    reference to this.
  * Thus scrolling forward/backward can be done even with a non-mutable
    reference.
* I need `RefCell` to hold the vector of `TaskResult`s.
  * Only time this is `borrow_mut` is when the `DataSource` callback is
    invoked.
  * This is safe because it holds the mutable reference on the vector
    for only the short method to update.
* The compiler cannot reason about lifetimes of the `TaskResult` records
  in the vector. Thus methods like `current_task` must clone.
  * This means that there is no way for the user to mutate individual
    tasks.
  * This is probably a good thing. Any changes will tell the
    `DataSource` to refresh, pull afresh, and pump records down.
  * Will make even more sense when we have a `Filter` component. How
    will the filterer know when you have mutated a `Task` such that it
    no longer meets the filter criteria?
* I am unhappy with the `#results` method. This passes a `Ref` to the
  vector of task results out to the caller.
  * It's used by `TaskResultsWindow` to iterate and display
    `TaskResult`s, which is innocent.
  * The problem is that there is a borrow on the vector the entire time
    anyone is using it.
  * The fear is that someone outside could trigger a
    `DataSource#refresh`, which would want a mutable borrow and our
    program would die!
  * It only feels okay to use `RefCell` if the `Ref` is *constrained
    within the single class*.
  * Else we might as well just not use Rust, right?

## TODO

* Describe `Rc` situation.
* What is the true point of enforcing just one mutable reference?
  * I guess it's allocation stuff like vectors?
  * Is the idea reentrancy? Many scenarios don't have a problem where
    moves are going to happen...
* Cargo fmt and lint.
* Talk about interior mutability from scroller.
