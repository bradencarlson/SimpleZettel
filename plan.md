# Zk plan

## Program flow

- [x] Process all command line args and subcommands (see Subcommands and Args
  below)
- [x] Create file (or find file if it exists)

(user edits file, no need to handle anything here)

- [x] Depending on the exit code (this might have to be based on hashes instead),
  print out what has happened:
    - new file saved
    - file updated
    - error of some kind?
- [x] Should store a hash of the note to edit (when this command is passed),
  then computes a new hash for the file after the use has finished editing. This
  way, the git prompt is shown only when the user actually changes the file in
  some way.
- [ ] handle any backlinks?
- [x] Prompt user if they would like to add a commit message
- [x] Commit changes to box `git` tracking.

## Subcommands and Args

### High level commands

An 'x' here in the second box simply means that the command/option has been built into `clap`, an
'x' in the first box indicates that the feature has been implemented.
If there is only a single box, then an 'x' indicates that this has been
implemented.
List of subcommands desired:
- [x], [x] add: for adding a new note
- [x], [x] edit: for editing an existing note
- [x], [x] rm: for removing an existing note
- [x], [x] show: for showing an existing note
    - user should be able to specify a note in a different box for all of the
      above commands
- [x], [x] ls: for listing all files in a box
    - this could have subcommands just like real `ls`
        - [x], [x] patterns can be passed for matching filenames
        - [ ], [ ] `-t` for listing tags
        - [ ], [ ] `-d <num>` for specifying a maximum depth to stop at when
          listing numbered files.
        - [x] files which are numbered (1.1.1.md) are listed in their numerical
          order.
        - [x], [x] the user can specify the list a different box (via a flag or some
          other notation)
        - [x], [x] -1 alias for passing the pattern ^1 to ls. like this: `zk -1`? 
          this would probably be easier: `zk -n 2`
- [ ], [x] search: searches for a string among files in box
- [x], [x] box: for managing boxes
    - [x], [x] ls: for listing boxes (maybe this should be the default behavior if no
      subcommand is found
        - [x], [x] add subcommand `-a` for listing all boxes, not just tracked ones
    - [x], [x] add: for adding a box
    - [x], [x] remove: for removing a box (only removes tracking, not files)
    - [x], [x] track: for tracking a box previously removed
    - [x], [x] use: select a box to use
- [ ], [ ] follow: for traversing the files. This should require a name which is
  the starting point, then shows that note, and provides some sort of menu
  (which is a list of all notes that the current note references) for the user
  to select. The menu should probably include the notes children  
  so the user can go down a sequence like 1.1 -> 1.1.1 -> 1.1.5 -> 1.1.5.2
  Perhaps this should drop the user into some sort of 'shell', where they can
  specify a new note name to go to, or list notes. Yeah, this is a big item.
- [x], [x] import: copy a file into the box directory.
- [x], [x] config: check config syntax, or print out that no config exists. 
    - [ ], [x] show: show table for defining which commands to use when calling
      the `show` command.
        - [x], [x] md: command for markdown files
        - [x], [x] pdf: command for pdf files
    - [ ], [ ] general: table for general settings.
        - [ ], [ ] highlight: color used for arrow in boxes and filenames
    - [ ], [ ] filetype-prefix:
        - [ ], [ ] md: prefix to use for markdown files
        - [ ], [ ] pdf: prefix to use for pdf files
- [ ], [ ] mv: rename a note

### Low level commands

- [ ], [ ] log: view git log for box
    - could optionally be able to pass `git log` options to this command

## Implementation

### File Structure

This is what the file structure of the `.zk` directory will look like.
`zk` should be able to run without any of the hidden folders or track files or
cache directories. Meaning, if I were to drop a bunch of markdown files into a
directory, it could detect it and add any needed information for faster
processing in the future.

The `.current` file simple denotes which box I am currently looking at. If this
is absent, `zk` should report that it is in a "headless" state.

The `.track` file in a box directory simply denotes that that box is tracked by
`zk`. Removing a box simply means deleting this file.

You will see that each box has a `.git` directory, the command `zk box add ...`
should not only create the box, but initialize git tracking there as well.

~/.zk
|-- .current
|-- box1
|-- box2
|-- ...
|-- boxn
|   |-- .git
|   |-- .track
|   |-- file1
|   |-- file2
|   |-- ...
|   |-- filen

### Numbering of files

This is left up to the user. Currently, valid 'numbers' for filenames are
filenames matching
```
^(\d\.)*\d$
```
For example '1', '1.2.12.3', or '10.9', or any valid sequence of characters
which are able to be read into a String in rust (namely, UTF-8 characters). When
listing files in a box, `zk` will treat these differently. Since `zk` was
designed to manage a Zettelkasten, filenames matching the regular expression
above are assumed to come before any alphanumeric filenames. See the next
section for an example.

### Listing of files

This is done by filename. Specifically, each file name (minus the extension) is
assumed to be of type ZkNumber, which is either a number (i.e. 1.12.3.2), a
string (i.e. note-one, or any other valid sequence of UTF-8 characters), 
or is invalid (string cannot be parsed). Notes are then
listed acording to the ZkNumber ordering defined in the program (numbers first,
in order, then strings, in alphabetical order, then any invalids). For example,
if the box directory contains the following list of files (notice that these are
in the order that `ls | sort` puts them in):
```
1.1.md
1.2.md
1.3.1.md
1.3.2.md
1.3.md
2.1.1.md
2.1.md
example.md
file-one.md
```
Then `zk` will list these in the following order:
```
1.1.md
1.2.md
1.3.md
1.3.1.md
1.3.2.md
2.1.md
2.1.1.md
example.md
file-one.md
```
So all files that are named as a 'number' come first, in the correct order, with
other files listed alphabetically below.

## Things that are *not* implemented

- todos: I'm just going to focus on the zettelkasten, not the todo lists. I like
  `nb` for this anyway.
