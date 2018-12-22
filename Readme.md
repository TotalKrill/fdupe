# Ripdupe

A small software tool that can compare one set of file, to find duplicates in another set. It has
some differences to similar tools due to the fact it can limit the traversal depth in folders, thus
allow to only change or include files that not in subfolders.

Uses file hashing code from [dupe-krill](https://github.com/kornelski/dupe-krill)

## Warning

Currently, if the originals folder and the check folder is the same, every file will be marked as
duplicate. DONT DO THIS!

I take no responsibility of files lost, since this program is specifically created to delete files.

