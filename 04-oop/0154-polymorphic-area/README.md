# 0154 — Polymorphic Area

Sum the areas of a Rectangle(2,3)=6 and a Triangle(base=4,height=4)=8 through a common Shape interface, printing `total area: 14`. A `Vec<Box<dyn Shape>>` enables dynamic dispatch over a shared trait.

## Run

    cargo run
