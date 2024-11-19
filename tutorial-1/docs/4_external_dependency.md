# External Dependency

In Rust, an external dependency referes to a libray, crate, or package that your project relies on but 
is not part of the Rust standad library. These dependencies are managed using the Cargo package manager and are declared in your project's Cargo.toml file. External dependencies provide reusable functionality, simplifying development by offering solutions for common tasks. 

## Key Features of External Dependencies 
* Reusable Functionality: They encapsulate code that solve specific problems. 
* Version Management: add version number and name in your project's Cargo.toml, and import it to your workspace you can use it, just like the maven dependencies. 
* Open Source: many external crates are hosted and can be found on crates.io.


## Are Most of external dependencies provide their abilities via trait ? 
(This is coming from the GPT) Yes, **most but not all** external dependencies in Rust provide their additional functionality(or "external features") through traits, but not strict.  

## Advantages of using External Dependnecies 
* Reusability
* Focus on Core Logic
* Community Standard

## Consideration in Selecting Dependencies 
* Maintenance 
* Excessive Dependency 
* Understandability 