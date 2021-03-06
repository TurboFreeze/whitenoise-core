[![Build Status](https://travis-ci.org/opendifferentialprivacy/whitenoise-core.svg?branch=develop)](https://travis-ci.org/opendifferentialprivacy/whitenoise-core)

<a href="http://opendp.io"><img src="images/WhiteNoise Logo/SVG/Full_grey.svg" align="left" height="80" vspace="8" hspace="18"></a>

---

## WhiteNoise Core Differential Privacy
The WhiteNoise Core is built around a data representation for a statistical analysis. There are three types of projects:
- Validator: validates that an analysis is differentially private
- Runtime: execute analysis
- Bindings: helpers to create analysis

The runtime and bindings may be written in any language. The core data representation is in protobuf, and the validator is written in Rust. All projects implement protobuf code generation, protobuf serialization/deserialization, communication over FFI, handle distributable packaging, and have at some point compiled cross-platform (more testing needed). All projects communicate via proto definitions from the `prototypes` directory.  

#### Validator
The rust validator compiles to binaries that expose C foreign function interfaces and read/automatically generate code for protobuf. A validator C FFI is described in the wiki.  

#### Runtimes
The Rust runtime uses a package called ndarray, which feels somewhat like writing numpy in Rust.  

#### Bindings
There are two language bindings, one in Python, one in R. Both support building binaries into an installable package.  

The Python package is more developed, with helper classes, syntax sugar for building analyses, and visualizations.  

The R package uses a shim library in C to interface with compiled binaries. There isn't a programmer interface like in Python yet, but there is a pattern for exposing the C FFI in R code, as well as protobuf generation.  

The steps for adding bindings in a new language are essentially:  
1. set up package management  
2. set up dependency management  
3. pack binaries with the given language's tools  
4. protobuf code generation  
5. FFI implementation and protobuf encoding/decoding  
6. write programmer interface  


### Install
1. Clone the repository  

    git clone $REPOSITORY_URI
  
2. Install Rust

    Mac, Linux:
    
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        
    Close terminal and open new terminal to add cargo to path.
    You can test with `rustc --version`

3. Install protobuf compiler  
    Mac:  

        brew install protobuf  
        
    Ubuntu:  

        sudo snap install protobuf --classic  

    Windows:  

        choco install protoc  

* For non-Chocolatey users: download and install the latest build
  + https://github.com/protocolbuffers/protobuf/releases/latest


4. Install instructions for the bindings, validator and runtime are located in their respective folders.  

---

[WhiteNoise Rust Documentation](https://opendifferentialprivacy.github.io/whitenoise-core/)
 
