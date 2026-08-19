# Zk plan

## Program flow

- [ ] Process all command line args and subcommands (see Subcommands and Args
  below)
- [ ] Create file (or find file if it exists)
    - [ ] Note that this will require some sort of data structure to keep track 
    of all the files already made.
        - list of hashes? Could store these after every operation in a dotfile.
          This way, even if it is deleted, it is not hard to recreate.

(user edits file, no need to handle anything here)

- [ ] Depending on the exit code (this might have to be based on hashes instead), 
  print out what has happened: 
    - new file saved
    - file updated
    - error of some kind?
- [ ] handle any backlinks?
- [ ] Prompt user if they would like to add a commit message
- [ ] Commit changes to box `git` tracking.

## Subcommands and Args

### High level commands

List of subcommands desired:
- add: for adding a new note
- edit: for editing an existing note
- tags: list all tags found in notes
- rm: for removing an existing note
- show: for showing an existing note
- ls: for listing all files in a box
    - this could have subcommands just like real `ls`
- search: searches for a string amoung files in box
- box: for managing boxes
    - add: for adding a box
    - remove: for removing a box (only removes tracking, not files)
    - track: for tracking a box previously removed

### Low level commands

- log: view git log for box
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

## Things that are *not* implemented

- todos: I'm just going to focus on the zettelkasten, not the todo lists. I like
  `nb` for this anyway.
