## Async and Await

Many operations we ask the computer to do can take a while to finish. e.g., creating and exporting
videos which could take anywhere from minutes to hours. Or downloading a video could take awhile
too.

The video export will use as much CPU and GPU power as it can. If you only had one CPU core, and
your operating system never paused that export until it completed, you couldn't do anything else on
your computer while it was running. That would be a pretty fucking frustrating. Instead, the OS can
(and does) interrupt the export often to let you work on other thing while exporting.

The file download is different. It does not take up very much CPU time. Instead, the CPU needs to
wait on data to arrive from the network. While you can start reading the data once some of it is
present, it might take a while for the rest to show up. Even once the data is all present, a video
can be quite large, so it might take some time to load it all. Maybe it only takes a second or two,
but that's a long time for modern processors, which can do billions of operations every second. It
would be nice to be able to put the CPU to use for other things while waiting for the network call
to finish. So again, the OS invisibly interrupts the program so other things can happen while the
network operation is still ongoing.
> The video export is the kind of operation often described as "CPU-bound" or "compute-bound". It's
> limited by the speed of the computer's ability to process data within the CPU or GPU, and how much
> of that speed it can use. The video download is the kind of operation often described as
> "IO-bound," since it's limited by the speed of the computer’s input and output. It can only go as
> fast as the data can be sent across the network.

In the above examples, the OS's invisible interrupts provide a form of concurrency, which happens at
the level of a whole program: the operating system interrupts one program to let other programs do
work. Since we understand our programs better than the operating system does, we can spot more
opportunities for concurrency the OS can't see.

e.g. building a tool to manage file downloads. It should be written in a way that starting one
download doesn't lock up the UI, and users should be able to start multiple downloads at the same
time. Many OS APIs for interacting with the network are *blocking*; these APIs block the program's
progress until the data that they are processing is completely ready.
> we normally reserve the term "blocking" for function calls that interact with files, the network,
> or other computer resources, b/c those are places where an individual program would benefit from
> the operation being "non-blocking".

We could avoid blocking our main thread by spawning a dedicated thread to download each file, but we would eventually find that the overhead of those threads would be a problem. It'd also be nicer if the call wasn't blocking in the first place. Lastly, it'd be better if we could write in the same direct style we use in blocking code.
- e.g like:
    ```rust
    let data = fetch_data_from(url).await;
    println!("{data}");
    ```
    This is exactly what Rust's **async** abstraction gives us.
    
### Parallelism and Concurrency

***Concurrency*** is like an individual working on several different tasks before any of them is complete. Maybe you have two different projects on your computer, and when you get bored or stuck on one, you switch to the other. You are just one person, so you can't make progress on both tasks tasks at the exact same time, but you can multi-task (progress on multiple tasks by switching between them).
<img src="https://rust-book.cs.brown.edu/img/trpl17-01.svg" width=800 height=200>

***Parallelism*** is like agreeing to split up a group of tasks between people on a team, and having each person take one task and work on it alone. Each person on the team makes progress at the exact same time.
<img src="https://rust-book.cs.brown.edu/img/trpl17-02.svg" width=800 height=400>

If the way things happen is *serial*, that means things happenining one after the other (like a series).

With both of these situations, you might have to coordinate between different tasks. Maybe you thought the task that one person was working on was totally independent from everyone else’s work, but it actually needs something finished by another person on the team. Some of the work could be done in parallel, but some of it was actually serial: it could only happen in a series, one thing after the other. Likewise, you might realize that one of your own tasks depends on another of your tasks. Now your concurrent work has also become serial.