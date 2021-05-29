---
name: 'Bug: Console::new() fails with error code 2. (File not found.)'
about: If Console::new() failes with a file not found error please use this and not
  the basic "Bug report" issue.
title: Console::new() fails with ERR_FILE_NOT_FOUND
labels: bug, console_worker.exe file not found
assignees: Foxcirc

---

Usually this is caused by a unexpected build environment.
This is because under the hood, the "build.rs" file actually does a lot of stuff
It builds the "console_worker" and attempts to copy it to the default output directory.

** Did you build your project using "rustc" instead of cargo? **

** Please paste the command you used to build here. (E.g. "cargo run --bin myproject") **

-------------------------------------------------------------------------------------------------
** Pleae paste you "Cargo.toml" file contents here. **
(Please leave the "------" lines there.)
------------------------------------------------------------------------------------------------

** Do the paths to your project contain any special characters **

** Is there a console-worker executable somewhere? **
Please perform a search for the file name "console_worker" AND the file name "console-worker" on your whole disk. If there is a EXECUTABLE (.exe) file wich has that string in its name please answer with "Yes: " and then paste the file name and path here (You can obfuscate your name etc. no problem there).

** If there is no such file, is there a folder named "console-worker" or "console_worker"? **
Yes / No + Folder path

** Please generate an ascii-view of you folder structure using the "tree" program and paste it in here **
A tree of your project folder is enough. Executing the program directly on your main drive is likely to big for this tiny issue form and not needed.

** Any additional system information **
Is something about your system installation special. Did you do any weird things with e.g. environment variables.
Also anything else that you think could be useful goes in here.

Thank you for doing all this :).
