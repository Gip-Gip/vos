# VOS, The Vintage OS for Vintage Computers

I have always wanted to make a platform which is semi-universal across old hardware,
so that computers like old TI-99s and Commadore 64s aren't left in the past without
any modern support. I aim to provide a very lean unix-like experience that allows
for cross-platform application distribution and a basic universal interface so that
users and programmers can have a modern and well supported experience on forgotten
and abandoned hardware.

## Standards

### VASM & VIST

To enable cross-platform compatability all applications are translated from VASM,
vintage assembly, to VIST, vintage instructions. They are meant to be primarily 8-bit
centric with most operations being two-byte pairs, though addresses can be 24-bit
as well
