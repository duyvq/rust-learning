This is sumary of solution for 4 excercise of class 2 - VBI- Rust Workshop
(I also commented in main.rs file)

1) Excercise 1:
- Solution 1: add "&mut" to front of second argument 20 (line 9)
    
- Solution 2: remove "reference" of argument output (line 14) and remove "dereference" of output inside function (line 16, 19 & 22)


2) Excercise 2:
- Solution 1: 
  - Step 1, argument "p" is changed to borrow type (line 46)
  - Step 2, borrow primes instead of taking ownership (line 37)
  - Step 3, dereference "i" (line 49)

- Revise logic:
  - Step 1, add counter (line 47)
  - Step 2, don't return immediately after if but increase counter to check iterately over all the element of vector to ensure number isn't divisible by other number than itself (line 51)
  - Step 3, if counter equals length of vector, num is prime number. Return true (line 56)
  - Step 4, line 60 should be return false


3) Excercise 3:
- Solution 1: reborrow "v", from "v" to "&mut *v" (line 74)
- Solution 2: add into_iter() to "v" (line74)


4) Exercise 4:
- Solution: 
  - Step 1, change a to mutable (line 92)
  - Step 2, mutable reference a instead of taking ownership (line 96 & 102)

- Revise logic:
  - Instead of checking i >=5, check if c equals sum of a's element to quit the loop (line 98)
