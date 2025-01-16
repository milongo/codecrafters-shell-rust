---
title: "Introduction to Shell"
date: 2025-01-14
---

# What is even a shell?

From [Wikipedia](https://en.wikipedia.org/wiki/Shell_(computing)):

`"A shell is a computer program that exposes an operating system's services to a human user or other program..."`

From the [Open Group Base specification](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html) (linked as well in the challenge description as a source on POSIX compliancy):

`"The shell is a command language interpreter."`.

It's not entirely clear to me what this means! I've been programming for about 8 years, interating with terminals in Unix-like systems, using commands like `cd`, `pwd`, `ls` and more on a daily basis, but I rarely questioned how this worked internally: I just took it for granted that I could accomplish certain things (find specific files in filesystems, SSH into remote servers, manage work and personal repositories using `git`, etc.) required by my job by typing commands into a terminal, googling, using Stack Overflow, and more recently using LLMs like ChatGPT. 

Here's one more attempt at understanding the shell, from ChatGPT (gpt4o):

`"A shell is a command-line interface (CLI) that allows users to interact with an operating system by entering textual commands. It's essentially a layer between the user and the operating system's kernel, enabling the execution of commands and programs."`

So now we have 3 different (?) definitions of a shell. Are they consistent with each other? We also have mentions of an operating system and a kernel. We are going to have to go into a bit of rabbit-hole to understand these things better.

# The rabbit hole

## The operating system

Surprisingly, I will use [Wikipedia](https://en.wikipedia.org/wiki/Operating_system) again:

`An operating system is difficult to define,[6] but has been called "the layer of software that manages a computer's resources for its users and their applications".[7] Operating systems include the software that is always running, called a kernel—but can include other software as well.[6][8]`.

Computer programs need access to physical resources: memory, CPU, storage, network interfaces, etc. Most of the time, there are multiple programs running at the same time in a computer. One of the OS's responsibilities is to coordinate the different programs and ensure none of them monopolizes the hardware resources available. 

## The kernel

Even more surprisingly, I will look at [Wikipedia](https://en.wikipedia.org/wiki/Kernel_(operating_system)) again:

`"A kernel is a computer program at the core of a computer's operating system that always has complete control over everything in the system."`.

When you type a command in a CLI, here's how the kernel gets involved:

1. The CLI sends a system call to the kernel to execute a program.
2. The kernel allocates resources and starts the program as a process.
3. The program interacts with the kernel to access hardware or perform I/O tasks.
4. The kernel sends output back to the CLI.

So the kernel is responsible for the OS's ability to coordinate resources, which is what I wrote about the OS in the section above: we could think the kernel and the OS are the same thing, but both the OS and the kernel have more to them than what I just said. I encourage the reader to explore by (surprise!) reading the Wikipedia articles I linked above.

## How it all fits together

Step 1: Opening the Terminal:
The terminal is a graphical interface for interacting with a shell program.
When you open the terminal, it spawns a child process to start your default shell (e.g., `zsh`).

Step 2: Typing a Command:
You type a command, and the terminal sends it as input to the shell via standard input (stdin).

Step 3: Shell Parses the Command:
The shell breaks the input into:
- Command: The program to run (e.g., `ls`).
- Arguments: Additional data for the command (e.g., `-la`).

The shell checks:
- If it’s a built-in command (e.g., `cd`).
- Or if it’s an executable file in the directories listed in `$PATH`.

Step 4: Shell Requests the OS to Execute:
For external commands, the shell makes a system call (e.g., `execve`) to the kernel.

Step 5: Kernel Executes the Command.
The kernel:
- Locates the program file.
- Allocates resources (memory, CPU).
- Creates a new process to run the program.

Step 6: Program Produces Output:
The program sends its output to standard output (stdout), managed by the kernel.
The shell receives this output and passes it to the terminal to display.

Step 7: Shell Waits for Input:
After the program finishes, the kernel signals the shell that it’s ready to accept new commands.

```scss
Terminal (Graphical Interface)
    ↓
Shell (Command Interpreter)
    ↓
System Call (Request to Kernel)
    ↓
Kernel (Core of OS)
    ↙       ↘
Process   Hardware
```


# Back to the shell

Now, there are many shells! Here's a non-comprehensive list:
- `sh`
- `bash`
- `zsh`
- `fish`
- `PowerShell` (yuck!)
- ...

I personally use `zsh`. It's pretty.

What is the difference between all these shells? Why should one pick one over another? What makes a shell useful?
This is where I think POSIX compliancy comes into play. The Portable Operating System Interface (POSIX) is a series of standards for maintaining compatibility between operating systems. And they have a standard for shells! If one implements a POSIX compliant shell, it's fairly likely to wolk well across different operating systems (mac OS, Linux, etc.) Interestingly, I've always preferred `zsh` for my personal/work computers, and `bash` tends to be the default shell installed in most Linux distributions. 
