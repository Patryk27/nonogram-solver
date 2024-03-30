# nonogram-solver

Solves monochromatic [nonograms](https://en.wikipedia.org/wiki/Nonogram),
powered Rust and Z3, pretty rudimentary.

## Usage

(requires rustup and z3)

``` 
$ cargo run -- \
     --cols '1|1|2|4|7|9|2,8|1,8|8|1,9|2,7|3,4|6,4|8,5|1,11|1,7|8|1,4,8|6,8|4,7|2,4|1,4|5|1,4|1,5|7|5|3|1|1' \
     --rows '8,7,5,7|5,4,3,3|3,3,2,3|4,3,2,2|3,3,2,2|3,4,2,2|4,5,2|3,5,1|4,3,2|3,4,2|4,4,2|3,6,2|3,2,3,1|4,3,4,2|3,2,3,2|6,5|4,5|3,3|3,3|1,1'

# https://en.wikipedia.org/wiki/Nonogram#/media/File:Nonogram_wiki.svg
```

```
******** ******* ***** *******
  *****   ****    ***    ***  
   ***     ***    **     ***  
   ****     ***   **     **   
    ***     ***  **      **   
    ***     **** **     **    
    ****     *****      **    
     ***     *****      *     
     ****     ***      **     
      ***     ****     **     
      ****    ****    **      
       ***   ******   **      
       ***   ** ***   *       
       **** *** **** **       
        *** **   *** **       
        ******   *****        
         ****    *****        
         ***      ***         
         ***      ***         
          *        *
```

## Implementation

Nonogram can be represented as a set of regular expressions - e.g. column with
numbers `1` and `8` is:

```
^[ ]*\*{1}[ ]+\*{8}[ ]*$
```

... or, with a couple of comments:

```
^ [ ]* \*{1} [ ]+ \*{8} [ ]* $
  1--- 2---- 3--- 4---- 5--- 

1 - any number of leading spaces
2 - exactly one asterisk
3 - at least one separating space
4 - exactly eight asterisks
5 - any number of trailing spaces
```

Following on this, nonogram-solver builds a regular expression for each column
and row, then sends those constraints to Z3 and parses its response back.

## License

Copyright (c) 2024, Patryk Wychowaniec, <pwychowaniec@pm.me>

Licensed under the MIT license.
