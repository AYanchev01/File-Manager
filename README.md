# changer

changer is a terminal-based file manager inspired by the [ranger](https://ranger.github.io/) project. Crafted for users who love working within the terminal, it offers intuitive VIM-inspired navigation, versatile file operations, and a minimalist interface that leverages familiar keybindings. 

![demo](https://github.com/AYanchev01/changer/assets/72613335/3a3d2510-f28e-4e60-960f-98ac1cee882d)

## Features
* Cross-platform support (Linux/Windows) 
* UTF-8 Support
* Multi-column display
* VIM-inspired navigation in the file system
* Search Capability
* Directory & text file previews
* Common file operations (create/copy/cut/delete/rename/cut/chmod/...)

## Getting Started
Simply launch changer in your terminal, and you'll be presented with the files and directories of your current location. Use the following keybindings to navigate and make changes in your file system: 

    File navigation
        j: Move down
        k: Move up
        l: Move into a directory or file
        h: Move out of a directory
        Ctrl+u: Move up half a page
        Ctrl+d: Move down half a page
        gg: Go to the top
        G: Go to the bottom
        /: Initiate search
        n: Jump to next match
        N: Jump to previous match

    Text file preview navigation:
        Alt+j: Scroll down
        Alt+k: Scroll up
        Alt+d: Scroll down half a page
        Alt+u: Scroll up half a page
        Alt+gg: Go to the top
        Alt+G: Go to the bottom

    File Operations
        y: Copy
        p: Paste
        D: Delete
        x: Cut
        r: Rename
        a: Create a new file
        A: Create a new directory
        c: Change file or directory permissions

## Dependencies
* For opening text files with changer, make sure that either your VISUAL or EDITOR environment variables are set. Default editors will be vim for Unix-based operating systems and notepad for Windows.
