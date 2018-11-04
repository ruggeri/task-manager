# TODOs

**Yes**
* Record completion as a task event.
* Fix `Scroller` to limit results again.
* Have recurring tasks.
* Have ability to view tasks completed today.
* Try to build in some "reflection" aspect.

**Maybe**
* Deal with duplication of `TaskUpdateAction`.
* Add whether the task needs outside travel.

**No?**
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

## Further thoughts about interacting components

I have tried to factor my code into many interacting components.
However, there isn't a clear hierarchy of components.

If A were to contain B and C, but D needs to interact with both B and C,
then A needs to present the union of B and C's interfaces. It quickly
gets to the point where we just have a god object.

I wanted to give everyone references to everyone else, and have everyone
live in a `Reviewer` class which was kind of like the global state.

However, I had a problem where no one understood the lifetimes of each
other. I figured that if A contained B and C, then B and C could have
references to each other, because neither would live past A.

But if I do that, then it means that A is not moveable. So either: A
must be boxed, or B and C must be boxed. The easiest solution would be
to box A. But I also had problems that I can't reference even partially
initialized values of A: I can't feed &A.B when making C. That feels
like the compiler just being grumpy...

No matter what I tried, I could get this to work. I think what I am
trying to do is simply not supported. You could say that it is truly not
safe for A.C to have a reference to A.B, because it is not for sure what
the *destruction order* will be. Imagine that the destructor of A.C uses
the A.B reference. Then this is unsafe if A.B will be destroyed first.
So maybe it makes sense Rust doesn't want me to do this.

## My solution: `Rc` everywhere

I used `Rc` liberally to solve my problems. I felt a little dirty doing
this. I worried about reference cycles. Now my top-level object didn't
have quite the exclusive ownership it used to.

I could have dealt with this by using weak references. But that would be
a PITA because I'd have to constantly deal with `Option`s when
dereferencing.

Another problem was that `Rc` makes it hard to get mutable references to
things. Back when I was using normal references, I could get mutable
references when I needed them. But even if you own an `Rc`, that doesn't
mean *other* `Rc`s exist pointing to the same data.

This forced me to use `Cell` and `RefCell`, which I didn't really want
to do. On the other hand, it was good to make my interfaces have fewer
methods that require mutability.

## For next time

Maybe I have way too many references stored in my components: maybe I
should just pass the `Reviewer` in to every method. I wanted to avoid
this, by saving the "connections" between components in reference
fields. But maybe that is too much trouble; perhaps the entire context
should be constantly "pumped down."

That would ease ownership and mutability complaints.

However, it also feels pretty unnatural...

## StackOverflow Advice

This directly addresses my concerns:

    https://stackoverflow.com/questions/28113504/structure-containing-fields-that-know-each-other

They suggest the idea of `Cell<Option<&'a T>`, which I also considered.
It feels horribly hacky though.

They also show a way to get things to work through some unsafe tricks.
They rely on `Box` never moving the underlying data, just like I wanted
to rely on. I commented about the destruction order problem.

They recommend not taking this approach where there is unclear
ownership. Or accepting life with `Rc`. And they mention the mutability
concerns. So pretty much everything I thought about was valid.

That makes me feel a bit better :-)

## TODO

* What is the true point of enforcing just one mutable reference?
  * I guess it's allocation stuff like vectors?
  * Is the idea reentrancy? Many scenarios don't have a problem where
    moves are going to happen...
