# Zk plan

## Program flow

- [x] Process all command line args and subcommands (see Subcommands and Args
  below)
- [x] Create file (or find file if it exists)

(user edits file, no need to handle anything here)

- [ ] Depending on the exit code (this might have to be based on hashes instead),
  print out what has happened:
    - new file saved
    - file updated
    - error of some kind?
- [ ] handle any backlinks?
- [ ] convert any 'numbered' links to 'named' links (for compatibility with
  other systems)?
- [ ] Prompt user if they would like to add a commit message
- [ ] Commit changes to box `git` tracking.

## Subcommands and Args

### High level commands

An 'x' here in the second box simply means that the command/option has been built into `clap`, an
'x' in the first box indicates that the feature has been implemented.
List of subcommands desired:
- [x], [x] add: for adding a new note
- [ ], [x] edit: for editing an existing note
- [x], [x] rm: for removing an existing note
- [x], [x] show: for showing an existing note
    - this currently uses just a print statement
- [ ], [x] ls: for listing all files in a box
    - this could have subcommands just like real `ls`
        - [ ], [ ] `-t` for listing tags
    - subcommand for showing based on numbering in front matter.
- search: searches for a string among files in box
- [x], [x] box: for managing boxes
    - [x], [x] ls: for listing boxes (maybe this should be the default behavior if no
      subcommand is found
    - [x], [x] add: for adding a box
    - [x], [x] remove: for removing a box (only removes tracking, not files)
    - [x], [x] track: for tracking a box previously removed
    - [x], [x] use: select a box to use

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

The `cache` directory in each box contains information that could speed up
processing in the future. This could be information such as which files have
which number, which files link to others, etc.

The `.index` file is simply a copy of what `nb` does, keeps track of the files
created in order, for numbering them.

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
|   |-- .index
|   |-- files
|   |   |-- file1
|   |   |-- file2
|   |   |-- ...
|   |   |-- filen
|   |-- cache
|   |   |-- info

### Numbering of files

Files could simply have a `number` field in their front matter, but that won't
do if I would like to add other file types.

I could also keep track of numbering in a dotfile in the box directory as well,
this is what `nb` does (a huge inspiration and a great project!)

### Listing of files

This could be done based to front matter, as explained in the previous section.

I could also simply use whatever argument the user passes as a regex to select
which files to show.  This makes it the user's job to use some numbering system
as the names of the notes.  For example, if they run `zk add 1.1.2.1`, they
could later run something like `zk ls 1.1.*` and that note would show up.

I think I like the idea of using front matter, since I believe that I would want
to use this for links and references anyway, but I worry about the time it
might take to process all the files in a box.

## Things that are *not* implemented

- todos: I'm just going to focus on the zettelkasten, not the todo lists. I like
  `nb` for this anyway.
