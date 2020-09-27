# reorger

A small collection of tools for manipulating large collections of files. Most of these were written to help
deal with image data sets for Machine Learning.

* *split* - Takes a glob pattern in the current directory and creates batches of 1000 files that match the glob pattern
  into subdirectories.
* *unsplit* - Reverts the *split* command by looking into all directories (with no further depth traversal) and moves them
  into the current directory. If a directory that is traversed ends up being empty, it will be removed.
* *sample* - Copies every n^th file that matches a glob pattern from the current directory into a new directory.