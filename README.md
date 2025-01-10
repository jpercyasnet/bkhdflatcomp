# bkhdflatcomp
Rust program: Read the md5sum of a file in csv of HD database and see if it is in csv of Backup Database. 

example:

bkhdflatcomp01 bkfilesorted.csv hdinitsorted.csv exclude.excfile nnnn

where nnnn is an optional input to read the hdinit.csv starting at nnnn row

bkfilesorted.csv is a sorted dump of the backup database

hdinit.csv is a sorted dump of the hd database or sorted output from hdbkmd5sum

exclude.excfile is a text file which excludes files and directories.

see documentation repository for additional information
