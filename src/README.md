# Architecture

Main calls clap.rs 
clap.rs gives command

`gl add .` or `gl add /path/path/` calls :

1. walker.rs in given dir ("." == current dir)
2. walker output is then passed to par_indexer() 
3. par_indexer() then parses Vec<PathBuf> into Vec<Metadata> with the help of metadata_to_struct() & write_index() 
4. Vec<FileMeta> is then passed to index_loader() which then refines it for other works like `gl status` `gl diff` etc.

# What to do now ?


1. Load old index from index.dat 
2. Read new index into memory 
3. Preform comparison operation in memory 
4. Be flexible for `gl diff/status` & `gl add path`

# Dealing with Index.dat 

for a operation like gl add path/ path2/ , we dont need a full repo scan so we will do :

path validation including the path is of which type?,

path exists in index? if yes then compare FileMeta else append in Index.dat with file meta.

while for a gl diff or gl status you are doing a full repo scan hence there you do some additional operations like delete.

