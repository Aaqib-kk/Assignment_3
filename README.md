# Assignment_3
## Muhammad Aqib Khan
Roll number IOT 047671 and timings 1:15 to 5:15

## Fn
The Fn traits are provided by the standard library. All closures implement at least one 
of the traits: Fn, FnMut or FnOnce. 

## FnOnce
FnOnce takes ownership and move into the closure, once indicates that clouser can take ownership only once
All closures implement FnOnce because they can all be called at least once.

## FnMut
Closures that do not move the captured variables also implement FnMut, 

Fn and FnMut are subtraits of FnOnce, any instances of Fn or FnOnce can be used where a FnOnce is expected
