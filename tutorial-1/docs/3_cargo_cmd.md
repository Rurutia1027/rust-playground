# Cargo Commands 

* generate package `tutorial-1`'s lib crate structure 
```
$cargo modules structure --lib --package tutorial-1

crate tutorial_1
├── mod customer: pub(crate)
│   └── struct Customer: pub
├── mod order: pub(crate)
│   └── struct Order: pub(self)
│       ├── fn calculate_discount: pub(self)
│       └── fn total_bill: pub(self)
└── mod product: pub(crate)
    ├── struct Product: pub
    │   ├── fn calculate_tax: pub(self)
    │   └── fn product_price: pub
    └── mod category: pub(self)
        └── enum Category: pub$
```